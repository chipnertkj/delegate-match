//! Utilities for working with expressions.

use quote::ToTokens as _;
use syn::{parse::ParseStream, Token};

use crate::util::debug_trace;

/// Trait that answers whether the expression syntactically requires a trailing comma when used as a
/// match arm body.
pub trait NeedsCommaAsArmBody {
    fn needs_comma(&self) -> bool;
}

impl NeedsCommaAsArmBody for syn::Expr {
    fn needs_comma(&self) -> bool {
        #[allow(
            clippy::enum_glob_use,
            reason = "way too much repetition for this one..."
        )]
        use syn::Expr::*;
        match self {
            // Do not require a comma.
            If(_) | Match(_) | Block(_) | Unsafe(_) | While(_) | Loop(_) | ForLoop(_)
            | TryBlock(_) | Const(_) => false,
            // Require a comma.
            Array(_) | Assign(_) | Async(_) | Await(_) | Binary(_) | Break(_) | Call(_)
            | Cast(_) | Closure(_) | Continue(_) | Field(_) | Group(_) | Index(_) | Infer(_)
            | Let(_) | Lit(_) | Macro(_) | MethodCall(_) | Paren(_) | Path(_) | Range(_)
            | RawAddr(_) | Reference(_) | Repeat(_) | Return(_) | Struct(_) | Try(_) | Tuple(_)
            | Unary(_) | Yield(_) | Verbatim(_) => true,
            // Missing case.
            _ => panic!(
                "unsupported expression type (NeedsComma check): {}",
                self.to_token_stream()
            ),
        }
    }
}

/// Token-level parser that collects a token stream which "looks like" an expression (or any other
/// self-contained syntactic construct) by keeping track of delimiter depth.
///
/// The collected fragment ends at the first comma that is not surrounded by
/// parentheses, brackets or braces.
///
/// If there is no such comma and the last encountered token is a braced group,
/// the parser stops right after that group.
/// This covers block-like expressions that are commonly written without the trailing comma
/// in a match arm.
/// In every other situation the whole remaining input is returned (leaving the decision to the caller).
pub fn parse_tokens(input: ParseStream<'_>) -> syn::Result<proc_macro2::TokenStream> {
    use proc_macro2::{Delimiter, TokenTree};

    let mut collected = Vec::<TokenTree>::new();

    while !input.is_empty() {
        // If we are not inside delimited stuff and the next token is a comma,
        // the expression-like fragment is over.
        // Leave the comma in the buffer for the outer parser to consume.
        if input.peek(Token![,]) {
            break;
        }

        let tt: TokenTree = input.parse()?;
        match &tt {
            TokenTree::Group(group) => {
                let delim = group.delimiter();
                debug_trace!("parsed group: {tt}");
                collected.push(tt.clone());
                if matches!(
                    delim,
                    Delimiter::Parenthesis | Delimiter::Bracket | Delimiter::Brace
                ) {
                    // Special case.
                    // Block-like expression without a trailing comma inside a match arm.
                    // Look ahead. If the expression does not continue, we finish right here.
                    if !expr_continues(input) {
                        break;
                    }
                }
                continue;
            }
            _ => {
                debug_trace!("parsed: {tt}");
            }
        }

        collected.push(tt);
    }

    Ok(collected.into_iter().collect())
}

/// Return `true` when the next token clearly continues the current expression.
///
/// This is a heuristic; it only has to cover the patterns that are realistically going to appear
/// inside the `delegate_match!` macro.
///
/// (Method calls, field access, `?`, indexing, etc.)
fn expr_continues(input: ParseStream<'_>) -> bool {
    // Postfix continuation by a single `.` or `?` token.
    if input.peek(Token![.]) || input.peek(Token![?]) {
        return true;
    }

    // A call or indexing expression directly after the previous token.
    if input.peek(syn::token::Paren) || input.peek(syn::token::Bracket) {
        return true;
    }

    // A struct pattern or generic argument.
    if input.peek(syn::token::Brace) || input.peek(syn::token::Lt) || input.peek(syn::token::Gt) {
        return true;
    }

    // A path segment or method dispatch starting with `::`.
    if input.peek(Token![::]) {
        return true;
    }

    // Type cast: `expr as Ty`.
    if input.peek(Token![as]) {
        return true;
    }

    false
}
