use enumx::export::*;
use enumx::predefined::*;
use cex::*;

pub struct A; // was: FatalError1
pub struct B; // was: FatalError2
pub struct C; // was: LocalError1
pub struct D; // was: LocalError2

#[cex] fn may_throw_a()    -> Result!( () throws A   ) { ret!(()); }
#[cex] fn may_throw_b()    -> Result!( () throws B   ) { ret!(()); }
#[cex] fn may_throw_ab()   -> Result!( () throws A,B ) { ret!(()); }
#[cex] fn may_throw_abcd() -> Result!(
                 Result!( () throws C,D ) throws A,B ) { ret!( Ok(()) ); }

fn handle_c( _c: C ) {}
fn handle_d( _d: D ) {}

#[cex]
fn returns_only_fatal_errors_v4() -> Result!( () throws A,B ) {
    may_throw_a()?;
    may_throw_b()?;
    // may_throw_c()?; // <----- compile error, too
    may_throw_ab()?;

    may_throw_abcd()?
    .or_else( |e| #[ty_pat] match e { // <----- ty_pat means type pattern match
        C(c) => Ok( handle_c( c )),
        D(d) => Ok( handle_d( d )),
    })
}

#[test]
fn returns_only_fatal_errors_v4_works() {
    assert_eq!( returns_only_fatal_errors_v4().ok().unwrap(), () );
}

#[cex] fn may_throw_abcd_v2() -> Result!( () throws A,B,C,D ) { ret!(()); }

#[cex]
fn returns_only_fatal_errors_v5() -> Result!( () throws A,B ) {
    may_throw_a()?;
    may_throw_b()?;
    // may_throw_c()?; // <----- compile error
    may_throw_ab()?;

    may_throw_abcd_v2()
    .or_else( |e| #[ty_pat(gen_throws)] match e { // generates arms to throw A,B
        C(c) => Ok( handle_c( c )),
        D(d) => Ok( handle_d( d )),
    })
}

#[test]
fn returns_only_fatal_errors_v5_works() {
    assert_eq!( returns_only_fatal_errors_v5().ok().unwrap(), () );
}
