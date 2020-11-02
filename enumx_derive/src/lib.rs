// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

//! This project provides derive implementation for the enumx crate .
//!
//! See [enumx README](https://github.com/oooutlk/enumx/blob/master/enumx/README.md) for more.

#![recursion_limit="128"]

extern crate proc_macro;
use self::proc_macro::{TokenStream, TokenTree};

use indexmap::{
    IndexMap,
    IndexSet,
};

use quote::{
    quote,
    quote_spanned,
};

use std::{
    cell::Cell,
    hash::{Hash, Hasher},
    iter::FromIterator,
    mem,
    ops::Range,
};

use syn::{
    Attribute,
    DeriveInput,
    Expr,
    ExprClosure,
    ExprMacro,
    ExprRange,
    Fields,
    GenericParam,
    Generics,
    Ident,
    ImplItem,
    Item,
    ItemEnum,
    ItemFn,
    ItemImpl,
    ItemMacro,
    Lit,
    MacroDelimiter,
    Pat,
    Path,
    PathArguments,
    RangeLimits,
    ReturnType,
    Stmt,
    Token,
    Type,
    TypeParamBound,
    TypeMacro,
    TypeParam,
    TypePath,
    Variant,
    Visibility,
    WhereClause,
    braced,
    bracketed,
    export::Span,
    parse_macro_input,
    parse_quote,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
};

macro_rules! syntax_error {
    () => {
        panic!( "`enum`s deriving `FromVariant`/`Proto` should be in the form of \"enum MyEnum { Foo(Type), Bar(AnotherType),... }\"" );
    }
}

macro_rules! parse_quote_spanned {
    ( $span:expr => $($tt:tt)+ ) => {{
        let quoted = quote_spanned!( $span => $($tt)+ );
        parse_quote!( #quoted )
    }};
}

/// `Enum!( Type0, Type1, ..., TypeM )` denotes a predefined enum, the definition of which looks like:
///
/// ```rust
/// pub enum EnumN {
///     _0( Type0 ),
///     _1( Type1 ),
///     // ...
///     _M( TypeM ),
/// }
/// ```
///
/// where M+1 == N.
///
/// Especially, `Enum!()` denotes a never type `Enum0`:
///
/// ```rust
/// pub enum Enum0 {}
/// ```
#[proc_macro]
#[allow( non_snake_case )]
pub fn Enum( input: TokenStream ) -> TokenStream {
    let type_list = parse_macro_input!( input as TypeList );
    let types = type_list.0.iter();
    let name = make_ident( &format!( "Enum{}", types.len() ));
    let expanded = if types.len() == 0 {
        quote!( #name )
    } else {
        quote!( #name::<#(#types),*> )
    };
    expanded.into()
}

struct TypeList( Punctuated<Type, Token![,]> );

impl Parse for TypeList {
    fn parse( input: ParseStream ) -> syn::Result<Self> {
        let types = Punctuated::<Type, Token![,]>::parse_terminated( input )?;
        Ok( TypeList( types ))
    }
}

struct RangedEnums {
    attrs        : Vec<Attribute>,
    vis          : Visibility,
    ident        : Ident,
    range        : Range<usize>,
    where_clause : Option<WhereClause>,
}

enum EnumDef {
    Ranged(   RangedEnums ),
    Single(   ItemEnum    ),
    Snapshot( ItemEnum    ),
    None,
}

struct EnumDefImpls( EnumDef, Vec<ItemImpl> );

impl Parse for EnumDefImpls {
    fn parse( input: ParseStream ) -> syn::Result<Self> {
        if input.peek( Ident ) && input.peek2( Token![!] ) {
            let def = input.parse::<ItemMacro>()?;
            match path_ident_name( &def.mac.path ).as_deref() {
                Some( "_def" ) => (),
                _ => panic!( "expect _def!{{}}" ),
            }
            let item_enum = def.mac.parse_body::<ItemEnum>()?;

            let mut impls = vec![];
            while !input.is_empty() {
                impls.push( input.parse::<ItemImpl>()? );
            }

            return Ok( EnumDefImpls(
                EnumDef::Snapshot( item_enum ),
                impls,
            ));
        }

        if input.peek( Token![impl] ) || input.peek( Token![unsafe] ) && input.peek2( Token![impl] ) {
            let mut impls = vec![];
            while !input.is_empty() {
                impls.push( input.parse::<ItemImpl>()? );
            }

            return Ok( EnumDefImpls(
                EnumDef::None,
                impls,
            ));
        }

        let attrs = if input.peek( Token![#] ) {
            input.call( Attribute::parse_outer )?
        } else {
            vec![]
        };
        let vis = input.parse::<Visibility>()?;
        let enum_token = input.parse::<Token![enum]>()?;
        let ident = input.parse::<Ident>()?;

        if input.peek( Token![!] ) {
            input.parse::<Token![!]>()?;

            let content;
            bracketed!( content in input );
            let range = parse_range( content.parse::<ExprRange>()? )
                .expect("ranges should be expressed in literial integers");

            let where_clause = if input.peek( Token![where] ) {
                Some( input.parse::<WhereClause>()? )
            } else {
                None
            };

            if input.peek( Token![;] ) {
                input.parse::<Token![;]>()?;
            }

            let mut impls = vec![];
            while !input.is_empty() {
                impls.push( input.parse::<ItemImpl>()? );
            }

            return Ok( EnumDefImpls(
                EnumDef::Ranged( RangedEnums{ attrs, vis, ident, range, where_clause }),
                impls,
            ));
        } else {
            let content;
            let generics = input.parse::<Generics>()?;
            let brace_token = braced!( content in input );
            let variants = Punctuated::<Variant, Token![,]>::parse_terminated( &content )?;

            let mut impls = vec![];
            while !input.is_empty() {
                impls.push( input.parse::<ItemImpl>()? );
            }

            return Ok( EnumDefImpls(
                EnumDef::Single( ItemEnum{ attrs, vis, enum_token, ident, generics, brace_token, variants }),
                impls,
            ));
        }
    }
}

fn path_ident( path: &Path ) -> Option<Ident> {
    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return None;
    }

    let the_segment = path.segments.first().unwrap();
    if the_segment.arguments != PathArguments::None {
        return None;
    }

    Some( the_segment.ident.clone() )
}

fn path_ident_name( path: &Path ) -> Option<String> {
    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return None;
    }

    let the_segment = path.segments.first().unwrap();
    if the_segment.arguments != PathArguments::None {
        return None;
    }

    Some( the_segment.ident.to_string() )
}

fn parse_range( expr_range: ExprRange ) -> Option<Range<usize>> {
    let from = match expr_range.from {
        Some( from ) => loop {
            if let Expr::Lit( expr_lit ) = *from {
                if let Lit::Int( lit_int ) = expr_lit.lit {
                    if let Ok( from ) = lit_int.base10_parse::<usize>() {
                        break from;
                    }
                }
            }
            return None;
        },
        None => 0,
    };
    let to = match expr_range.to {
        Some( to ) => loop {
            if let Expr::Lit( expr_lit ) = *to {
                if let Lit::Int( lit_int ) = expr_lit.lit {
                    if let Ok( to ) = lit_int.base10_parse::<usize>() {
                        break to;
                    }
                }
            }
            return None;
        },
        None => 0,
    };
    let to = match expr_range.limits {
        RangeLimits::HalfOpen(_) => to,
        RangeLimits::Closed(_) => to+1,
    };

    Some( from..to )
}

/// defines enums, with the syntax support of
///
/// 1. generates a range of `enum`s in the syntax of `Enum![ range ]`
///
/// 2. implements traits for enums the variants of which have implemented
///
/// # Example
///
/// The macro below generates enum type `Enum1`, `Enum2` `Enum3` with 1,2,3 variants of generic types respectively.
/// The optional impl blocks implement Clone and Display for these enum types.
///
/// ```no_run
/// def_impls! {
///     #[derive( Debug, PartialEq )] pub enum Enum[ 1..=3 ] where _Variants!(): Clone;
///
///     impl Clone for Enum!(1..=3) where _Variants!(): Clone {
///         fn clone( &self ) -> Self {
///             _match!( _enum!( _variant!().clone() ))
///         }
///     }
///
///     impl Display for Enum!(1..=3) where _Variants!(): Display {
///         fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
///             _match!( _variant!().fmt(f) )
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn def_impls( input: TokenStream ) -> TokenStream {
    match parse_macro_input!( input as EnumDefImpls ) {
        EnumDefImpls( EnumDef::Ranged( RangedEnums{ attrs, vis, ident, range, where_clause }), item_impls ) => {
            let vnames = (0..range.end).map( |i| make_ident( &format!( "_{}", i )));
            let vtypes = (0..range.end).map( |i| make_ident( &format!( "_T{}", i )));
            let itypes = (0..range.end).map( |i| make_ident( &format!( "_I{}", i )));

            let is_proto = ident.to_string() == "__";

            let mut enums = Vec::<ItemEnum>::new();
            let mut impls = Vec::<ItemImpl>::new();

            for index in range.clone() {
                let ident = make_ident( &format!( "{}{}", ident, index ));

                let vnames1 = vnames.clone().take( index );
                let vnames2 = vnames1.clone();
                let vtypes1 = vtypes.clone().take( index );
                let vtypes2 = vtypes1.clone();
                let vtypes3 = vtypes1.clone();
                let vtypes4 = vtypes.clone();
                let itypes1 = itypes.clone().take( index );
                let itypes2 = itypes1.clone();
                let itypes3 = itypes1.clone();

                let generics: Option<Generics>;
                let mut expanded_where_clause: Option<WhereClause>;

                if index == 0 {
                    generics = None;
                    expanded_where_clause = None;
                } else {
                    generics = Some( parse_quote!( <#(#vtypes1),*> ));

                    expanded_where_clause = where_clause.clone();

                    EnumWhere::expand(
                        &mut expanded_where_clause,
                        vtypes.clone().map( |ident| {
                            let ty: Type = parse_quote!( #ident );
                            ty
                        }).collect::<Vec<_>>()
                    );
                }

                enums.push( parse_quote! {
                    #(#attrs)* #vis enum #ident #generics #expanded_where_clause {
                        #( #vnames1( #vtypes2 ), )*
                    }
                });

                if is_proto && index != 0 {
                    impls.push( parse_quote! {
                        impl<#(#itypes1,)* #(#vtypes3,)* Src, Dest> enumx::ExchangeFrom<Src, enumx::EnumToEnum<(#(#itypes2,)*)>> for Dest
                            where Src  : enumx::Proto<Type=enumx::proto::#ident #generics>
                                , Dest : #( enumx::ExchangeFrom<#vtypes4,#itypes3> )+*
                        {
                            fn exchange_from( src: Src ) -> Dest {
                                match src.into_proto() {
                                    #( enumx::proto::#ident::#vnames2(v) => Dest::exchange_from(v) ),*
                                }
                            }
                        }
                    });
                }
            }

            let mut expanded = TokenStream::from( quote!( #( #enums )* #( #impls )* ));
            for item_impl in item_impls {
                Extend::<TokenTree>::extend( &mut expanded, expand_enum_impl( item_impl, None ));
            }
            expanded.into()
        },
        EnumDefImpls( EnumDef::Single( item_enum ), item_impls ) => {
            let mut expanded_impls = TokenStream::new();
            for item_impl in item_impls {
                expanded_impls.extend( expand_enum_impl( item_impl, Some( &item_enum )));
            }

            let mut expanded_item_enum = TokenStream::from( quote!( #item_enum ));
            Extend::<TokenTree>::extend( &mut expanded_item_enum, expanded_impls );
            expanded_item_enum.into()
        },
        EnumDefImpls( EnumDef::Snapshot( item_enum ), item_impls ) => {
            let mut expanded_impls = TokenStream::new();
            for item_impl in item_impls {
                expanded_impls.extend( expand_enum_impl( item_impl, Some( &item_enum )));
            }
            expanded_impls
        },
        EnumDefImpls( EnumDef::None, item_impls ) => {
            let mut expanded_impls = TokenStream::new();
            for item_impl in item_impls {
                expanded_impls.extend( expand_enum_impl( item_impl, None ));
            }
            expanded_impls
        },
    }
}

/// Derives the trait `enumx::FromVariant` for user defined enum types.
#[proc_macro_derive( FromVariant )]
pub fn derive_from_variant( input: TokenStream ) -> TokenStream {
    let input: DeriveInput = syn::parse( input ).unwrap();

    match input.data {
        syn::Data::Enum( ref data ) => {
            if data.variants.len() == 0 {
                return TokenStream::new();
            }

            let name = &input.ident;

            let variant_names = data.variants.iter().map( |v| &v.ident );

            let (ref impl_generics, ref ty_generics, ref where_clause) = input.generics.split_for_impl();

            let variant_types = data.variants.iter().map( |ref v| {
                if let syn::Fields::Unnamed( ref fields ) = v.fields {
                    let mut iter = fields.unnamed.iter();
                    if iter.len() == 1 {
                        let field = iter.next().unwrap();
                        return &field.ty;
                    }
                }
                syntax_error!();
            });
            let variant_types = variant_types.clone();

            let mut impls = Vec::<ItemImpl>::new();
            let mut indices = Vec::<Ident>::new();

            for (i, (vname,vtype) ) in variant_names.zip( variant_types ).enumerate() {
                let v: Type = parse_quote!( [(); #i] );
                indices.push( make_ident( &format!( "_I{}", i )));
                impls.push( parse_quote! {
                    impl #impl_generics enumx::FromVariant<#vtype,#v> for #name #ty_generics #where_clause {
                        fn from_variant( src: #vtype ) -> Self { #name::#vname( src )}
                    }
                });
            }

            let impls = impls.iter();

            let expanded = quote!( #(#impls)* );
            expanded.into()
        },
        _ => panic!( "Only `enum`s can be constructed `FromVariant`s." ),
    }
}

/// Since `enum`s in Rust do not have prototypes, this macro derives them.
#[proc_macro_derive( Proto )]
pub fn derive_proto( input: TokenStream ) -> TokenStream {
    let input: DeriveInput = syn::parse( input ).unwrap();

    match input.data {
        syn::Data::Enum( ref data ) => {
            let name = &input.ident;

            let named_variant_fp = data.variants.iter().map( |v| &v.ident );
            let named_variant_tp = data.variants.iter().map( |v| &v.ident );

            let variant_cnt = data.variants.len();

            let protox = &make_ident( &format!( "__{}", variant_cnt ));

            let named_enum_fp = (0..variant_cnt).map( |_| name );
            let named_enum_tp = named_enum_fp.clone();

            let unamed_enum_fp = (0..variant_cnt).map( |_| protox );
            let unamed_enum_tp = unamed_enum_fp.clone();

            let unamed_variant_fp = (0..variant_cnt).map( |index| make_ident( &format!( "_{}", index )));
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

            let enumx_ty: syn::Type = parse_quote!{ enumx::proto::#protox<#(#variant_ty),*> };

            let expanded = quote! {
                impl #impl_generics enumx::Proto for #name #ty_generics #where_clause {
                    type Type = #enumx_ty;

                    fn from_proto( src: #enumx_ty ) -> Self {
                        match src {
                            #( enumx::proto::#unamed_enum_fp::#unamed_variant_fp(v) => #named_enum_fp::#named_variant_fp(v), )*
                        }
                    }

                    fn into_proto( self ) -> #enumx_ty {
                        match self {
                            #( #named_enum_tp::#named_variant_tp(v) => enumx::proto::#unamed_enum_tp::#unamed_variant_tp(v), )*
                        }
                    }
                }
            };
            expanded.into()
        },
        _ => panic!( "Only `enum`s can have `Proto`-type." ),
    }
}

/// derives `enumx::Exchange` trait for custom defined `enum`s
///
/// # Examples
///
///```no_run
/// #[derive( Exchange )]
/// enum Three<A, B, C> {
///     First(A),
///     Second(B),
///     Third(C),
/// }
///```
#[proc_macro_derive( Exchange )]
pub fn derive_exchange( input: TokenStream ) -> TokenStream {
    let mut output = derive_from_variant( input.clone() );
    output.extend( derive_proto( input ));
    output
}

fn make_ident( sym: &str ) -> Ident {
    Ident::new( sym, Span::call_site() )
}

fn add_generics( generics: &mut Generics, type_param: TypeParam ) {
    generics.params.push( GenericParam::Type( type_param ));
}

struct TypePathList( Vec<Path> );

impl Parse for TypePathList {
    fn parse( input: ParseStream ) -> syn::Result<Self> {
        let types = Punctuated::<Type, Token![,]>::parse_terminated( input )?;
        Ok( TypePathList( types.into_iter().map( |ty| match ty {
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

type Variants = IndexSet<TypeIndex>;

struct Enum {
    variants : IndexSet<TypeIndex>,
}

struct EnumxTag {
    enum_ : Option<Enum>,
}

impl EnumxTag {
    fn new() -> Self {
        EnumxTag{ enum_: None }
    }

    fn parse_type_path_list( input: TokenStream ) -> syn::Result<TypePathList> {
        Ok( syn::parse::<TypePathList>( input )? )
    }
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
                    Some( TokenTree::Ident( ident )) => match ident.to_string().as_str() {
                        "gen_variants" => {
                            return Some( TyPatAttr::GenVariants );
                        },
                        "gen" => {
                            let mut variants = IndexSet::new();
                            let types = EnumxTag::parse_type_path_list( TokenStream::from_iter( iter )).expect("type list");
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

impl VisitMut for EnumxTag {
    fn visit_type_mut( &mut self, node: &mut Type ) {
        visit_mut::visit_type_mut( self, node );

        if let Type::Macro( type_macro ) = node {
            let mac = &type_macro.mac;
            if mac.path.leading_colon.is_none() && mac.path.segments.len() == 1 {
                let seg = mac.path.segments.first().unwrap();
                if seg.arguments == PathArguments::None && seg.ident == "Enum" {
                    let ts = TokenStream::from( mac.tokens.clone() );
                    let mut variants = IndexSet::new();
                    let types = EnumxTag::parse_type_path_list( ts ).expect("type list");
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
                    self.enum_ = Some( Enum{ variants }); // rewritable
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
                    expr_match.expr = Box::new( parse_quote_spanned!( match_span => enumx::ExchangeFrom::exchange_from( #match_expr ) ));
                    let mut index = 0_u32;
                    let checked = expr_match.arms.iter_mut().fold( IndexMap::<Path,Cell<u32>>::new(), |mut acc, arm| {
                        let mut add_type_pattern = |path: &mut Path| {
                            let mut nth = index;
                            acc.entry( path.clone() )
                                .and_modify( |n| { nth = n.get() })
                                .or_insert( Cell::new( nth ));
                            if nth == index { index += 1; }

                            let _n = make_ident( &format!( "_{}", nth ));
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

                    let checked = IndexSet::<TypeIndex>::from_iter( checked.clone().into_iter().map( |(t,i)| TypeIndex(t,i) ));
                    let unexhausted = match &ty_pat_attrs.0 {
                        TyPatAttr::None => Vec::new(),
                        TyPatAttr::GenVariants => self.enum_.as_ref().expect("def_impls!( variants ... ) parsed").variants.difference( &checked ).collect::<Vec<_>>(),
                        TyPatAttr::Gen( variants ) => variants.difference( &checked ).collect::<Vec<_>>(),
                    };

                    unexhausted.iter().for_each( |TypeIndex(_,i)| {
                        i.set( index );
                        let _n = make_ident( &format!( "_{}", index ));
                        expr_match.arms.push(
                            parse_quote_spanned!( match_span => __EnumxAdhocEnum::#_n(v) => v.exchange_into(), ),
                        );
                        index += 1;
                    });

                    let (unexhausted_types, unexhausted_indices): (Vec<_>, Vec<_>) = unexhausted.iter().map( |TypeIndex(t,i)| (t,i) ).unzip();
                    let unexhausted_indices = unexhausted_indices.iter().map( |n| make_ident( &format!( "_{}", n.get() )));

                    let (checked_types, checked_indices): (Vec<_>, Vec<_>) = checked.iter().map( |TypeIndex(t,i)| (t,i) ).unzip();
                    let checked_indices = checked_indices.iter().map( |n| make_ident( &format!( "_{}", n.get() )));

                    let adhoc_enum = quote_spanned!{ match_span =>
                        #[derive( ::enumx::FromVariant, ::enumx::Proto )]
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
            _ => {
                visit_mut::visit_expr_mut( self, expr );
            },
        }
    }
}

/// tag an `fn` with `#[enumx]` to enable "type pattern matching" in `match` expressions that are tagged with `#[ty_pat]`/`#[ty_pat(gen_variants)]/`#[ty_pat(gen A,B,..)]`.
#[proc_macro_attribute]
pub fn enumx( _args: TokenStream, input: TokenStream ) -> TokenStream {
    if let Ok( mut stmt ) = syn::parse::<Stmt>( input.clone() ) {
        match stmt {
            Stmt::Item( Item::Fn( mut item_fn )) => {
                let mut enumx_tag = EnumxTag::new();
                enumx_tag.visit_signature_mut( &mut item_fn.sig );
                enumx_tag.visit_block_mut( &mut *item_fn.block );

                let expanded = quote_spanned!( item_fn.span() => #item_fn );
                return expanded.into();
            },
            Stmt::Local(_) => {
                let mut enumx_tag = EnumxTag::new();
                visit_mut::visit_stmt_mut( &mut enumx_tag, &mut stmt );

                let expanded = quote_spanned!( stmt.span() => #stmt );
                return expanded.into();
            },
            _ => (),
        }
    } else if let Ok( mut expr_closure ) = syn::parse::<ExprClosure>( input ) {
        let mut enumx_tag = EnumxTag::new();
        enumx_tag.visit_return_type_mut( &mut expr_closure.output );
        enumx_tag.visit_expr_mut( &mut *expr_closure.body );

        let expanded = quote_spanned!( expr_closure.span() => #expr_closure );
        return expanded.into();
    }

    panic!( "#[enumx] for functions, closures, local let-bindings and impl blocks only!!!" );
}

#[derive( Default )]
struct IterativeImpl {
    self_ty : Option<Type>,
    index   : usize,
    vnames  : Vec<Ident>,
    vtypes  : Vec<Type>,
}

impl VisitMut for IterativeImpl {
    fn visit_expr_mut( &mut self, expr: &mut Expr ) {
        visit_mut::visit_expr_mut( self, expr );

        if let Expr::Macro( expr_macro ) = expr {
            if let Some( name ) = path_ident_name( &expr_macro.mac.path ) {
                if name == "_match" {
                    let MatchInput{ self_expr, arm_expr } = expr_macro.mac.parse_body::<MatchInput>()
                        .expect("def_impls!{ impl }: expects _match!( expr => expr ) or _match!( expr )");
                    let mut match_expander = MatchExpander::from( self );
                    let arm_exprs = (0..=self.index).fold( Vec::new(), |mut arm_exprs, i| {
                        match_expander.index = i;
                        let mut arm_expr = arm_expr.clone();
                        match_expander.visit_expr_mut( &mut arm_expr );
                        arm_exprs.push( arm_expr );
                        arm_exprs
                    });

                    let self_ty = &self.self_ty;
                    let enum_ty = (0..=self.index).map( |_| self_ty.clone() ).collect::<Vec<_>>();
                    let vnames = self.vnames.iter();

                    *expr = parse_quote! {
                        match #self_expr {
                            #( #enum_ty::#vnames( __variant ) => #arm_exprs ),*
                        }
                    };
                }
            }
        }
    }
}

struct MatchInput {
    self_expr : Expr,
    arm_expr  : Expr,
}

impl Parse for MatchInput {
    fn parse( input: ParseStream ) -> syn::Result<Self> {
        let expr = input.parse::<Expr>()?;
        if input.peek( Token![=>] ) {
            input.parse::<Token![=>]>()?;
            let arm_expr = input.parse::<Expr>()?;
            Ok( MatchInput {
                self_expr : expr,
                arm_expr  ,
            })
        } else {
            Ok( MatchInput {
                self_expr : parse_quote!( self ),
                arm_expr  : expr,
            })
        }
    }
}

struct MatchExpander {
    self_ty : Option<Type>,
    index   : usize,
    vtypes  : Vec<Type>,
}

impl MatchExpander {
    fn from( iterative_impl: &IterativeImpl ) -> Self {
        MatchExpander {
            self_ty : iterative_impl.self_ty.clone(),
            index   : 0,
            vtypes  : iterative_impl.vtypes.clone(),
        }
    }
}

impl VisitMut for MatchExpander {
    fn visit_expr_mut( &mut self, expr: &mut Expr ) {
        visit_mut::visit_expr_mut( self, expr );

        if let Expr::Macro( expr_macro ) = expr {
            if let Some( name ) = path_ident_name( &expr_macro.mac.path ) {
                match name.as_str() {
                    "_variant" => {
                        if expr_macro.mac.tokens.is_empty() || expr_macro.mac.tokens.to_string() == "self" {
                            *expr = parse_quote_spanned!( expr.span() => __variant );
                        } else {
                            let other = expr_macro.mac.parse_body::<Expr>()
                                .expect("def_impls!{ impl }: _variant!()'s argument should be an expression");
                            let self_ty = self.self_ty.as_ref().unwrap();
                            let vname = make_ident( &format!( "_{}", self.index ));
                            *expr = parse_quote_spanned! { expr.span() =>
                                if let #self_ty::#vname(v) = #other {
                                    v
                                } else {
                                    panic!("def_impls!{ impl }: _variant!()'s argument should be of the same variant with `self`");
                                }
                            };
                        }
                    },
                    "_enum" => {
                        let mut inner = expr_macro.mac.parse_body::<Expr>()
                            .expect("def_impls!{ impl }: _enum!()'s argument should be an expression");
                        self.visit_expr_mut( &mut inner );
                        let self_ty = self.self_ty.as_ref().unwrap();
                        let vname = make_ident( &format!( "_{}", self.index ));
                        *expr = parse_quote_spanned! { expr.span() =>
                            #self_ty::#vname( #inner )
                        };
                    },
                    _ => (),
                }
            }
        }
    }

    fn visit_type_mut( &mut self, node: &mut Type ) {
        visit_mut::visit_type_mut( self, node );

        if let Type::Macro( type_macro ) = node {
            if let Some( name ) = path_ident_name( &type_macro.mac.path ) {
                if name == "_Variant" {
                    if type_macro.mac.tokens.is_empty() {
                        let vtype = &self.vtypes[ self.index ];
                        *node = parse_quote!( #vtype );
                    } else {
                        panic!("def_impls!{ impl }: Variant!() should have no argument");
                    }
                }
            }
        }
    }
}

struct EnumWhere {
    nth_variant       : usize,
    variant_types     : Vec<Type>,
    contains_variants : bool,
}

impl EnumWhere {
    fn new( variant_types: Vec<Type> ) -> Self {
        EnumWhere {
            nth_variant       : 0,
            variant_types     ,
            contains_variants : false,
        }
    }

    fn expand( where_clause: &mut Option<WhereClause>, variant_types: Vec<Type> ) {
        if let Some( where_clause ) = where_clause.as_mut() {
            let mut predicates = Punctuated::new();
            mem::swap( &mut predicates, &mut where_clause.predicates );
            for predicate in predicates.into_iter() {
                let variant_count = variant_types.len();
                let mut enum_where = EnumWhere::new( variant_types.clone() );
                enum_where.visit_where_predicate( &predicate );
                if enum_where.contains_variants {
                    for i in 0..variant_count {
                        let mut predicate = predicate.clone();
                        enum_where.nth_variant = i;
                        enum_where.visit_where_predicate_mut( &mut predicate );
                        where_clause.predicates.push( predicate );
                    }
                } else {
                    where_clause.predicates.push( predicate );
                }
            }
        }
    }
}

impl<'a> Visit<'a> for EnumWhere {
    fn visit_type_macro( &mut self, type_macro: &TypeMacro ) {
        visit::visit_type_macro( self, type_macro );

        if let Some( name ) = path_ident_name( &type_macro.mac.path ) {
            if name == "_Variants" {
                if type_macro.mac.tokens.is_empty() {
                    self.contains_variants = true;
                } else {
                    panic!("def_impls!{ impl }: Variants!() should have no argument");
                }
            }
        }
    }
}

impl VisitMut for EnumWhere {
    fn visit_type_mut( &mut self, node: &mut Type ) {
        visit_mut::visit_type_mut( self, node );

        if let Type::Macro( type_macro ) = node {
            if let Some( name ) = path_ident_name( &type_macro.mac.path ) {
                if name == "_Variants" {
                    if type_macro.mac.tokens.is_empty() {
                        let ty = &self.variant_types[ self.nth_variant ];
                        *node = parse_quote!( #ty );
                    } else {
                        panic!("def_impls!{ impl }: Variants!() should have no argument");
                    }
                }
            }
        }
    }
}

fn expand_enum_impl( mut item_impl: ItemImpl, item_enum: Option<&ItemEnum> ) -> TokenStream {
    let mut iterative_impl = IterativeImpl::default();
    let mut def_enum_in_self_ty = None;
    let mut item_enum = item_enum;

    loop {
        if let Type::Macro( type_macro ) = &*item_impl.self_ty {
            let ident = path_ident( &type_macro.mac.path ).expect("def_impls!{}: the path of the macro as self type in impl blocks should be an ident");

            if ident == "_def" {
                def_enum_in_self_ty = Some( type_macro.mac.parse_body::<ItemEnum>().expect( "def_impls!{}: expect _def!{ ItemEnum }" ));
                item_enum = def_enum_in_self_ty.as_ref();
                break;
            }

            let mut range;
            match type_macro.mac.delimiter {
                MacroDelimiter::Bracket(_) => match type_macro.mac.parse_body::<ExprRange>() {
                    Ok( expr_range ) => {
                        range = parse_range( expr_range ).expect( &format!( "expect {ident}![ literial_range ], e.g. {ident}![ 1..=16 ]", ident = ident ));
                    },
                    Err( _ ) => panic!( "expect {}![ expr_range ]", ident ),
                },
                _ => panic!( "expect {}![]", ident ),
            }

            if range.start > 0 {
                range.start -= 1;
            }
            if range.end > 0 {
                range.end -= 1;
            }

            let mut vtypes = Vec::<Type>::new();

            for i in 0..range.start {
                let gen_ty = make_ident( &format!( "_T{}", i ));
                let gen_ty: Type = parse_quote!( #gen_ty );
                {
                    let gen_ty = &gen_ty;
                    add_generics( &mut item_impl.generics, parse_quote!( #gen_ty ));
                }
                vtypes.push( gen_ty.clone() );
                iterative_impl.vtypes.push( gen_ty );
                let vname = make_ident( &format!( "_{}", i ));
                iterative_impl.vnames.push( vname );
            }

            let mut impls = Vec::<ItemImpl>::new();

            for index in range {
                let gen_ty = make_ident( &format!( "_T{}", index ));
                let gen_ty: Type = parse_quote!( #gen_ty );
                {
                    let gen_ty = &gen_ty;
                    add_generics( &mut item_impl.generics, parse_quote!( #gen_ty ));
                }

                vtypes.push( gen_ty.clone() );
                iterative_impl.vtypes.push( gen_ty );
                let vname = make_ident( &format!( "_{}", index ));
                iterative_impl.vnames.push( vname );

                let mut item_impl = item_impl.clone();
                let self_ty = make_ident( &format!( "{}{}", ident, index+1 ));
                {
                    let self_ty = &self_ty;
                    item_impl.self_ty = parse_quote!( #self_ty <#(#vtypes),*> );
                    iterative_impl.self_ty = Some( parse_quote!( #self_ty ));
                }

                EnumWhere::expand( &mut item_impl.generics.where_clause, vtypes.clone() );

                item_impl.items = item_impl.items.into_iter().map( |mut item| {
                    match item {
                        ImplItem::Method( ref mut impl_item_method ) => {
                            iterative_impl.index = index;
                            iterative_impl.visit_block_mut( &mut impl_item_method.block );
                        },
                        _ => (),
                    }
                    item
                }).collect();
                impls.push( item_impl );
            }

            let expanded = quote_spanned!( item_impl.span() => #( #impls )* );
            return expanded.into();
        }
        break;
    }

    if let Some( item_enum ) = item_enum {
        let enum_ident = &item_enum.ident;
        iterative_impl.self_ty = Some( parse_quote!( #enum_ident ));

        if def_enum_in_self_ty.is_some() {
            let (_, ty_generics, _) = item_enum.generics.split_for_impl();
            item_impl.self_ty = parse_quote!( #enum_ident #ty_generics );
        }

        for variant in item_enum.variants.iter() {
            loop {
                if let Fields::Unnamed( fields_unnamed ) = &variant.fields {
                    if fields_unnamed.unnamed.len() == 1 {
                        let field = fields_unnamed.unnamed.first().unwrap();
                        if field.ident.is_none() && field.colon_token.is_none() {
                            iterative_impl.vnames.push( variant.ident.clone() );
                            iterative_impl.vtypes.push( field.ty.clone() );
                            break;
                        }
                    }
                }
                panic!( "expect `{}( VariantType )`", variant.ident );
            }
        }

        let variant_count = item_enum.variants.len();
        EnumWhere::expand( &mut item_impl.generics.where_clause, iterative_impl.vtypes.clone() );

        item_impl.items = item_impl.items.into_iter().map( |mut item| {
            match item {
                ImplItem::Method( ref mut impl_item_method ) => {
                    iterative_impl.index = variant_count-1;
                    iterative_impl.visit_block_mut( &mut impl_item_method.block );
                },
                _ => (),
            }
            item
        }).collect();

        let expanded = quote!( #item_impl );
        return expanded.into();
    } else {
        panic!( "The enum's variants are unknown to `def_impls!{}`. Consider using `_def!{}` to provide the variants of the enum defined elsewhere." );
    };
}

#[derive( Default )]
struct VariantLabel {
    which_impl_trait : Option<Ident>,
    label            : Option<Ident>,
}

impl Parse for VariantLabel {
    fn parse( input: ParseStream ) -> syn::Result<Self> {
        if input.is_empty() {
            Ok( VariantLabel::default() )
        } else {
            let ident = Some( input.parse::<Ident>()? );

            if input.is_empty() {
                Ok( VariantLabel{ which_impl_trait: None,  label: ident })
            } else if input.peek( Token![=>] ) {
                input.parse::<Token![=>]>()?;
                if input.peek( Token![_] ) {
                    input.parse::< Token![_] >()?;
                    if !input.is_empty() {
                        panic!("#[variant( which_impl_trait => _ )] expected, extra tokens after `_` got ");
                    }
                    Ok( VariantLabel{ which_impl_trait: ident, label: None })
                } else {
                    Ok( VariantLabel{ which_impl_trait: ident, label: Some( input.parse::<Ident>()? )})
                }
            } else {
                panic!("#[variant]: expected `.`");
            }
        }
    }
}

fn take_variant_label( attrs: &mut Vec<Attribute>, which_impl_trait: &Option<Ident> ) -> Option<VariantLabel> {
    let mut variant_label = None::<VariantLabel>;

    let mut attributes = Vec::new();
    mem::swap( &mut attributes, attrs );

    for attr in attributes {
        if attr.path.leading_colon.is_none() && attr.path.segments.len() == 1 {
            if attr.path.segments.first().unwrap().ident == "variant" {
                let ts = TokenStream::from( attr.tokens.clone() );
                let mut iter = ts.into_iter();
                if let Some( TokenTree::Group( group )) = iter.next() {
                    let inner_stream = group.stream();
                    let label: VariantLabel = syn::parse::<VariantLabel>( inner_stream )
                        .expect("expect #[variant( tag )] or #[variant( impl_tag.variant_tag )]");
                    if &label.which_impl_trait == which_impl_trait {
                        variant_label = Some( label );
                    } else {
                        attrs.push( attr );
                        continue;
                    }
                } else {
                    variant_label = Some( VariantLabel::default() );
                }
                continue;
            }
        }
        attrs.push( attr );
    }

    return variant_label;
}

struct Sum {
    which_impl_trait : Option<Ident>,
    variant_count    : usize,
    labeled_variants : IndexMap<Ident, usize>,
}

impl Sum {
    fn new( which_impl_trait: Option<Ident> ) -> Self {
        Sum {
            which_impl_trait ,
            variant_count    : 0,
            labeled_variants : IndexMap::new(),
        }
    }
}

impl VisitMut for Sum {
    fn visit_expr_mut( &mut self, expr: &mut Expr ) {
        macro_rules! wrap {
            ( $expr:expr, $expr_:expr ) => {{
                let mut vname = None;
                let mut is_variant = true;
                loop {
                    let self_variant_count = self.variant_count;
                    if let Some( variant_label ) = take_variant_label( &mut $expr_.attrs, &self.which_impl_trait ) {
                        if let Some( label ) = variant_label.label {
                            if label != "" {
                                let variant_count = self.labeled_variants
                                    .entry( label )
                                    .or_insert( self_variant_count );
                                if *variant_count != self_variant_count {
                                    vname = Some( make_ident( &format!( "_{}", *variant_count )));
                                    break;
                                }
                            }
                        }
                    } else {
                        is_variant = false;
                        break;
                    }

                    vname = Some( make_ident( &format!( "_{}", self.variant_count )));
                    self.variant_count += 1;
                    break;
                }
                if is_variant {
                    let expr_ = $expr_;
                    *$expr = parse_quote!{
                        __SumType::#vname( #expr_ )
                    }
                } else {
                    visit_mut::visit_expr_mut( self, expr );
                }
            }};
        }

        match expr {
            Expr::Array( expr_array ) => wrap!( expr, expr_array ),
            Expr::Assign( expr_assign ) => wrap!( expr, expr_assign ),
            Expr::AssignOp( expr_assign_op ) => wrap!( expr, expr_assign_op ),
            Expr::Async( expr_async ) => wrap!( expr, expr_async ),
            Expr::Await( expr_await ) => wrap!( expr, expr_await ),
            Expr::Binary( expr_binary ) => wrap!( expr, expr_binary ),
            Expr::Block( expr_block ) => wrap!( expr, expr_block ),
            Expr::Box( expr_box ) => wrap!( expr, expr_box ),
            Expr::Break( expr_break ) => wrap!( expr, expr_break ),
            Expr::Call( expr_call ) => wrap!( expr, expr_call ),
            Expr::Cast( expr_cast ) => wrap!( expr, expr_cast ),
            Expr::Closure( expr_closure ) => wrap!( expr, expr_closure ),
            Expr::Continue( expr_break ) => wrap!( expr, expr_break ),
            Expr::Field( expr_field ) => wrap!( expr, expr_field ),
            Expr::ForLoop( expr_for_loop ) => wrap!( expr, expr_for_loop ),
            Expr::Group( expr_group ) => wrap!( expr, expr_group ),
            Expr::If( expr_if ) => wrap!( expr, expr_if ),
            Expr::Index( expr_index ) => wrap!( expr, expr_index ),
            Expr::Let( expr_let ) => wrap!( expr, expr_let ),
            Expr::Lit( expr_lit ) => wrap!( expr, expr_lit ),
            Expr::Loop( expr_loop ) => wrap!( expr, expr_loop ),
            Expr::Macro( expr_macro ) => wrap!( expr, expr_macro ),
            Expr::Match( expr_match ) => wrap!( expr, expr_match ),
            Expr::MethodCall( expr_method_call ) => wrap!( expr, expr_method_call ),
            Expr::Paren( expr_paren ) => wrap!( expr, expr_paren ),
            Expr::Path( expr_path ) => wrap!( expr, expr_path ),
            Expr::Range( expr_range ) => wrap!( expr, expr_range ),
            Expr::Reference( expr_reference ) => wrap!( expr, expr_reference ),
            Expr::Repeat( expr_repeat ) => wrap!( expr, expr_repeat ),
            Expr::Return( expr_return ) => wrap!( expr, expr_return ),
            Expr::Struct( expr_struct ) => wrap!( expr, expr_struct ),
            Expr::Try( expr_try ) => wrap!( expr, expr_try ),
            Expr::TryBlock( expr_try_block ) => wrap!( expr, expr_try_block ),
            Expr::Tuple( expr_tuple ) => wrap!( expr, expr_tuple ),
            Expr::Type( expr_type ) => wrap!( expr, expr_type ),
            Expr::Unary( expr_unary ) => wrap!( expr, expr_unary ),
            Expr::Unsafe( expr_unsafe ) => wrap!( expr, expr_unsafe ),
            Expr::Verbatim( _ ) => (),
            Expr::While( expr_while ) => wrap!( expr, expr_while ),
            Expr::Yield( expr_yield ) => wrap!( expr, expr_yield ),
            _ => (),
        }
    }

    fn visit_item_mut( &mut self, _item: &mut Item ) {}
}

struct ReplaceIdent {
    placehoder : Ident,
    real_ident : Ident,
}

impl ReplaceIdent {
    fn new( placehoder: Ident, real_ident: Ident ) -> Self {
        ReplaceIdent{ placehoder, real_ident }
    }
}

impl VisitMut for ReplaceIdent {
    fn visit_ident_mut( &mut self, ident: &mut Ident ) {
        if *ident == self.placehoder {
            *ident = self.real_ident.clone();
        }
    }

    fn visit_item_mut( &mut self, _item: &mut Item ) {}
}

#[derive( Default )]
struct SumArgs {
    which_impl_trait : Option<Ident>,
    impl_generics    : Option<Punctuated<GenericParam, Token![,]>>,
    trait_path       : Option<Path>,
    enum_prefix      : Option<Ident>,
}

impl Parse for SumArgs {
    fn parse( input: ParseStream ) -> syn::Result<Self> {
        if input.is_empty() {
            Ok( SumArgs::default() )
        } else {
            let which_impl_trait = if input.peek( Ident ) && input.peek2( Token![=>] ) {
                let which_impl_trait = input.parse::<Ident>()?;
                input.parse::<Token![=>]>()?;
                Some( which_impl_trait )
            } else {
                None
            };

            if input.peek( Token![impl] ) {
                input.parse::<Token![impl]>()?;

                let impl_generics = if input.peek( Token![<] ) {
                    input.parse::<Token![<]>()?;
                    let impl_generics = Punctuated::<GenericParam, Token![,]>::parse_terminated( input )?;
                    input.parse::<Token![>]>()?;
                    Some( impl_generics )
                } else {
                    None
                };

                let trait_path = Some( input.parse::<Path>()? );

                if input.is_empty() {
                    Ok( SumArgs {
                        which_impl_trait ,
                        impl_generics    ,
                        trait_path       ,
                        enum_prefix      : None,
                    })
                } else {
                    input.parse::<Token![for]>()?;
                    Ok( SumArgs {
                        which_impl_trait ,
                        impl_generics    ,
                        trait_path       ,
                        enum_prefix      : Some( input.parse::<Ident>()? ),
                    })
                }
            } else if input.peek( Ident ) {
                Ok( SumArgs {
                        which_impl_trait ,
                        impl_generics    : None,
                        trait_path       : None,
                        enum_prefix      : Some( input.parse::<Ident>()? ),
                })
            } else {
                input.parse::<Token![_]>()?;
                if !input.is_empty() {
                    panic!("#[sum( which_impl_trait => _ )] expected, extra tokens after `_` got ");
                }
                Ok( SumArgs {
                        which_impl_trait ,
                        impl_generics    : None,
                        trait_path       : None,
                        enum_prefix      : None,
                })
            }
        }
    }
}

/// collects all returned values in exit branches of the function, into an enum type, returning an `impl` trait
///
/// #Examples
///
/// ```no_run
/// #[sum]
/// fn f( cond: bool ) -> impl Clone {
///     if cond {
///         #[variant] 1_i32
///     } else {
///         #[variant] "false"
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn sum( args: TokenStream, input: TokenStream ) -> TokenStream {
    let SumArgs{ which_impl_trait, mut impl_generics, trait_path, enum_prefix } = parse_macro_input!( args as SumArgs );
    let mut item_fn = parse_macro_input!( input as ItemFn );

    let mut sum = Sum::new( which_impl_trait );
    sum.visit_block_mut( &mut item_fn.block );

    let variant_count = sum.variant_count;

    let (enum_basename, enum_ident);
    let enum_def;
    let enum_impl: Option<ExprMacro>;

    match enum_prefix {
        None => {
            enum_basename = make_ident( "__SumType" );
            enum_ident = make_ident( &format!( "__SumType{}", variant_count ));
            let enum_basename = &enum_basename;

            let trait_path = trait_path.unwrap_or_else( || loop {
                let type_impl_trait = loop {
                    if let ReturnType::Type( _, ty ) = &item_fn.sig.output {
                        if let Type::ImplTrait( it ) = &**ty {
                            break it.clone();
                        }
                    }
                    panic!("#[sum] fn should return `impl Trait`");
                };

                let mut iter = type_impl_trait.bounds.iter();
                match iter.next() {
                    Some( bound ) => if let TypeParamBound::Trait( trait_bound ) = bound {
                        let mut path = trait_bound.path.clone();
                        path.segments.last_mut().map( |path_seg| {
                            match path_seg.ident.to_string().as_str() {
                                "Fn" | "FnMut" | "FnOnce" => {
                                    path_seg.arguments = PathArguments::AngleBracketed( parse_quote_spanned!( path_seg.arguments.span() => <Args> ));
                                    let mut punct = Punctuated::new();
                                    punct.push( parse_quote_spanned!( path_seg.arguments.span() => Args ));
                                    impl_generics = Some( punct );
                                },
                                _ => {
                                    path_seg.arguments = PathArguments::None;
                                },
                            }
                        });
                        break path;
                    },
                    None => panic!("#[sum]: expected trait bound after `-> impl`"),
                }
            });
            enum_def = Some( quote!( enum #enum_basename![#variant_count..=#variant_count]; ));
            enum_impl = Some( parse_quote!( impl_all_traits!{ _impl!(#impl_generics) #trait_path _for!( #enum_basename![#variant_count..=#variant_count] )}));
        },
        Some( enum_prefix ) => {
            enum_ident = make_ident( &format!( "{}{}", enum_prefix.to_string(), variant_count ));
            enum_def = None;
            enum_impl = None;
        },
    }

    let enum_def = match enum_def {
        Some( enum_def ) => {
            let token_stream = def_impls( enum_def.into() );
            Some( parse_macro_input!( token_stream as ItemEnum ))
        },
        None => None,
    };

    let placeholder = make_ident( "__SumType" );
    ReplaceIdent::new( placeholder, enum_ident ).visit_block_mut( &mut item_fn.block );

    let mut block = parse_quote!({});
    mem::swap( &mut block, &mut item_fn.block );
    item_fn.block = parse_quote_spanned! { block.span() => {
        #enum_def
        #enum_impl
        #block
    }};

    let expanded = quote!( #item_fn );
    expanded.into()
}

struct SumErr;

impl VisitMut for SumErr {
    fn visit_expr_mut( &mut self, expr: &mut Expr ) {
        visit_mut::visit_expr_mut( self, expr );

        if let Expr::Try( expr_try ) = expr {
            let the_expr = &*expr_try.expr;
            *expr = parse_quote! {
                match #the_expr {
                    Ok(  v ) => v,
                    Err( e ) => return Err( #[variant] e ),
                }
            };
        }
    }

    fn visit_item_mut( &mut self, _item: &mut Item ) {}
}

/// To translate the `expr?` expressions in a different manner than the Rust's
/// default:
///
/// ```rust
/// match expr {
///     Ok( value ) => value,
///     Err( error ) => return Err( #[variant] error ),
/// }
/// ```
///
/// A `#[sum]` tagged function should be tagged with `#[sum_err]` if it contains `?`
/// expressions.
///
/// ## Example
///
/// ```rust
/// #[sum_err]
/// #[sum( impl Clone )]
/// fn foo( branch: i32 ) -> Result<(), impl Clone> {
///     match branch % 3 {
///         0 => Ok(()),
///         1 => Ok( Err( 0 )? ),
///         2 => Ok( Err( "lorum" )? ),
///         _ => unreachable!(),
///     }
/// }
/// ```
///
/// Note: put `#[sum_err]` **before** `#[sum]`.
#[proc_macro_attribute]
pub fn sum_err( _args: TokenStream, input: TokenStream ) -> TokenStream {
    let mut item_fn = parse_macro_input!( input as ItemFn );
    SumErr.visit_block_mut( &mut *item_fn.block );
    let expanded = quote!( #item_fn );
    expanded.into()
}
