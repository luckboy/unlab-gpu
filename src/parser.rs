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
    
    fn parse_expr10(&mut self) -> Result<Box<Expr>>
    {
        match self.tokens.next().transpose()? {
            Some((Token::LParen, _)) => {
                let expr = self.parse_expr()?;
                match self.tokens.next().transpose()? {
                    Some((Token::RParen, _)) => (),
                    Some((_, pos2)) => return Err(Error::Parser(pos2, String::from("unclosed parenthesis"))),
                    None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
                }
                Ok(expr)
            },
            Some((token @ (Token::ColonColon | Token::Root | Token::Ident(_)), pos)) => {
                self.tokens.undo(Ok((token, pos)));
                let (name, name_pos) = self.parse_name()?;
                Ok(Box::new(Expr::Var(name, name_pos)))
            },
            Some((token, pos)) => {
                self.tokens.undo(Ok((token, pos)));
                let (lit, lit_pos) = self.parse_lit()?;
                Ok(Box::new(Expr::Lit(lit, lit_pos)))
            },
            None => Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
        }
    }

    fn parse_expr9(&mut self) -> Result<Box<Expr>>
    {
        let mut expr = self.parse_expr10()?;
        loop {
            match self.tokens.next().transpose()? {
                Some((Token::LParen, pos)) => {
                    expr = Box::new(Expr::App(expr, self.parse_zero_or_more_with_commas(&[Some(Token::RParen)], Self::parse_expr)?, pos));
                    match self.tokens.next().transpose()? {
                        Some((Token::RParen, _)) => (),
                        Some((_, pos2)) => return Err(Error::Parser(pos2, String::from("unclosed parenthesis"))),
                        None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
                    }
                },
                Some((Token::LBracket, pos)) => {
                    expr = Box::new(Expr::BinOp(BinOp::Mul, expr, self.parse_expr()?, pos));
                    match self.tokens.next().transpose()? {
                        Some((Token::RBracket, _)) => (),
                        Some((_, pos2)) => return Err(Error::Parser(pos2, String::from("unclosed bracket"))),
                        None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
                    }
                },
                Some((Token::Dot, pos)) => expr = Box::new(Expr::Field(expr, self.parse_ident()?.0, pos)),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(expr)
    }
    
    fn parse_expr8(&mut self) -> Result<Box<Expr>>
    {
        match self.tokens.next().transpose()? {
            Some((Token::Minus, pos)) => Ok(Box::new(Expr::UnaryOp(UnaryOp::Neg, self.parse_expr8()?, pos))),
            Some((Token::DotMinus, pos)) => Ok(Box::new(Expr::UnaryOp(UnaryOp::DotNeg, self.parse_expr8()?, pos))),
            Some((Token::Not, pos)) => Ok(Box::new(Expr::UnaryOp(UnaryOp::Not, self.parse_expr8()?, pos))),
            Some((token, pos)) => {
                self.tokens.undo(Ok((token, pos)));
                let mut expr = self.parse_expr9()?;
                loop {
                    match self.tokens.next().transpose()? {
                        Some((Token::Apos, pos2)) => expr = Box::new(Expr::UnaryOp(UnaryOp::Transpose, expr, pos2)),
                        Some((token2, pos2)) => {
                            self.tokens.undo(Ok((token2, pos2)));
                            break;
                        },
                        None => break,
                    }
                }
                Ok(expr)
            },
            None => Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
        }
    }
    
    fn parse_expr7(&mut self) -> Result<Box<Expr>>
    {
        let mut expr = self.parse_expr8()?;
        loop {
            match self.tokens.next().transpose()? {
                Some((Token::Star, pos)) => expr = Box::new(Expr::BinOp(BinOp::Mul, expr, self.parse_expr8()?, pos)),
                Some((Token::DotStar, pos)) => expr = Box::new(Expr::BinOp(BinOp::DotMul, expr, self.parse_expr8()?, pos)),
                Some((Token::Slash, pos)) => expr = Box::new(Expr::BinOp(BinOp::Div, expr, self.parse_expr8()?, pos)),
                Some((Token::DotSlash, pos)) => expr = Box::new(Expr::BinOp(BinOp::DotDiv, expr, self.parse_expr8()?, pos)),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(expr)
    }
    
    fn parse_expr6(&mut self) -> Result<Box<Expr>>
    {
        let mut expr = self.parse_expr7()?;
        loop {
            match self.tokens.next().transpose()? {
                Some((Token::Plus, pos)) => expr = Box::new(Expr::BinOp(BinOp::Add, expr, self.parse_expr7()?, pos)),
                Some((Token::DotPlus, pos)) => expr = Box::new(Expr::BinOp(BinOp::DotAdd, expr, self.parse_expr7()?, pos)),
                Some((Token::Minus, pos)) => expr = Box::new(Expr::BinOp(BinOp::Sub, expr, self.parse_expr7()?, pos)),
                Some((Token::DotMinus, pos)) => expr = Box::new(Expr::BinOp(BinOp::DotSub, expr, self.parse_expr7()?, pos)),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(expr)
    }
    
    fn parse_expr5(&mut self) -> Result<Box<Expr>>
    {
        let mut expr = self.parse_expr6()?;
        match self.tokens.next().transpose()? {
            Some((Token::To, pos)) => {
                let expr2 = self.parse_expr6()?;
                let expr3 = match self.tokens.next().transpose()? {
                    Some((Token::By, _)) => Some(self.parse_expr6()?),
                    Some((token2, pos2)) => {
                        self.tokens.undo(Ok((token2, pos2)));
                        None
                    },
                    None => None,
                };
                expr = Box::new(Expr::Range(expr, expr2, expr3, pos));
            },
            Some((token, pos)) => self.tokens.undo(Ok((token, pos))),
            None => (),
        }
        Ok(expr)
    }

    fn parse_expr4(&mut self) -> Result<Box<Expr>>
    {
        let mut expr = self.parse_expr5()?;
        loop {
            match self.tokens.next().transpose()? {
                Some((Token::Lt, pos)) => expr = Box::new(Expr::BinOp(BinOp::Lt, expr, self.parse_expr5()?, pos)),
                Some((Token::GtEq, pos)) => expr = Box::new(Expr::BinOp(BinOp::Ge, expr, self.parse_expr5()?, pos)),
                Some((Token::Gt, pos)) => expr = Box::new(Expr::BinOp(BinOp::Gt, expr, self.parse_expr5()?, pos)),
                Some((Token::LtEq, pos)) => expr = Box::new(Expr::BinOp(BinOp::Le, expr, self.parse_expr5()?, pos)),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(expr)
    }
    
    fn parse_expr3(&mut self) -> Result<Box<Expr>>
    {
        let mut expr = self.parse_expr4()?;
        loop {
            match self.tokens.next().transpose()? {
                Some((Token::EqEq, pos)) => expr = Box::new(Expr::BinOp(BinOp::Eq, expr, self.parse_expr4()?, pos)),
                Some((Token::ExEq, pos)) => expr = Box::new(Expr::BinOp(BinOp::Ne, expr, self.parse_expr4()?, pos)),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(expr)
    }

    fn parse_expr2(&mut self) -> Result<Box<Expr>>
    {
        let mut expr = self.parse_expr3()?;
        loop {
            match self.tokens.next().transpose()? {
                Some((Token::And, pos)) => expr = Box::new(Expr::BinOp(BinOp::And, expr, self.parse_expr3()?, pos)),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(expr)
    }
    
    fn parse_expr(&mut self) -> Result<Box<Expr>>
    {
        let mut expr = self.parse_expr2()?;
        loop {
            match self.tokens.next().transpose()? {
                Some((Token::Or, pos)) => expr = Box::new(Expr::BinOp(BinOp::Or, expr, self.parse_expr2()?, pos)),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(expr)
    }

    fn parse_lit(&mut self) -> Result<(Lit, Pos)>
    { Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)) }
    
    fn parse_name(&mut self) -> Result<(Name, Pos)>
    {
        let mut idents: Vec<String> = Vec::new();
        let mut last_ident = String::new();
        let mut is_last_ident = true;
        let mut name_pos = Pos::new(self.path.clone(), 1, 1);
        let mut is_name_pos = false;
        let mut is_first_colon_colon = false;
        let mut is_root = true;
        match self.tokens.next().transpose()? {
            Some((Token::ColonColon, pos)) => {
                name_pos = pos.clone();
                is_name_pos = true;
                is_first_colon_colon = true;
            },
            Some((token, pos)) => self.tokens.undo(Ok((token, pos))),
            None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
        }
        match self.tokens.next().transpose()? {
            Some((Token::Root, pos)) => {
                if !is_name_pos {
                    name_pos = pos;
                }
                is_root = true
            },
            Some((Token::Ident(ident), pos)) => {
                if !is_name_pos {
                    name_pos = pos;
                }
                last_ident = ident;
                is_last_ident = true;
            },
            Some((_, pos)) => return Err(Error::Parser(pos, String::from("unexpected token"))),
            None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
        }
        loop {
            match self.tokens.next().transpose()? {
                Some((Token::ColonColon, _)) => (),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
            if is_last_ident {
                idents.push(last_ident.clone());
            }
            last_ident = self.parse_ident()?.0;
            is_last_ident = true;
        }
        if !is_last_ident {
            return Err(Error::Parser(name_pos, String::from("no last identifier")))
        }
        if is_root {
            Ok((Name::Abs(idents, last_ident), name_pos))
        } else {
            if idents.is_empty() {
                if is_first_colon_colon {
                    Ok((Name::Rel(Vec::new(), last_ident), name_pos))
                } else {
                    Ok((Name::Var(last_ident), name_pos))
                }
            } else {
                Ok((Name::Rel(idents, last_ident), name_pos))
            }
        }
    }
    
    fn parse_ident(&mut self) -> Result<(String, Pos)>
    {
        match self.tokens.next().transpose()? {
            Some((Token::Ident(ident), pos)) => Ok((ident, pos)),
            Some((_, pos)) => Err(Error::Parser(pos, String::from("unexpected token"))),
            None => Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
        }
    }
}
