#![cfg_attr( test, feature(
    fn_traits,
    generator_trait,
    generators,
    proc_macro_hygiene,
    stmt_expr_attributes,
    trusted_len,
    unboxed_closures,
))]

#![allow( unused_imports, unused_macros )]

use enumx::export::{def_impls, impl_all_traits};

def_impls! {
    enum Enum![1..=3];
}

macro_rules! impl_trait {
    ($(_impl!($($gen:tt),*))* std::fmt::Write _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> std::fmt::Write for $($ty)+
                where _Variants!(): std::fmt::Write
                      $($(, $pred)*)*
            {
                fn write_str( &mut self, s: &str ) -> std::fmt::Result {
                    _match!( _variant!().write_str( s ))
                }
            }
        }
    };
    ($($tt:tt)+) => {
        enumx::impl_trait!{ $($tt)+ }
    };
}

macro_rules! impl_super_traits {
    ($(_impl!($($gen:tt),*))* ExtraTrait<$t:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        // omitted
    };
    ($($tt:tt)+) => {
        enumx::impl_super_traits!{ $($tt)+ }
    };
}

#[test]
fn impl_std_fmt_write() {
    impl_trait!{ std::fmt::Write _for!( Enum![1..3] )}
}
