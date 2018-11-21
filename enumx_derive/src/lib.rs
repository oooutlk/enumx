// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

#![recursion_limit="128"]

extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;

use syn::export::Span;
use syn::fold::Fold;
use syn::{parse_macro_input, parse_quote, Block, DeriveInput, Expr, GenericArgument, Ident, ItemFn, Pat, ReturnType, Path, PathArguments, Type, TypePath};

#[derive( Default )]
struct EnumxFn {
    closure_type : Option<Type>,
    enumx_path   : Option<Path>,
    enum_types   : Vec<Type>,
}

macro_rules! syntax_err {
    () => {
        panic!( "#[enumx] functions should return something like `Enum_<_,..>`." );
    }
}

// gets mutable references of type `ty`'s generic types
fn mut_generics<'a>( ty: &'a mut Type ) -> impl Iterator<Item=&'a mut Type> {
    if let Type::Path( TypePath{ qself:_, ref mut path }) = ty {
        if let Some( segment ) = path.segments.iter_mut().last() {
            if let PathArguments::AngleBracketed( ref mut generic ) = segment.arguments {
                return generic.args.iter_mut().filter_map( |arg| {
                    if let GenericArgument::Type( ref mut ty ) = *arg {
                        Some( ty )
                    } else {
                        None
                    }
                });
            }
        }
    }
    syntax_err!();
}

impl Fold for EnumxFn {
    // Does not modify return type but gathers type infomation.
    fn fold_return_type( &mut self, ret: ReturnType ) -> ReturnType {
        if let ReturnType::Type( _, mut closure_type ) = ret.clone() {
            self.enumx_path = if let Type::Path( TypePath{ qself:_, path }) = &*closure_type { Some( path.clone() )} else { None };
            self.enum_types = mut_generics( &mut closure_type ).map( |err| err.clone() ).collect::<Vec<_>>();

            let concrete_types = self.enum_types.iter();
            *closure_type = parse_quote!( __EnumX<#(#concrete_types),*> );
            self.closure_type = Some( *closure_type );
        }
        ret
    }

    // Here is the magic.
    fn fold_block( &mut self, mut block: Block ) -> Block {
        let closure_type = self.closure_type.take().expect( "closure must be of some type." );
        let enumx_path = self.enumx_path.take().expect( "enumx must be of some path." );
        let enumx_name = enumx_path.segments.iter().last().unwrap_or_else( || syntax_err!() ).ident.to_string();

        let enum_cnt = if &enumx_name[..4] == "Enum" {
            block = EnumxBlock.fold_block( block );
            enumx_name[4..].parse::<usize>().expect( "Type name should start with \"Enum\" and followed by an unsigned integer." )
        } else {
            syntax_err!();
        };

        fn make_generics( count: usize ) -> syn::Generics {
            syn::parse_str::<syn::Generics>(
                &format!( "<{}>",
                    (1..count).fold(
                        String::from( "E0"), |acc,index| format!( "{},E{}", acc, index ))))
                .expect("Generics must be something like <E0,E1,...>")
        }

        let generics = make_generics( enum_cnt );
        let ( _, ty_generics, _ ) = generics.split_for_impl();

        let concrete_enums = self.enum_types.iter();

        let impl_from_concrete_enums = self.enum_types.iter().enumerate().fold( Vec::<syn::ItemImpl>::new(), |mut impls,(index,enum_type)| {
            let variant = ident( &format!( "_{}", index ));
            let concrete_enums = concrete_enums.clone();
            impls.push( parse_quote!{
                impl From<#enum_type> for __EnumX<#(#concrete_enums),*> {
                    fn from( e: #enum_type ) -> Self {
                        __EnumX( <#enumx_path>::#variant( e ))
                    }
                }
            });
            impls
        });

        let impl_from_enumx_types = (1..=enum_cnt).fold( Vec::<syn::ItemImpl>::new(), |mut acc,index| {
            let concrete_enums = self.enum_types.iter();
            let current_enumx_name = format!( "Enum{}", index );
            let current_enumx_name = ident( &current_enumx_name );

            let generics = make_generics( index );
            let ( impl_generics, ty_generics, _ ) = generics.split_for_impl();

            acc.push( syn::ItemImpl::from(
                parse_quote! {
                    impl #impl_generics From<#current_enumx_name #ty_generics> for __EnumX<#(#concrete_enums),*>
                        where Self: From<<#current_enumx_name #ty_generics as EnumX>::LR>
                    {
                        fn from( src: #current_enumx_name #ty_generics ) -> Self {
                            __EnumX::from( src.into_lr() )
                        }
                    }
                }
            ));
            acc
        });

        let enumx_name = ident( &enumx_name );
        let concrete_errors_ = concrete_enums.clone();

        parse_quote! {{
            use enumx::*;
            struct __EnumX #ty_generics ( #enumx_name #ty_generics );

            impl From<Enum0> for __EnumX<#(#concrete_enums),*> {
                fn from( _: Enum0 ) -> Self {
                    unreachable!()
                }
            }

            #( #impl_from_concrete_enums )*

            impl<L,R> From<LR<L,R>> for __EnumX<#(#concrete_errors_),*>
                where L : Into<Self>
                    , R : Into<Self>
            {
                fn from( src: LR<L,R> ) -> Self {
                    match src {
                        LR::L( lhs ) => lhs.into(),
                        LR::R( rhs ) => rhs.into(),
                    }
                }
            }

            #( #impl_from_enumx_types )*
 
            let __enumx_result = {
                let mut _enumx_closure = || -> #closure_type { #block };
                _enumx_closure()
            };
            __enumx_result.0
        }}
    }
}

struct EnumxBlock;

impl Fold for EnumxBlock { 
    fn fold_expr( &mut self, expr: Expr ) -> Expr {
        match expr {
            Expr::Return( _ ) => {
                parse_quote!{{ 
                    __EnumX::from( #expr )
                }}
            },
            Expr::Try( _ ) => {
                parse_quote!{{ 
                    #expr
                }}
            },
            _ => syn::fold::fold_expr( self, expr ),
        }
    }
}

#[proc_macro_attribute]
pub fn enumx( _args: TokenStream, input: TokenStream ) -> TokenStream {
    let input = parse_macro_input!( input as ItemFn );
    let output = EnumxFn::default().fold_item_fn( input );
    let expanded = quote!( #output );
    TokenStream::from( expanded )
}

#[proc_macro_derive( EnumXDerives )]
pub fn derive_enumx( input: TokenStream ) -> TokenStream {
    let input: DeriveInput = syn::parse( input ).unwrap();

    let ( impl_generics, ty_generics, _ ) = input.generics.split_for_impl();

    let ty = input.ident;
    let type_name = ty.to_string();

    let ( enum_name, enum_cnt );
    if &type_name[..4] == "Enum" {
        enum_cnt = type_name[4..].parse::<usize>().expect( "Type name should start with \"Enum\" and followed by an unsigned integer." );
        enum_name = vec![ ident( &format!( "Enum{}", &type_name[4..] )); enum_cnt ];
    } else {
        syntax_err!();
    }

    let enum_name = enum_name.iter();
    let enum_name_ = enum_name.clone();

    let lr = syn::parse_str::<Type>(
                &format!( "{}Enum0{}"
                    , (0..enum_cnt).fold( String::new(), |acc,i| format!( "{}LR<E{},", acc, i ))
                    , ">".repeat( enum_cnt )
                )
             ).unwrap();

    let (pattern,variant) = (0..enum_cnt).fold( (Vec::new(),Vec::new()), |(mut pattern, mut variant), index| {
        pattern.push( syn::parse_str::<Pat>(
            &format!( "{}{}{}"
                , "LR::R(".repeat( index )
                , "LR::L(e)"
                , ")".repeat( index )
            )
        ).unwrap() );
        variant.push( syn::parse_str::<Ident>(
            &format!( "_{}", index )
        ).unwrap() );
        (pattern,variant)
    });

    let ( pattern, variant ) = ( pattern.iter(), variant.iter() );
    let ( pattern_, variant_ ) = ( pattern.clone(), variant.clone() );

    let expanded = quote! {
        impl #impl_generics EnumX for #ty #ty_generics {
            type LR = #lr;

            fn from_lr( lr: Self::LR ) -> Self {
                match lr {
                    #( #pattern => #enum_name::#variant(e), )*
                    _ => panic!( "Nested enumx::LR's last variant should be enumx::Enum0" ),
                }
            }

            fn into_lr( self ) -> Self::LR {
                match self {
                    #( #enum_name_::#variant_(e) => #pattern_, )*
                }
            }
        }
    };
    expanded.into()
}

fn ident( sym: &str ) -> Ident {
    Ident::new( sym, Span::call_site() )
}
