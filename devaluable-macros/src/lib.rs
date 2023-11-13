use syn::DeriveInput;

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;

    let expanded = quote::quote! {
        impl #impl_generics crate::FromValue for #name #type_generics
            #where_clause
        {
            fn from_value(value: Value) -> Self {
                unimplemented!()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
