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

#[derive(Clone)]
struct StatementFactory {
    generics: Generics,
    ident: Ident,
}

impl StatementFactory {
    pub fn new(ident: Ident, generics: Generics) -> Self {
        Self { ident, generics }
    }

    pub fn ident(&self) -> &proc_macro2::Ident {
        &self.ident
    }

    pub fn make_definition(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let (_, type_generics, where_clause) = self.generics.split_for_impl();
        quote::quote! {
            struct #ident #type_generics #where_clause
        }
    }

    pub fn make_impl(&self, trait_name: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();
        quote::quote! {
            impl #impl_generics #trait_name for #ident #type_generics #where_clause
        }
    }
}

struct FromValueImplFactory<'a>(pub(crate) &'a StatementFactory);
impl<'a> FromValueImplFactory<'a> {
    pub fn make(&self, implementation: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let trait_name = quote::quote!(FromValue);
        let impl_statement = self.0.make_impl(&trait_name);
        quote::quote! {
            #impl_statement {
                fn from_value(value: ::valuable::Value) -> Option<Self> {
                    #implementation
                }
            }
        }
    }
}

struct IntoImplFactory<'a>(pub(crate) &'a StatementFactory);
impl<'a> IntoImplFactory<'a> {
    pub fn make(
        &self,
        target: &proc_macro2::TokenStream,
        implementation: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let trait_name = quote::quote!(Into<#target>);
        let impl_statement = self.0.make_impl(&trait_name);
        quote::quote! {
            #impl_statement {
                fn into(self) -> #target {
                    #implementation
                }
            }
        }
    }
}

struct VisitImplFactory<'a>(pub(crate) &'a StatementFactory);
impl<'a> VisitImplFactory<'a> {
    pub fn make_named_fields(
        &self,
        implementation: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let trait_name = quote::quote!(::valuable::Visit);
        let impl_statement = self.0.make_impl(&trait_name);
        quote::quote! {
            #impl_statement {
                fn visit_value(&mut self, _: ::valuable::Value<'_>) {
                    unreachable!()
                }

                fn visit_named_fields(&mut self, named_values: &::valuable::NamedValues<'_>) {
                    #implementation
                }
            }
        }
    }
}

struct NamedStructImpl {
    visitor: NamedFieldVisitor,
    factory: StatementFactory,
}

impl FromValueImpl for NamedStructImpl {
    fn expand(&self) -> proc_macro::TokenStream {
        let visitor_type = self.visitor.ident();
        let factory = FromValueImplFactory(&self.factory);
        let from_value_impl = factory.make(quote::quote! {
            if let ::valuable::Value::Structable(structable) = value {
                let mut visitor: #visitor_type = ::core::default::Default::default();
                structable.visit(&mut visitor);
                Some(visitor.into())
            } else {
                None
            }
        });

        let visitor_definition = self.visitor.definition();
        let visitor_impl = self.visitor.visit_impl();
        let visitor_into_impl = self.visitor.into_target_impl();

        proc_macro::TokenStream::from(quote::quote! {
            #visitor_definition
            #visitor_impl
            #visitor_into_impl
            #from_value_impl
        })
    }
}

fn impl_factory(input: DeriveInput) -> Box<dyn FromValueImpl> {
    let ident = Ident::new(&(input.ident.to_string() + "Visitor"), Span::call_site());
    let target_factory = StatementFactory::new(input.ident, input.generics.clone());
    let visitor_factory = StatementFactory::new(ident, input.generics);
    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(data),
            ..
        }) => Box::new(NamedStructImpl {
            factory: target_factory.clone(),
            visitor: NamedFieldVisitor {
                data,
                target_factory,
                visitor_factory,
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
