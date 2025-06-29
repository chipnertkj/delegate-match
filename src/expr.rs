//! Utilities for working with expressions.

use quote::ToTokens as _;
use syn::{parse::ParseStream, Token};

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
/// The collected fragment ends **at the first comma that is not surrounded by any kind of
/// parentheses / brackets / braces**.  This is the delimiter that normally separates list
/// entries (e.g. match arms or the comma-separated entries inside `{ ... }`).
///
/// If there is **no** such comma _and_ the last encountered token is a braced group (`{ .. }`),
/// the parser stops **right after that group** – this covers block-like expressions that are
/// commonly written without the trailing comma in a match arm.  In every other situation the
/// whole remaining input is returned (leaving the decision to the caller).
pub fn parse_tokens(input: ParseStream<'_>) -> syn::Result<proc_macro2::TokenStream> {
    use proc_macro2::{Delimiter, TokenTree};

    let mut collected = Vec::<TokenTree>::new();
    let mut paren_depth: usize = 0; // (), [], {}

    while !input.is_empty() {
        // If we are *not* inside any parentheses / brackets / braces and the next token is a
        // comma, the expression-like fragment is over – leave the comma in the buffer for the
        // outer parser to consume.
        if paren_depth == 0 && input.peek(Token![,]) {
            break;
        }

        let tt: TokenTree = input.parse()?;

        match &tt {
            // A `Group` is a *single* token that already contains its matching delimiter, so
            // we treat it as "enter → exit" immediately.  The body of the group does *not* need
            // to be inspected at this point.
            TokenTree::Group(group) => {
                let delim = group.delimiter();
                if matches!(
                    delim,
                    Delimiter::Parenthesis | Delimiter::Bracket | Delimiter::Brace
                ) {
                    // Enter.
                    paren_depth += 1;
                }
                collected.push(tt.clone());
                if matches!(
                    delim,
                    Delimiter::Parenthesis | Delimiter::Bracket | Delimiter::Brace
                ) {
                    // Exit again – the whole group is one balanced token.
                    paren_depth -= 1;

                    // Special case: block-like expression without a trailing comma inside a match
                    // arm.  Once we pop back to *zero* nesting, look ahead – if the very next
                    // token would *not* continue the expression, we finish right here.
                    if paren_depth == 0 && !input.is_empty() && !expr_continues(input) {
                        break;
                    }
                }
                continue;
            }
            // Opening punctuation of bracketed constructs that are *not* represented as a
            // `Group` (angle brackets in generic arguments, for example).
            TokenTree::Punct(p)
                if p.as_char() == '<' && matches!(p.spacing(), proc_macro2::Spacing::Alone) =>
            {
                paren_depth += 1;
            }
            TokenTree::Punct(p)
                if p.as_char() == '>' && matches!(p.spacing(), proc_macro2::Spacing::Alone) =>
            {
                paren_depth = paren_depth.saturating_sub(1);
            }
            _ => {}
        }

        collected.push(tt);
    }

    Ok(collected.into_iter().collect())
}

/// Return `true` when the next token clearly *continues* the current expression – i.e. writing
///     <expr> <token>
/// would still be valid Rust syntax without any intervening comma.
///
/// This is a heuristic; it only has to cover the patterns that are realistically going to appear
/// inside the `delegate_match!` macro (method calls, field access, `?`, indexing, etc.).
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
