// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

#![recursion_limit="128"]

extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;

use syn::fold::Fold;

use syn::{ Block, Expr, ItemFn, parse_macro_input, parse_quote };

struct CexFn;

impl Fold for CexFn {
    fn fold_block( &mut self, block: Block ) -> Block {
        let block = CexBlock.fold_block( block );

        parse_quote! {{
            { #block }
        }}
    }
}

struct CexBlock;

impl Fold for CexBlock { 
    fn fold_expr( &mut self, expr: Expr ) -> Expr {
        if let Expr::Try( ref try1 ) = expr {
            if let Expr::Try( try2 ) = &*try1.expr {
                if let Expr::Try( try3 ) = &*try2.expr {
                    let expr = &*try3.expr;
                    return parse_quote!{  #expr.may_rethrow_log( log!() )? };
                } else {
                    let expr = &*try2.expr;
                    return parse_quote!{  #expr.may_throw_log( log!() )? };
                }
            }
        }
        syn::fold::fold_expr( self, expr )
    }
}

#[proc_macro_attribute]
pub fn cex( _args: TokenStream, input: TokenStream ) -> TokenStream {
    let input = parse_macro_input!( input as ItemFn );
    let output = CexFn.fold_item_fn( input );
    let expanded = quote!( #output );
    TokenStream::from( expanded )
}
