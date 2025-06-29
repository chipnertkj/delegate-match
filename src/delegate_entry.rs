//! Data structure and parsing logic for a single *entry* inside a grouped
//! delegate arm.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse::ParseStream, Token};

use crate::util::debug_trace;

/// One item inside the entry list of a [`DelegateArm`]: `{ ... }`.
/// It consists of a pattern plus an optional *associated* token stream after `:`.
/// During expansion, those two pieces are available as `$entry_pat` and `$assoc_ts`
/// [placeholders](crate::shared::substitute::substitute_tokens_raw) inside the delegate arm's body.
///
/// [DelegateArm]: crate::delegate_arm::DelegateArm
#[derive(Clone)]
pub struct DelegateEntry {
    pub pat: syn::Pat,
    pub associated: Option<(Token![:], TokenStream2)>,
    pub _comma: Option<Token![,]>,
}

impl syn::parse::Parse for DelegateEntry {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        debug_trace!("parsing entry in: {}", input);
        let pat = syn::Pat::parse_single(input)?;
        debug_trace!("parsed pat: {}", pat.to_token_stream());
        debug_trace!("left: {}", input);
        let associated = Self::parse_associated_tokens(input)?;
        if let Some((_, tokens)) = &associated {
            debug_trace!("parsed associated: {tokens}");
        } else {
            debug_trace!("no associated tokens");
        }
        let comma = if input.is_empty() {
            None
        } else if input.peek(Token![,]) {
            Some(input.parse()?)
        } else {
            return Err(input.error("expected comma after entry"));
        };
        debug_trace!("parsed entry: {}", pat.to_token_stream());
        Ok(Self {
            pat,
            associated,
            _comma: comma,
        })
    }
}

impl DelegateEntry {
    /// Parse a comma-separated list of [`DelegateEntry`] items until the end of `input`.
    pub(crate) fn parse_multiple(input: ParseStream<'_>) -> syn::Result<Vec<Self>> {
        let mut v = Vec::new();
        while !input.is_empty() {
            v.push(input.parse()?);
        }
        Ok(v)
    }

    /// Return the user-supplied token stream that followed a `:` after the entry, if any.
    pub(crate) fn associated_tokens(&self) -> Option<&TokenStream2> {
        self.associated.as_ref().map(|(_, tokens)| tokens)
    }

    /// Parse the optional `: <tokens>` part that can accompany a pattern inside the entry list.
    fn parse_associated_tokens(
        input: ParseStream<'_>,
    ) -> syn::Result<Option<(Token![:], TokenStream2)>> {
        if input.peek(Token![:]) {
            debug_trace!("parsing associated tokens");
            let colon_token = input.parse()?;
            let tokens = crate::expr::parse_tokens(input)?;
            Ok(Some((colon_token, tokens)))
        } else {
            Ok(None)
        }
    }
}
