// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

//! This project provides derive implementation for user-defined enum exchange.
//! 
//! See [enumx README](https://github.com/oooutlk/enumx/blob/master/enumx/README.md) for more.

#![recursion_limit="128"]

extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;

use syn::export::Span;
use syn::{parse_quote, DeriveInput, Ident};

macro_rules! syntax_error {
    () => {
        panic!( "`enum`s deriving `Exchange` should be in the form of \"enum MyEnum { Foo(Type), Bar(AnotherType),... }\"" );
    }
}

#[proc_macro_derive( Exchange )]
pub fn derive_exchange( input: TokenStream ) -> TokenStream {
    let input: DeriveInput = syn::parse( input ).unwrap();

    match input.data {
        syn::Data::Enum( ref data ) => {
            let name = &input.ident;

            let named_variant_fp = data.variants.iter().map( |v| &v.ident );
            let named_variant_tp = data.variants.iter().map( |v| &v.ident );

            let variant_cnt = data.variants.len();

            let enumx = &ident( &format!( "Enum{}", variant_cnt ));

            let named_enum_fp = (0..variant_cnt).map( |_| name );
            let named_enum_tp = named_enum_fp.clone();

            let unamed_enum_fp = (0..variant_cnt).map( |_| enumx );
            let unamed_enum_tp = unamed_enum_fp.clone();

            let unamed_variant_fp = (0..variant_cnt).map( |index| ident( &format!( "_{}", index )));
            let unamed_variant_tp = unamed_variant_fp.clone();

            let ( ref impl_generics, ref ty_generics, ref where_clause ) = input.generics.split_for_impl();

            let variant_ty = data.variants.iter().map( |ref v| {
                if let syn::Fields::Unnamed( ref fields ) = v.fields {
                    let mut iter = fields.unnamed.iter();
                    if iter.len() == 1 {
                        let field = iter.next().unwrap();
                        return &field.ty;
                    }
                }
                syntax_error!();
            });

            let enumx_ty: syn::Type = parse_quote!{ #enumx<#(#variant_ty),*> };

            let expanded = quote! {
                impl #impl_generics enumx::Exchange for #name #ty_generics #where_clause {
                    type Proto = #enumx_ty;

                    fn from_proto( src: #enumx_ty ) -> Self {
                        match src {
                            #( #unamed_enum_fp::#unamed_variant_fp(v) => #named_enum_fp::#named_variant_fp(v), )*
                        }
                    }

                    fn into_proto( self ) -> #enumx_ty {
                        match self {
                            #( #named_enum_tp::#named_variant_tp(v) => #unamed_enum_tp::#unamed_variant_tp(v), )*
                        }
                    }
                }
            };
            expanded.into()
        },
        _ => panic!( "Only `enum`s can be `Exchange`." ),
    }
}

fn ident( sym: &str ) -> Ident {
    Ident::new( sym, Span::call_site() )
}
