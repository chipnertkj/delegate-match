//! Raw token-stream substitution.

use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;

use crate::util::debug_trace;

const OPERATOR: char = '$';
const ENTRY_PAT: &str = "entry_pat";
const ASSOC_TS: &str = "assoc_ts";

/// Substitute placeholders with concrete tokens.
pub fn substitute(
    tokens: &TokenStream2,
    entry_pat: &syn::Pat,
    assoc_ts: Option<&TokenStream2>,
) -> TokenStream2 {
    debug_trace!("substitution pass");
    let mut out = Vec::new();
    let mut iter = tokens.clone().into_iter().peekable();

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Punct(ref punct) if punct.as_char() == OPERATOR => {
                // Look ahead at the identifier following `$` (if any).
                if let Some(TokenTree::Ident(ident)) = iter.peek() {
                    let ident_name = ident.to_string();
                    debug_trace!("found placeholder: ${}", ident_name);
                    let replacement = match ident_name.as_str() {
                        ENTRY_PAT => quote!(#entry_pat),
                        ASSOC_TS => quote!(#assoc_ts),
                        _ => {
                            // Unexpected identifier after `$`.
                            // Leave the `$` in place and fall back to default handling below.
                            out.push(TokenTree::Punct(punct.clone()));
                            continue;
                        }
                    };
                    // Consume the identifier we just peeked at.
                    iter.next();
                    out.extend(replacement);
                    continue;
                }
                // `$` not followed by ident. Treat it as a normal punctuation token.
                out.push(punct.clone().into());
            }
            TokenTree::Group(group) => {
                let inner = substitute(&group.stream(), entry_pat, assoc_ts);
                let new_group = proc_macro2::Group::new(group.delimiter(), inner);
                out.push(new_group.into());
            }
            other => out.push(other),
        }
    }

    let result: TokenStream2 = out.into_iter().collect();
    debug_trace!("substitution result: {}", result);
    result
}
