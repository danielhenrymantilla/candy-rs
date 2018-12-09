//! Run with `cargo run --example fallible`

#[macro_use] extern crate candy;

use ::std::{
    *,
    io::Write,
};

/// Basic usage
fallible!{
fn main ()
    -> ()
    =>! String
:
    if try_print_all(&[0b101010, 0x45]).is_err() {
        throw!("Could not write to stdout");
    };
}

/// Using it with type parameters
fallible!{
pub fn try_print_all <T: fmt::Debug, Iterable: IntoIterator<Item = T>> (
    iterable: Iterable,
)   -> ()
    =>! io::Error
:
    let to_stdout = &mut io::stdout();
    write!(to_stdout, "[")?;
    let mut iterator = iterable.into_iter();
    if let Some(first) = iterator.next() {
        write!(to_stdout, "{:?}", first)?;
        while let Some(next) = iterator.next() {
            write!(to_stdout, ", {:?}", next)?;
        };
    };
    write!(to_stdout, "]\n")?;
}

trait TryInto<T> {
    type Err;

    /// Using it with a function header
    fallible!(
    fn try_into (
        self: Self,
    )   -> T
        =>! Self::Err );
}

/// Using it with `where` clauses (/!\ Need braces /!\)
fallible!{
fn bar <X, Displayable> (
    x: X,
)   -> String
    =>! X::Err
where {
    Displayable: fmt::Display,
    X: TryInto<Displayable>,
} {
    let displayable = x.try_into()?;
    format!("{}", displayable)
}}
