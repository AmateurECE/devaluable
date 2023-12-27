use syn::{FieldsNamed, Generics};

pub struct NamedFieldVisitor {
    pub(crate) ident: proc_macro2::Ident,
    pub(crate) generics: Generics,
    pub(crate) data: FieldsNamed,
    pub(crate) target: proc_macro2::Ident,
}

impl crate::VisitorImpl for NamedFieldVisitor {
    fn ident(&self) -> &proc_macro2::Ident {
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
