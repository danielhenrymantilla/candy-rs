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

fn debug_print_all<T: fmt::Debug> (
    iterable: impl IntoIterator<Item = T>,
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
