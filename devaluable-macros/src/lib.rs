use proc_macro2::{Ident, Span};
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, Generics};

trait FromValueImpl {
    fn expand(&self) -> proc_macro::TokenStream;
}

trait VisitorImpl {
    fn ident(&self) -> &Ident;
    fn definition(&self) -> proc_macro2::TokenStream;
    fn into_target_impl(&self) -> proc_macro2::TokenStream;
    fn visit_impl(&self) -> proc_macro2::TokenStream;
}

struct NamedFieldVisitor {
    ident: Ident,
    generics: Generics,
    data: FieldsNamed,
    target: Ident,
}

impl VisitorImpl for NamedFieldVisitor {
    fn ident(&self) -> &Ident {
        &self.ident
    }

    fn definition(&self) -> proc_macro2::TokenStream {
        let visitor_ident = &self.ident;
        let (_, type_generics, where_clause) = self.generics.split_for_impl();
        let fields = self.data.named.iter().map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote::quote!(#ident: #ty)
        });

        quote::quote! {
            #[derive(Default)]
            struct #visitor_ident #type_generics #where_clause {
                #(#fields ,)*
            }
        }
    }

    fn into_target_impl(&self) -> proc_macro2::TokenStream {
        let visitor_type = &self.ident;
        let target_type = &self.target;
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        let fields = self.data.named.iter().map(|field| {
            let ident = &field.ident;
            quote::quote!(#ident: self.#ident)
        });

        quote::quote! {
            impl #impl_generics Into<#target_type> for #visitor_type #type_generics #where_clause {
                fn into(self) -> #target_type {
                    #target_type {
                        #(#fields ,)*
                    }
                }
            }
        }
    }

    fn visit_impl(&self) -> proc_macro2::TokenStream {
        let visitor_ident = &self.ident;
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();
        let arms = self.data.named.iter().map(|field| {
            let string = field
                .ident
                .as_ref()
                .expect("Unnamed fields are not supported")
                .to_string();
            let ident = &field.ident;
            let ty = &field.ty;
            quote::quote! {
                #string => {
                    let result: Option<#ty> = ::devaluable::FromValue::from_value(*value);
                    if let Some(#ident) = result {
                        self.#ident = #ident;
                    }
                }
            }
        });

        quote::quote! {
            impl #impl_generics ::valuable::Visit for #visitor_ident #type_generics
                #where_clause
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

struct NamedStructImpl {
    visitor: NamedFieldVisitor,
    target_type: Ident,
    generics: Generics,
}

impl FromValueImpl for NamedStructImpl {
    fn expand(&self) -> proc_macro::TokenStream {
        let target_type = &self.target_type;
        let visitor_type = self.visitor.ident();
        let visitor_definition = self.visitor.definition();
        let visitor_impl = self.visitor.visit_impl();
        let visitor_into_impl = self.visitor.into_target_impl();
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        let expanded = quote::quote! {
            #visitor_definition
            #visitor_impl
            #visitor_into_impl

            impl #impl_generics ::devaluable::FromValue for #target_type #type_generics
                #where_clause
            {
                fn from_value(value: ::valuable::Value) -> Option<Self> {
                    if let ::valuable::Value::Structable(structable) = value {
                        let mut visitor: #visitor_type = ::core::default::Default::default();
                        structable.visit(&mut visitor);
                        Some(visitor.into())
                    } else {
                        None
                    }
                }
            }
        };

        proc_macro::TokenStream::from(expanded)
    }
}

fn impl_factory(input: DeriveInput) -> Box<dyn FromValueImpl> {
    let ident = Ident::new(&(input.ident.to_string() + "Visitor"), Span::call_site());
    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(data),
            ..
        }) => Box::new(NamedStructImpl {
            target_type: input.ident.clone(),
            generics: input.generics.clone(),
            visitor: NamedFieldVisitor {
                ident,
                generics: input.generics,
                data,
                target: input.ident,
            },
        }),
        _ => panic!("Only structs with named fields are supported"),
    }
}

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let from_value = impl_factory(input);
    from_value.expand()
}
