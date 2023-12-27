use proc_macro2::Span;
use syn::DeriveInput;

trait FromValueImpl {
    fn expand(self) -> proc_macro::TokenStream;
}

struct NamedFieldVisitorImpl {
    ident: proc_macro2::Ident,
    generics: syn::Generics,
    data: syn::Data,
    target: proc_macro2::Ident,
}

impl NamedFieldVisitorImpl {
    fn for_type(input: DeriveInput) -> Self {
        Self {
            ident: proc_macro2::Ident::new(
                &(input.ident.to_string() + "Visitor"),
                Span::call_site(),
            ),
            target: input.ident,
            generics: input.generics,
            data: input.data,
        }
    }

    fn ident(&self) -> &proc_macro2::Ident {
        &self.ident
    }

    fn definition(&self) -> proc_macro2::TokenStream {
        let visitor_ident = &self.ident;
        let (_, type_generics, where_clause) = self.generics.split_for_impl();
        let data = match &self.data {
            syn::Data::Struct(data) => data,
            _ => panic!("Expected struct, found enum or union"),
        };
        let fields = data.fields.iter().map(|field| {
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

        let data = match &self.data {
            syn::Data::Struct(data) => data,
            _ => panic!("Expected struct, found enum or union"),
        };
        let fields = data.fields.iter().map(|field| {
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

    fn expand(self) -> proc_macro2::TokenStream {
        let definition = self.definition();
        let into_target_impl = self.into_target_impl();
        let Self {
            ident: visitor_ident,
            generics,
            data,
            ..
        } = self;
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
        let data = match data {
            syn::Data::Struct(data) => data,
            _ => panic!("Expected struct, found enum or union"),
        };
        let arms = data.fields.iter().map(|field| {
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
            #definition
            #into_target_impl
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

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    StructFromValueImpl::new(input).expand()
}
