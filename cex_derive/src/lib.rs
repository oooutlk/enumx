// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>

//! This crate is the proc-macro implementation for `cex` crate.
//! It provides `cex::Logger` derive for enum, and `#[cex]` attribute for functions, closures and try blocks.

#![recursion_limit="128"]

extern crate proc_macro;
use self::proc_macro::{TokenStream,TokenTree};

use quote::quote;

use syn::{ DeriveInput, Generics, Ident };
use syn::export::Span;

use syn::{ Expr, ItemFn, parse_quote };
use syn::visit_mut::VisitMut;

macro_rules! syntax_error {
    () => {
        panic!( "A type deriving `Logger` should be in the form of \"enum Name { Foo(Type), Bar(AnotherType),... }\"" );
    }
}

/// Implements `cex::Logger` for an `enum`.
#[proc_macro_derive( Logger )]
pub fn derive_logger( input: TokenStream ) -> TokenStream {
    let input: DeriveInput = syn::parse( input ).unwrap();

    match input.data {
        syn::Data::Enum( ref data ) => {
            let name = &input.ident;
            let variant_cnt = data.variants.len();
            let enum_name  = (0..variant_cnt).map( |_| name );
            let enum_name_ = (0..variant_cnt).map( |_| name );
            let variant_names  = data.variants.iter().map( |v| &v.ident );
            let variant_names_ = data.variants.iter().map( |v| &v.ident );

            let mut impl_generics = input.generics.clone();
            add_generics( &mut impl_generics, "Agent" );
            let ( impl_generics, _, _ ) = impl_generics.split_for_impl();

            let ( _, ty_generics, where_clause ) = input.generics.split_for_impl();
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

            let expanded = quote! {
                impl #impl_generics cex::Logger<Agent> for #name #ty_generics
                    where #(#variant_ty: cex::Logger<Agent>),*
                        , Agent: LogAgent
                        , #clause
                {
                    fn log( self, item: Agent::Item ) -> Self {
                        match self {
                            #( #enum_name::#variant_names( v ) =>
                                #enum_name_::#variant_names_( cex::Logger::<Agent>::log( v, item )),
                            )*
                        }
                    }
                }
            };
            expanded.into()
        }
        _ => panic!( "Only `enum`s can derive `Logger`." ),
    }
}

fn add_generics( generics: &mut Generics, name: &'static str ) {
    let name = ident( name );
    generics.params.push( parse_quote!( #name ));
}

fn ident( sym: &str ) -> Ident {
    Ident::new( sym, Span::call_site() )
}

enum CexAttr {
    Log(    Option<Expr> ),
    ToLog(  Option<Expr> ),
    MapErr( Expr ),
}

struct CexTag {
    tag  : bool,
    attr : Option<CexAttr>,
}

impl CexTag {
    fn new( tag: bool, attr: Option<CexAttr> ) -> Self {
        CexTag{ tag, attr }
    }
}

enum Given { Raw, Log, MapError }

fn given( expr: &Expr ) -> Given {
    if let Expr::MethodCall( expr ) = expr {
        match expr.method.to_string().as_str() {
            "map_error" => Given::MapError,
            "map_err_to_log" | "map_err_log" | "map_err" => Given::Log,
            _ => Given::Raw,
        }
    } else {
        Given::Raw
    }
}

impl VisitMut for CexTag { 
    fn visit_item_fn_mut( &mut self, item_fn: &mut ItemFn ) {
        let tag = self.tag;
        self.tag = false;
        self.visit_block_mut( &mut *item_fn.block );
        self.tag = tag;
    }

    fn visit_expr_mut( &mut self, expr: &mut Expr ) {
        match expr {
            Expr::Try( try_ ) => if self.tag {
                let try_expr = &mut try_.expr;
                syn::visit_mut::visit_expr_mut( self, try_expr );

                match given( try_expr ) {
                    Given::Raw => try_.expr = self.attr.as_ref().map( |attr|
                        match attr {
                            CexAttr::Log(    Some( log )) => parse_quote!{ #try_expr.map_err_log(#log).map_error() },
                            CexAttr::ToLog(  Some( log )) => parse_quote!{ #try_expr.map_err_to_log(#log).map_error() },
                            CexAttr::Log(   None ) => parse_quote!{ #try_expr.map_err_log(frame!()).map_error() },
                            CexAttr::ToLog( None ) => parse_quote!{ #try_expr.map_err_to_log(frame!()).map_error() },
                            CexAttr::MapErr( expr ) => parse_quote!{ #try_expr.map_err(#expr).map_error() },
                        }
                    ).unwrap_or(
                        parse_quote!{ #try_expr.map_error() }
                    ),
                    Given::Log => {
                        try_.expr = parse_quote!{ #try_expr.map_error() };
                    }
                    Given::MapError => (),
                }
            },
            Expr::TryBlock(_) | Expr::Closure(_) => {
                let tag = self.tag;
                self.tag = false;
                syn::visit_mut::visit_expr_mut( self, expr );
                self.tag = tag;
            },
            _ => syn::visit_mut::visit_expr_mut( self, expr ),
        }
    }
}

fn grouped_expr( tt: Option<TokenTree> ) -> Option<Expr> {
    tt.map( |tt| if let TokenTree::Group( group ) = tt {
        syn::parse::<Expr>( group.stream() )
            .expect( "an expression inside log/to_log/map_err" )
    } else {
        panic!("expect log(expr)/to_log(expr)/map_err(expr).");
    })
}

fn parse_cex_attr( ts: TokenStream ) -> Option<CexAttr> {
    let mut iter = ts.into_iter();
    match iter.next() {
        Some( TokenTree::Ident( ident )) => match ident.to_string().as_str() {
            "log"     => Some( CexAttr::Log(    grouped_expr( iter.next() ))),
            "to_log"  => Some( CexAttr::ToLog(  grouped_expr( iter.next() ))),
            "map_err" => Some( CexAttr::MapErr( grouped_expr( iter.next() )
                .expect("an expression inside map_err(...)") )),
            _ => panic!( "Only log/to_log/map_err are supported in #[cex(...)]"),
        },
        _ => None,
    }
}

/// `#[cex]` attribute for functions, closures and try blocks, to append
/// combinators to try expressions.
///
/// # Syntax
///
/// * A `#[cex]` will append `.map_error()`s to try expressions, unless they end
/// with it already.
/// 
/// * Besides, a `#[cex(log)]` will append `.map_err_log(frame!())`s to try
/// expressions.
/// 
/// * Besides, a `#[cex(log(expr))]` will append `.map_err_log(#expr)`s to try
/// expressions.
/// 
/// * A `#[cex(to_log)]` is similar with `#[cex(log)]`, but provides
/// `map_err_to_log` instead of `map_err_log`.
/// 
/// * A `#[cex(to_log(expr)]` is similar with `#[cex(log(expr))]`, but provides
/// `map_err_to_log` instead of `map_err_log`.
/// 
/// * Besides, a `#[cex(map_err(expr))]` will append `.map_err(#expr)`s to try
/// expressions.
/// 
/// * All the logging attributes will append nothing if the try expressions end with
/// `.map_err_to_log()`/`.map_err_log()`/`.map_err()` already.
/// 
/// * All the `#[cex]` tags will append nothing for functions, closures and try
/// expressions inside the `#[cex] fn` that are not tagged with `#[cex]` themselves.
#[proc_macro_attribute]
pub fn cex( args: TokenStream, input: TokenStream ) -> TokenStream {
    let cex_arg = parse_cex_attr( args );

    if let Ok( mut item_fn ) = syn::parse::<ItemFn>( input.clone() ) {
        CexTag::new( true, cex_arg ).visit_block_mut( &mut *item_fn.block );
        let expanded = quote!( #item_fn );
        return TokenStream::from( expanded );
    } else if let Ok( expr ) = syn::parse::<Expr>( input ) {
        if let Expr::Closure( mut closure ) = expr {
            CexTag::new( true, cex_arg ).visit_expr_mut( &mut *closure.body );
            let expanded = quote!( #closure );
            return TokenStream::from( expanded );
        } else if let Expr::TryBlock( mut try_block ) = expr {
            CexTag::new( true, cex_arg ).visit_block_mut( &mut try_block.block );
            let expanded = quote!( #try_block );
            return TokenStream::from( expanded );
        }
    }
    panic!( "#[cex] for functions, closures and try blocks only" );
}
