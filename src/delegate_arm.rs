//! Implementation of the *grouped* arm syntax.

use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt as _};
use syn::{parse::ParseStream, spanned::Spanned as _, Token};

use crate::{
    delegate_entry::DelegateEntry,
    expr::NeedsCommaAsArmBody as _,
    util::{debug_trace, SynErrorContext as _},
};

/// The special grouped arm syntax handled by the `delegate_match!` macro:
/// `path::{ Foo[: bar], ... } [pat] [if guard] => body,`
#[derive(Clone)]
pub struct DelegateArm {
    pub attrs: Vec<syn::Attribute>,
    pub path: Option<syn::Path>,
    pub path_sep: Option<Token![::]>,
    pub _brace_token: syn::token::Brace,
    pub entries: Vec<DelegateEntry>,
    pub pat: Option<TokenStream2>,
    pub guard: Option<(Token![if], TokenStream2)>,
    pub fat_arrow_token: Token![=>],
    /// Raw token stream of the match-arm body. We postpone actual `syn::Expr` parsing until after
    /// placeholder substitution.
    pub body: TokenStream2,
    pub comma: Option<Token![,]>,
}

impl ToTokens for DelegateArm {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self.to_arms() {
            Ok(arms) => tokens.append_all(&arms),
            Err(e) => {
                // Turn the `compile_error!` invocation into a valid match arm so that the
                // surrounding `match` expression remains well-formed and the actual error message
                // is not hidden behind additional parser errors.
                let err = e.to_compile_error();
                tokens.append_all(quote! {
                    _ => { #err; unreachable!("compile error in delegate arm") }
                });
            }
        }
    }
}

impl syn::parse::Parse for DelegateArm {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        debug_trace!("parsing arm");
        let attrs = input.call(syn::Attribute::parse_outer)?;
        debug_trace!("parsing path");
        let path = Self::parse_path(input)?;
        let path_sep = Self::parse_path_sep(input, path.as_ref())?;
        debug_trace!("parsing entries");
        let (brace_token, entries) = Self::parse_entries(input)?;
        let pat = Self::parse_pat(input)?;
        let guard = Self::parse_guard(input)?;
        let fat_arrow_token = input.parse()?;
        debug_trace!("parsing body tokens");
        let body = crate::expr::parse_tokens(input)
            .wrap_err(input.error("failed to parse body tokens"))?;
        debug_trace!("parsed body tokens: {body}");
        let comma = input.parse()?;
        Ok(Self {
            attrs,
            path,
            path_sep,
            _brace_token: brace_token,
            entries,
            pat,
            guard,
            fat_arrow_token,
            body,
            comma,
        })
    }
}

impl DelegateArm {
    /// Parse the optional path (e.g. `::abc::SomeEnum`) before `::{`.
    fn parse_path(input: ParseStream<'_>) -> syn::Result<Option<syn::Path>> {
        if input.peek(syn::token::Brace) {
            return Ok(None);
        }
        let mut tokens: Vec<TokenTree> = vec![];
        while !input.is_empty() {
            if input.peek(Token![::]) && input.peek3(syn::token::Brace) {
                if tokens.is_empty() {
                    return Err(syn::Error::new(
                        input.span(),
                        "found leading path separator, expected non-crate path",
                    ));
                }
                break;
            }
            let tt = input.parse()?;
            tokens.push(tt);
        }
        let ts: TokenStream2 = tokens.into_iter().collect();
        debug_trace!("parsed path: {ts}");
        syn::parse2(ts)
            .map(Some)
            .wrap_err(input.error("failed to parse delegate arm path"))
    }

    /// Parse the `{ ... }` part that contains one or more [`DelegateEntry`] items.
    fn parse_entries(
        input: ParseStream<'_>,
    ) -> syn::Result<(syn::token::Brace, Vec<DelegateEntry>)> {
        let content;
        let brace_token = syn::braced!(content in input);
        let entries = DelegateEntry::parse_multiple(&content).wrap_err(syn::Error::new(
            content.span(),
            "failed to parse delegate arm entry",
        ))?;
        Ok((brace_token, entries))
    }

    /// Parse tokens while tracking delimiter depth.
    fn parse_tokens_until<F>(input: ParseStream<'_>, mut f: F) -> syn::Result<TokenStream2>
    where
        F: FnMut(ParseStream<'_>, &TokenTree) -> bool,
    {
        let mut tokens = Vec::<TokenTree>::new();
        while !input.is_empty() {
            let tt: TokenTree = input.parse()?;
            let is_end = f(input, &tt);
            tokens.push(tt);
            if is_end {
                break;
            }
        }
        Ok(tokens.into_iter().collect())
    }

    fn parse_tokens_with_depth<F>(input: ParseStream<'_>, mut f: F) -> syn::Result<TokenStream2>
    where
        F: FnMut(ParseStream<'_>, &TokenTree, i32) -> bool,
    {
        let mut depth: i32 = 0;
        Self::parse_tokens_until(input, |input, tt| {
            if let TokenTree::Punct(p) = tt {
                match p.as_char() {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => depth -= 1,
                    _ => {}
                }
            }
            f(input, tt, depth)
        })
    }

    /// Parse an optional pattern that follows the entry list.
    fn parse_pat(input: ParseStream<'_>) -> syn::Result<Option<TokenStream2>> {
        if input.peek(Token![if]) || input.peek(Token![=>]) {
            return Ok(None);
        }
        let tokens = Self::parse_tokens_with_depth(input, |input, _tt, depth| {
            depth == 0 && (input.peek(Token![if]) || input.peek(Token![=>]))
        })?;
        Ok(Some(tokens))
    }

    /// Parse an optional `if <expr>` guard that can accompany a match arm.
    fn parse_guard(input: ParseStream<'_>) -> syn::Result<Option<(Token![if], TokenStream2)>> {
        if !input.peek(Token![if]) {
            return Ok(None);
        }
        let if_token: Token![if] = input.parse()?;
        let tokens = Self::parse_tokens_with_depth(input, |input, _tt, depth| {
            depth == 0 && input.peek(Token![=>])
        })?;
        Ok(Some((if_token, tokens.into_iter().collect())))
    }

    /// Parse a `::` token if outer path is present.
    fn parse_path_sep(
        input: ParseStream<'_>,
        outer_path: Option<&syn::Path>,
    ) -> syn::Result<Option<Token![::]>> {
        if outer_path.is_some() {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }

    /// Expand the grouped delegate arm into a list of concrete [`syn::Arm`]s.
    fn to_arms(&self) -> syn::Result<Vec<syn::Arm>> {
        self.entries
            .iter()
            .map(|entry| self.to_arm_with(entry))
            .collect()
    }

    /// Build one concrete [`syn::Arm`] from the template combined with the given `entry`.
    fn to_arm_with(&self, entry: &DelegateEntry) -> syn::Result<syn::Arm> {
        let pat = self.to_pattern_with(entry)?;
        let body_expr = Self::to_substituted_expr_with(
            &self.body,
            self.body.span(),
            syn::Expr::parse_with_earlier_boundary_rule,
            entry,
        )
        .wrap_err_with(|| syn::Error::new(self.body.span(), "failed to parse delegate arm body"))?;

        // Keep whatever comma the user wrote; emit error if missing when needed.
        let comma = self.comma;
        if comma.is_none() && body_expr.needs_comma() {
            return Err(syn::Error::new(
                self.body.span(),
                "missing comma after match arm body",
            ));
        }

        let guard = match &self.guard {
            Some((if_tok, guard_ts)) => {
                let guard_expr = Self::to_substituted_expr_with(
                    guard_ts,
                    guard_ts.span(),
                    syn::Expr::parse_with_earlier_boundary_rule,
                    entry,
                )
                .wrap_err(syn::Error::new(
                    guard_ts.span(),
                    "failed to parse guard expression",
                ))?;
                Some((*if_tok, Box::new(guard_expr)))
            }
            None => None,
        };

        Ok(syn::Arm {
            attrs: self.attrs.clone(),
            pat,
            guard,
            fat_arrow_token: self.fat_arrow_token,
            body: Box::new(body_expr),
            comma,
        })
    }

    /// Combine path, the current entry pattern and pattern into the
    /// final pattern of the generated arm.
    fn to_pattern_with(&self, entry: &DelegateEntry) -> syn::Result<syn::Pat> {
        use quote::quote;
        let path_ref = &self.path;
        let sep = self.path_sep.as_ref();

        let associated = entry.associated_tokens();
        let arm_pat_ts = self
            .pat
            .as_ref()
            .map(|ts| crate::substitute::substitute(ts, Some(&entry.pat), associated));

        let entry_pat_tokens = &entry.pat;

        let verbatim_join =
            || syn::Pat::Verbatim(quote!(#path_ref #sep #entry_pat_tokens #arm_pat_ts));
        let final_pat = match &entry.pat {
            syn::Pat::Ident(_) | syn::Pat::Path(_) => verbatim_join(),
            syn::Pat::TupleStruct(_) | syn::Pat::Struct(_) if arm_pat_ts.is_none() => {
                verbatim_join()
            }
            _ => {
                if arm_pat_ts.is_some() {
                    return Err(syn::Error::new(
                        entry.pat.span(),
                        "entry pattern incompatible with arm pattern",
                    ));
                }
                entry.pat.clone()
            }
        };

        Ok(final_pat)
    }

    /// Substitute placeholders in the user-provided body for the given entry.
    fn to_substituted_expr_with<F>(
        ts: &TokenStream2,
        span: proc_macro2::Span,
        f: F,
        entry: &DelegateEntry,
    ) -> syn::Result<syn::Expr>
    where
        F: FnOnce(ParseStream<'_>) -> syn::Result<syn::Expr>,
    {
        debug_trace!("tokenizing substituted expr");
        let associated = entry.associated_tokens();
        debug_trace!("input: {}", ts);
        let tokens = crate::substitute::substitute(ts, Some(&entry.pat), associated);
        debug_trace!("substituted tokens: {tokens}");
        let expr = syn::parse::Parser::parse2(f, tokens).wrap_err(syn::Error::new(
            span,
            "failed to parse expr after substitution",
        ))?;
        Ok(expr)
    }
}
