#![feature(fn_traits, unboxed_closures)]
#![feature(const_fn)]
#![feature(const_generics)]
#![feature(unsized_locals)]
#![feature(trivial_bounds)]
#![allow(incomplete_features)]

use std::marker::PhantomData;
use tuple_list::{Tuple, TupleList};

pub trait Monoid<N> {
     fn identity() -> Self;
    fn operator(first: Self, second: Self) -> Self;
}

pub trait RepeatedOperation<T, N> {
    fn operation(self) -> T;
}

impl<T, N> RepeatedOperation<T, N> for ()
where
    T: Monoid<N>,
{
    #[inline(always)]
    fn operation(self) -> T {
        T::identity()
    }
}

impl<T, R, N> RepeatedOperation<T, N> for (T, R)
where
    T: Monoid<N>,
    R: RepeatedOperation<T, N>,
    Self: TupleList,
{
    #[inline(always)]
    fn operation(self) -> T {
        Monoid::operator(self.0, self.1.operation())
    }
}

pub struct VariFunc<T:  ?Sized, N:  ?Sized> {
    a: PhantomData<T>,
    b: PhantomData<N>,
}

impl<I, T, N> FnOnce<I> for VariFunc<T, N>
where
    I: tuple_list::Tuple,
    <I as Tuple>::TupleList: RepeatedOperation<T, N>,
{
    type Output = T;
    #[inline(always)]
    extern "rust-call" fn call_once(self, args: I) -> T {
        RepeatedOperation::operation(Tuple::into_tuple_list(args))
    }
}

pub const fn gen_function<T: ?Sized+ PartialEq+ Eq, N>() -> VariFunc<T, N> {
    VariFunc::<T, N> { a: PhantomData, b: PhantomData }
}
