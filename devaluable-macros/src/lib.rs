use proc_macro2::{Ident, Span};
use syn::{Data, DataStruct, DeriveInput, Fields, Generics};

mod named_field_visitor;

use named_field_visitor::NamedFieldVisitor;

trait FromValueImpl {
    fn expand(&self) -> proc_macro::TokenStream;
}

trait VisitorImpl {
    fn ident(&self) -> &Ident;
    fn definition(&self) -> proc_macro2::TokenStream;
    fn into_target_impl(&self) -> proc_macro2::TokenStream;
    fn visit_impl(&self) -> proc_macro2::TokenStream;
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
