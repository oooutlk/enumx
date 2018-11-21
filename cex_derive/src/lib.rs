// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

#![recursion_limit="128"]

extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;

use syn::export::Span;
use syn::fold::Fold;
use syn::{parse_macro_input, parse_quote, Block, DeriveInput, Expr, GenericArgument, Ident, ItemFn, Pat, ReturnType, Path, PathArguments, Type, TypePath};

struct CexFn {
    closure_type : Option<Type>,
    cex_path     : Option<Path>,
    err_types    : Vec<Type>,
    name         : String,
}

impl CexFn {
    fn new( name: String ) -> Self {
        CexFn { 
            closure_type : None,
            cex_path     : None,
            err_types    : Vec::new(),
            name         ,
        }
    }
}

macro_rules! syntax_err {
    () => {
        panic!( "#[cex] functions should return something like `Result<T,Ce_<_,_,_>>` or `Result<T,Enum_<_,_,_>>`." );
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

impl Fold for CexFn {
    // Does not modify return type but gathers type infomation.
    fn fold_return_type( &mut self, ret: ReturnType ) -> ReturnType {
        if let ReturnType::Type( _, mut closure_type ) = ret.clone() {
            let gen_types = mut_generics( &mut *closure_type );
            if let Some( cex_type ) = gen_types.last() {
                self.cex_path = if let Type::Path( TypePath{ qself:_, path }) = cex_type { Some( path.clone() )} else { None };
                self.err_types = mut_generics( cex_type ).map( |err| err.clone() ).collect::<Vec<_>>();
                let concrete_types = self.err_types.iter();
                *cex_type = parse_quote!( __CeX<#(#concrete_types),*> );
            }
            self.closure_type = Some( *closure_type );
        }
        ret
    }

    // Here is the magic.
    fn fold_block( &mut self, mut block: Block ) -> Block {
        let closure_type = self.closure_type.take().expect( "closure must be of some type." );
        let cex_path = self.cex_path.take().expect( "cex must be of some path." );
        let cex_name = cex_path.segments.iter().last().unwrap_or_else( || syntax_err!() ).ident.to_string();

        let ( deriving_cex, err_cnt, cex_trait );
        let trace_throw_point: syn::Expr;

        if &cex_name[..2] == "Ce" {
            deriving_cex = true;
            err_cnt = cex_name[2..].parse::<usize>().expect( "Type name should start with \"Ce\" and followed by an unsigned integer." );
            cex_trait = ident( "CeX" );
            let mut cex_block = CexBlock::new( self.name.clone() );
            block = cex_block.fold_block( block );
            // hook early exit for backtrace.
            trace_throw_point = parse_quote!{{ err.0.backtrace.0.push( __cex_throw_point.expect( "All errors should have a throw point." )); () }};
        } else if &cex_name[..4] == "Enum" {
            deriving_cex = false;
            err_cnt = cex_name[4..].parse::<usize>().expect( "Type name should start with \"Enum\" and followed by an unsigned integer." );
            cex_trait = ident( "EnumX" );
            let mut cex_block = CexBlock::new( self.name.clone() );
            block = cex_block.fold_block( block );
            // EnumX does not need hook early exit for backtrace.
            trace_throw_point = parse_quote!(());
        } else {
            unreachable!();
        }

        fn make_generics( count: usize ) -> syn::Generics {
            syn::parse_str::<syn::Generics>(
                &format!( "<{}>",
                    (1..count).fold(
                        String::from( "E0"), |acc,index| format!( "{},E{}", acc, index ))))
                .expect("Generics must be something like <E0,E1,...>")
        }

        let generics = make_generics( err_cnt );
        let ( _, ty_generics, _ ) = generics.split_for_impl();

        let concrete_errors = self.err_types.iter();

        let impl_from_concrete_errors = self.err_types.iter().enumerate().fold( Vec::<syn::ItemImpl>::new(), |mut impls,(index,err_type)| {
            let variant = ident( &format!( "_{}", index ));
            let concrete_errors = concrete_errors.clone();
            impls.push( parse_quote!{
                impl From<#err_type> for __CeX<#(#concrete_errors),*> {
                    fn from( e: #err_type ) -> Self {
                        __CeX( <#cex_path>::#variant( e ))
                    }
                }
            });
            impls
        });

        let impl_from_cex_types = (1..=err_cnt).fold( Vec::<syn::ItemImpl>::new(), |mut acc,index| {
            let concrete_errors = self.err_types.iter();
            let current_cex_name = if deriving_cex {
                format!( "Ce{}", index )
            } else {
                format!( "Enum{}", index )
            };
            let current_cex_name = ident( &current_cex_name );

            let generics = make_generics( index );
            let ( impl_generics, ty_generics, _ ) = generics.split_for_impl();

            acc.push( syn::ItemImpl::from(
                if deriving_cex {
                    parse_quote! {
                        impl #impl_generics From<#current_cex_name #ty_generics> for __CeX<#(#concrete_errors),*>
                            where Self: From<<#current_cex_name #ty_generics as #cex_trait>::LR>
                        {
                            fn from( src: #current_cex_name #ty_generics ) -> Self {
                                let ( lr, backtrace ) = src.into_lr();
                                let mut _cex = __CeX::from( lr );
                                _cex.0.backtrace = backtrace;
                                _cex
                            }
                        }
                    }
                } else {
                    parse_quote! {
                        impl #impl_generics From<#current_cex_name #ty_generics> for __CeX<#(#concrete_errors),*>
                            where Self: From<<#current_cex_name #ty_generics as #cex_trait>::LR>
                        {
                            fn from( src: #current_cex_name #ty_generics ) -> Self {
                                __CeX::from( src.into_lr() )
                            }
                        }
                    }
                }
            ));
            acc
        });

        let cex_name = ident( &cex_name );
        let concrete_errors_ = concrete_errors.clone();

        parse_quote! {{
            use cex::*;
            struct __CeX #ty_generics ( #cex_name #ty_generics );

            impl From<Enum0> for __CeX<#(#concrete_errors),*> {
                fn from( _: Enum0 ) -> Self {
                    unreachable!()
                }
            }

            #( #impl_from_concrete_errors )*

            impl<L,R> From<LR<L,R>> for __CeX<#(#concrete_errors_),*>
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

            #( #impl_from_cex_types )*
 
            let mut __cex_throw_point: Option<ThrowPoint> = None; 
            let __cex_result = {
                let mut _cex_closure = || -> #closure_type { #block };
                _cex_closure()
            };
            __cex_result.map_err( |mut err| {
                #trace_throw_point;
                err.0
            })
        }}
    }
}

struct CexBlock {
    function_name : String,
}

impl CexBlock {
    fn new( function_name: String ) -> Self {
        CexBlock{ function_name }
    }
}

impl Fold for CexBlock { 
    fn fold_expr( &mut self, expr: Expr ) -> Expr {
        match expr {
            Expr::Return( _ ) => {
                let cex_throw_point = ident( "__cex_throw_point" );
                let function_name = &self.function_name;
                parse_quote!{{ 
                    #cex_throw_point = Some( cex::ThrowPoint::new( line!(), column!(), #function_name, module_path!(), file!() ));
                    #expr.map_err( |err| __CeX::from( err ))
                }}
            },
            Expr::Try( _ ) => {
                let cex_throw_point = ident( "__cex_throw_point" );
                let function_name = &self.function_name;
                parse_quote!{{ 
                    #cex_throw_point = Some( cex::ThrowPoint::new( line!(), column!(), #function_name, module_path!(), file!() ));
                    #expr
                }}
            },
            _ => syn::fold::fold_expr( self, expr ),
        }
    }
}

#[proc_macro_attribute]
pub fn cex( _args: TokenStream, input: TokenStream ) -> TokenStream {
    let input = parse_macro_input!( input as ItemFn );
    let function_name = input.ident.to_string();
    let output = CexFn::new( function_name ).fold_item_fn( input );
    let expanded = quote!( #output );
    TokenStream::from( expanded )
}

#[proc_macro_derive( CeXDerives )]
pub fn derive_cex_or_enum( input: TokenStream ) -> TokenStream {
    let input: DeriveInput = syn::parse( input ).unwrap();

    let ( impl_generics, ty_generics, _ ) = input.generics.split_for_impl();

    let ty = input.ident;
    let type_name = ty.to_string();

    let ( deriving_cex, enum_name, err_cnt );
    if &type_name[..2] == "Ce" {
        deriving_cex = true;
        err_cnt = type_name[2..].parse::<usize>().expect( "Type name should start with \"Ce\" and followed by an unsigned integer." );
        enum_name = vec![ ident( &format!( "Enum{}", &type_name[2..] )); err_cnt ];
    } else if &type_name[..4] == "Enum" {
        deriving_cex = false;
        err_cnt = type_name[4..].parse::<usize>().expect( "Type name should start with \"Enum\" and followed by an unsigned integer." );
        enum_name = vec![ ident( &format!( "Enum{}", &type_name[4..] )); err_cnt ];
    } else {
        unreachable!();
    }

    let enum_name = enum_name.iter();
    let enum_name_ = enum_name.clone();

    let lr = syn::parse_str::<Type>(
                &format!( "{}Enum0{}"
                    , (0..err_cnt).fold( String::new(), |acc,i| format!( "{}LR<E{},", acc, i ))
                    , ">".repeat( err_cnt )
                )
             ).unwrap();

    let (pattern,variant) = (0..err_cnt).fold( (Vec::new(),Vec::new()), |(mut pattern, mut variant), idx| {
        pattern.push( syn::parse_str::<Pat>(
            &format!( "{}{}{}"
                , "LR::R(".repeat( idx )
                , "LR::L(e)"
                , ")".repeat( idx )
            )
        ).unwrap() );
        variant.push( syn::parse_str::<Ident>(
            &format!( "_{}", idx )
        ).unwrap() );
        (pattern,variant)
    });

    let ( pattern, variant ) = ( pattern.iter(), variant.iter() );
    let ( pattern_, variant_ ) = ( pattern.clone(), variant.clone() );

    let expanded = if deriving_cex {
        let struct_name = ty.clone();
        quote! {
            impl #impl_generics CeX for #ty #ty_generics {
                type LR = #lr;

                fn from_lr( lr: Self::LR, backtrace: Backtrace ) -> Self {
                    #struct_name {
                        error: match lr {
                            #( #pattern => #enum_name::#variant(e), )*
                            _ => panic!( "Nested cex::LR's last variant should be cex::Enum0" ),
                        },
                        backtrace,
                    }
                }

                fn into_lr( self ) -> ( Self::LR, Backtrace ) {
                    match self.error {
                        #( #enum_name_::#variant_(e) => (#pattern_,self.backtrace), )*
                    }
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics EnumX for #ty #ty_generics {
                type LR = #lr;

                fn from_lr( lr: Self::LR ) -> Self {
                    match lr {
                        #( #pattern => #enum_name::#variant(e), )*
                        _ => panic!( "Nested cex::LR's last variant should be cex::Enum0" ),
                    }
                }

                fn into_lr( self ) -> Self::LR {
                    match self {
                        #( #enum_name_::#variant_(e) => #pattern_, )*
                    }
                }
            }
        }
    };
    expanded.into()
}

fn ident( sym: &str ) -> Ident {
    Ident::new( sym, Span::call_site() )
}
