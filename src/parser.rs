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

#[derive(Clone, Debug)]
enum FillableExprs
{
    Exprs(Vec<Box<Expr>>),
    FilledExprs(Box<Expr>, Box<Expr>),
}

pub struct Parser<'a>
{
    path: Arc<String>,
    tokens: PushbackIter<&'a mut dyn Iterator<Item = Result<(Token, Pos)>>>,
}

impl<'a> Parser<'a>
{
    pub fn new(path: Arc<String>, tokens: &'a mut dyn Iterator<Item = Result<(Token, Pos)>>) -> Self
    { Parser { path, tokens: PushbackIter::new(tokens), } }
    
    fn parse_newline(&mut self) -> Result<()>
    {
        match self.tokens.next().transpose()? {
            Some((Token::Newline, _)) => Ok(()),
            Some((_, pos)) => Err(Error::Parser(pos, String::from("unexpected token"))),
            None => Err(Error::ParserEof(self.path.clone(), ParserEofFlag::Repetition)),
        }
    }

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
    
    fn parse_zero_or_more_with_newlines<T, F>(&mut self, end_tokens: &[Option<Token>], flag: ParserEofFlag, mut f: F) -> Result<Vec<T>>
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
                None => return Err(Error::ParserEof(self.path.clone(), flag)),
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

    fn parse_zero_or_more_with_commas<T, F>(&mut self, end_tokens: &[Option<Token>], flag: ParserEofFlag, mut f: F) -> Result<Vec<T>>
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
                None => return Err(Error::ParserEof(self.path.clone(), flag)),
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
            let expr_pos = expr.pos().clone();
            match self.tokens.next().transpose()? {
                Some((Token::LParen, _)) => {
                    expr = Box::new(Expr::App(expr, self.parse_zero_or_more_with_commas(&[Some(Token::RParen)], ParserEofFlag::NoRepetition, Self::parse_expr)?, expr_pos));
                    match self.tokens.next().transpose()? {
                        Some((Token::RParen, _)) => (),
                        Some((_, pos2)) => return Err(Error::Parser(pos2, String::from("unclosed parenthesis"))),
                        None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
                    }
                },
                Some((Token::LBracket, _)) => {
                    expr = Box::new(Expr::BinOp(BinOp::Index, expr, self.parse_expr()?, expr_pos));
                    match self.tokens.next().transpose()? {
                        Some((Token::RBracket, _)) => (),
                        Some((_, pos2)) => return Err(Error::Parser(pos2, String::from("unclosed bracket"))),
                        None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
                    }
                },
                Some((Token::Dot, _)) => expr = Box::new(Expr::Field(expr, self.parse_ident()?.0, expr_pos)),
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
                let expr_pos = pos.clone();
                self.tokens.undo(Ok((token, pos)));
                let mut expr = self.parse_expr9()?;
                loop {
                    match self.tokens.next().transpose()? {
                        Some((Token::Apos, _)) => expr = Box::new(Expr::UnaryOp(UnaryOp::Transpose, expr, expr_pos.clone())),
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
            let expr_pos = expr.pos().clone();
            match self.tokens.next().transpose()? {
                Some((Token::Star, _)) => expr = Box::new(Expr::BinOp(BinOp::Mul, expr, self.parse_expr8()?, expr_pos)),
                Some((Token::DotStar, _)) => expr = Box::new(Expr::BinOp(BinOp::DotMul, expr, self.parse_expr8()?, expr_pos)),
                Some((Token::Slash, _)) => expr = Box::new(Expr::BinOp(BinOp::Div, expr, self.parse_expr8()?, expr_pos)),
                Some((Token::DotSlash, _)) => expr = Box::new(Expr::BinOp(BinOp::DotDiv, expr, self.parse_expr8()?, expr_pos)),
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
            let expr_pos = expr.pos().clone();
            match self.tokens.next().transpose()? {
                Some((Token::Plus, _)) => expr = Box::new(Expr::BinOp(BinOp::Add, expr, self.parse_expr7()?, expr_pos)),
                Some((Token::DotPlus, _)) => expr = Box::new(Expr::BinOp(BinOp::DotAdd, expr, self.parse_expr7()?, expr_pos)),
                Some((Token::Minus, _)) => expr = Box::new(Expr::BinOp(BinOp::Sub, expr, self.parse_expr7()?, expr_pos)),
                Some((Token::DotMinus, _)) => expr = Box::new(Expr::BinOp(BinOp::DotSub, expr, self.parse_expr7()?, expr_pos)),
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
        let expr_pos = expr.pos().clone();
        match self.tokens.next().transpose()? {
            Some((Token::To, _)) => {
                let expr2 = self.parse_expr6()?;
                let expr3 = match self.tokens.next().transpose()? {
                    Some((Token::By, _)) => Some(self.parse_expr6()?),
                    Some((token2, pos2)) => {
                        self.tokens.undo(Ok((token2, pos2)));
                        None
                    },
                    None => None,
                };
                expr = Box::new(Expr::Range(expr, expr2, expr3, expr_pos));
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
            let expr_pos = expr.pos().clone();
            match self.tokens.next().transpose()? {
                Some((Token::Lt, _)) => expr = Box::new(Expr::BinOp(BinOp::Lt, expr, self.parse_expr5()?, expr_pos)),
                Some((Token::GtEq, _)) => expr = Box::new(Expr::BinOp(BinOp::Ge, expr, self.parse_expr5()?, expr_pos)),
                Some((Token::Gt, _)) => expr = Box::new(Expr::BinOp(BinOp::Gt, expr, self.parse_expr5()?, expr_pos)),
                Some((Token::LtEq, _)) => expr = Box::new(Expr::BinOp(BinOp::Le, expr, self.parse_expr5()?, expr_pos)),
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
            let expr_pos = expr.pos().clone();
            match self.tokens.next().transpose()? {
                Some((Token::EqEq, _)) => expr = Box::new(Expr::BinOp(BinOp::Eq, expr, self.parse_expr4()?, expr_pos)),
                Some((Token::ExEq, _)) => expr = Box::new(Expr::BinOp(BinOp::Ne, expr, self.parse_expr4()?, expr_pos)),
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
            let expr_pos = expr.pos().clone();
            match self.tokens.next().transpose()? {
                Some((Token::And, _)) => expr = Box::new(Expr::BinOp(BinOp::And, expr, self.parse_expr3()?, expr_pos)),
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
            let expr_pos = expr.pos().clone();
            match self.tokens.next().transpose()? {
                Some((Token::Or, _)) => expr = Box::new(Expr::BinOp(BinOp::Or, expr, self.parse_expr2()?, expr_pos)),
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
    {
        match self.tokens.next().transpose()? {
            Some((Token::False, pos)) => Ok((Lit::Bool(false), pos)),
            Some((Token::True, pos)) => Ok((Lit::Bool(true), pos)),
            Some((Token::Int(n), pos)) => Ok((Lit::Int(n), pos)),
            Some((Token::Float(n), pos)) => Ok((Lit::Float(n), pos)),
            Some((Token::Inf, pos)) => Ok((Lit::Float(f32::INFINITY), pos)),
            Some((Token::Nan, pos)) => Ok((Lit::Float(f32::NAN), pos)),
            Some((Token::String(s), pos)) => Ok((Lit::String(s), pos)),
            Some((Token::LBracket, pos)) => {
                self.parse_newlines()?;
                match self.tokens.next().transpose()? {
                    Some((Token::RBracket, _)) => Ok((Lit::Matrix(Vec::new()), pos)),
                    Some((token2, pos2)) => {
                        self.tokens.undo(Ok((token2, pos2)));
                        let matrix_row = self.parse_matrix_row()?;
                        let lit = match self.tokens.next().transpose()? {
                            Some((Token::Newline, _)) => {
                                self.parse_newlines()?;
                                match self.tokens.next().transpose()? {
                                    Some((Token::Fill, _)) => {
                                        let expr = self.parse_expr()?;
                                        self.parse_newlines()?;
                                        Lit::FilledMatrix(matrix_row, expr)
                                    },
                                    Some((token4, pos4))=> {
                                        self.tokens.undo(Ok((token4, pos4)));
                                        let mut matrix_rows = vec![matrix_row];
                                        matrix_rows.extend_from_slice(self.parse_zero_or_more_with_newlines(&[Some(Token::RBracket)], ParserEofFlag::Repetition, Self::parse_matrix_row)?.as_slice());
                                        Lit::Matrix(matrix_rows)
                                    },
                                    None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::Repetition)),
                                }
                            },
                            Some((token3, pos3)) => {
                                self.tokens.undo(Ok((token3, pos3)));
                                Lit::Matrix(vec![matrix_row])
                            },
                            None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::Repetition)),
                        };
                        match self.tokens.next().transpose()? {
                            Some((Token::RBracket, _)) => Ok((lit, pos)),
                            Some((_, pos3)) => Err(Error::Parser(pos3, String::from("unclosed bracket"))),
                            None => Err(Error::ParserEof(self.path.clone(), ParserEofFlag::Repetition)),
                        }
                    },
                    None => Err(Error::ParserEof(self.path.clone(), ParserEofFlag::Repetition)),
                }
            },
            Some((_, pos)) => Err(Error::Parser(pos, String::from("unexpected token"))),
            None => Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
        }
    }

    fn parse_fillable_exprs(&mut self, end_tokens: &[Option<Token>]) -> Result<FillableExprs>
    {
        match self.tokens.next().transpose()? {
            Some((token, _)) if end_tokens.contains(&Some(token.clone())) => Ok(FillableExprs::Exprs(Vec::new())),
            Some((token, pos)) => {
                self.tokens.undo(Ok((token, pos)));
                let expr = self.parse_expr()?;
                match self.tokens.next().transpose()? {
                    Some((Token::Comma, _)) => {
                        let mut exprs = vec![expr];
                        exprs.extend_from_slice(self.parse_zero_or_more_with_commas(end_tokens, ParserEofFlag::Repetition, Self::parse_expr)?.as_slice());
                        Ok(FillableExprs::Exprs(exprs))
                    },
                    Some((Token::Fill, _)) => Ok(FillableExprs::FilledExprs(expr, self.parse_expr()?)),
                    Some((token2, pos2)) => {
                        self.tokens.undo(Ok((token2, pos2)));
                        Ok(FillableExprs::Exprs(vec![expr]))
                    },
                    None => Ok(FillableExprs::Exprs(vec![expr]))
                }
            },
            None if end_tokens.contains(&None) => Ok(FillableExprs::Exprs(Vec::new())),
            None => Err(Error::ParserEof(self.path.clone(), ParserEofFlag::Repetition)),
        }
    }

    fn parse_matrix_row(&mut self) -> Result<MatrixRow>
    {
        match self.parse_fillable_exprs(&[Some(Token::RBracket), Some(Token::Newline)])? {
            FillableExprs::Exprs(exprs) => Ok(MatrixRow::Row(exprs)),
            FillableExprs::FilledExprs(expr, expr2) => Ok(MatrixRow::FilledRow(expr, expr2)),
        }
    }
    
    fn parse_lvalue2(&mut self) -> Result<Box<Lvalue>>
    {
        match self.tokens.next().transpose()? {
            Some((Token::LParen, _)) => {
                let lvalue = self.parse_lvalue()?;
                match self.tokens.next().transpose()? {
                    Some((Token::RParen, _)) => (),
                    Some((_, pos2)) => return Err(Error::Parser(pos2, String::from("unclosed parenthesis"))),
                    None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
                }
                Ok(lvalue)
            },
            Some((token, pos)) => {
                self.tokens.undo(Ok((token, pos)));
                let (name, name_pos) = self.parse_name()?;
                Ok(Box::new(Lvalue::Var(name, name_pos)))
            },
            None => Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
        }
    }
    
    fn parse_lvalue(&mut self) -> Result<Box<Lvalue>>
    {
        let mut lvalue = self.parse_lvalue2()?;
        loop {
            let lvalue_pos = lvalue.pos().clone();
            match self.tokens.next().transpose()? {
                Some((Token::LBracket, _)) => {
                    lvalue = Box::new(Lvalue::Index(lvalue, self.parse_expr()?, lvalue_pos));
                    match self.tokens.next().transpose()? {
                        Some((Token::RBracket, _)) => (),
                        Some((_, pos2)) => return Err(Error::Parser(pos2, String::from("unclosed bracket"))),
                        None => return Err(Error::ParserEof(self.path.clone(), ParserEofFlag::NoRepetition)),
                    }
                },
                Some((Token::Dot, _)) => lvalue = Box::new(Lvalue::Field(lvalue, self.parse_ident()?.0, lvalue_pos)),
                Some((token, pos)) => {
                    self.tokens.undo(Ok((token, pos)));
                    break;
                },
                None => break,
            }
        }
        Ok(lvalue)
    }    
    
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
