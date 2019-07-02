//! Run with `cargo run --example fallible`

#[macro_use] extern crate candy;

use ::std::{
    *,
    io::Write,
};

fallible! {
/// Basic usage
fn main ()
    ->  ()
    =>! String
:
    if try_print_all(&[0b101010, 0x45]).is_err() {
        throw!("Could not write to stdout");
    };
}

fallible! {
/// Using it with type parameters
pub fn try_print_all <Iterable: IntoIterator<Item = impl fmt::Debug>> (
    iterable: Iterable,
)   ->  ()
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

    fallible!(
    /// Using it with a function header
    fn try_into (
        self: Self,
    )   ->  T
        =>! Self::Err);
}

fallible! {
/// Using it with `where` clauses (/!\ Need braces /!\)
fn bar <X, Displayable> (
    x: X,
)   ->  String
    =>! X::Err
where {
    Displayable : fmt::Display,
    X : TryInto<Displayable>,
} {
    let displayable = x.try_into()?;
    format!("{}", displayable)
}}
