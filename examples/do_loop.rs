//! Run with `cargo run --example do_loop -- <number>`

#[macro_use] extern crate candy;

use ::std::*;

type ErrorMsg = borrow::Cow<'static, str>;

fallible!{
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
