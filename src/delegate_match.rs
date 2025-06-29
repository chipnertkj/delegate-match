//! Top-level parser for the `delegate_match!` procedural macro.
//!
//! This module defines [`ExprDelegateMatch`], a syntactic representation of the
//! user-written `match` expression as well as the helper [`Arm`] enum that
//! distinguishes between regular Rust arms and the custom *delegate* arms.
//!
//! During macro expansion each parsed structure is converted back into tokens
//! via the [`ToTokens`] implementations defined here.

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, TokenStreamExt as _};
use syn::{parse::discouraged::Speculative as _, Token};

use crate::delegate_arm::DelegateArm;

/// Represents the entire input to the `delegate_match!` procedural macro.
/// Corresponds 1-to-1 to the user-written `match` expression.
#[allow(clippy::module_name_repetitions)]
pub struct ExprDelegateMatch {
    pub outer_attrs: Vec<syn::Attribute>,
    pub match_token: Token![match],
    pub expr: Box<syn::Expr>,
    pub brace_token: syn::token::Brace,
    pub inner_attrs: Vec<syn::Attribute>,
    pub arms: Vec<Arm>,
}

impl ToTokens for ExprDelegateMatch {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(&self.outer_attrs);
        self.match_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_all(&self.inner_attrs);
            for arm in &self.arms {
                arm.to_tokens(tokens);
            }
        });
    }
}

impl syn::parse::Parse for ExprDelegateMatch {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let outer_attrs = input.call(syn::Attribute::parse_outer)?;
        let match_token: Token![match] = input.parse()?;
        let expr = syn::Expr::parse_without_eager_brace(input)?;
        let content;
        let brace_token = syn::braced!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;
        let arms = Arm::parse_all(&content)?;
        Ok(Self {
            outer_attrs,
            match_token,
            expr: Box::new(expr),
            brace_token,
            inner_attrs,
            arms,
        })
    }
}

/// Either a delegate arm (special syntax) or a regular Rust match arm.
#[derive(Clone)]
pub enum Arm {
    Delegate(DelegateArm),
    Regular(syn::Arm),
}

impl Arm {
    fn parse_all(input: syn::parse::ParseStream<'_>) -> syn::Result<Vec<Self>> {
        let mut arms = Vec::new();
        while !input.is_empty() {
            arms.push(input.parse()?);
        }
        Ok(arms)
    }
}

impl syn::parse::Parse for Arm {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let fork = input.fork();
        // Try a normal arm first
        let mut delegate_err = None;
        let result = fork.parse::<syn::Arm>().map_or_else(
            |e| {
                delegate_err = Some(e);
                input.parse::<DelegateArm>().map(Arm::Delegate)
            },
            |regular| {
                input.advance_to(&fork);
                Ok(Self::Regular(regular))
            },
        );
        result.map_or_else(
            move |mut regular_err| {
                input.advance_to(&fork);
                if let Some(delegate_err) = delegate_err {
                    regular_err.combine(delegate_err);
                }
                Err(regular_err)
            },
            Ok,
        )
    }
}

impl ToTokens for Arm {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Delegate(d) => d.to_tokens(tokens),
            Self::Regular(r) => r.to_tokens(tokens),
        }
    }
}
