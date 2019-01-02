// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

#![recursion_limit="128"]

extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;

use syn::{ Abi, Attribute, Block, FnArg, FnDecl, Generics, Ident, ItemFn, ReturnType, Token, Type, Variant, Visibility, parenthesized, parse_macro_input, parse_quote };
use syn::parse::{ Parse, ParseStream };
use syn::punctuated::Punctuated;
use syn::token::{ self, Async, Comma, Const, Paren, RArrow, Unsafe };
 
macro_rules! syntax_err {
    () => { panic!("A `cex` function should use `throws` to enumerate all its error types."); }
}

struct CexOutput {
    arrow  : RArrow,
    ok_ty  : Type,
    errors : Punctuated<Variant, Comma>,
}

impl Parse for CexOutput {
    fn parse( input: ParseStream ) -> syn::parse::Result<Self> {
        let arrow  : RArrow = input.parse()?;
        let ok_ty  : Type   = input.parse()?;
        let throws : Ident  = input.parse()?;
        if throws.to_string() != "throws" {
            syntax_err!();
        }

        let mut errors = Punctuated::<Variant,Comma>::new();
        loop {
            errors.push_value( input.parse()? );
            if input.peek( token::Brace ) {
                break;
            }
            errors.push_punct( input.parse()? );
        }

        Ok( CexOutput {
            arrow,
            ok_ty,
            errors,
        })
    }
}

struct CexFn {
    attrs       : Vec<Attribute>,
    vis         : Visibility,
    constness   : Option<Const>,
    unsafety    : Option<Unsafe>,
    asyncness   : Option<Async>,
    abi         : Option<Abi>,
    ident       : Ident,
    fn_token    : token::Fn,
    generics    : Generics,
    paren_token : Paren,
    inputs      : Punctuated<FnArg, Comma>,
    output      : CexOutput,
    block       : Box<Block>,
}

impl Parse for CexFn {
    fn parse( input: ParseStream ) -> syn::parse::Result<Self> {
        let attrs = input.call( Attribute::parse_outer )?;
        let vis: Visibility = input.parse()?;
        let constness: Option<Token![const]> = input.parse()?;
        let unsafety: Option<Token![unsafe]> = input.parse()?;
        let asyncness: Option<Token![async]> = input.parse()?;
        let abi: Option<Abi> = input.parse()?;
        let fn_token: Token![fn] = input.parse()?;
        let ident: Ident = input.parse()?;
        let generics: Generics = input.parse()?;

        let content;
        let paren_token = parenthesized!( content in input );
        let inputs = content.parse_terminated(FnArg::parse)?;

        let output = input.parse()?;

        let block: Box<Block> = input.parse()?;

        Ok( CexFn {
            attrs,
            vis,
            constness,
            unsafety,
            asyncness,
            abi,
            ident,
            fn_token,
            generics,
            paren_token,
            inputs,
            output,
            block,
        })
    }
}

#[proc_macro]
pub fn cex( input: TokenStream ) -> TokenStream {
    fn define_tilde( input: TokenStream ) -> TokenStream {
        use proc_macro::{TokenTree,Group};

        let mut acc = TokenStream::new();
        let mut tilde = false;
        for tt in input {
            match tt {
                TokenTree::Group( group ) => {
                    if tilde {
                        tilde = false;
                    }
                    acc.extend( TokenStream::from(
                        TokenTree::Group( Group::new(
                            group.delimiter(),
                            define_tilde( group.stream() )))))
                },
                TokenTree::Punct( punct ) => {
                    if punct.as_char() == '~' {
                        if tilde {
                            acc.extend( TokenStream::from( quote!( .may_rethrow() )));
                            tilde = false;
                        } else {
                            tilde = true;
                        }
                    } else {
                        if tilde {
                            acc.extend( TokenStream::from( quote!( .may_throw() )));
                            acc.extend( TokenStream::from( TokenTree::Punct( punct )));
                            tilde = false;
                        } else {
                            acc.extend( TokenStream::from( TokenTree::Punct( punct )));
                        }
                    }
                },
                _ => {
                    if tilde {
                        tilde = false;
                    }
                    acc.extend( TokenStream::from( tt ));
                },
            }
        }
        if tilde {
            acc.extend( TokenStream::from( quote!( .may_throw() )));
        }
        acc
    }

    let input = define_tilde( input );

    let CexFn {
        attrs,
        vis,
        constness,
        unsafety,
        asyncness,
        abi,
        ident,
        fn_token,
        generics,
        paren_token,
        inputs,
        output,
        block,
    } = parse_macro_input!( input as CexFn );

    let CexOutput {
        arrow,
        ok_ty,
        errors,
    } = output;

    let variants = errors.iter();
    let fn_mod = ident.clone();
    let fn_mod_ = fn_mod.clone();
    let ret_ty: Type = parse_quote! {
        Result<#ok_ty, Cex<#fn_mod::Err>>
    };

    let output = ReturnType::Type( arrow, Box::new( ret_ty ));

    let decl = Box::new( FnDecl {
        fn_token,
        generics,
        paren_token,
        inputs,
        variadic: None,
        output,
    });

    let visibility = vis.clone();

    let item_fn = ItemFn {
        attrs,
        vis,
        constness,
        unsafety,
        asyncness,
        abi,
        ident,
        decl,
        block,
    };

    let expanded = quote! {
        #visibility mod #fn_mod_ {
            use super::*;
            use enumx::prelude::*;
            #[derive( enumx_derive::Exchange, Debug )]
            pub enum Err {
                #(#variants),*
            }
        }

        #item_fn
    };

    expanded.into()
}
