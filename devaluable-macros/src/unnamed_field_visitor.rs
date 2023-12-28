use syn::FieldsUnnamed;

use crate::{IntoImplFactory, StatementFactory, VisitImplFactory};

pub struct UnnamedFieldVisitor {
    pub(crate) data: FieldsUnnamed,
    pub(crate) target_factory: StatementFactory,
    pub(crate) visitor_factory: StatementFactory,
}

impl crate::VisitorImpl for UnnamedFieldVisitor {
    fn ident(&self) -> &proc_macro2::Ident {
        self.visitor_factory.ident()
    }

    fn definition(&self) -> proc_macro2::TokenStream {
        let types = self.data.unnamed.iter().map(|field| field.ty.clone());

        let definition = self.visitor_factory.make_definition();
        quote::quote! {
            #[derive(Default)]
            #definition ( #(#types ,)* );
        }
    }

    fn into_target_impl(&self) -> proc_macro2::TokenStream {
        let fields = (0..self.data.unnamed.len()).map(|index| {
            let index = syn::Index::from(index);
            quote::quote!(self.#index)
        });

        let target_type = self.target_factory.ident();
        let factory = IntoImplFactory(&self.visitor_factory);
        factory.make(
            &quote::quote!(#target_type),
            quote::quote! {
                #target_type (
                    #(#fields ,)*
                )
            },
        )
    }

    fn visit_impl(&self) -> proc_macro2::TokenStream {
        let statements = (0..self.data.unnamed.len()).map(|index| {
            let index = syn::Index::from(index);
            quote::quote! {
                self.#index = iter
                    .next()
                    .and_then(|value| ::devaluable::FromValue::from_value(*value))
                    .unwrap_or(::core::default::Default::default());
            }
        });

        let factory = VisitImplFactory(&self.visitor_factory);
        factory.make_unnamed_fields(quote::quote! {
            let mut iter = values.iter();
            #(#statements )*
        })
    }
}
