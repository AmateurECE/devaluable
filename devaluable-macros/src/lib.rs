use proc_macro2::Span;
use syn::{DeriveInput, Lifetime, LifetimeParam};

struct Collector {
    definitions: proc_macro2::TokenStream,
    name: proc_macro2::Ident,
}

impl Collector {
    fn new(input: DeriveInput) -> Self {
        let collector_name =
            proc_macro2::Ident::new(&(input.ident.to_string() + "Collector"), Span::call_site());
        let name = &input.ident;
        let mut collector_generics = input.generics.clone();
        let lifetime = Lifetime::new("'refcollector", Span::call_site());
        collector_generics
            .params
            .push(syn::GenericParam::Lifetime(LifetimeParam::new(
                lifetime.clone(),
            )));
        let (impl_generics, collector_generics, where_clause) = collector_generics.split_for_impl();
        let (_, type_generics, _) = input.generics.split_for_impl();

        let value = proc_macro2::Ident::new("value", Span::call_site());
        let arms = Collector::match_clauses(input.data, quote::quote!(*#value));
        Self {
            definitions: quote::quote! {
                struct #collector_name #collector_generics (& #lifetime mut #name #type_generics)
                    #where_clause;
                impl #impl_generics ::valuable::Visit for #collector_name #collector_generics
                    #where_clause
                {
                    fn visit_value(&mut self, _: ::valuable::Value<'_>) {
                        unreachable!()
                    }

                    fn visit_named_fields(&mut self, named_values: &::valuable::NamedValues<'_>) {
                        named_values.iter().for_each(|(field, #value)| {
                            match field.name() {
                                #arms
                                _ => {}
                            }
                        });
                    }
                }
            },
            name: collector_name,
        }
    }

    fn match_clauses(data: syn::Data, value: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match data {
            syn::Data::Struct(data_struct) => {
                let arms = data_struct.fields.iter().map(|field| {
                    let string = field
                        .ident
                        .as_ref()
                        .expect("Unnamed fields are not supported")
                        .to_string();
                    let name = &field.ident;
                    let field_type = &field.ty;
                    quote::quote! {
                        #string => {
                            if let Some(#name) = #field_type::from_value(#value) {
                                self.0.#name = #name;
                            }
                        }
                    }
                });

                quote::quote! {
                    #(#arms)*
                }
            }
            syn::Data::Enum(_) => todo!(),
            syn::Data::Union(_) => todo!(),
        }
    }
}

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let collector = Collector::new(input.clone());
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident.clone();

    let collector_definitions = collector.definitions;
    let collector_name = collector.name;
    let expanded = quote::quote! {
        #collector_definitions

        impl #impl_generics crate::FromValue for #name #type_generics
            #where_clause
        {
            fn from_value(value: ::valuable::Value) -> Option<Self> {
                if let ::valuable::Value::Structable(structable) = value {
                    let mut result = Self::default();
                    let mut collector = #collector_name (&mut result);
                    structable.visit(&mut collector);
                    Some(result)
                } else {
                    None
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
