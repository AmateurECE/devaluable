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
                        unimplemented!()
                    }
                }
            },
            name: collector_name,
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
