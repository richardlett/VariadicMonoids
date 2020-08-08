# VariadicMonoids
This small crate provides (experimental) support for creating variadic functions from monoids. It requires nightly and relies on unstable features.

More simply, given a `fn f(T,T) -> T`, this crate allows you to produce a variadic function `f'` such that

```rust
f'(a: T,b: T, c: T,....) == f(a,f(b,f(c,...f(z,identity)..)))
```

Where Identity is an identity element on f. (Specifically, only `f(a, identity) == a` must actually hold).

How to Use:
```rust
use variadic_monoids::*;

// You must create a name for your monoid with a struct. This allows you to
// create multiple monoids per type (and they can be for external too)
struct AddMonoid;

impl  Monoid<AddMonoid> for  i32 {
   fn  identity() -> Self { 0 }
   fn  operator(a: Self, b: Self) -> Self { a + b }
}

// Call the (constant function) gen_function to retrieve your function
// With type parameters as the implemented Type, and Name of your monoid.
const sum: VariFunc<i32, AddMonoid>  =  gen_function::<i32, AddMonoid>();

fn main() {
   println!("{}", sum(1,2,3,4));
}
```

In your ```Cargo.toml```

    [dependencies]
    ...
    variadic_monoids="0.1.1"
