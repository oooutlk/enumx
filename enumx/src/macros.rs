//! # The predfined macros for implementing traits for enums

/// For frequently used traits in std, this library provides macros such as
/// `impl_trait!()` to implement these traits without the need of writing trait
/// methods.
///
/// ## Syntax of `impl_trait!{}`
///
/// The full form is
///
/// ```text
/// impl_trait! {
///     _impl!( Generics ) Path::Of::Trait _for!( Type ) _where!( Clause )
/// }
/// ```
///
/// `Generics` and `Clause` are optional:
///
/// ```text
/// impl_trait!{ _impl!() Path::Of::Trait _for!( Type ) _where!() }
/// ```
/// and the wrapped macros can be omitted:
///
/// ```text
/// impl_trait!{ Path::Of::Trait _for!( Type )}
/// ```
///
/// ## Supported forms of types in `_for!()`
///
/// The `_for!()` macro supports two forms of types.
///
/// One is ad-hoc enums:
///
///
/// ```text
/// impl_trait!{ Path::Of::Trait _for!( Enum![1..=16] )}
/// ```
///
/// The other is the enum type definition copied in `_def!()` macro:
///
///
/// ```text
/// impl_trait!{ Path::Of::Trait _for!( _def!(
///     enum Value {
///         Bin( Vec<u8> ),
///         Text( String ),
///     }
/// ))}
/// ```
///
/// Note: `_def!()` does not define any enum, so `Value` should have been defined elsewhere.
///
/// ## The `_where!()` macro
///
/// You can write any where clause in this macro.
///
/// Note: you do not need write `_where!( _Variants!(): Path::Of::Trait )` which the
/// `impl_trait!{}` macro will generate it silently.
///
/// ## Traits in std prelude
///
/// `AsRef`
///
/// `AsMut`
///
/// `DoubleEndedIterator`
///
/// `ExactSizeIterator`
///
/// `Extend`
///
/// `Fn`
///
/// `Iterator`
///
/// The example of implementing `Iterator`:
///
/// ```text
/// impl_trait!{ Iterator _for!( Type )}
/// ```
///
/// The example of implementing `Fn`:
///
/// ```text
/// impl_trait!{ _impl!(Args) Fn<Args> _for!( Type )}
/// ```
///
/// ## Traits with full path
///
/// `std::error::Error`
///
/// `std::fmt::Debug`
///
/// `std::fmt::Display`
///
/// `std::iter::FusedIterator`
///
/// `std::iter::TrustedLen`
///
/// `std::io::BufRead`
///
/// `std::io::Read`
///
/// `std::io::Seek`
///
/// `std::io::Write`
///
/// `std::ops::Deref`
///
/// `std::ops::DerefMut`
///
/// `std::ops::Generator`
///
/// `std::ops::Index`
///
/// `std::ops::IndexMut`
///
/// `std::ops::RangeBounds`
///
/// The example of implementing `std::ops::Generator`:
///
/// ```text
/// impl_trait!{ _impl!(R) std::ops::Generator<R> _for!( Type )}
/// ```
///
/// ## Unstable traits
///
/// To implement these traits, the crate feature "unstable" should be opted in.
///
/// `Fn`
///
/// `std::iter::TrustedLen`
///
/// `std::ops::Generator`
///
/// ## macro inheritance
///
/// If the library users want to support extra traits, they can write the extra
/// implementations in their macro, and delegate other traits to
/// `enumx::impl_trait!()`.
///
/// ```rust
/// use enumx::export::{def_impls, impl_all_traits};
///
/// macro_rules! impl_trait {
///     ($(_impl!($($gen:tt),*))* ExtraTrait<$t:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
///         // omitted
///     };
///     ($($tt:tt)+) => {
///         enumx::impl_trait!{ $($tt)+ }
///     };
/// }
///
///
/// macro_rules! impl_super_traits {
///     ($(_impl!($($gen:tt),*))* ExtraTrait<$t:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
///         // omitted
///     };
///     ($($tt:tt)+) => {
///         enumx::impl_super_traits!{ $($tt)+ }
///     };
/// }
/// ```
#[macro_export]
macro_rules! impl_trait {
    ($(_impl!($($gen:tt),*))* AsRef<$t:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> AsRef<$t> for $($ty)+
                where _Variants!(): AsRef<$t>
                      $($(, $pred)*)*
            {
                fn as_ref( &self ) -> &$t { _match!( _variant!().as_ref() )}
            }
        }
    };
    ($(_impl!($($gen:tt),*))* AsMut<$t:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> AsMut<$t> for $($ty)+
                where _Variants!(): AsMut<$t>
                      $($(, $pred)*)*
            {
                fn as_mut( &mut self ) -> &mut $t { _match!( _variant!().as_mut() )}
            }
        }
    };
    ($(_impl!($($gen:tt),*))* Clone _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> Clone for $($ty)+
                where _Variants!(): Clone
                      $($(, $pred)*)*
            {
                fn clone( &self ) -> Self { _match!( _enum!( _variant!().clone() ))}
            }
        }
    };
    ($(_impl!($($gen:tt),*))* DoubleEndedIterator _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Item> DoubleEndedIterator for $($ty)+
                where _Variants!() : DoubleEndedIterator<Item=_Item>
                    , Self         : Iterator<Item=_Item>
                      $($(, $pred)*)*
            {
                fn next_back( &mut self ) -> Option<_Item> {
                    _match!( _variant!().next_back() )
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* ExactSizeIterator _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Item> ExactSizeIterator for $($ty)+
                where _Variants!() : ExactSizeIterator<Item=_Item>
                    , Self         : Iterator<Item=_Item>
                      $($(, $pred)*)*
            {
            }
        }
    };
    ($(_impl!($($gen:tt),*))* Extend<$a:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> Extend<$a> for $($ty)+
                where _Variants!(): Extend<$a>
                      $($(, $pred)*)*
            {
                fn extend<T>( &mut self, iter: T ) where T: IntoIterator<Item=$a> {
                    _match!( _variant!().extend( iter ))
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* Fn<$args:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Output> Fn<$args> for $($ty)+
                where _Variants!() : Fn<$args, Output=_Output>
                    ,         Self : FnMut<$args, Output=_Output>
                      $($(, $pred)*)*
            {
                extern "rust-call" fn call( &self, args: $args ) -> Self::Output {
                    _match!( _variant!().call( args ))
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* FnMut<$args:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Output> FnMut<$args> for $($ty)+
                where _Variants!() : FnMut<$args, Output=_Output>
                    ,         Self : FnOnce<$args, Output=_Output>
                      $($(, $pred)*)*
            {
                extern "rust-call" fn call_mut( &mut self, args: $args ) -> Self::Output {
                    _match!( _variant!().call_mut( args ))
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* FnOnce<$args:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Output> FnOnce<$args> for $($ty)+
                where _Variants!() : FnOnce<$args, Output=_Output>
                      $($(, $pred)*)*
            {
                type Output = _Output;
                extern "rust-call" fn call_once( self, args: $args ) -> Self::Output {
                    _match!( _variant!().call_once( args ))
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* Iterator _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Item> Iterator for $($ty)+
                where _Variants!(): Iterator<Item=_Item>
                      $($(, $pred)*)*
            {
                type Item = _Item;
                fn next( &mut self ) -> Option<_Item> { _match!( _variant!().next() )}
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::error::Error _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> std::error::Error for $($ty)+
                where _Variants!() : std::error::Error
                    ,         Self : std::fmt::Debug
                                   + std::fmt::Display
                      $($(, $pred)*)*
            {
               fn source( &self ) -> Option<&(dyn std::error::Error + 'static)> { _match!( _variant!().source() )}
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::fmt::Debug _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> std::fmt::Debug for $($ty)+
                where _Variants!() : std::fmt::Debug
                      $($(, $pred)*)*
            {
                fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
                    _match!( _variant!().fmt(f) )
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::fmt::Display _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> std::fmt::Display for $($ty)+
                where _Variants!(): std::fmt::Display
                      $($(, $pred)*)*
            {
                fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
                    _match!( _variant!().fmt(f) )
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::iter::FusedIterator _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Item> std::iter::FusedIterator for $($ty)+
                where _Variants!() : std::iter::FusedIterator<Item=_Item>
                    , Self         : Iterator<Item=_Item>
                      $($(, $pred)*)*
            {
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::iter::TrustedLen _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            unsafe impl<$($($gen,)*)* _Item> std::iter::TrustedLen for $($ty)+
                where _Variants!() : std::iter::TrustedLen<Item=_Item>
                    , Self         : Iterator<Item=_Item>
                      $($(, $pred)*)*
            {
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::io::BufRead _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> std::io::BufRead for $($ty)+
                where _Variants!() : std::io::BufRead
                    ,         Self : std::io::Read
                      $($(, $pred)*)*
            {
                fn fill_buf( &mut self ) -> std::io::Result<&[u8]> {
                    _match!( _variant!().fill_buf() )
                }

                fn consume( &mut self, amt: usize ) {
                    _match!( _variant!().consume( amt ))
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::io::Read _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> std::io::Read for $($ty)+
                where _Variants!(): std::io::Read
                      $($(, $pred)*)*
            {
                fn read( &mut self, buf: &mut [u8] ) -> std::io::Result<usize> {
                    _match!( _variant!().read( buf ))
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::io::Seek _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*>  std::io::Seek for $($ty)+
                where _Variants!(): std::io::Seek
                      $($(, $pred)*)*
            {
                fn seek( &mut self, pos: std::io::SeekFrom ) -> std::io::Result<u64> {
                    _match!( _variant!().seek( pos ))
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::io::Write _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> std::io::Write for $($ty)+
                where _Variants!(): std::io::Write
                      $($(, $pred)*)*
            {
                fn write( &mut self, buf: &[u8] ) -> std::io::Result<usize> {
                    _match!( _variant!().write( buf ))
                }

                fn flush( &mut self ) -> std::io::Result<()> {
                    _match!( _variant!().flush() )
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::ops::Deref _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Target> std::ops::Deref for $($ty)+
                where _Variants!(): std::ops::Deref<Target=_Target>
                      $($(, $pred)*)*
            {
                type Target = _Target;
                fn deref( &self ) -> &_Target { _match!( _variant!().deref() )}
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::ops::DerefMut _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Target> std::ops::DerefMut for $($ty)+
                where _Variants!() : std::ops::DerefMut<Target=_Target>
                    ,         Self : std::ops::Deref<Target=_Target>
                      $($(, $pred)*)*
            {
                fn deref_mut( &mut self ) -> &mut _Target { _match!( _variant!().deref_mut() )}
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::ops::Generator<$r:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Yield, _Return> std::ops::Generator<$r> for $($ty)+
                where _Variants!(): std::ops::Generator<$r,Yield=_Yield,Return=_Return>
                      $($(, $pred)*)*
            {
                type Yield = _Yield;
                type Return = _Return;
                fn resume( self: std::pin::Pin<&mut Self>, arg: $r ) -> std::ops::GeneratorState<Self::Yield, Self::Return> {
                    _match!( unsafe{ self.get_unchecked_mut() } =>
                         unsafe{ std::pin::Pin::new_unchecked( _variant!() )}.resume( arg )
                    )
                }
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::ops::Index<$idx:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Output> std::ops::Index<$idx> for $($ty)+
                where _Variants!(): std::ops::Index<$idx,Output=_Output>
                      $($(, $pred)*)*
            {
                type Output = _Output;
                fn index( &self, index: $idx ) -> &_Output { _match!( _variant!().index( index ))}
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::ops::IndexMut<$idx:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)* _Output> std::ops::IndexMut<$idx> for $($ty)+
                where _Variants!() : std::ops::IndexMut<$idx,Output=_Output>
                    ,         Self : std::ops::Index<$idx,Output=_Output>
                      $($(, $pred)*)*
            {
                fn index_mut( &mut self, index: $idx ) -> &mut _Output { _match!( _variant!().index_mut( index ))}
            }
        }
    };
    ($(_impl!($($gen:tt),*))* std::ops::RangeBounds<$t:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        def_impls! {
            impl<$($($gen,)*)*> std::ops::RangeBounds<$t> for $($ty)+
                where _Variants!(): std::ops::RangeBounds<$t>
                      $($(, $pred)*)*
            {
                fn start_bound( &self ) -> std::ops::Bound<&T> { _match!( _variant!().start_bound() )}
                fn end_bound( &self ) -> std::ops::Bound<&T> { _match!( _variant!().end_bound() )}
            }
        }
    };
}

/// The `impl_super_traits!{}` macro helps to implement the super trait(s) of the
/// mentioned trait, e.g. `impl_super_traits!{ _impl!(Args) Fn<Args> _for!( Type )}`
///  will implement `FnMut` and `FnOnce` for `Type`, but **NOT** `Fn`.
#[macro_export]
macro_rules! impl_super_traits {
    ($(_impl!($($gen:tt),*))* DoubleEndedIterator _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* Iterator _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($(_impl!($($gen:tt),*))* ExactSizeIterator _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* Iterator _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($(_impl!($($gen:tt),*))* Fn<$args:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* FnMut<$args> _for!($($ty)+) $(_where!($($pred)*))*);
        impl_trait!($(_impl!($($gen),*))* FnOnce<$args> _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($(_impl!($($gen:tt),*))* FnMut<$args:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* FnOnce<$args> _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($(_impl!($($gen:tt),*))* std::error::Error _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* std::fmt::Debug _for!($($ty)+) $(_where!($($pred)*))*);
        impl_trait!($(_impl!($($gen),*))* std::fmt::Display _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($(_impl!($($gen:tt),*))* std::iter::FusedIterator _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* Iterator _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($(_impl!($($gen:tt),*))* std::iter::TrustedLen _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* Iterator _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($(_impl!($($gen:tt),*))* std::io::BufRead _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* std::io::Read _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($(_impl!($($gen:tt),*))* std::ops::DerefMut _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* std::ops::Deref _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($(_impl!($($gen:tt),*))* std::ops::IndexMut _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        impl_trait!($(_impl!($($gen),*))* std::ops::Index _for!($($ty)+) $(_where!($($pred)*))*);
    };
    ($($_tt:tt)+) => {};
}

/// The `impl_all_traits!{}` macro does what `impl_trait!{}` and
/// `impl_super_traits!{}` does, e.g.
/// `impl_all_traits!{ _impl!(Args) Fn<Args> _for!( Type )}` will implement `Fn`,
/// `FnMut` and `FnOnce` for `Type`.
#[macro_export]
macro_rules! impl_all_traits {
    ($($tt:tt)+) => {
        impl_trait!{ $($tt)+ }
        impl_super_traits!{ $($tt)+ }
    };
}
