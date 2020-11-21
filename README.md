# matches
This macro implements a match-like expresison with different semantics, one that allows to match expressions in the match arms without the verbose syntax.

Right now, this piece of code does not do what you expect.

```rust
let a = 3;
match 2 {
    a => println!("same as a"),
    _ => println!("not same as a"),
}
```

One would expect that the `_` case would be executed, however in Rust, this code actually sets the value of `a` in the match expression to 2 and executes the first arm.

To achieve the expected result, one has to write
```rust
let a = 3;
match 2 {
    v if v == a => println!("same as a"),
    _ => println!("not same as a"),
}
```

which seems verbose.

With this macro, one can write it more succinctly as
```rust
let a = 3;
matches!(a,
    2 => { println!("same as a") },
    _ => { println!("not same as a") },
);
```