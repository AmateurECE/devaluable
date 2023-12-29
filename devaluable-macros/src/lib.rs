use proc_macro2::{Ident, Span};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Generics};

mod named_field_visitor;
mod unnamed_field_visitor;

use named_field_visitor::NamedFieldVisitor;
use unnamed_field_visitor::UnnamedFieldVisitor;

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
    fn make(
        &self,
        signature: proc_macro2::TokenStream,
        implementation: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let trait_name = quote::quote!(::valuable::Visit);
        let impl_statement = self.0.make_impl(&trait_name);
        quote::quote! {
            #impl_statement {
                fn visit_value(&mut self, _: ::valuable::Value<'_>) {
                    unreachable!()
                }

                #signature {
                    #implementation
                }
            }
        }
    }

    pub fn make_named_fields(
        &self,
        implementation: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        self.make(
            quote::quote! {
                fn visit_named_fields(&mut self, named_values: &::valuable::NamedValues<'_>)
            },
            implementation,
        )
    }

    pub fn make_unnamed_fields(
        &self,
        implementation: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        self.make(
            quote::quote! {
                fn visit_unnamed_fields(&mut self, values: &[::valuable::Value<'_>])
            },
            implementation,
        )
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

struct UnnamedStructImpl {
    visitor: UnnamedFieldVisitor,
    factory: StatementFactory,
}

impl FromValueImpl for UnnamedStructImpl {
    fn expand(&self) -> proc_macro::TokenStream {
        let target_type = self.factory.ident().to_string();
        let visitor_type = self.visitor.ident();
        let factory = FromValueImplFactory(&self.factory);
        let from_value_impl = factory.make(quote::quote! {
            if let ::valuable::Value::Structable(structable) = value {
                match structable.definition() {
                    ::valuable::StructDef::Static {
                        name: #target_type,
                        fields: ::valuable::Fields::Unnamed(_),
                        ..
                    } => {
                        let mut visitor: #visitor_type = ::core::default::Default::default();
                        structable.visit(&mut visitor);
                        Some(visitor.into())
                    }
                    _ => None,
                }
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

fn make_visitor_ident(ident: &Ident) -> Ident {
    Ident::new(&(ident.to_string() + "Visitor"), Span::call_site())
}

struct EnumImpl {
    factory: StatementFactory,
    generics: Generics,
    data: DataEnum,
}

impl FromValueImpl for EnumImpl {
    fn expand(&self) -> proc_macro::TokenStream {
        let variant_arms = self
            .data
            .variants
            .iter()
            .map(|variant| match &variant.fields {
                Fields::Named(_) => {
                    let name_string = variant.ident.to_string();
                    let visitor_ident = make_visitor_ident(&variant.ident);
                    quote::quote! {
                        (#name_string, ::valuable::Fields::Named(_)) => {
                            let mut visitor: #visitor_ident = ::core::default::Default::default();
                            enumerable.visit(&mut visitor);
                            Some(visitor.into())
                        }
                    }
                }
                Fields::Unnamed(_) => {
                    let name_string = variant.ident.to_string();
                    let visitor_ident = make_visitor_ident(&variant.ident);
                    quote::quote! {
                        (#name_string, ::valuable::Fields::Unnamed(_)) => {
                            let mut visitor: #visitor_ident = ::core::default::Default::default();
                            enumerable.visit(&mut visitor);
                            Some(visitor.into())
                        }
                    }
                }
                Fields::Unit => {
                    let variant_name = &variant.ident;
                    let name_string = variant.ident.to_string();
                    let enum_name = self.factory.ident();
                    quote::quote!((#name_string, _) => Some(#enum_name::#variant_name))
                }
            });

        let target_type = self.factory.ident().to_string();
        let factory = FromValueImplFactory(&self.factory);
        let from_value_impl = factory.make(quote::quote! {
            if let ::valuable::Value::Enumerable(enumerable) = value {
                if let (#target_type, ::valuable::Variant::Static(variant)) =
                    (enumerable.definition().name(), enumerable.variant())
                {
                    match (variant.name(), variant.fields()) {
                        #(#variant_arms ,)*
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        });

        let variant_impls = self.data.variants.iter().map(|variant| {
            let visitor_ident = make_visitor_ident(&variant.ident);
            match &variant.fields {
                Fields::Named(named) => {
                    let enum_ident = self.factory.ident();
                    let variant_ident = &variant.ident;
                    let visitor = NamedFieldVisitor::with_constructed_type(
                        named.clone(),
                        quote::quote!(#enum_ident),
                        quote::quote!(#enum_ident::#variant_ident),
                        StatementFactory::new(visitor_ident, self.generics.clone()),
                    );

                    let definition = visitor.definition();
                    let visit_impl = visitor.visit_impl();
                    let into_target_impl = visitor.into_target_impl();
                    quote::quote! {
                        #definition
                        #visit_impl
                        #into_target_impl
                    }
                }

                Fields::Unnamed(unnamed) => {
                    let enum_ident = self.factory.ident();
                    let variant_ident = &variant.ident;
                    let visitor = UnnamedFieldVisitor::with_constructed_type(
                        unnamed.clone(),
                        quote::quote!(#enum_ident),
                        quote::quote!(#enum_ident::#variant_ident),
                        StatementFactory::new(visitor_ident, self.generics.clone()),
                    );

                    let definition = visitor.definition();
                    let visit_impl = visitor.visit_impl();
                    let into_target_impl = visitor.into_target_impl();
                    quote::quote! {
                        #definition
                        #visit_impl
                        #into_target_impl
                    }
                }

                // No visitor for unit variants
                Fields::Unit => quote::quote!(),
            }
        });

        quote::quote! {
            #(#variant_impls)*
            #from_value_impl
        }
        .into()
    }
}

fn impl_factory(input: DeriveInput) -> Box<dyn FromValueImpl> {
    let target = &input.ident;
    let ident = Ident::new(&(input.ident.to_string() + "Visitor"), Span::call_site());
    let target_factory = StatementFactory::new(input.ident.clone(), input.generics.clone());
    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(data),
            ..
        }) => Box::new(NamedStructImpl {
            factory: target_factory.clone(),
            visitor: NamedFieldVisitor::new(
                data,
                quote::quote!(#target),
                StatementFactory::new(ident, input.generics),
            ),
        }),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(data),
            ..
        }) => Box::new(UnnamedStructImpl {
            visitor: UnnamedFieldVisitor::new(
                data,
                quote::quote!(#target),
                StatementFactory::new(ident, input.generics),
            ),
            factory: target_factory,
        }),
        Data::Enum(data) => Box::new(EnumImpl {
            factory: target_factory,
            generics: input.generics,
            data,
        }),
        _ => panic!("Unions are not supported"),
    }
}

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let from_value = impl_factory(input);
    from_value.expand()
}
