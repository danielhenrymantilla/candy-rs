# candy-rs
Syntaxic sugar for Rust

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)][Repository]
[![Latest version](https://img.shields.io/crates/v/candy.svg)][crates.io]
[![Documentation](https://docs.rs/candy/badge.svg)][Documentation]
[![License](https://img.shields.io/crates/l/candy.svg)](https://github.com/danielhenrymantilla/candy-rs#license)

## Example
```rust
#[macro_use] extern crate candy;

use ::std::*;

type ErrorMsg = borrow::Cow<'static, str>;

fallible!{
fn main ()
    -> ()
    =>! ErrorMsg
:
    let input_number: u64 = {
        let (mb_argv_0, mb_argv_1) = {
            let mut args = env::args();
            (args.next(), args.next())
        };
        let prog_name = mb_argv_0.unwrap();
        match mb_argv_1
            	.and_then(|argv_1| argv_1.parse().ok())
        {
        	Some(number) => number,
        	_ => throw!(format!("Usage: {} <number>", prog_name)),
        }
    };
    collatz_conjecture(input_number);
}

fn collatz_conjecture (mut n: u64)
{
    do_loop!({
        println!("n = {}", n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        };
    } while n != 1);
    println!("Did reach 1.");
}
```

## Usage

- Add this line to your `Cargo.toml` (under `[dependencies]`):
  ```toml
  candy = "0.1.0"
  ```

- Add this to your `.rs` code:
  ```rust
  #[macro_use] extern crate candy;
  ```
