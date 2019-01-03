#![cfg_attr(feature = "nightly",
    feature(external_doc)
)]

//! Ever wanted to **suck**? Here, have some **sweet sugared** candy.
//!
//! #### This crate provides some syntaxic sugar for Rust, in the shape of macros.
#![cfg_attr(feature = "nightly",
    doc = "# Examples"
)]
#![cfg_attr(feature = "nightly",
    doc = "```rust"
)]
#![cfg_attr(feature = "nightly",
    doc(include = "../examples/do_loop.rs")
)]
#![cfg_attr(feature = "nightly",
    doc = "```"
)]


#![cfg_attr(feature = "nightly",
    doc = ""
)]

#![cfg_attr(feature = "nightly",
    doc = "```rust"
)]
#![cfg_attr(feature = "nightly",
    doc(include = "../examples/catch.rs")
)]
#![cfg_attr(feature = "nightly",
    doc = "```"
)]


#![cfg_attr(feature = "try-trait",
    feature(try_trait)
)]

#![no_std]
#![doc(html_root_url = "https://docs.rs/candy/0.1.0")]

// #[cfg(test)]
// #[macro_use]
// extern crate std;

// Re-export libcore using an alias so that the macros can work without
// requiring `extern crate core` downstream.
#[doc(hidden)]
pub extern crate core as _core;




/// Sugar for `Default::default()`.
#[macro_export]
macro_rules! default {() => (
    $crate::_core::default::Default::default()
)}

/// Enhances the readability of the negation unary operator (`!`).
///
/// # Example
///
/// ```
/// #[macro_use] extern crate candy;
/// let v: Vec<String> = default!();
/// if not!(v.is_empty()) {
///     panic!("Dis is not possibeuhl!!")
/// }
/// ```
///
/// #### Note:
///
/// The negation operator is more than just a boolean negation;
/// it is actually a bitwise negation.
#[macro_export]
macro_rules! not {($cond:expr) => (
    !$cond
)}

/// Sugar for early-returning an error: `return Err($err_value.into())`
#[macro_export]
macro_rules! throw {($err_value:expr) => ({
    #[cfg(not(feature = "try-trait"))] {
        return $crate::_core::result::Result::Err($err_value.into())
    }
    #[cfg(feature = "try-trait")] {
        return $crate::_core::ops::Try::from_err($err_value.into())
    }
})}

/// Sugar for early-returning a success: `return Ok($ok_value.into())`
#[macro_export]
macro_rules! ret {($ok_value:expr) => ({
    #[cfg(not(feature = "try-trait"))] {
        return $crate::_core::result::Result::Ok($ok_value.into())
    }
    #[cfg(feature = "try-trait")] {
        return $crate::_core::ops::Try::from_ok($ok_value.into())
    }
})}


/// Sugar for an expression block catching early-returns.
///
/// To be used when the return type is a [`Result`],
/// to enable writing "try-catch"-like blocks within a function's body.
///
#[doc = "# Example"]
#[cfg_attr(feature = "nightly",
    doc = "```rust"
)]
#[cfg_attr(feature = "nightly",
    doc(include = "../examples/catch.rs")
)]
#[cfg_attr(feature = "nightly",
    doc = "```"
)]
#[cfg_attr(not(feature = "nightly"),
    doc = "See [crates.io](https://crates.io/crates/candy)"
)]
#[cfg_attr(not(feature = "nightly"),
    doc = "for an example."
)]
#[macro_export]
macro_rules! catch {
    (
        { $($body:tt)* } -> $ok_ty:ty =>! $err_ty:ty
    ) => (
        (|| -> $crate::_core::result::Result<$ok_ty, $err_ty>
        {
            #[cfg(not(feature = "try-trait"))] {
                $crate::_core::result::Result::Ok({$($body)*})
            }
            #[cfg(feature = "try-trait")] {
                $crate::_core::ops::Try::from_ok({$($body)*})
            }
        })()
    );

    {
        $($body:tt)*
    } => (
        catch!({$($body)*} -> _ =>! _)
    );
}


/// Sugar for the pervasive `-> Result<T, E>`
/// fallible return type pattern.
///
/// It transforms it into `-> T =>! E`, and disposes of the need to wrap
/// the implicit return value of the function in a `Ok(...)`.
/// In other words, **no more `Ok(())`!**
///
#[doc = "# Example"]
#[cfg_attr(feature = "nightly",
    doc = "```rust"
)]
#[cfg_attr(feature = "nightly",
    doc(include = "../examples/fallible.rs")
)]
#[cfg_attr(feature = "nightly",
    doc = "```"
)]
#[cfg_attr(not(feature = "nightly"),
    doc = "See [crates.io](https://crates.io/crates/candy)"
)]
#[cfg_attr(not(feature = "nightly"),
    doc = "for an example."
)]
#[macro_export]
macro_rules! fallible {
    // Main case
    (
        $vis:vis
        fn $fname:ident {$($ty_params:tt)*}
            ($($args:tt)*)
        -> $ret_ty:ty =>! $err_ty:ty
        where {
            $($wc:tt)*
        }
            $fbody:tt
    ) => (
        $vis
        fn $fname <$($ty_params)*>
            ($($args)*)
        -> $crate::_core::result::Result<$ret_ty, $err_ty>
        where
            $($wc)*
        {
            let _ret = $fbody;
            #[allow(unreachable_code)]
            #[cfg(not(feature = "try-trait"))] {
                $crate::_core::result::Result::Ok(_ret)
            }
            #[allow(unreachable_code)]
            #[cfg(feature = "try-trait")] {
                $crate::_core::ops::Try::from_ok(_ret)
            }
        }
    );

    // Ellision of where clause(s)
    (
        $vis:vis
        fn $fname:ident {$($ty_params:tt)*}
            ($($args:tt)*)
        -> $ret_ty:ty =>! $err_ty:ty
        :
            $($fbody:tt)*
    ) => (fallible!{
        $vis
        fn $fname {$($ty_params)*}
            ($($args)*)
        -> $ret_ty =>! $err_ty
        where {}
            {$($fbody)*}
    });

    // Ellision of type parameters (and thus where clause(s))
    (
        $vis:vis
        fn $fname:ident
            ($($args:tt)*)
        -> $ret_ty:ty =>! $err_ty:ty
        :
            $($fbody:tt)*
    ) => (fallible!{
        $vis
        fn $fname {}
            ($($args)*)
        -> $ret_ty =>! $err_ty
        :
            $($fbody)*
    });

    // Handle method signatures (traits) - Main case
    (
        $(pub$(($($vis:tt)*))*)* // $vis:vis does not work here :(
        fn $fname:ident {$($ty_params:tt)*}
            ($($args:tt)*)
        -> $ret_ty:ty =>! $err_ty:ty
        where {
            $($wc:tt)*
        }
    ) => (
        $(pub$(($($vis)*))*)*
        fn $fname <$($ty_params)*>
            ($($args)*)
        -> ::std::result::Result<$ret_ty, $err_ty>
        where
            $($wc)*
        ;
    );

    // Handle method signatures (traits) - Ellision of where clause(s)
    (
        $(pub$(($($vis:tt)*))*)* // $vis:vis does not work here :(
        fn $fname:ident {$($ty_params:tt)*}
            ($($args:tt)*)
        -> $ret_ty:ty =>! $err_ty:ty
    ) => (fallible!{
        $(pub$(($($vis)*))*)*
        fn $fname {$($ty_params)*}
            ($($args)*)
        -> $ret_ty =>! $err_ty
        where {}
    });

    // Handle method signatures (traits) - Ellision of type parameters
    (
        $(pub$(($($vis:tt)*))*)* // $vis:vis does not work here :(
        fn $fname:ident
            ($($args:tt)*)
        -> $ret_ty:ty =>! $err_ty:ty
    ) => (fallible!{
        $(pub$(($($vis)*))*)*
        fn $fname {}
            ($($args)*)
        -> $ret_ty =>! $err_ty
    });

    // start parsing type parameters
    (
        $(pub$(($($vis:tt)*))*)*
        fn $fname:ident
        < $($other:tt)*
    ) => (fallible!{@unsugaring_ty_params
        $(pub$(($($vis)*))*)*
        fn $fname
        [ < ] // depth
        {}    // type params
        $($other)*
    });

    // end parsing type parameters
    (@unsugaring_ty_params
        $(pub$(($($vis:tt)*))*)*
        fn $fname:ident
        [ < ]
        $type_parameters:tt
        > $($other:tt)*
    ) => (fallible!{
        $(pub$(($($vis)*))*)*
        fn $fname
        $type_parameters
        $($other)*
    });

    // add one layer of depth
    (@unsugaring_ty_params
        $(pub$(($($vis:tt)*))*)*
        fn $fname:ident
        [ $($depth:tt)* ]
        {$($type_parameters:tt)*}
        < $($other:tt)*
    ) => (fallible!{@unsugaring_ty_params
        $(pub$(($($vis)*))*)* fn $fname
        [ $($depth)* < ]
        { $($type_parameters)* < }
        $($other)*
    });

    // remove one layer of depth
    (@unsugaring_ty_params
        $(pub$(($($vis:tt)*))*)*
        fn $fname:ident
        [ < $($depth:tt)* ]
        {$($type_parameters:tt)*}
        > $($other:tt)*
    ) => (fallible!{@unsugaring_ty_params
        $(pub$(($($vis)*))*)*
        fn $fname
        [ $($depth)* ]
        { $($type_parameters)* > }
        $($other)*
    });

    // Disambiguate >> into > >
    (@unsugaring_ty_params
        $(pub$(($($vis:tt)*))*)*
        fn $fname:ident
        $depth:tt
        $type_parameters:tt
        >> $($other:tt)*
    ) => (fallible!{@unsugaring_ty_params
        $(pub$(($($vis)*))*)*
        fn $fname
        $depth
        $type_parameters
        > >
        $($other)*
    });

    // parse one tt
    (@unsugaring_ty_params
        $(pub$(($($vis:tt)*))*)*
        fn $fname:ident
        $depth:tt
        { $($type_parameters:tt)* }
        $single_tt:tt $($other:tt)*
    ) => (fallible!{@unsugaring_ty_params
        $(pub$(($($vis)*))*)*
        fn $fname
        $depth
        { $($type_parameters)* $single_tt }
        $($other)*
    });
}

#[doc = "Sugar for the `do { ... } while (...);` C construct in Rust."]
#[doc = ""]
#[doc = "# Example"]
#[cfg_attr(feature = "nightly",
    doc = "```rust"
)]
#[cfg_attr(feature = "nightly",
    doc(include = "../examples/do_loop.rs")
)]
#[cfg_attr(feature = "nightly",
    doc = "```"
)]
#[cfg_attr(not(feature = "nightly"),
    doc = "See [crates.io](https://crates.io/crates/candy)"
)]
#[cfg_attr(not(feature = "nightly"),
    doc = "for an example."
)]

#[macro_export]
macro_rules! do_loop {(
    { $($body:tt)* } while $cond:expr
) => (
    loop {
        $($body)* ;
        if !$cond { break };
    }
)}

/// See [this thread](
/// https://users.rust-lang.org/t/22080) or [this issue](
/// https://github.com/rust-lang/rust/issues/47819).
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_assert {
    ($($arg:tt)*) => ({ assert!($($arg)*); })
}

/// See [this thread](
/// https://users.rust-lang.org/t/22080) or [this issue](
/// https://github.com/rust-lang/rust/issues/47819).
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_assert {
    ($($arg:tt)*) => ( {} )
}


/// See [this thread](
/// https://users.rust-lang.org/t/22080) or [this issue](
/// https://github.com/rust-lang/rust/issues/47819).
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_assert_eq {
    ($($arg:tt)*) => ({ assert_eq!($($arg)*); })
}

/// See [this thread](
/// https://users.rust-lang.org/t/22080) or [this issue](
/// https://github.com/rust-lang/rust/issues/47819).
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_assert_eq {
    ($($arg:tt)*) => ( {} )
}

/// See [this thread](
/// https://users.rust-lang.org/t/22080) or [this issue](
/// https://github.com/rust-lang/rust/issues/47819).
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_assert_ne {
    ($($arg:tt)*) => ({ assert_ne!($($arg)*); })
}

/// See [this thread](
/// https://users.rust-lang.org/t/22080) or [this issue](
/// https://github.com/rust-lang/rust/issues/47819).
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_assert_ne {
    ($($arg:tt)*) => ( {} )
}
