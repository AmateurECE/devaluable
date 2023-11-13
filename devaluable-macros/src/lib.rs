use proc_macro2::Span;
use syn::{DeriveInput, LifetimeParam, Lifetime};

fn collector_struct(input: DeriveInput) -> proc_macro2::TokenStream {
    let collector_name = proc_macro2::Ident::new(&(input.ident.to_string() + "Collector"), Span::call_site());
    let name = &input.ident;
    let mut collector_generics = input.generics.clone();
    let lifetime = Lifetime::new("'refcollector", Span::call_site());
    collector_generics.params.push(syn::GenericParam::Lifetime(LifetimeParam::new(lifetime.clone())));
    let (_, collector_generics, where_clause) = collector_generics.split_for_impl();
    let (_, type_generics, _) = input.generics.split_for_impl();

    quote::quote! {
        struct #collector_name #collector_generics (& #lifetime mut #name #type_generics)
            #where_clause;
    }
}

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let collector = collector_struct(input.clone());
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident.clone();

    let expanded = quote::quote! {
        #collector
        impl #impl_generics crate::FromValue for #name #type_generics
            #where_clause
        {
            fn from_value(value: ::valuable::Value) -> Option<Self> {
                unimplemented!()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
