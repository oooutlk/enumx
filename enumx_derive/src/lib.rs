// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

//! This project provides derive implementation for user-defined enum exchange.
//!
//! See [enumx README](https://github.com/oooutlk/enumx/blob/master/enumx/README.md) for more.

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
    Attribute, DeriveInput, Expr, ExprLet, Ident, ItemFn, Pat, Path, PathArguments, Stmt, Token, Type, TypePath,
};

use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    iter::FromIterator,
};

macro_rules! syntax_error {
    () => {
        panic!( "`enum`s deriving `EnumX` should be in the form of \"enum MyEnum { Foo(Type), Bar(AnotherType),... }\"" );
    }
}

macro_rules! parse_quote_spanned {
    ( $span:expr => $($tt:tt)+ ) => {{
        let quoted = quote_spanned!( $span => $($tt)+ );
        parse_quote!( #quoted )
    }};
}

#[proc_macro_derive( EnumX )]
pub fn derive_enumx( input: TokenStream ) -> TokenStream {
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
                impl #impl_generics enumx::EnumX for #name #ty_generics #where_clause {
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
        _ => panic!( "Only `enum`s can be `EnumX`." ),
    }
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

type Variants = HashSet<TypeIndex>;

struct EnumxFn {
    stack : Vec<EnumxTag>,
}

impl EnumxFn {
    fn new() -> Self {
        EnumxFn {
            stack : vec![ EnumxTag{ enum_: None }],
        }
    }

    fn peek( &mut self ) -> &mut EnumxTag { self.stack.last_mut().expect( "EnumxFn's stack cannot be empty" )}
    fn push( &mut self, tag: EnumxTag ) { self.stack.push( tag ); }
    fn pop( &mut self ) { self.stack.pop(); }
}

struct Enum {
    variants : HashSet<TypeIndex>,
}

struct EnumxTag {
    enum_ : Option<Enum>,
}

impl EnumxTag {
    fn new() -> Self {
        EnumxTag{ enum_: None }
    }

    fn parse_type_list( input: TokenStream ) -> syn::Result<TypeList> {
        Ok( syn::parse::<TypeList>( input )? )
    }
}

fn is_enumx_attr( attr: &Attribute ) -> bool {
    if attr.path.leading_colon.is_none() && attr.path.segments.len() == 1 {
        if attr.path.segments.first().unwrap().ident == "enumx" {
            return true;
        }
    }
    return false;
}

#[derive( PartialEq )]
enum TyPatAttr {
    None,
    GenVariants,
    Gen( Variants ),
}

fn parse_ty_pat_attr( attr: &Attribute ) -> Option<TyPatAttr> {
    if attr.path.leading_colon.is_none() && attr.path.segments.len() == 1 {
        if attr.path.segments.first().unwrap().ident == "ty_pat" {
            let ts = TokenStream::from( attr.tokens.clone() );
            let mut iter = ts.into_iter();
            if let Some( TokenTree::Group( group )) = iter.next() {
                let mut iter = group.stream().into_iter();
                match iter.next() {
                    Some( TokenTree::Ident( ident_throws )) => match ident_throws.to_string().as_str() {
                        "gen_variants" => {
                            return Some( TyPatAttr::GenVariants );
                        },
                        "gen" => {
                            let mut variants = HashSet::new();
                            let types = EnumxTag::parse_type_list( TokenStream::from_iter( iter )).expect("type list");
                            if types.0.len() == 0 {
                                return Some( TyPatAttr::GenVariants );
                            } else {
                                types.0.into_iter().for_each( |ty| { variants.insert( TypeIndex( ty, Cell::new(0) )); });
                                return Some( TyPatAttr::Gen( variants ));
                            }
                        },
                        _ => panic!("invalid #[ty_pat] argument: only #[ty_pat(gen_variants)] and #[ty_pat(gen)] are supported"),
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

impl VisitMut for EnumxFn {
    fn visit_type_mut( &mut self, node: &mut Type ) {
        visit_mut::visit_type_mut( self, node );

        if let Type::Macro( type_macro ) = node {
            let mac = &type_macro.mac;
            if mac.path.leading_colon.is_none() && mac.path.segments.len() == 1 {
                let seg = mac.path.segments.first().unwrap();
                if seg.arguments == PathArguments::None && seg.ident == "Enum" {
                    let ts = TokenStream::from( mac.tokens.clone() );
                    let mut variants = HashSet::new();
                    let types = EnumxTag::parse_type_list( ts ).expect("type list");
                    types.0.into_iter().for_each( |ty| {
                        let mut type_ = Type::Path( TypePath{ qself: None, path: ty });
                        self.visit_type_mut( &mut type_ );
                        match type_ {
                            Type::Path( type_path ) => { variants.insert( TypeIndex( type_path.path, Cell::new(0) )); },
                            _ => unreachable!(),
                        }
                    });

                    let variant = variants.iter().map( |type_index| &type_index.0 );
                    let ty: Type = parse_quote_spanned!( mac.span() => Enum!(#(#variant),*) );
                    *node = ty.clone();
                    self.peek().enum_ = Some( Enum{ variants }); // rewritable
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
                for (index, attr) in expr_match.attrs.iter().enumerate() {
                    if let Some( attrs ) = parse_ty_pat_attr( attr ) {
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
                            *path = parse_quote!{ __EnumxAdhocEnum::#_n };
                        };

                        match &mut arm.pat {
                            Pat::TupleStruct( pat_tuple_struct ) => {
                                if pat_tuple_struct.pat.elems.len() > 1 {
                                    panic!("#[enumx] supports tuple struct variant in newtype form only.");
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
                                        panic!("#[enumx] dost not support ident in arm with ref/mut/sub pattern.");
                                    } else {
                                        pat_ident = Some( ident.clone() );
                                    }
                                } else {
                                    panic!("#[enumx] unsupported pattern in match arm");
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
                    let enumx_tag = self.peek();
                    let unexhausted = match &ty_pat_attrs.0 {
                        TyPatAttr::None => Vec::new(),
                        TyPatAttr::GenVariants => enumx_tag.enum_.as_ref().expect("enumx!( variants ... ) parsed").variants.difference( &checked ).collect::<Vec<_>>(),
                        TyPatAttr::Gen( variants ) => variants.difference( &checked ).collect::<Vec<_>>(),
                    };

                    unexhausted.iter().for_each( |TypeIndex(_,i)| {
                        i.set( index );
                        let _n = ident( &format!( "_{}", index ));
                        expr_match.arms.push(
                            parse_quote_spanned!( match_span => __EnumxAdhocEnum::#_n(v) => v.into_enumx(), ),
                        );
                        index += 1;
                    });

                    let (unexhausted_types, unexhausted_indices): (Vec<_>, Vec<_>) = unexhausted.iter().map( |TypeIndex(t,i)| (t,i) ).unzip();
                    let unexhausted_indices = unexhausted_indices.iter().map( |n| ident( &format!( "_{}", n.get() )));

                    let (checked_types, checked_indices): (Vec<_>, Vec<_>) = checked.iter().map( |TypeIndex(t,i)| (t,i) ).unzip();
                    let checked_indices = checked_indices.iter().map( |n| ident( &format!( "_{}", n.get() )));

                    let adhoc_enum = quote_spanned!{ match_span =>
                        #[derive( enumx::EnumX )]
                        enum __EnumxAdhocEnum {
                            #( #checked_indices( #checked_types ), )*
                            #( #unexhausted_indices( #unexhausted_types ), )*
                        }
                    };

                    *expr = syn::parse::<Expr>( quote_spanned!{ match_span => {
                        #adhoc_enum
                        #expr
                    }}.into() ).unwrap();
                }
            },
            Expr::Closure( expr_closure ) => {
                let mut enumx_attr_index = None;
                for (index,attr) in expr_closure.attrs.iter().enumerate() {
                    if is_enumx_attr( &attr ) {
                        enumx_attr_index = Some( index );
                        self.push( EnumxTag::new() );
                        break;
                    }
                }

                self.visit_return_type_mut( &mut expr_closure.output );
                self.visit_expr_mut( &mut *expr_closure.body );

                enumx_attr_index.map( |index| {
                    expr_closure.attrs.remove( index );
                    self.pop();
                });
            },
            _ => {
                visit_mut::visit_expr_mut( self, expr );
            },
        }
    }

    fn visit_stmt_mut( &mut self, node: &mut Stmt ) {
        let mut enumx_attr_index = None;

        if let Stmt::Local( local ) = node {
            for (index,attr) in local.attrs.iter().enumerate() {
                if is_enumx_attr( &attr ) {
                    enumx_attr_index = Some( index );
                    self.push( EnumxTag::new() );
                    break;
                }
            }
        }

        visit_mut::visit_stmt_mut( self, node );

        if let Some( index ) = enumx_attr_index {
            if let Stmt::Local( local ) = node {
                local.attrs.remove( index );
                self.pop();
            }
        }
    }

    fn visit_expr_let_mut( &mut self, expr_let: &mut ExprLet ) {
        let mut enumx_attr_index = None;
        for (index,attr) in expr_let.attrs.iter().enumerate() {
            if is_enumx_attr( &attr ) {
                enumx_attr_index = Some( index );
                self.push( EnumxTag::new() );
                break;
            }
        }

        visit_mut::visit_expr_let_mut( self, expr_let );

        enumx_attr_index.map( |index| {
            expr_let.attrs.remove( index );
            self.pop();
        });
    }

    fn visit_item_fn_mut( &mut self, item_fn: &mut ItemFn ) {
        let mut enumx_attr_index = None;
        for (index,attr) in item_fn.attrs.iter().enumerate() {
            if is_enumx_attr( &attr ) {
                enumx_attr_index = Some( index );
                self.push( EnumxTag::new() );
                break;
            }
        }

        visit_mut::visit_item_fn_mut( self, item_fn );

        enumx_attr_index.map( |index| {
            item_fn.attrs.remove( index );
            self.pop();
        });
    }
}

/// tag an `fn` with `#[enumx]` to enable "type pattern matching" in `match` expressions that are tagged with `#[ty_pat]`/`#[ty_pat(gen_variants)]/`#[ty_pat(gen A,B,..)]`.
#[proc_macro_attribute]
pub fn enumx( _args: TokenStream, input: TokenStream ) -> TokenStream {
    if let Ok( mut item_fn ) = syn::parse::<ItemFn>( input.clone() ) {
        let mut enumx_fn = EnumxFn::new();

        enumx_fn.visit_signature_mut( &mut item_fn.sig );
        enumx_fn.visit_block_mut( &mut *item_fn.block );
        let expanded = quote_spanned!( item_fn.span() => #item_fn );
        return TokenStream::from( expanded );
    } else {
        panic!( "#[enumx] for functions, closures and try blocks only" );
    }
}
