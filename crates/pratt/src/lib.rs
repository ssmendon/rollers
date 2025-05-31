//! Operator precedence parsing for [`winnow`].
//!
//! A port of [winnow-rs/winnow#620][winnow-pr] from GitHub user [@39555][ghuser-39555],
//! because it's currently in draft.
//!
//! The best way to learn is by using the test cases and examples in this crate,
//! since the API for Pratt parsing is verbose.
//!
//! The main things to look at are:
//! * [`precedence::precedence`], the Pratt Parser entrypoint
//! * [`shunting_yard::precedence`], the Shunting Yard entrypoint
//!
//! Slightly less interesting, but also useful:
//! * [`precedence::Assoc`], the associativity of an operator
//! * [`precedence::Power`], a type alias for precedence power
//!
//! [winnow-pr]: <https://github.com/winnow-rs/winnow/pull/620>
//! [ghuser-39555]: <https://github.com/39555>

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// Edited from the PR from GitHub user @39555, see winnow-rs/winnow#620.
// winnow is licensed under MIT terms.

pub mod precedence;

#[cfg(feature = "alloc")]
#[allow(dead_code)]
pub mod shunting_yard;
