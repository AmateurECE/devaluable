use proc_macro2::Span;
use syn::{DeriveInput, Lifetime, LifetimeParam};

trait FromValueImpl {
    fn expand(self) -> proc_macro::TokenStream;
}

struct NamedFieldVisitorImpl {
    ident: proc_macro2::Ident,
    target_type: proc_macro2::Ident,
    generics: syn::Generics,
    data: syn::Data,
}

impl NamedFieldVisitorImpl {
    fn for_type(input: DeriveInput) -> Self {
        Self {
            ident: proc_macro2::Ident::new(
                &(input.ident.to_string() + "Visitor"),
                Span::call_site(),
            ),
            target_type: input.ident,
            generics: input.generics,
            data: input.data,
        }
    }

    fn ident(&self) -> &proc_macro2::Ident {
        &self.ident
    }

    fn signatures(&self) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let lifetime = Lifetime::new("'refvisitor", Span::call_site());
        let mut visitor_generics = self.generics.clone();
        visitor_generics
            .params
            .push(syn::GenericParam::Lifetime(LifetimeParam::new(
                lifetime.clone(),
            )));
        let (impl_generics, visitor_generics, where_clause) = visitor_generics.split_for_impl();

        let target_ident = &self.target_type;
        let visitor_ident = &self.ident;
        let (_, type_generics, _) = self.generics.split_for_impl();

        let definition = quote::quote! {
            struct #visitor_ident #visitor_generics (& #lifetime mut #target_ident #type_generics)
                #where_clause;
        };
        let impl_expression = quote::quote! {
            impl #impl_generics ::valuable::Visit for #visitor_ident #visitor_generics
                #where_clause
        };
        (definition, impl_expression)
    }

    fn expand(self) -> proc_macro2::TokenStream {
        let data = match &self.data {
            syn::Data::Struct(data) => data,
            _ => panic!("Expected struct, found enum or union"),
        };
        let arms = data.fields.iter().map(|field| {
            let string = field
                .ident
                .as_ref()
                .expect("Unnamed fields are not supported")
                .to_string();
            let name = &field.ident;
            let field_type = &field.ty;
            quote::quote! {
                #string => {
                    let result: Option<#field_type> = ::devaluable::FromValue::from_value(*value);
                    if let Some(#name) = result {
                        self.0.#name = #name;
                    }
                }
            }
        });

        let (definition, impl_expression) = self.signatures();
        quote::quote! {
            #definition
            #impl_expression
            {
                fn visit_value(&mut self, _: ::valuable::Value<'_>) {
                    unreachable!()
                }

                fn visit_named_fields(&mut self, named_values: &::valuable::NamedValues<'_>) {
                    named_values
                        .iter()
                        .for_each(|(field, value)| match field.name() {
                            #(#arms)*
                            _ => {}
                        });
                }
            }
        }
    }
}

struct StructFromValueImpl {
    visitor: NamedFieldVisitorImpl,
    target_type: proc_macro2::Ident,
    generics: syn::Generics,
}

impl StructFromValueImpl {
    fn new(input: DeriveInput) -> Self {
        Self {
            target_type: input.ident.clone(),
            generics: input.generics.clone(),
            visitor: NamedFieldVisitorImpl::for_type(input),
        }
    }
}

impl FromValueImpl for StructFromValueImpl {
    fn expand(self) -> proc_macro::TokenStream {
        let Self {
            visitor,
            generics,
            target_type,
        } = self;
        let visitor_type = visitor.ident().clone();
        let visitor_definition = visitor.expand();
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let expanded = quote::quote! {
            #visitor_definition

            impl #impl_generics ::devaluable::FromValue for #target_type #type_generics
                #where_clause
            {
                fn from_value(value: ::valuable::Value) -> Option<Self> {
                    if let ::valuable::Value::Structable(structable) = value {
                        let mut result = Self::default();
                        let mut visitor = #visitor_type (&mut result);
                        structable.visit(&mut visitor);
                        Some(result)
                    } else {
                        None
                    }
                }
            }
        };

        proc_macro::TokenStream::from(expanded)
    }
}

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    StructFromValueImpl::new(input).expand()
}
