//! Data structure and parsing logic for a single *entry* inside a grouped
//! delegate arm.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse::ParseStream, spanned::Spanned, Token};

use crate::{
    associated::Associated,
    util::{debug_trace, SynErrorContext as _},
};

/// One item inside the entry list of a [`DelegateArm`]: `{ ... }`.
/// It consists of a pattern plus an optional *associated* token stream after `:`.
/// During expansion, the two are available as `$entry_pat` and `$assoc_ts`
/// [placeholders](crate::substitute::substitute) inside the delegate arm's body.
///
/// [DelegateArm]: crate::delegate_arm::DelegateArm
#[derive(Clone)]
pub struct DelegateEntry {
    pub pat: syn::Pat,
    pub associated: Option<(Token![:], Associated)>,
    pub _comma: Option<Token![,]>,
}

impl syn::parse::Parse for DelegateEntry {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        debug_trace!("parsing entry in: {input}");
        let pat = syn::Pat::parse_single(input)?;
        debug_trace!("parsed pat: {}", pat.to_token_stream());
        let associated = Self::parse_associated(input)?;
        if let Some((_, associated)) = &associated {
            debug_trace!("parsed associated: {}", associated.to_token_stream());
        } else {
            debug_trace!("no associated tokens");
        }
        let comma = if input.is_empty() {
            None
        } else if input.peek(Token![,]) {
            Some(input.parse()?)
        } else {
            let mut err = syn::Error::new(pat.span(), "expected comma after entry");
            if let Some((_, associated)) = &associated {
                err = syn::Error::new(
                    associated.span(),
                    format!("note: associated was parsed as `{}`", associated.as_ref()),
                )
                .wrap_err(err);
            }
            return Err(err);
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

    /// Return the user-supplied token stream that followed a `:` after the entry, if available.
    pub(crate) fn associated_tokens(&self) -> Option<TokenStream2> {
        self.associated
            .as_ref()
            .map(|(_, associated)| associated.to_token_stream())
    }

    /// Parse the optional `: <tokens>` part that can accompany a pattern inside the entry list.
    fn parse_associated(input: ParseStream<'_>) -> syn::Result<Option<(Token![:], Associated)>> {
        if input.peek(Token![:]) {
            debug_trace!("parsing associated tokens in: {input}");
            let colon_token = input.parse()?;
            let associated = input.parse()?;
            Ok(Some((colon_token, associated)))
        } else {
            Ok(None)
        }
    }
}
