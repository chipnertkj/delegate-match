use syn::parse::{discouraged::Speculative as _, ParseStream};

use crate::util::SynErrorContext as _;

/// Associated syntax item.
/// Any [`DelegateEntry`] can have one.
///
/// These can be any of the following:
///
/// - [`Expression`]
/// - [`Pattern`]
/// - [`Type`]
///
/// Associated tokens are parsed in that exact order.
/// If all parsing attempts fail, errors describing them are emitted.
///
/// This classification is technically "wrong" for many inputs, but the output tokens are identical.
/// In the end, this doesn't matter for raw substitution.
///
/// [`DelegateEntry`]: crate::delegate_entry::DelegateEntry
/// [`Expression`]: syn::Expr
/// [`Pattern`]: syn::Pat
/// [`Type`]: syn::Type
#[derive(Clone)]
pub enum Associated {
    Expr(syn::Expr),
    Pat(syn::Pat),
    Type(syn::Type),
}

impl AsRef<str> for Associated {
    fn as_ref(&self) -> &str {
        match self {
            Self::Expr(_) => "expression",
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
            Self::Pat(pat) => pat.to_tokens(tokens),
            Self::Type(ty) => ty.to_tokens(tokens),
        }
    }
}

impl Associated {
    /// Forks the input stream and attempts to parse as type `T`.
    /// If successful, returns the parsed value wrapped in `Self`.
    fn try_parse_as<T>(
        input: ParseStream<'_>,
        parse_fn: fn(ParseStream<'_>) -> syn::Result<T>,
        what: &str,
    ) -> syn::Result<T> {
        let fork = input.fork();
        match parse_fn(&fork) {
            Ok(t) => {
                input.advance_to(&fork);
                Ok(t)
            }
            Err(e) => Err(e.wrap_err(syn::Error::new(
                fork.span(),
                format!("unable to parse assoc_ts as {what}"),
            ))),
        }
    }
}

impl syn::parse::Parse for Associated {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        // Try expression.
        let expr_err = match Self::try_parse_as(input, syn::Expr::parse, "expression") {
            Ok(expr) => return Ok(Self::Expr(expr)),
            Err(e) => e,
        };
        // Try pattern.
        let pat_err =
            match Self::try_parse_as(input, syn::Pat::parse_multi_with_leading_vert, "pattern") {
                Ok(pat) => return Ok(Self::Pat(pat)),
                Err(e) => e,
            };
        // Try type.
        let type_err = match Self::try_parse_as(input, syn::Type::parse, "type") {
            Ok(ty) => return Ok(Self::Type(ty)),
            Err(e) => e,
        };
        // All attempts failed, emit errors.
        let err = type_err
            .wrap_err(pat_err)
            .wrap_err(expr_err)
            .wrap_err(syn::Error::new(
                input.span(),
                "expected associated syntax item",
            ));
        Err(err)
    }
}
