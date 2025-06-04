use core::fmt::Debug;

use winnow::{Parser, error::ContextError, stream::TokenSlice, token::literal};

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Token<'s> {
    kind: TokenKind,
    raw: &'s str,
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // For brevity, when using `winnow/debug`
        match self.kind {
            TokenKind::Dice => Debug::fmt(self.raw, f),
            TokenKind::Value => Debug::fmt(self.raw, f),
            TokenKind::Oper(oper) => Debug::fmt(&oper, f),
            TokenKind::Label => f.write_str("Label"),
            TokenKind::OpenParen => f.write_str("OpenParen"),
            TokenKind::CloseParen => f.write_str("CloseParen"),
            TokenKind::Unknown => f.write_str("Unknown"),
            TokenKind::Eof => f.write_str("Eof"),
        }
    }
}

impl PartialEq<TokenKind> for Token<'_> {
    fn eq(&self, other: &TokenKind) -> bool {
        self.kind == *other
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Dice,
    Value,
    Oper(Oper),
    Label,
    OpenParen,
    CloseParen,
    Unknown,
    Eof,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Oper {
    Add,
    Sub,
    Mul,
    Div,
}

pub(crate) type Tokens<'i> = TokenSlice<'i, Token<'i>>;

// trait impls
impl<'i> Parser<Tokens<'i>, &'i Token<'i>, ContextError> for TokenKind {
    fn parse_next(
        &mut self,
        input: &mut Tokens<'i>,
    ) -> winnow::Result<&'i Token<'i>, ContextError> {
        literal(*self).parse_next(input).map(|t| &t[0])
    }
}
impl winnow::stream::ContainsToken<&'_ Token<'_>> for TokenKind {
    #[inline(always)]
    fn contains_token(&self, token: &'_ Token<'_>) -> bool {
        *self == token.kind
    }
}
impl winnow::stream::ContainsToken<&'_ Token<'_>> for &'_ [TokenKind] {
    #[inline(always)]
    fn contains_token(&self, token: &'_ Token<'_>) -> bool {
        self.iter().any(|t| *t == token.kind)
    }
}
impl<const LEN: usize> winnow::stream::ContainsToken<&'_ Token<'_>> for &'_ [TokenKind; LEN] {
    #[inline]
    fn contains_token(&self, token: &'_ Token<'_>) -> bool {
        self.iter().any(|t| *t == token.kind)
    }
}
impl<const LEN: usize> winnow::stream::ContainsToken<&'_ Token<'_>> for [TokenKind; LEN] {
    #[inline]
    fn contains_token(&self, token: &'_ Token<'_>) -> bool {
        self.iter().any(|t| *t == token.kind)
    }
}
// end trait impls
