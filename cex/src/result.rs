use enumx::ExchangeInto;
use crate::log::{Log, LogAgent, Logger, ToLog};

pub struct _WrapOk;
pub struct _WrapErr<Index>( Index );
pub struct _MapErr <Index>( Index );

pub trait Ret<Type,Index> {
    fn ret( self ) -> Type;
}

impl<T,E> Ret<Result<T,E>,_WrapOk> for T {
    fn ret( self ) -> Result<T,E> { Ok( self )}
}

impl<T,E,F,Index> Ret<Result<T,F>,_MapErr<Index>> for Result<T,E>
    where E: ExchangeInto<F,Index>
{
    fn ret( self ) -> Result<T,F> {
        self.map_err( |e| e.exchange_into() )
    }
}

pub trait Throw<Type,Index> {
    fn throw( self ) -> Type;
}

impl<T,E,F,I> Throw<Result<T,F>,_WrapErr<I>> for E
    where E: ExchangeInto<F,I>
{
    fn throw( self ) -> Result<T,F> {
        Err( self.exchange_into() )
    }
}

pub struct _ToLog        <Index>( Index );
pub struct _Log          <Index>( Index );
pub struct _MapErrToLog  <Index>( Index );
pub struct _MapErrLog    <Index>( Index );

pub trait RetLog<Type,Agent,Index>
    where Agent: LogAgent
{
    fn ret_log( self, item: impl Fn() -> Agent::Item ) -> Type;
}

impl<T,E,A> RetLog<Result<T,E>,A,_WrapOk> for T
    where A: LogAgent
{
    fn ret_log( self, _item: impl Fn() -> A::Item ) -> Result<T,E> { Ok( self )}
}

impl<T,E,F,A,I> RetLog<Result<T,F>,A,_MapErrToLog<I>> for Result<T,E>
    where A       : LogAgent
        , E       : ToLog<A>
        , Log<E,A>: ExchangeInto<F,I>
{
    fn ret_log( self, item: impl Fn() -> A::Item ) -> Result<T,F> {
        self.map_err( |e| e.to_log( item() ).exchange_into() )
    }
}

impl<T,E,F,A,I> RetLog<Result<T,F>,A,_MapErrLog<I>> for Result<T,E>
    where A : LogAgent
        , E : Logger<A>
            + ExchangeInto<F,I>
{
    fn ret_log( self, item: impl Fn() -> A::Item ) -> Result<T,F> {
        self.map_err( |e| e.log( item() ).exchange_into() )
    }
}

pub trait ThrowLog<Type,Agent,Index>
    where Agent: LogAgent,
{
    fn throw_log( self, item: impl Fn() -> Agent::Item ) -> Type;
}

impl<T,E,F,A,I> ThrowLog<Result<T,F>,A,_ToLog<I>> for E
    where A       : LogAgent
        , E       : ToLog<A>
        , Log<E,A>: ExchangeInto<F,I>
{
    fn throw_log( self, item: impl Fn() -> A::Item ) -> Result<T,F> {
        Err( self.to_log( item() ).exchange_into() )
    }
}

impl<T,E,F,A,I> ThrowLog<Result<T,F>,A,_Log<I>> for E
    where A : LogAgent
        , E : Logger<A>
            + ExchangeInto<F,I>
{
    fn throw_log( self, item: impl Fn() -> A::Item ) -> Result<T,F> {
        Err( self.log( item() ).exchange_into() )
    }
}

pub trait MapErrorLog<T,E,F,A,I>
    where Self : Into<Result<T,E>>
        , A    : LogAgent
        , E    : ThrowLog<Result<T,F>,A,I>
{
    fn map_error_log( self, item: impl Fn() -> A::Item ) -> Result<T,F> {
        match self.into() {
            Ok( t ) => Ok( t ),
            Err( e ) => e.throw_log( item ),
        }
    }
}

impl<R,T,E,F,A,I> MapErrorLog<T,E,F,A,I> for R
    where R : Into<Result<T,E>>
        , A : LogAgent
        , E : ThrowLog<Result<T,F>,A,I>
{
}
