//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use crate::error::*;
use crate::lexer::*;
use crate::tree::*;
use crate::utils::*;

pub struct Parser<'a>
{
    path: Arc<String>,
    tokens: PushbackIter<&'a mut dyn Iterator<Item = Result<(Token, Pos)>>>,
}

impl<'a> Parser<'a>
{
    pub fn new(path: Arc<String>, tokens: &'a mut dyn Iterator<Item = Result<(Token, Pos)>>) -> Self
    { Parser { path, tokens: PushbackIter::new(tokens), } }
    
    fn parse_newlines(&mut self) -> Result<()>
    {
        loop {
            match self.tokens.next().transpose()? {
                Some((Token::Newline, _)) => (),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(())
    }
    
    fn parse_zero_or_more_with_newlines<T, F>(&mut self, end_tokens: &[Option<Token>], mut f: F) -> Result<Vec<T>>
        where F: FnMut(&mut Self) -> Result<T>
    {
        let mut xs: Vec<T> = Vec::new();
        self.parse_newlines()?;
        loop {
            match self.tokens.next().transpose()? {
                    Some((token, pos)) if end_tokens.contains(&Some(token.clone())) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                Some((token, pos)) => self.tokens.undo(Ok((token, pos))),
                None if end_tokens.contains(&None) => break,
                None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::Repetition)),
            }
            xs.push(f(self)?);
            match self.tokens.next().transpose()? {
                Some((Token::Newline, _)) => (),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
            self.parse_newlines()?;
        }
        Ok(xs)
    }

    fn parse_zero_or_more_with_commas<T, F>(&mut self, end_tokens: &[Option<Token>], mut f: F) -> Result<Vec<T>>
        where F: FnMut(&mut Self) -> Result<T>
    {
        let mut xs: Vec<T> = Vec::new();
        loop {
            match self.tokens.next().transpose()? {
                    Some((token, pos)) if end_tokens.contains(&Some(token.clone())) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                Some((token, pos)) => self.tokens.undo(Ok((token, pos))),
                None if end_tokens.contains(&None) => break,
                None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
            }
            xs.push(f(self)?);
            match self.tokens.next().transpose()? {
                Some((Token::Comma, _)) => (),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(xs)
    }
}
