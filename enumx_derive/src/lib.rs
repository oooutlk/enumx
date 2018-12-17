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

            let named_variant_fx = data.variants.iter().map( |v| &v.ident );
            let named_variant_tx = data.variants.iter().map( |v| &v.ident );

            let variant_cnt = data.variants.len();

            let enumx = &ident( &format!( "Enum{}", variant_cnt ));

            let named_enum_fx = (0..variant_cnt).map( |_| name );
            let named_enum_tx = named_enum_fx.clone();

            let unamed_enum_fx = (0..variant_cnt).map( |_| enumx );
            let unamed_enum_tx = unamed_enum_fx.clone();

            let unamed_variant_fx = (0..variant_cnt).map( |index| ident( &format!( "_{}", index )));
            let unamed_variant_tx = unamed_variant_fx.clone();

            let ( ref impl_generics, ref ty_generics, ref where_clause ) = input.generics.split_for_impl();

            let mut generics_vi = input.generics.clone();
            add_generics( &mut generics_vi, &[ "Variant", "Index" ]);
            let ( impl_generics_vi, _, _ ) = generics_vi.split_for_impl();

            let mut generics_ix = input.generics.clone();
            add_generics( &mut generics_ix, &[ "Dest", "Indices" ]);
            let ( impl_generics_ix, _, _ ) = generics_ix.split_for_impl();

            let clause = where_clause.map( |where_clause| &where_clause.predicates );

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

            let expanded = quote! {
                impl #impl_generics Exchange for #name #ty_generics #where_clause {
                    type EnumX = #enumx_ty;
                }

                impl #impl_generics From<#enumx_ty> for #name #ty_generics #where_clause {
                    fn from( src: #enumx_ty ) -> Self {
                        match src {
                            #( #unamed_enum_fx::#unamed_variant_fx(v) => #named_enum_fx::#named_variant_fx(v), )*
                        }
                    }
                }

                impl #impl_generics Into<#enumx_ty> for #name #ty_generics #where_clause {
                    fn into( self ) -> #enumx_ty {
                        match self {
                            #( #named_enum_tx::#named_variant_tx(v) => #unamed_enum_tx::#unamed_variant_tx(v), )*
                        }
                    }
                }

                impl #impl_generics_vi FromVariant<Variant,Index> for #name #ty_generics
                    where Self      : From<#enumx_ty>
                        , #enumx_ty : FromVariant<Variant,Index>
                        , #(#clause)*
                {
                    fn from_variant( variant: Variant ) -> Self {
                        #name::from( #enumx::<#(#variant_ty),*>::from_variant( variant ))
                    }
                }

                impl #impl_generics_ix IntoEnumX<Dest,Indices> for #name #ty_generics
                    where Self      : Into<#enumx_ty>
                        , #enumx_ty : IntoEnumX<Dest,Indices>
                {
                    fn into_enumx( self ) -> Dest {
                        let enumx: #enumx_ty = self.into();
                        enumx.into_enumx()
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
