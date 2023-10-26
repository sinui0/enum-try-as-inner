// Copyright 2015-2018 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![doc = include_str!("../README.md")]
#![warn(
    clippy::default_trait_access,
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::unimplemented,
    clippy::use_self,
    missing_copy_implementations,
    missing_docs,
    non_snake_case,
    non_upper_case_globals,
    rust_2018_idioms,
    unreachable_pub
)]

use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DeriveInput, Visibility};

/// returns first the types to return, the match names, and then tokens to the field accesses
fn unit_fields_return(
    variant_name: &syn::Ident,
    err_name: &syn::Ident,
    ty_generics: &syn::TypeGenerics<'_>,
    (function_name_is, doc_is): (&Ident, &str),
    (function_name_ref, doc_ref): (&Ident, &str),
    (function_name_val, doc_val): (&Ident, &str),
) -> TokenStream {
    quote!(
        #[doc = #doc_is]
        #[inline]
        pub fn #function_name_is(&self) -> bool {
            matches!(self, Self::#variant_name)
        }

        #[doc = #doc_ref ]
        #[inline]
        pub fn #function_name_ref(&self) -> ::core::result::Result<&(), #err_name #ty_generics> {
            match self {
                Self::#variant_name => {
                    ::core::result::Result::Ok(&())
                }
                _ => {
                    ::core::result::Result::Err(#err_name::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::None,
                    ))
                }
            }
        }

        #[doc = #doc_val ]
        #[inline]
        pub fn #function_name_val(self) -> ::core::result::Result<(), #err_name #ty_generics> {
            match self {
                Self::#variant_name => {
                    ::core::result::Result::Ok(())
                }
                _ => {
                    ::core::result::Result::Err(#err_name::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::Some(self),
                    ))
                }
            }
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
#[allow(clippy::too_many_arguments)]
fn unnamed_fields_return(
    variant_name: &syn::Ident,
    err_name: &syn::Ident,
    ty_generics: &syn::TypeGenerics<'_>,
    (function_name_is, doc_is): (&Ident, &str),
    (function_name_mut_ref, doc_mut_ref): (&Ident, &str),
    (function_name_ref, doc_ref): (&Ident, &str),
    (function_name_val, doc_val): (&Ident, &str),
    fields: &syn::FieldsUnnamed,
) -> TokenStream {
    let (returns_mut_ref, returns_ref, returns_val, matches) = match fields.unnamed.len() {
        1 => {
            let field = fields.unnamed.first().expect("no fields on type");

            let returns = &field.ty;
            let returns_mut_ref = quote!(&mut #returns);
            let returns_ref = quote!(&#returns);
            let returns_val = quote!(#returns);
            let matches = quote!(inner);

            (returns_mut_ref, returns_ref, returns_val, matches)
        }
        0 => (quote!(()), quote!(()), quote!(()), quote!()),
        _ => {
            let mut returns_mut_ref = TokenStream::new();
            let mut returns_ref = TokenStream::new();
            let mut returns_val = TokenStream::new();
            let mut matches = TokenStream::new();

            for (i, field) in fields.unnamed.iter().enumerate() {
                let rt = &field.ty;
                let match_name = Ident::new(&format!("match_{}", i), Span::call_site());
                returns_mut_ref.extend(quote!(&mut #rt,));
                returns_ref.extend(quote!(&#rt,));
                returns_val.extend(quote!(#rt,));
                matches.extend(quote!(#match_name,));
            }

            (
                quote!((#returns_mut_ref)),
                quote!((#returns_ref)),
                quote!((#returns_val)),
                quote!(#matches),
            )
        }
    };

    quote!(
        #[doc = #doc_is ]
        #[inline]
        #[allow(unused_variables)]
        pub fn #function_name_is(&self) -> bool {
            matches!(self, Self::#variant_name(#matches))
        }

        #[doc = #doc_mut_ref ]
        #[inline]
        pub fn #function_name_mut_ref(&mut self) -> ::core::result::Result<#returns_mut_ref, #err_name #ty_generics> {
            match self {
                Self::#variant_name(#matches) => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_name::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::None,
                    ))
                }
            }
        }

        #[doc = #doc_ref ]
        #[inline]
        pub fn #function_name_ref(&self) -> ::core::result::Result<#returns_ref, #err_name #ty_generics> {
            match self {
                Self::#variant_name(#matches) => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_name::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::None,
                    ))
                }
            }
        }

        #[doc = #doc_val ]
        #[inline]
        pub fn #function_name_val(self) -> ::core::result::Result<#returns_val, #err_name #ty_generics> {
            match self {
                Self::#variant_name(#matches) => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_name::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::Some(self),
                    ))
                }
            }
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
#[allow(clippy::too_many_arguments)]
fn named_fields_return(
    variant_name: &syn::Ident,
    err_name: &syn::Ident,
    ty_generics: &syn::TypeGenerics<'_>,
    (function_name_is, doc_is): (&Ident, &str),
    (function_name_mut_ref, doc_mut_ref): (&Ident, &str),
    (function_name_ref, doc_ref): (&Ident, &str),
    (function_name_val, doc_val): (&Ident, &str),
    fields: &syn::FieldsNamed,
) -> TokenStream {
    let (returns_mut_ref, returns_ref, returns_val, matches) = match fields.named.len() {
        1 => {
            let field = fields.named.first().expect("no fields on type");
            let match_name = field.ident.as_ref().expect("expected a named field");

            let returns = &field.ty;
            let returns_mut_ref = quote!(&mut #returns);
            let returns_ref = quote!(&#returns);
            let returns_val = quote!(#returns);
            let matches = quote!(#match_name);

            (returns_mut_ref, returns_ref, returns_val, matches)
        }
        0 => (quote!(()), quote!(()), quote!(()), quote!(())),
        _ => {
            let mut returns_mut_ref = TokenStream::new();
            let mut returns_ref = TokenStream::new();
            let mut returns_val = TokenStream::new();
            let mut matches = TokenStream::new();

            for field in fields.named.iter() {
                let rt = &field.ty;
                let match_name = field.ident.as_ref().expect("expected a named field");

                returns_mut_ref.extend(quote!(&mut #rt,));
                returns_ref.extend(quote!(&#rt,));
                returns_val.extend(quote!(#rt,));
                matches.extend(quote!(#match_name,));
            }

            (
                quote!((#returns_mut_ref)),
                quote!((#returns_ref)),
                quote!((#returns_val)),
                quote!(#matches),
            )
        }
    };

    quote!(
        #[doc = #doc_is ]
        #[inline]
        #[allow(unused_variables)]
        pub fn #function_name_is(&self) -> bool {
            matches!(self, Self::#variant_name{ #matches })
        }

        #[doc = #doc_mut_ref ]
        #[inline]
        pub fn #function_name_mut_ref(&mut self) -> ::core::result::Result<#returns_mut_ref, #err_name #ty_generics> {
            match self {
                Self::#variant_name{ #matches } => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_name::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::None,
                    ))
                }
            }
        }

        #[doc = #doc_ref ]
        #[inline]
        pub fn #function_name_ref(&self) -> ::core::result::Result<#returns_ref, #err_name #ty_generics> {
            match self {
                Self::#variant_name{ #matches } => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_name::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::None,
                    ))
                }
            }
        }

        #[doc = #doc_val ]
        #[inline]
        pub fn #function_name_val(self) -> ::core::result::Result<#returns_val, #err_name #ty_generics> {
            match self {
                Self::#variant_name{ #matches } => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_name::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::Some(self),
                    ))
                }
            }
        }
    )
}

fn impl_all_as_fns(
    name: &Ident,
    err_name: &Ident,
    generics: &syn::Generics,
    data: &DataEnum,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut stream = TokenStream::new();
    let mut variant_names = TokenStream::new();
    for variant_data in &data.variants {
        let variant_name = &variant_data.ident;
        let function_name_ref = Ident::new(
            &format!("try_as_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_ref = format!(
            "Returns references to the inner fields if this is a `{}::{}`, otherwise an `{}`",
            name, variant_name, &err_name,
        );
        let function_name_mut_ref = Ident::new(
            &format!("try_as_{}_mut", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_mut_ref = format!(
            "Returns mutable references to the inner fields if this is a `{}::{}`, otherwise an `{}`",
            name,
            variant_name,
            &err_name,
        );

        let function_name_val = Ident::new(
            &format!("try_into_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_val = format!(
            "Returns the inner fields if this is a `{}::{}`, otherwise returns back the enum in the `Err` case of the result",
            name,
            variant_name,
        );

        let function_name_is = Ident::new(
            &format!("is_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_is = format!(
            "Returns true if this is a `{}::{}`, otherwise false",
            name, variant_name,
        );

        let tokens = match &variant_data.fields {
            syn::Fields::Unit => unit_fields_return(
                variant_name,
                err_name,
                &ty_generics,
                (&function_name_is, &doc_is),
                (&function_name_ref, &doc_ref),
                (&function_name_val, &doc_val),
            ),
            syn::Fields::Unnamed(unnamed) => unnamed_fields_return(
                variant_name,
                err_name,
                &ty_generics,
                (&function_name_is, &doc_is),
                (&function_name_mut_ref, &doc_mut_ref),
                (&function_name_ref, &doc_ref),
                (&function_name_val, &doc_val),
                unnamed,
            ),
            syn::Fields::Named(named) => named_fields_return(
                variant_name,
                err_name,
                &ty_generics,
                (&function_name_is, &doc_is),
                (&function_name_mut_ref, &doc_mut_ref),
                (&function_name_ref, &doc_ref),
                (&function_name_val, &doc_val),
                named,
            ),
        };

        stream.extend(tokens);

        let variant_name = match &variant_data.fields {
            syn::Fields::Unit => quote!(Self::#variant_name => stringify!(#variant_name),),
            syn::Fields::Unnamed(_) => {
                quote!(Self::#variant_name(..) => stringify!(#variant_name),)
            }
            syn::Fields::Named(_) => quote!(Self::#variant_name{..} => stringify!(#variant_name),),
        };

        variant_names.extend(variant_name);
    }

    quote!(
        impl #impl_generics #name #ty_generics #where_clause {
            #stream

            /// Returns the name of the variant.
            fn variant_name(&self) -> &'static str {
                match self {
                    #variant_names
                    _ => unreachable!(),
                }
            }
        }
    )
}

fn impl_err(
    name: &Ident,
    err_name: &Ident,
    vis: &Visibility,
    generics: &syn::Generics,
    attrs: &[syn::Attribute],
) -> TokenStream {
    let doc_err = format!("An error type for the `{}::try_as_*` functions", name);

    // get the derives for the error type
    let mut derives = Vec::new();
    let mut derive_debug = false;
    for attr in attrs {
        if attr.path().is_ident("derive_err") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("Debug") {
                    derive_debug = true;
                } else {
                    derives.push(meta.path);
                }

                Ok(())
            })
            .expect("failed to parse derive nested meta");
        }
    }

    let derive_err = if derives.is_empty() {
        quote!()
    } else {
        quote!(#[derive(#(#derives),*)])
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut err_impl = quote!(
        #[doc = #doc_err ]
        #derive_err
        #vis struct #err_name #generics {
            expected: &'static str,
            actual: &'static str,
            value: ::core::option::Option<#name #ty_generics>,
        }

        impl #impl_generics #err_name #ty_generics #where_clause {
            /// Creates a new error indicating the expected variant and the actual variant.
            fn new(
                expected: &'static str,
                actual: &'static str,
                value: ::core::option::Option<#name #ty_generics>
            ) -> Self {
                Self {
                    expected,
                    actual,
                    value,
                }
            }

            /// Returns the name of the variant that was expected.
            pub fn expected(&self) -> &'static str {
                self.expected
            }

            /// Returns the name of the actual variant.
            pub fn actual(&self) -> &'static str {
                self.actual
            }

            /// Returns a reference to the actual value, if present.
            pub fn value(&self) -> ::core::option::Option<&#name #ty_generics> {
                self.value.as_ref()
            }

            /// Returns the actual value, if present.
            pub fn into_value(self) -> ::core::option::Option<#name #ty_generics> {
                self.value
            }
        }
    );

    if derive_debug {
        let impl_debug_body = {
            let where_clause = if let Some(where_clause) = where_clause {
                quote!(#where_clause, #name #ty_generics: ::core::fmt::Debug)
            } else {
                quote!(where #name #ty_generics: ::core::fmt::Debug)
            };

            quote!(
                impl #impl_generics ::core::fmt::Debug for #err_name #ty_generics #where_clause {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        f.debug_struct(stringify!(#err_name))
                            .field("expected", &self.expected)
                            .field("actual", &self.actual)
                            .field("value", &self.value)
                            .finish()
                    }
                }
            )
        };

        let impl_display_body = {
            let display_fmt = format!("expected {name}::{{}}, but got {name}::{{}}");
            quote!(
                impl #impl_generics ::core::fmt::Display for #err_name #ty_generics #where_clause {
                    fn fmt(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        write!(
                            formatter,
                            #display_fmt,
                            self.expected(),
                            self.actual(),
                        )
                    }
                }
            )
        };

        let impl_err_body = {
            let where_clause = if let Some(where_clause) = where_clause {
                quote!(#where_clause, #name #ty_generics: ::core::fmt::Debug)
            } else {
                quote!(where #name #ty_generics: ::core::fmt::Debug)
            };

            quote!(
                impl #impl_generics ::std::error::Error for #err_name #ty_generics #where_clause {}
            )
        };

        err_impl.extend(quote!(
            #impl_debug_body

            #impl_display_body

            #impl_err_body
        ))
    }

    err_impl
}

/// Derive functions on an Enum for easily accessing individual items in the Enum
#[proc_macro_derive(EnumTryAsInner, attributes(derive_err))]
pub fn enum_try_as_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // get a usable token stream
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let err_name = Ident::new(&format!("{}Error", name), Span::call_site());
    let generics = &ast.generics;
    let vis = &ast.vis;

    let enum_data = if let syn::Data::Enum(data) = &ast.data {
        data
    } else {
        panic!("{} is not an enum", name);
    };

    let mut expanded = TokenStream::new();

    // Build the impl
    let fns = impl_all_as_fns(name, &err_name, generics, enum_data);

    // Build the error
    let err = impl_err(name, &err_name, vis, generics, &ast.attrs);

    expanded.extend(fns);
    expanded.extend(err);

    proc_macro::TokenStream::from(expanded)
}
