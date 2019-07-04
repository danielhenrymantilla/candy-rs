# candy-rs
Syntaxic sugar for Rust: macros for lighter error handling code, and more.

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)][Repository]
[![Latest version](https://img.shields.io/crates/v/candy.svg)][crates.io]
[![Documentation](https://docs.rs/candy/badge.svg)][Documentation]
[![License](https://img.shields.io/crates/l/candy.svg)](https://github.com/danielhenrymantilla/candy-rs#license)

## Examples
```rust
//! Run with `cargo run --example do_loop -- <number>`

#[macro_use] extern crate candy;

use ::std::*;

type ErrorMsg = borrow::Cow<'static, str>;

fallible! {
fn main ()
    ->  ()
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

```rust
//! Run with `cargo run --example catch`

#[macro_use] extern crate candy;

use ::std::{
    *,
    io::Write,
};

fn main ()
{
    debug_print_all([0b101010, 0x45].iter())
}

fn debug_print_all (
    iterable: impl IntoIterator<Item = impl fmt::Debug>,
)
{
    let to_stdout = &mut io::stdout();
    
    // `catch!` allows using the `?` operator. Isn't that nice?
    match catch!({
        write!(to_stdout, "[")?;
        let mut iterator = iterable.into_iter();
        let mut count = 0;
        if let Some(first) = iterator.next() {
            count += 1;
            write!(to_stdout, "{:?}", first)?;
            while let Some(next) = iterator.next() {
                count += 1;
                write!(to_stdout, ", {:?}", next)?;
            };
        };
        write!(to_stdout, "]\n")?;
        count
    } -> usize =>! io::Error)
    {
        Err(io_err) => {
            eprintln!(
                "{:?} : could not write to stdout!? Oh well, who cares?",
                io_err,
            );
        },
        Ok(n) => {
            eprintln!("Successfully wrote {} elements to stdout", n);
        },
    }
}
```

## Usage

- Add this line to your `Cargo.toml` (under `[dependencies]`):
  ```toml
  candy = "0.1.5"
  ```

- Add this to your `.rs` code:
  ```rust
  #[macro_use] extern crate candy;
  ```

[Repository]: https://github.com/danielhenrymantilla/candy-rs
[Documentation]: https://docs.rs/candy/0.1.5/
[crates.io]: https://crates.io/crates/candy
