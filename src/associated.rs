use syn::parse::{discouraged::Speculative as _, ParseStream};

use crate::util::SynErrorContext as _;

/// Associated syntax item.
/// Any [entry](`crate::delegate_entry::DelegateEntry`) can have one.
///
/// These can be any of the following:
///
/// - Expression
/// - Statement
/// - Pattern
/// - Type
///
/// They are also parsed in that order. If all parsing attempts fail, an error is returned.
#[derive(Clone)]
pub enum Associated {
    Expr(syn::Expr),
    Stmt(syn::Stmt),
    Pat(syn::Pat),
    Type(syn::Type),
}

impl AsRef<str> for Associated {
    fn as_ref(&self) -> &str {
        match self {
            Self::Expr(_) => "expression",
            Self::Stmt(_) => "statement",
            Self::Pat(_) => "pattern",
            Self::Type(_) => "type",
        }
    }
}

impl From<syn::Pat> for Associated {
    fn from(pat: syn::Pat) -> Self {
        Self::Pat(pat)
    }
}

impl quote::ToTokens for Associated {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Expr(expr) => expr.to_tokens(tokens),
            Self::Stmt(stmt) => stmt.to_tokens(tokens),
            Self::Pat(pat) => pat.to_tokens(tokens),
            Self::Type(ty) => ty.to_tokens(tokens),
        }
    }
}

impl syn::parse::Parse for Associated {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let fork = input.fork();
        let expr_err = match fork.parse() {
            Ok(expr) => {
                input.advance_to(&fork);
                return Ok(Self::Expr(expr));
            }
            Err(e) => e,
        };
        let fork = input.fork();
        let stmt_err = match fork.parse() {
            Ok(stmt) => {
                input.advance_to(&fork);
                return Ok(Self::Stmt(stmt));
            }
            Err(e) => e,
        };
        let fork = input.fork();
        let pat_err = match syn::Pat::parse_multi_with_leading_vert(&fork) {
            Ok(pat) => {
                input.advance_to(&fork);
                return Ok(pat.into());
            }
            Err(e) => e,
        };
        let fork = input.fork();
        let type_err = match fork.parse() {
            Ok(ty) => {
                input.advance_to(&fork);
                return Ok(Self::Type(ty));
            }
            Err(e) => e,
        };
        let err = expr_err
            .wrap_err(stmt_err)
            .wrap_err(pat_err)
            .wrap_err(type_err)
            .wrap_err(syn::Error::new(
                input.span(),
                "expected associated syntax item",
            ));
        Err(err)
    }
}
