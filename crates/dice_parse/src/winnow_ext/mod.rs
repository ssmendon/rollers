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
//! <div class="warning">
//!
//! This is not designed for use outside this crate, and it is
//! gated behind the `unstable-pratt` feature flag.
//!
//! </div>
//!
//! [winnow-pr]: <https://github.com/winnow-rs/winnow/pull/620>
//! [ghuser-39555]: <https://github.com/39555>

// Edited from the PR from GitHub user @39555, see winnow-rs/winnow#620.
// winnow is licensed under MIT terms.

pub mod precedence;
pub mod shunting_yard;
