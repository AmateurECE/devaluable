use syn::FieldsNamed;

use crate::{IntoImplFactory, StatementFactory, VisitImplFactory};

pub struct NamedFieldVisitor {
    data: FieldsNamed,
    target_type: proc_macro2::TokenStream,
    construted_type: proc_macro2::TokenStream,
    visitor_factory: StatementFactory,
}

impl NamedFieldVisitor {
    pub fn new(
        data: FieldsNamed,
        target_type: proc_macro2::TokenStream,
        visitor_factory: StatementFactory,
    ) -> Self {
        Self::with_constructed_type(data, target_type.clone(), target_type, visitor_factory)
    }

    pub fn with_constructed_type(
        data: FieldsNamed,
        target_type: proc_macro2::TokenStream,
        construted_type: proc_macro2::TokenStream,
        visitor_factory: StatementFactory,
    ) -> Self {
        Self {
            data,
            construted_type,
            target_type,
            visitor_factory,
        }
    }
}

impl crate::VisitorImpl for NamedFieldVisitor {
    fn ident(&self) -> &proc_macro2::Ident {
        self.visitor_factory.ident()
    }

    fn definition(&self) -> proc_macro2::TokenStream {
        let definition = self.visitor_factory.make_definition();
        let fields = self.data.named.iter().map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote::quote!(#ident: #ty)
        });

        quote::quote! {
            #[derive(Default)]
            #definition {
                #(#fields ,)*
            }
        }
    }

    fn into_target_impl(&self) -> proc_macro2::TokenStream {
        let fields = self.data.named.iter().map(|field| {
            let ident = &field.ident;
            quote::quote!(#ident: self.#ident)
        });

        let target_type = &self.target_type;
        let construted_type = &self.construted_type;
        let factory = IntoImplFactory(&self.visitor_factory);
        factory.make(
            &quote::quote!(#target_type),
            quote::quote! {
                #construted_type {
                    #(#fields ,)*
                }
            },
        )
    }

    fn visit_impl(&self) -> proc_macro2::TokenStream {
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

        let factory = VisitImplFactory(&self.visitor_factory);
        factory.make_named_fields(quote::quote! {
            named_values
                .iter()
                .for_each(|(field, value)| match field.name() {
                    #(#arms)*
                    _ => {}
                });
        })
    }
}
