// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>

//! This crate is the proc-macro implementation for `cex` crate.
//!
//! # Features
//!
//! - `Result!( OkType throws A,B,... )` which is equivalant to `Result<OkType, enumx::Enum!(A,B,...)>`
//!
//! - ret!() Ok Type or Result
//!
//! - throw!() Error Types listed in throws
//!
//! - `#[ty_pat] match`, which enables "using types as patterns in match arms".
//!
//! - `#[cex] let local_var: pattern = expression;`, which enables all the mentioned features in the expression, e.g try blocks.
//!
//! - `#[cex]` on closures, which enables all the mentioned features in the closure.
//!
//! - `#[cex] fn`, which enables all the mentioned features in the function.
//!
//! - `cex::Logger` derive for enum.
//!
//! See more details in `cex` crate's documents.

#![recursion_limit="128"]

extern crate proc_macro;
use self::proc_macro::{TokenStream, TokenTree};

use quote::{
    quote,
    quote_spanned,
};

use syn::{
    export::Span,
    parse_quote,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    visit_mut::{self, VisitMut},
    Attribute, DeriveInput, Expr, ExprLet, Generics, Ident, ItemFn, Pat, Path, PathArguments, Stmt, Token, Type, TypePath
};

use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    iter::FromIterator,
};

macro_rules! parse_quote_spanned {
    ( $span:expr => $($tt:tt)+ ) => {{
        let quoted = quote_spanned!( $span => $($tt)+ );
        parse_quote!( #quoted )
    }};
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
                panic!( "A type deriving `Logger` should be in the form of \"enum Name { Foo(Type), Bar(AnotherType),... }\"" );
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

struct TypeList( Vec<Path> );

impl Parse for TypeList {
    fn parse( input: ParseStream ) -> syn::Result<Self> {
        let types = Punctuated::<Type, Token![,]>::parse_terminated( input )?;
        Ok( TypeList( types.into_iter().map( |ty| match ty {
            Type::Path( type_path ) => type_path.path,
            _  => parse_quote!( TyPat::<#ty> ),
        }).collect::<Vec<_>>() ))
    }
}

#[derive( Eq )]
struct TypeIndex( Path, Cell<u32> );

impl PartialEq for TypeIndex {
    fn eq( &self, other: &Self ) -> bool { self.0.eq( &other.0 )}
    fn ne( &self, other: &Self ) -> bool { self.0.ne( &other.0 )}
}

impl Hash for TypeIndex {
    fn hash<H:Hasher>( &self, state: &mut H ) { self.0.hash( state )}
}

type Throws = HashSet<TypeIndex>;

struct CexFn {
    stack  : Vec<CexTag>,
    logger : Logger,
}

impl CexFn {
    fn new( logger: Logger ) -> Self {
        CexFn {
            stack : vec![ CexTag{ ret: None }],
            logger,
        }
    }

    fn peek( &mut self ) -> &mut CexTag { self.stack.last_mut().expect( "CexFn's stack cannot be empty" )}
    fn push( &mut self, tag: CexTag ) { self.stack.push( tag ); }
    fn pop( &mut self ) { self.stack.pop(); }
}

#[derive( Copy, Clone, PartialEq )]
enum Logger {
    None,
    Static,
    EnvOpt,
}

impl From<&'static str> for Logger {
    fn from( name: &'static str ) -> Self {
        match name {
            "cex"         => Logger::None,
            "cex_log"     => Logger::Static,
            "cex_env_log" => Logger::EnvOpt,
            _             => panic!("only cex, cex_log, cex_env_log are supported"),
        }
    }
}

struct Ret {
    throws : HashSet<TypeIndex>,
    ty     : Type,
}

struct CexTag {
    ret : Option<Ret>,
}

impl CexTag {
    fn new() -> Self {
        CexTag{ ret: None }
    }

    fn parse_type_list( logger: Logger, input: TokenStream ) -> syn::Result<TypeList> {
        let mut types = syn::parse::<TypeList>( input )?;
        match logger {
            Logger::None   => (),
            Logger::Static => types.0.iter_mut().for_each( |ty| *ty = parse_quote_spanned!( ty.span() => Log<#ty> )),
            Logger::EnvOpt => types.0.iter_mut().for_each( |ty| *ty = parse_quote_spanned!( ty.span() => Log<#ty, cex::Env<Vec<cex::Frame>>> )),
        }
        Ok( types )
    }
}

fn to_compact_string( input: impl Into<TokenStream> ) -> String {
    input.into().into_iter().fold( String::new(), |acc, tt| format!( "{}{}", acc, tt ))
}

fn is_cex_attr( attr: &Attribute ) -> bool {
    if attr.path.leading_colon.is_none() && attr.path.segments.len() == 1 {
        match attr.path.segments.first().unwrap().ident.to_string().as_str() {
            "cex"         |
            "cex_log"     |
            "cex_env_log" => return true,
            _             => (),
        }
    }
    return false;
}

#[derive( PartialEq )]
enum TyPatAttr {
    None,
    GenThrows,
    Gen( Throws ),
}

fn parse_ty_pat_attr( logger: Logger, attr: &Attribute ) -> Option<TyPatAttr> {
    if attr.path.leading_colon.is_none() && attr.path.segments.len() == 1 {
        if attr.path.segments.first().unwrap().ident == "ty_pat" {
            let ts = TokenStream::from( attr.tokens.clone() );
            let mut iter = ts.into_iter();
            if let Some( TokenTree::Group( group )) = iter.next() {
                let mut iter = group.stream().into_iter();
                match iter.next() {
                    Some( TokenTree::Ident( ident_throws )) => match ident_throws.to_string().as_str() {
                        "gen_throws" => {
                            return Some( TyPatAttr::GenThrows );
                        },
                        "gen" => {
                            let mut throws = HashSet::new();
                            let types = CexTag::parse_type_list( logger, TokenStream::from_iter( iter )).expect("type list");
                            if types.0.len() == 0 {
                                return Some( TyPatAttr::GenThrows );
                            } else {
                                types.0.into_iter().for_each( |ty| { throws.insert( TypeIndex( ty, Cell::new(0) )); });
                                return Some( TyPatAttr::Gen( throws ));
                            }
                        },
                        _ => panic!("invalid #[ty_pat] argument: only #[ty_pat(gen_throws)] and #[ty_pat(gen)] are supported"),
                    },
                    Some( _ ) => panic!("invalid #[ty_pat] argument"),
                    None => return Some( TyPatAttr::None ),
                }
            } else {
                return Some( TyPatAttr::None );
            }
        }
    }
    return None;
}

impl VisitMut for CexFn {
    fn visit_type_mut( &mut self, node: &mut Type ) {
        visit_mut::visit_type_mut( self, node );

        if let Type::Macro( type_macro ) = node {
            let mac = &type_macro.mac;
            if mac.path.leading_colon.is_none() && mac.path.segments.len() == 1 {
                let seg = mac.path.segments.first().unwrap();
                if seg.arguments == PathArguments::None && seg.ident == "Result" {
                    let ts = TokenStream::from( mac.tokens.clone() );
                    let mut iter = ts.into_iter();
                    let mut ok = TokenStream::new();
                    while let Some(tt) = iter.next() {
                        if let TokenTree::Ident( ident ) = &tt {
                            if ident.to_string() == "throws" {
                                break;
                            }
                        }
                        ok.extend( std::iter::once( tt ));
                    }
                    let mut ok = syn::parse::<Type>( ok ).expect("Result!( OkType ... )");
                    self.visit_type_mut( &mut ok );

                    let mut throws = HashSet::new();
                    let rest = TokenStream::from_iter( iter );
                    let types = CexTag::parse_type_list( self.logger, rest ).expect("type list");
                    types.0.into_iter().for_each( |ty| {
                        let mut type_ = Type::Path( TypePath{ qself: None, path: ty });
                        self.visit_type_mut( &mut type_ );
                        match type_ {
                            Type::Path( type_path ) => { throws.insert( TypeIndex( type_path.path, Cell::new(0) )); },
                            _ => unreachable!(),
                        }
                    });

                    let err = throws.iter().map( |type_index| &type_index.0 );
                    let ty: Type = parse_quote_spanned!( mac.span() => Result<#ok, Enum!(#(#err),*)> );
                    *node = ty.clone();
                    self.peek().ret = Some( Ret{ throws, ty }); // rewritable
                }
            }
        }
    }

    fn visit_expr_mut( &mut self, expr: &mut Expr ) {
        match expr {
            Expr::Match( expr_match ) => {
                self.visit_expr_mut( &mut *expr_match.expr );

                expr_match.arms.iter_mut().for_each( |arm| {
                    arm.guard.as_mut().map( |guard| self.visit_expr_mut( &mut *guard.1 ));
                    self.visit_expr_mut( &mut *arm.body );
                });

                let mut ty_pat_attrs = None;
                let logger = self.logger;
                for (index, attr) in expr_match.attrs.iter().enumerate() {
                    if let Some( attrs ) = parse_ty_pat_attr( logger, attr ) {
                        ty_pat_attrs = Some(( attrs, index ));
                        break;
                    }
                }
                ty_pat_attrs.as_ref().map( |(_,index)| expr_match.attrs.remove( *index ));

                if let Some( ty_pat_attrs ) = ty_pat_attrs {
                    let match_expr = &*expr_match.expr;
                    let match_span = match_expr.span();
                    expr_match.expr = Box::new( parse_quote_spanned!( match_span => ExchangeFrom::exchange_from( #match_expr ) ));
                    let mut index = 0_u32;
                    let checked = expr_match.arms.iter_mut().fold( HashMap::<Path,Cell<u32>>::new(), |mut acc, arm| {
                        let mut add_type_pattern = |path: &mut Path| {
                            let mut nth = index;
                            acc.entry( path.clone() )
                                .and_modify( |n| { nth = n.get() })
                                .or_insert( Cell::new( nth ));
                            if nth == index { index += 1; }

                            let _n = ident( &format!( "_{}", nth ));
                            *path = parse_quote!{ __CexAdhocEnum::#_n };
                        };

                        match &mut arm.pat {
                            Pat::TupleStruct( pat_tuple_struct ) => {
                                if pat_tuple_struct.pat.elems.len() > 1 {
                                    panic!("#[cex] supports tuple struct variant in newtype form only.");
                                }
                                add_type_pattern( &mut pat_tuple_struct.path );
                                acc
                            },
                            Pat::Path( pat_path ) => {
                                add_type_pattern( &mut pat_path.path );
                                acc
                            },
                            Pat::Wild(_) if arm.guard.is_none() => {
                                acc
                            },
                            _ => {
                                let pat_ident;
                                if let Pat::Ident( ident ) = &mut arm.pat {
                                    if ident.by_ref.is_some() || ident.mutability.is_some() || ident.subpat.is_some() {
                                        panic!("#[cex] dost not support ident in arm with ref/mut/sub pattern.");
                                    } else {
                                        pat_ident = Some( ident.clone() );
                                    }
                                } else {
                                    panic!("#[cex] unsupported pattern in match arm");
                                }

                                pat_ident.map( |pat_ident| {
                                    let ident = &pat_ident.ident;
                                    let mut path: Path = parse_quote!( #ident );
                                    add_type_pattern( &mut path );
                                    arm.pat = parse_quote!( #path(_) );
                                });

                                acc
                            },
                        }
                    });

                    let checked = HashSet::<TypeIndex>::from_iter( checked.clone().into_iter().map( |(t,i)| TypeIndex(t,i) ));
                    let logger = self.logger;
                    let cex_tag = self.peek();
                    let unexhausted = match &ty_pat_attrs.0 {
                        TyPatAttr::None => Vec::new(),
                        TyPatAttr::GenThrows => cex_tag.ret.as_ref().expect("Result!( OkType throws ... ) parsed").throws.difference( &checked ).collect::<Vec<_>>(),
                        TyPatAttr::Gen( throws ) => throws.difference( &checked ).collect::<Vec<_>>(),
                    };

                    unexhausted.iter().for_each( |TypeIndex(_,i)| {
                        i.set( index );
                        let _n = ident( &format!( "_{}", index ));
                        let ret_type = &cex_tag.ret.as_ref().expect("Result!( OkType throws ... ) parsed").ty;
                        expr_match.arms.push(
                            match logger {
                                Logger::None   => parse_quote_spanned!( match_span => __CexAdhocEnum::#_n(v) => cex::   Throw::<#ret_type,                       _>::throw(     v), ),
                                Logger::Static => parse_quote_spanned!( match_span => __CexAdhocEnum::#_n(v) => cex::ThrowLog::<#ret_type, Vec<Frame>          , _>::throw_log( v, || frame!() ), ),
                                Logger::EnvOpt => parse_quote_spanned!( match_span => __CexAdhocEnum::#_n(v) => cex::ThrowLog::<#ret_type, cex::Env<Vec<Frame>>, _>::throw_log( v, || frame!() ), ),
                            }
                        );
                        index += 1;
                    });

                    let (unexhausted_types, unexhausted_indices): (Vec<_>, Vec<_>) = unexhausted.iter().map( |TypeIndex(t,i)| (t,i) ).unzip();
                    let unexhausted_indices = unexhausted_indices.iter().map( |n| ident( &format!( "_{}", n.get() )));

                    let (checked_types, checked_indices): (Vec<_>, Vec<_>) = checked.iter().map( |TypeIndex(t,i)| (t,i) ).unzip();
                    let checked_indices = checked_indices.iter().map( |n| ident( &format!( "_{}", n.get() )));

                    let adhoc_enum = match logger {
                        Logger::None => quote_spanned! { match_span =>
                            #[derive( enumx::EnumX )]
                            enum __CexAdhocEnum {
                                #( #checked_indices( #checked_types ), )*
                                #( #unexhausted_indices( #unexhausted_types ), )*
                            }
                        },
                        Logger::Static => quote_spanned! { match_span =>
                            #[derive( enumx::EnumX, cex_derive::Logger )]
                            enum __CexAdhocEnum {
                                #( #checked_indices( cex::Log<#checked_types> ), )*
                                #( #unexhausted_indices( #unexhausted_types ), )*
                            }
                        },
                        Logger::EnvOpt => quote_spanned! { match_span =>
                            #[derive( enumx::EnumX, cex_derive::Logger )]
                            enum __CexAdhocEnum {
                                #( #checked_indices( cex::Log<#checked_types, cex::Env<Vec<cex::Frame>>> ), )*
                                #( #unexhausted_indices( #unexhausted_types ), )*
                            }
                        },
                    };

                    *expr = syn::parse::<Expr>( quote_spanned!{ match_span => {
                        #adhoc_enum
                        #expr
                    }}.into() ).unwrap();
                }
            },
            Expr::Try( expr_try ) => {
                let try_expr = &mut expr_try.expr;
                let try_expr_span = try_expr.span();
                syn::visit_mut::visit_expr_mut( self, try_expr );
                *try_expr = match self.logger {
                    Logger::None => {
                        parse_quote_spanned!( try_expr_span => #try_expr.map_error() )
                    },
                    Logger::Static => {
                        let s = to_compact_string( quote_spanned!( try_expr_span => #try_expr ));
                        parse_quote_spanned!( try_expr_span => #try_expr.map_error_log( || frame!(#s) ))
                    },
                    Logger::EnvOpt => {
                        let s = to_compact_string( quote_spanned!( try_expr_span => #try_expr ));
                        parse_quote_spanned!( try_expr_span => #try_expr.map_error_log( || frame!(#s) ))
                    },
                };
            },
            Expr::Closure( expr_closure ) => {
                let mut cex_attr_index = None;
                for (index,attr) in expr_closure.attrs.iter().enumerate() {
                    if is_cex_attr( &attr ) {
                        cex_attr_index = Some( index );
                        self.push( CexTag::new() );
                        break;
                    }
                }

                self.visit_return_type_mut( &mut expr_closure.output );
                self.visit_expr_mut( &mut *expr_closure.body );

                cex_attr_index.map( |index| {
                    expr_closure.attrs.remove( index );
                    self.pop();
                });
            },
            Expr::Macro( expr_macro ) => {
                let mut mac = &mut expr_macro.mac;
                if mac.path.leading_colon.is_none() && mac.path.segments.len() == 1 {
                    let seg = mac.path.segments.first().unwrap();
                    if seg.arguments == PathArguments::None {
                        let name = seg.ident.to_string();
                        match name.as_str() {
                            "ret" | "throw" => {
                                struct ExprList( Punctuated::<Expr, Token![,]> );

                                impl Parse for ExprList {
                                    fn parse( input: ParseStream ) -> syn::Result<Self> {
                                        Ok( ExprList( Punctuated::parse_terminated( input )? ))
                                    }
                                }

                                let mut expr_list = syn::parse::<ExprList>( TokenStream::from( mac.tokens.clone() )).unwrap();
                                expr_list.0.iter_mut().for_each( |expr| self.visit_expr_mut( expr ));
                                let mut exprs = expr_list.0.into_iter();
                                let logger = self.logger;
                                let ret_type = &self.peek().ret.as_ref().expect("Result!( OkType throws ... ) parsed").ty;
                                let span = mac.tokens.span();
                                *expr = match logger {
                                    Logger::None => {
                                        let the_expr = exprs.next().unwrap_or_else( || parse_quote_spanned!( span => () ));
                                        if name == "ret" {
                                            parse_quote_spanned!{ span => return cex::Ret::<#ret_type,_>::ret( #the_expr )}
                                        } else {
                                            parse_quote_spanned!{ span => return cex::Throw::<#ret_type,_>::throw( #the_expr )}
                                        }
                                    },
                                    Logger::Static => {
                                        let agent = quote!( Vec<Frame> );
                                        match exprs.len() {
                                            0 | 1 => {
                                                let the_expr = exprs.next().unwrap_or_else( || parse_quote_spanned!( span => () ));
                                                let s = to_compact_string( quote_spanned!( span => #mac ));
                                                if name == "ret" {
                                                    parse_quote_spanned!{ span => return cex::RetLog::<#ret_type,#agent,_>::ret_log( #the_expr, || frame!(#s) )}
                                                } else {
                                                    parse_quote_spanned!{ span => return cex::ThrowLog::<#ret_type,#agent,_>::throw_log( #the_expr, || frame!(#s) )}
                                                }
                                            },
                                            2 => {
                                                let the_expr = exprs.next().unwrap();
                                                let the_log  = exprs.next().unwrap();
                                                if name == "ret" {
                                                    parse_quote_spanned!{ span => return cex::RetLog::<#ret_type,#agent,_>::ret_log( #the_expr, #the_log )}
                                                } else {
                                                    parse_quote_spanned!{ span => return cex::ThrowLog::<#ret_type,#agent,_>::throw_log( #the_expr, #the_log )}
                                                }
                                            },
                                            _ => panic!("1 ret!()/throw!() should contain 1 or 2 argument(s)"),
                                        }
                                    },
                                    Logger::EnvOpt => {
                                        let agent = quote!( cex::Env<Vec<Frame>> );
                                        match exprs.len() {
                                            0 | 1 => {
                                                let the_expr = exprs.next().unwrap_or_else( || parse_quote_spanned!( span => () ));
                                                let s = to_compact_string( quote_spanned!( mac.span() => #mac ));
                                                if name == "ret" {
                                                    parse_quote_spanned!{ span => return cex::RetLog::<#ret_type,#agent,_>::ret_log( #the_expr, || frame!(#s) )}
                                                } else {
                                                    parse_quote_spanned!{ span => return cex::ThrowLog::<#ret_type,#agent,_>::throw_log( #the_expr, || frame!(#s) )}
                                                }
                                            },
                                            2 => {
                                                let the_expr = exprs.next().unwrap();
                                                let the_log  = exprs.next().unwrap();
                                                if name == "ret" {
                                                    parse_quote_spanned!{ span => return cex::RetLog::<#ret_type,#agent,_>::ret_log( #the_expr, #the_log )}
                                                } else {
                                                    parse_quote_spanned!{ span => return cex::ThrowLog::<#ret_type,#agent,_>::throw_log( #the_expr, #the_log )}
                                                }
                                            },
                                            _ => panic!("2 ret!()/throw!() should contain 1 or 2 argument(s)"),
                                        }
                                    },
                                };
                            },
                            _ => {
                                visit_mut::visit_macro_mut( self, &mut mac )
                            },
                        }
                    }
                }
            },
            _ => {
                visit_mut::visit_expr_mut( self, expr );
            },
        }
    }

    fn visit_stmt_mut( &mut self, node: &mut Stmt ) {
        let mut cex_attr_index = None;

        if let Stmt::Local( local ) = node {
            for (index,attr) in local.attrs.iter().enumerate() {
                if is_cex_attr( &attr ) {
                    cex_attr_index = Some( index );
                    self.push( CexTag::new() );
                    break;
                }
            }
        }

        visit_mut::visit_stmt_mut( self, node );

        if let Some( index ) = cex_attr_index {
            if let Stmt::Local( local ) = node {
                local.attrs.remove( index );
                self.pop();
            }
        }
    }

    fn visit_expr_let_mut( &mut self, expr_let: &mut ExprLet ) {
        let mut cex_attr_index = None;
        for (index,attr) in expr_let.attrs.iter().enumerate() {
            if is_cex_attr( &attr ) {
                cex_attr_index = Some( index );
                self.push( CexTag::new() );
                break;
            }
        }

        visit_mut::visit_expr_let_mut( self, expr_let );

        cex_attr_index.map( |index| {
            expr_let.attrs.remove( index );
            self.pop();
        });
    }

    fn visit_item_fn_mut( &mut self, item_fn: &mut ItemFn ) {
        let mut cex_attr_index = None;
        for (index,attr) in item_fn.attrs.iter().enumerate() {
            if is_cex_attr( &attr ) {
                cex_attr_index = Some( index );
                self.push( CexTag::new() );
                break;
            }
        }

        visit_mut::visit_item_fn_mut( self, item_fn );

        cex_attr_index.map( |index| {
            item_fn.attrs.remove( index );
            self.pop();
        });
    }
}

/// tag an `fn` with `#[cex]` to:
///
/// 1. enable "type pattern matching" in `match` expressions that are tagged with `#[ty_pat]`/`#[ty_pat(gen_variants)]/`#[ty_pat(gen A,B,..)]`.
///
/// 2. modify try(`?`) expressions to which append `map_error()` automatically.
///
/// 3. generate code for `Result!()`/`ret!()`/`throw!()` macro invocations.
#[proc_macro_attribute]
pub fn cex( _args: TokenStream, input: TokenStream ) -> TokenStream {
    expand_cex( "cex", _args, input )
}

/// tag an `fn` with `#[cex_log]` to:
///
/// 1. enable "type pattern matching" in `match` expressions that are tagged with `#[ty_pat]`/`#[ty_pat(gen_variants)]/`#[ty_pat(gen A,B,..)]`.
///
/// 2. modify try(`?`) expressions to which append `map_error_log()` automatically.
///
/// 3. generate code for `Result!()`/`ret!()`/`throw!()` macro invocations.
///
/// 4. backtrace enabled unconditionally.
#[proc_macro_attribute]
pub fn cex_log( _args: TokenStream, input: TokenStream ) -> TokenStream {
    expand_cex( "cex_log", _args, input )
}

/// tag an `fn` with `#[cex_env_log]` to:
///
/// 1. enable "type pattern matching" in `match` expressions that are tagged with `#[ty_pat]`/`#[ty_pat(gen_variants)]/`#[ty_pat(gen A,B,..)]`.
///
/// 2. modify try(`?`) expressions to which append `map_error_log()` automatically.
///
/// 3. generate code for `Result!()`/`ret!()`/`throw!()` macro invocations.
///
/// 4. backtrace enabled depending on the environment variable `RUST_BACKTRACE`.
#[proc_macro_attribute]
pub fn cex_env_log( _args: TokenStream, input: TokenStream ) -> TokenStream {
    expand_cex( "cex_env_log", _args, input )
}

fn expand_cex( tag_name: &'static str, _args: TokenStream, input: TokenStream ) -> TokenStream {
    if let Ok( mut item_fn ) = syn::parse::<ItemFn>( input.clone() ) {
        let mut cex_fn = CexFn::new( Logger::from( tag_name ));

        cex_fn.visit_signature_mut( &mut item_fn.sig );
        cex_fn.visit_block_mut( &mut *item_fn.block );
        let expanded = quote_spanned!( item_fn.span() => #item_fn );
        return TokenStream::from( expanded );
    } else {
        panic!( "#[throws] for functions, closures and try blocks only" );
    }
}
