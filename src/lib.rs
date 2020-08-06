/*
Copyright (c) 2020, Richard Lettich
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

* Neither the name of the copyright holder nor the names of its
  contributors may be used to endorse or promote products derived from
  this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE
*/

#![feature(fn_traits, unboxed_closures)]
#![feature(const_fn)]
#![feature(const_generics)]
#![feature(unsized_locals)]
#![feature(trivial_bounds)]

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
