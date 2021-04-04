#![cfg_attr( feature = "unstable", feature( proc_macro_hygiene, stmt_expr_attributes, try_blocks ))]

macro_rules! define_tests {
    () => {
        mod function {
            use super::*;

            #[cex] fn throws_never() -> Result!(()) { ret!(); }

            #[cex] fn throws_void() -> Result!( () throws () ) { throw!(); }

            #[cex] fn throws_str() -> Result!( i32 throws &'static str ) { throw!( "oops" ); ret!(0); }

            #[cex] fn throws_string() -> Result!( i32 throws String ) { throw!( String::from( "oops" )); ret!(0); }

            #[cex] fn throws_i32() -> Result!( i32 throws i32 ) { throw!( 0xbadbeef ); ret!(0); }

            struct UnitError;
            #[cex] fn throws_unit_error() -> Result!( i32 throws UnitError ) { throw!( UnitError ); ret!(0); }

            struct ErrorCode( i32 );
            #[cex] fn throws_error_code() -> Result!( i32 throws ErrorCode ) { throw!( ErrorCode( 0xbadbeef )); ret!(0); }

            struct A;
            #[cex] fn throws_a() -> Result!( () throws A ) { throw!(A); }

            struct B;
            #[cex] fn throws_ab() -> Result!( () throws A,B ) { throw!(A); throw!(B); }

            struct C;
            #[cex] fn throws_abc() -> Result!( () throws A,B,C ) { throw!(A); throw!(B); throw!(C); }

            #[cex] fn throws_ba_call_ab() -> Result!( () throws B,A ) { ret!( throws_ab()? ); }

            #[cex] fn throws_bca_call_ab() -> Result!( () throws B,C,A ) { throws_ab()?; throw!(C); }

            #[cex] fn throws_ab_call_bca() -> Result!( () throws A,B ) {
                throws_bca_call_ab().or_else( |err| #[ty_pat] match err {
                    B(b) => throw!(b),
                    A(a) => throw!(a),
                    C(_) => ret!(),
                })
            }

            #[cex] fn throws_ab_call_bca_v2() -> Result!( () throws A,B ) {
                throws_bca_call_ab().or_else( |err| #[ty_pat(gen_throws)] match err {
                    C(_) => ret!(),
                })
            }

            #[cex] fn throws_ab_call_bca_v3() -> Result!( () throws A,B ) {
                throws_bca_call_ab().or_else( |err| #[ty_pat(gen B)] match err {
                    C(_) => ret!(),
                    A(a) => throw!(a),
                })
            }

            #[cex] fn throws_ab_call_bca_v4() -> Result!( () throws A,B ) {
                throws_bca_call_ab().or_else( |err| #[ty_pat(gen B,A)] match err {
                    C(_) => ret!(),
                })
            }

            #[cfg( feature = "unstable" )]
            fn closure_throws_abc() {
                let _f = #[cex] || -> Result!( i32 throws A,B,C ) { throw!(A); throw!(B); throw!(C); ret!(0); };
            }

            #[cfg( feature = "unstable" )]
            fn try_throws_abc_v2() {
                #[cex] let _result: Result!( () throws A,B,C ) = try{ throws_abc()? };
            }

            #[cex] fn nested_results() -> Result!( Result!( () throws C ) throws A,B ) {
                throw!( A );
                throw!( B );
                // throw!( C ); // compile error
            }
        }
    };
}

#[allow( dead_code, unreachable_code )]
mod test {
    use enumx::export::*;
    use enumx::predefined::*;
    use cex_derive::cex;
    use cex::*;

    define_tests!();
}

#[allow( dead_code, unreachable_code )]
mod test_log {
    use enumx::export::*;
    use enumx::predefined::*;
    use cex_derive::cex_log as cex;
    use cex::*;

    define_tests!();
}

#[allow( dead_code, unreachable_code )]
mod test_env_log {
    use enumx::export::*;
    use enumx::predefined::*;
    use cex_derive::cex_env_log as cex;
    use cex::*;

    define_tests!();
}
