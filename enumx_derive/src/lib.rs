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
use syn::{parse_quote, DeriveInput, Generics, Ident};

#[proc_macro_derive( Exchange )]
pub fn derive_exchange( input: TokenStream ) -> TokenStream {
    let input: DeriveInput = syn::parse( input ).unwrap();

    match input.data {
        syn::Data::Enum( ref data ) => {
            let name_ex = input.ident.clone();
            let name_fx = input.ident.clone();
            let name_tx = input.ident.clone();
            let name_fv = input.ident;

            let named_variant_fx = data.variants.iter().map( |v| v.ident.clone() );
            let named_variant_tx = data.variants.iter().map( |v| v.ident.clone() );

            let variant_cnt = data.variants.len();

            let enumx_ex = ident( &format!( "Enum{}", variant_cnt ));
            let enumx_fx = enumx_ex.clone();
            let enumx_tx = enumx_fx.clone();
            let enumx_fv = enumx_fx.clone();

            let named_enum_fx = (0..variant_cnt).map( |_| name_fx.clone() );
            let named_enum_tx = named_enum_fx.clone();

            let unamed_enum_fx = (0..variant_cnt).map( |_| enumx_fx.clone() );
            let unamed_enum_tx = unamed_enum_fx.clone();

            let unamed_variant_fx = (0..variant_cnt).map( |index| ident( &format!( "_{}", index )));
            let unamed_variant_tx = unamed_variant_fx.clone();

            let ( impl_generics_ex, ty_generics_ex, where_clause_ex ) = input.generics.split_for_impl();
            let ( impl_generics_fx, ty_generics_fx, where_clause_fx ) = ( impl_generics_ex.clone(), ty_generics_ex.clone(), where_clause_ex.clone() );
            let ( impl_generics_tx, ty_generics_tx, where_clause_tx ) = ( impl_generics_fx.clone(), ty_generics_fx.clone(), where_clause_fx.clone() );
            let ( ty_generics_fv, where_clause_fv ) = ( ty_generics_fx.clone(), where_clause_fx.clone() );

            let mut generics = input.generics.clone();
            add_generics( &mut generics, &[ "Variant", "Index" ]);
            let ( impl_generics_fv, _, _ ) = generics.split_for_impl();

            let clause_fv = where_clause_fv .map( |where_clause_fv| &where_clause_fv.predicates );

            let expanded = quote! {
                impl #impl_generics_ex Exchange for #name_ex #ty_generics_ex #where_clause_ex {
                    type EnumX = #enumx_ex #ty_generics_ex;
                }

                impl #impl_generics_fx From<#enumx_fx #ty_generics_fx> for #name_fx #ty_generics_fx #where_clause_fx {
                    fn from( src: #enumx_fx #ty_generics_fx ) -> Self {
                        match src {
                            #( #unamed_enum_fx::#unamed_variant_fx(v) => #named_enum_fx::#named_variant_fx(v), )*
                        }
                    }
                }

                impl #impl_generics_tx From<#name_tx #ty_generics_tx> for #enumx_tx #ty_generics_tx #where_clause_tx {
                    fn from( src: #name_tx #ty_generics_tx ) -> Self {
                        match src {
                            #( #named_enum_tx::#named_variant_tx(v) => #unamed_enum_tx::#unamed_variant_tx(v), )*
                        }
                    }
                }

                impl #impl_generics_fv FromVariant<Variant,Index> for #name_fv #ty_generics_fv
                    where Self                      : From<#enumx_fv #ty_generics_fv>
                        , #enumx_fv #ty_generics_fv : FromVariant<Variant,Index>
                        , #(#clause_fv)*
                {
                    fn from_variant( variant: Variant ) -> Self {
                        #name_fv::from( #enumx_fv:: #ty_generics_fv::from_variant( variant ))
                    }
                }
            };
            expanded.into()
        },
        _ => panic!( "Only `enum`s can be `Exchange`." ),
    }
}

fn add_generics( generics: &mut Generics, names: &[&'static str] ) {
    for name in names {
        let name = ident( name );
        generics.params.push( parse_quote!( #name ));
    }
}

fn ident( sym: &str ) -> Ident {
    Ident::new( sym, Span::call_site() )
}
