#![cfg_attr( test, feature(
    fn_traits,
    generator_trait,
    generators,
    proc_macro_hygiene,
    stmt_expr_attributes,
    trusted_len,
    unboxed_closures,
))]

use enumx::export::*;

def_impls! {
    #[derive( Exchange )]
    enum Enum![1..=3];
}

#[test]
fn syntax() {
    fn _foo( i: i32 ) -> Enum!( String, &'static str ) {
        if i >= 0 { String::from("positive").exchange_into() } else { "negative".exchange_into() }
    }

    #[enumx] fn _bar( i: i32 ) -> Enum!( String, usize, &'static str ) {
        #[ty_pat] match _foo(i) {
            String(s) => if s.len() % 2 == 0 {
                s.len().exchange_into()
            } else {
                s.exchange_into()
            },
            TyPat::<&'static str>(s) => s.exchange_into(),
        }
    }

    #[enumx] fn _bar_v2( i: i32 ) -> Enum!( String, usize, &'static str ) {
        #[ty_pat(gen_variants)] match _foo(i) {
            String(s) => if s.len() % 2 == 0 {
                s.len().exchange_into()
            } else {
                s.exchange_into()
            },
        }
    }

    #[enumx] fn _bar_v3( i: i32 ) -> Enum!( String, usize, &'static str ) {
        #[ty_pat(gen &'static str)] match _foo(i) {
            String(s) => if s.len() % 2 == 0 {
                s.len().exchange_into()
            } else {
                s.exchange_into()
            },
        }
    }

    #[enumx] fn _baz( i: i32 ) -> Enum!( String, &'static str ) {
        #[ty_pat(gen_variants)] match _bar(i) {
            usize(v) => v.to_string().exchange_into(),
        }
    }

    let _bar = #[enumx] |i: i32| -> Enum!( String, usize, &'static str ) {
        #[ty_pat] match _foo(i) {
            String(s) => if s.len() % 2 == 0 {
                s.len().into_enum()
            } else {
                s.into_enum()
            },
            TyPat::<&'static str>(s) => s.into_enum(),
        }
    };

    #[enumx] let _bar_v2 = |i: i32| -> Enum!( String, usize, &'static str ) {
        #[ty_pat(gen_variants)] match _foo(i) {
            String(s) => if s.len() % 2 == 0 {
                s.len().into_enum()
            } else {
                s.into_enum()
            },
        }
    };
}

#[test]
fn sum_syntax() {
    #[sum] fn _if( cond: bool ) -> impl Clone {
        if cond {
            #[variant] 1_i32
        } else {
            #[variant] 0_u32
        }
    }

    #[sum]
    fn _fn( cond: bool ) -> impl Fn()->i32 {
        if cond {
            #[variant] || 1
        } else {
            #[variant] || 0
        }
    }

    struct ESI0;
    impl Iterator for ESI0 {
        type Item = ();
        fn next( &mut self ) -> Option<()> { None }
        fn size_hint( &self ) -> (usize, Option<usize>) { (0,Some(0)) }
    }
    impl ExactSizeIterator for ESI0 {}

    struct ESI1;
    impl Iterator for ESI1 {
        type Item = ();
        fn next( &mut self ) -> Option<()> { None }
        fn size_hint( &self ) -> (usize, Option<usize>) { (0,Some(0)) }
    }
    impl ExactSizeIterator for ESI1 {}

    #[sum] fn _exact_size_iterator( cond: bool ) -> impl ExactSizeIterator {
        if cond {
            #[variant] ESI0
        } else {
            #[variant] ESI1
        }
    }

    #[sum( ok  => impl Clone )]
    #[sum( err => impl Clone )]
    fn _sum_okeys_and_errors( branch: i32 ) -> Result<impl Clone, impl Clone> {
        match branch % 4 {
            0 => Ok(  #[variant( ok  => _ )] branch ),
            1 => Ok(  #[variant( ok  => _ )] () ),
            2 => Err( #[variant( err => _ )] branch ),
            3 => Err( #[variant( err => _ )] () ),
            _ => unreachable!(),
        }
    }

    #[sum_err]
    #[sum( impl Clone )]
    fn _sum_err( branch: i32 ) -> Result<(), impl Clone> {
        match branch % 3 {
            0 => Ok(()),
            1 => Ok( Err( 0 )? ),
            2 => Ok( Err( "lorum" )? ),
            _ => unreachable!(),
        }
    }
}

#[test]
fn impl_for_predefined() {
    impl_trait!{ _impl!(T) AsRef<T> _for!( Enum![1..=3] )}
    impl_trait!{ _impl!(T) AsMut<T> _for!( Enum![1..=3] )}
    impl_trait!{ DoubleEndedIterator _for!( Enum![1..=3] )}
    impl_trait!{ ExactSizeIterator _for!( Enum![1..=3] )}
    impl_trait!{ _impl!(A) Extend<A> _for!( Enum![1..=3] )}
    impl_trait!{ _impl!(Args) Fn<Args> _for!( Enum![1..=3] )}
    impl_trait!{ Iterator _for!( Enum![1..=3] )}
    impl_trait!{ std::error::Error _for!( Enum![1..=3] )}
    impl_trait!{ std::fmt::Debug _for!( Enum![1..=3] )}
    impl_trait!{ std::fmt::Display _for!( Enum![1..=3] )}
    impl_trait!{ std::iter::FusedIterator _for!( Enum![1..=3] )}
    impl_trait!{ std::iter::TrustedLen _for!( Enum![1..=3] )}
    impl_trait!{ std::io::BufRead _for!( Enum![1..=3] )}
    impl_trait!{ std::io::Read _for!( Enum![1..=3] )}
    impl_trait!{ std::io::Seek _for!( Enum![1..=3] )}
    impl_trait!{ std::io::Write _for!( Enum![1..=3] )}
    impl_trait!{ std::ops::Deref _for!( Enum![1..=3] )}
    impl_trait!{ std::ops::DerefMut _for!( Enum![1..=3] )}
    impl_trait!{ _impl!(R) std::ops::Generator<R> _for!( Enum![1..=3] )}
    impl_trait!{ _impl!(Idx) std::ops::Index<Idx> _for!( Enum![1..=3] )}
    impl_trait!{ _impl!(Idx) std::ops::IndexMut<Idx> _for!( Enum![1..=3] )}
    impl_trait!{ _impl!(T) std::ops::RangeBounds<T> _for!( Enum![1..=3] )}
}

#[test]
#[allow( dead_code )]
fn single_impl() {
    enum Value {
        Code( i32 ),
        Text( String ),
    }
    impl_trait! { std::fmt::Debug _for!(
        _def!{ enum Value {
            Code( i32 ),
            Text( String ),
        }}
    )}
}

#[test]
#[allow( dead_code )]
fn single_impl_with_generics() {
    enum Value<Other> {
        Code( i32 ),
        Text( &'static str ),
        Other( Other ),
    }
    impl_trait! {
        _impl!(Other) std::fmt::Debug _for!(
            _def!{ enum Value<Other> {
                Code( i32 ),
                Text( &'static str ),
                Other( Other ),
            }}
        )
    }
}

#[test]
#[allow( dead_code )]
fn impl_trait_for_def() {
    enum Value<Other> {
        Code( i32 ),
        Text( &'static str ),
        Other( Other ),
    }
    impl_trait! {
        _impl!(Other) std::fmt::Debug _for!(
            _def!{ enum Value<Other> {
                Code( i32 ),
                Text( &'static str ),
                Other( Other ),
            }}
        )
    }
}
