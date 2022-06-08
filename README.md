# Experiment with derive Debug

Show what `#[derive(Debug)]` acutally does.

```
wink@3900x 22-06-08T17:31:41.889Z:~/prgs/rust/myrepos/expr-derive-debug (main)
$ cat -n src/main.rs
     1  // Create a struct and derive Debug code so
     2  // all fields of the struct can be printed.
     3  #[derive(Debug)]
     4  struct MyStruct {
     5      i: i32,
     6      s: String,
     7  }
     8
     9  fn main() {
    10      // Create an instance of the struct
    11      let ms = MyStruct {
    12          i: 123,
    13          s: "hello".to_string(),
    14      };
    15
    16      // Print the fields manually, works without the derive(Debug)
    17      println!("MyStruct: i={} s={}", ms.i, ms.s);
    18
    19      // Print using the Debug derived code:
    20      println!("MyStruct={:?}", ms);
    21  }
    22
wink@3900x 22-06-08T17:31:48.857Z:~/prgs/rust/myrepos/expr-derive-debug (main)
$ cargo expand
    Checking expr-derive-debug v0.1.0 (/home/wink/prgs/rust/myrepos/expr-derive-debug)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
struct MyStruct {
    i: i32,
    s: String,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Self {
                i: ref __self_0_0,
                s: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "MyStruct");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "i", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "s", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
fn main() {
    let ms = MyStruct {
        i: 123,
        s: "hello".to_string(),
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["MyStruct: i=", " s=", "\n"],
            &[
                ::core::fmt::ArgumentV1::new_display(&ms.i),
                ::core::fmt::ArgumentV1::new_display(&ms.s),
            ],
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["MyStruct=", "\n"],
            &[::core::fmt::ArgumentV1::new_debug(&ms)],
        ));
    };
}
wink@3900x 22-06-08T17:32:03.748Z:~/prgs/rust/myrepos/expr-derive-debug (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/expr-derive-debug`
MyStruct: i=123 s=hello
MyStruct=MyStruct { i: 123, s: "hello" }
wink@3900x 22-06-08T17:32:07.255Z:~/prgs/rust/myrepos/expr-derive-debug (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
