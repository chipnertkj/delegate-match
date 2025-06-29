#![doc = include_str!("../README.md")]
// Deny lints.
#![deny(
    unsafe_code,
    reason = "this crate is not responsible for anything that requires `unsafe`"
)]
#![deny(
    clippy::unwrap_used,
    reason = "using `expect` instead shows intent better"
)]
#![deny(nonstandard_style, reason = "use commonly agreed on standards")]
#![deny(future_incompatible, reason = "provides easier maintenance")]
#![deny(deprecated_safe_2024, reason = "better safety guarantees")]
// Warn lints.
#![warn(missing_docs, reason = "helps with documentation coverage")]
#![warn(clippy::cargo, reason = "improves crate metadata quality")]
#![warn(
    clippy::cargo_common_metadata,
    reason = "improves crate metadata quality"
)]
#![warn(missing_debug_implementations, reason = "this is a library")]
#![warn(
    clippy::pedantic,
    reason = "useful but conservative lints - use #[allow] attributes on false positives"
)]
#![warn(
    clippy::nursery,
    reason = "experimental lints, use #[allow] to disable annoying ones"
)]

mod delegate_arm;
mod delegate_entry;
mod delegate_match;
mod expr;
mod substitute;
mod util;

use proc_macro::TokenStream;
use quote::ToTokens as _;
use syn::parse_macro_input;

/// Convenience macro for writing grouped `match` arms for different underlying types.
///
/// Writing repetitive `match` arms for enumerations (or other pattern-matching
/// constructs) &mdash; especially when types, but not the API, differ &mdash;
/// can quickly become boilerplate. `delegate_match!` lets you list
/// several patterns up-front once and then re-uses a single body for each
/// of them, automatically expanding into equivalent ordinary Rust code.
///
/// ## Syntax outline
///
/// ```text
/// match <scrutinee_expr> {
///     [<arm_path>::]{ <entry_pat> [: <assoc_ts>][, ...] } [<arm_pat>] [if <guard_expr>] => <body_expr>[[,] ...]
/// }
/// ```
///
/// - `arm_path` &mdash; optional path prefix (e.g. `MyEnum` or `::std::io`)
/// - `entry_pat` &mdash; individual *entry pattern*, also available as the `$entry_pat` placeholder.
/// - `assoc_ts` &mdash; *associated tokens*, also available as the `$assoc_ts` placeholder.
/// - `arm_pat` &mdash; an optional pattern appended to every entry.
/// - `guard_expr` &mdash; an optional `if` guard.
/// - `body_expr` &mdash; expression generated for each entry.
///
/// This expands to:
///
/// ```text
/// match <scrutinee_expr> {
///     <arm_path>::<entry_pat><arm_pat> if <guard_expr> => <body_expr>
/// }
/// ```
///
/// Two placeholders are substituted for every entry *before the code is
/// type-checked*, and they may appear in the following places:
///   - inside the delegate arm pattern `arm_pat` (if present),
///   - inside the match arm guard expression `guard_expr` (if present),
///   - inside the arm body expression `body_expr`.
///
/// The available placeholders are:
///   - `$entry_pat` &mdash; the entry pattern for a generated arm.
///   - `$assoc_ts` &mdash; the tokens following an entry, up until the next one (excluding the colon).
///
/// The macro is supposed to accept standard Rust `match` expression syntax, extended with the above.
/// Any other deviation should generally be considered a bug.
///
/// ## Semantics
///
/// - For each *entry* in a grouped arm, the macro generates a regular match arm.
/// - Everything else (outer attributes, guards, commas...) is preserved exactly as you write it.
/// - The only exception to that is placeholder substitution.
/// - This macro performs generation before type-checking is done, so
///   the generated code is capable of working with different types, if constructed appropriately.
/// - The order of generated arms is the order of entries in the source code.
///
/// ## Examples
///
/// ### Delegating to the same code for multiple enum variants
///
/// ```rust
/// use delegate_match::delegate_match;
///
/// enum MouseEvent { Scroll(i16, i16), Position(i32, i32) }
/// let ev = MouseEvent::Scroll(10, 20);
///
/// delegate_match! {
///     match ev {
///         // This expands to two individual arms.
///         MouseEvent::{ Scroll, Position }(x, y) => {
///             println!("mouse event: $entry_pat → ({x}, {y})")
///         }
///     }
/// }
/// ```
///
/// ### Using placeholders
///
/// ```rust
/// # use delegate_match::delegate_match;
/// # enum Msg { Ping, Log }
/// # let msg = Msg::Log;
/// delegate_match! {
///     match msg {
///         // `$assoc_ts` and `$entry_pat` are substituted at compile time.
///         Msg::{ Ping: "🏓", Log: "📝" } => {
///             // Outputs "🏓 Ping" or "📝 Log" depending on the variant.
///             println!("{} {}", $assoc_ts, stringify!($entry_pat))
///         }
///     }
/// }
/// ```
///
/// ### Adding an if guard to multiple entries
///
/// ```rust
/// # use delegate_match::delegate_match;
/// # enum Number { I32(i32), I64(i64) }
/// # let n = Number::I32(4);
/// delegate_match! {
///     match n {
///         // This works despite `val` being of different types for each variant.
///         // This is because a separate arm is generated for each entry!
///         Number::{ I32, I64 }(val) if val % 2 == 0 => {
///             println!("even {}", val)
///         }
///         // We must also account for the rest of the cases.
///         Number::{ I32, I64 }(val) => {
///             println!("odd {}", val)
///         }
///     }
/// }
/// ```
#[proc_macro_error2::proc_macro_error]
#[proc_macro]
pub fn delegate_match(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as delegate_match::ExprDelegateMatch);
    parsed.into_token_stream().into()
}

#[cfg(test)]
#[test]
fn trybuild_tests_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
