//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Arc;
use crate::error::*;
use crate::utils::*;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Token
{
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    DotLBracket,
    DotRBracket,
    Star,
    Slash,
    DotStar,
    DotSlash,
    Plus,
    Minus,
    Lt,
    GtEq,
    Gt,
    LtEq,
    Eq,
    EqEq,
    Apos,
    Dot,
    Colon,
    ColonColon,
    Comma,
    Newline,
    And,
    End,
    Else,
    False,
    Fill,
    For,
    Function,
    If,
    In,
    Module,
    Not,
    Or,
    Return,
    True,
    While,
    Int(i64),
    Float(f32),
    String(String),
    Ident(String),
}

pub struct Lexer<'a>
{
    path: Arc<String>,
    line: u64,
    eol_column: usize,
    reader: &'a mut dyn BufRead,
    line_tokens: Vec<Result<(Token, Pos)>>,
    line_token_index: usize,
    is_stopped: bool,
    keywords: HashMap<String, Token>,
}

impl<'a> Lexer<'a>
{
    pub fn new(path: Arc<String>, reader: &'a mut dyn BufRead) -> Self
    {
        Lexer {
            path,
            line: 1,
            eol_column: 0,
            reader,
            line_tokens: Vec::new(),
            line_token_index: 0,
            is_stopped: false,
            keywords: HashMap::new(),
        }
    }
    
    fn read_tokens(&mut self)
    {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => self.is_stopped = true,
            Ok(_) => {
                let path = self.path.clone();
                let line_count = self.line;
                let line_without_crnl = str_without_crnl(line.as_str());
                let mut cs = PushbackIter::new(line_without_crnl.chars().enumerate().map(|p| (p.1, Pos::new(path.clone(), line_count, p.0 + 1))));
                self.line_tokens.clear();
                self.eol_column = line_without_crnl.chars().count() + 1;
                while self.read_token(&mut cs) {}
                self.line_tokens.push(Ok((Token::Newline, Pos::new(path, line_count, self.eol_column))));
                self.line_token_index = 0;
            },
            Err(err) => {
                self.line_tokens = vec![Err(Error::ParserIo(self.path.clone(), err))];
                self.line_token_index = 0;
                self.is_stopped = true;
            },
        }
    }
    
    fn skip_spaces<T: Iterator<Item = (char, Pos)>>(&self, cs: &mut PushbackIter<T>)
    {
        loop {
            match cs.next() {
                Some((c, _)) if c.is_whitespace() => (),
                Some((c, pos)) => {
                    cs.undo((c, pos));
                    break;
                },
                None => break,
            }
        }
    }
    
    fn read_one_or_more_digits<T: Iterator<Item = (char, Pos)>>(&mut self, cs: &mut PushbackIter<T>, s: &mut String, s_pos: Option<&mut Pos>) -> bool
    {
        match cs.next() {
            Some((c, pos)) if c.is_ascii_digit() => {
                match s_pos {
                    Some(s_pos) => *s_pos = pos.clone(),
                    None => (),
                }
                s.push(c);
                loop {
                    match cs.next() {
                        Some((c2, _)) if c2.is_ascii_digit() => s.push(c2),
                        Some((c2, pos2)) => {
                            cs.undo((c2, pos2));
                            break;
                        },
                        None => break,
                    }
                }
                true
            },
            Some((_, pos)) => {
                self.line_tokens.push(Err(Error::Parser(pos, String::from("no decimal digits"))));
                self.is_stopped = true;
                false
            },
            None => {
                self.line_tokens.push(Err(Error::ParserEof(Pos::new(self.path.clone(), self.line, self.eol_column))));
                self.is_stopped = true;
                false
            },
        }
    }

    fn read_one_or_more_hexdigits<T: Iterator<Item = (char, Pos)>>(&mut self, cs: &mut PushbackIter<T>, s: &mut String, s_pos: Option<&mut Pos>) -> bool
    {
        match cs.next() {
            Some((c, pos)) if c.is_ascii_hexdigit() => {
                match s_pos {
                    Some(s_pos) => *s_pos = pos.clone(),
                    None => (),
                }
                s.push(c);
                loop {
                    match cs.next() {
                        Some((c2, _)) if c2.is_ascii_hexdigit() => s.push(c2),
                        Some((c2, pos2)) => {
                            cs.undo((c2, pos2));
                            break;
                        },
                        None => break,
                    }
                }
                true
            },
            Some((_, pos)) => {
                self.line_tokens.push(Err(Error::Parser(pos, String::from("no hexadecimal digits"))));
                self.is_stopped = true;
                false
            },
            None => {
                self.line_tokens.push(Err(Error::ParserEof(Pos::new(self.path.clone(), self.line, self.eol_column))));
                self.is_stopped = true;
                false
            },
        }
    }
    
    fn read_number_token<T: Iterator<Item = (char, Pos)>>(&mut self, cs: &mut PushbackIter<T>) -> bool
    {
        let mut s = String::new(); 
        let mut s_pos = Pos::new(self.path.clone(), self.line, self.eol_column);
        let mut is_dot_or_exp = false;
        match cs.next() {
            Some((c @ '0', pos)) => {
                match cs.next() {
                    Some(('X' | 'x', _)) => {
                        s_pos = pos;
                        if !self.read_one_or_more_hexdigits(cs, &mut s, None) {
                            return false;
                        }
                        match i64::from_str_radix(s.as_str(), 16) {
                            Ok(n) => self.line_tokens.push(Ok((Token::Int(n), s_pos))),
                            Err(_) => {
                                self.line_tokens.push(Err(Error::Parser(s_pos, String::from("invalid number"))));
                                self.is_stopped = true;
                                return false;
                            },
                        }
                        return true;
                    },
                    Some((c2, pos2)) => {
                        cs.undo((c2, pos2));
                        cs.undo((c, pos));
                    },
                    None => cs.undo((c, pos)),
                }
            },
            Some((c, pos)) => cs.undo((c, pos)),
            None => (),
        }
        if !self.read_one_or_more_digits(cs, &mut s, Some(&mut s_pos)) {
            return false;
        }
        match cs.next() {
            Some((c @ '.', _)) => {
                s.push(c);
                is_dot_or_exp = true;
                if !self.read_one_or_more_digits(cs, &mut s, None) {
                    return false;
                }
            }
            Some((c, pos)) => cs.undo((c, pos)),
            None => (),
        }
        match cs.next() {
            Some((c @ ('E' | 'e'), _)) => {
                s.push(c);
                is_dot_or_exp = true;
                match cs.next() {
                    Some((c2 @ ('+' | '-'), _)) => s.push(c2),
                    Some((c2, pos2)) => cs.undo((c2, pos2)),
                    None => (),
                }
                if !self.read_one_or_more_digits(cs, &mut s, None) {
                    return false;
                }
            }
            Some((c, pos)) => cs.undo((c, pos)),
            None => (),
        }
        if is_dot_or_exp {
            match s.parse::<f32>() {
                Ok(n) => self.line_tokens.push(Ok((Token::Float(n), s_pos))),
                Err(_) => {
                    self.line_tokens.push(Err(Error::Parser(s_pos, String::from("invalid number"))));
                    self.is_stopped = true;
                    return false;
                },
            }
        } else {
            match s.parse::<i64>() {
                Ok(n) => self.line_tokens.push(Ok((Token::Int(n), s_pos))),
                Err(_) => {
                    self.line_tokens.push(Err(Error::Parser(s_pos, String::from("invalid number"))));
                    self.is_stopped = true;
                    return false;
                },
            }
        }
        true
    }
    
    fn read_token<T: Iterator<Item = (char, Pos)>>(&mut self, cs: &mut PushbackIter<T>) -> bool
    {
        self.skip_spaces(cs);
        match cs.next() {
            Some(('#' | '%', _)) => return false,
            Some(('(', pos)) => self.line_tokens.push(Ok((Token::LParen, pos))),
            Some((')', pos)) => self.line_tokens.push(Ok((Token::RParen, pos))),
            Some(('[', pos)) => self.line_tokens.push(Ok((Token::LBracket, pos))),
            Some((']', pos)) => self.line_tokens.push(Ok((Token::RBracket, pos))),
            Some(('{', pos)) => self.line_tokens.push(Ok((Token::LBrace, pos))),
            Some(('}', pos)) => self.line_tokens.push(Ok((Token::RBrace, pos))),
            Some(('*', pos)) => self.line_tokens.push(Ok((Token::Star, pos))),
            Some(('/', pos)) => self.line_tokens.push(Ok((Token::Slash, pos))),
            Some(('+', pos)) => self.line_tokens.push(Ok((Token::Plus, pos))),
            Some(('-', pos)) => self.line_tokens.push(Ok((Token::Minus, pos))),
            Some(('<', pos)) => {
                match cs.next() {
                    Some(('=', _)) => self.line_tokens.push(Ok((Token::LtEq, pos))),
                    Some((c2, pos2)) => {
                        self.line_tokens.push(Ok((Token::Lt, pos)));
                        cs.undo((c2, pos2));
                    },
                    None => self.line_tokens.push(Ok((Token::Lt, pos))),
                }
            },
            Some(('>', pos)) => {
                match cs.next() {
                    Some(('=', _)) => self.line_tokens.push(Ok((Token::GtEq, pos))),
                    Some((c2, pos2)) => {
                        self.line_tokens.push(Ok((Token::Gt, pos)));
                        cs.undo((c2, pos2));
                    },
                    None => self.line_tokens.push(Ok((Token::Gt, pos))),
                }
            },
            Some(('=', pos)) => {
                match cs.next() {
                    Some(('=', _)) => self.line_tokens.push(Ok((Token::EqEq, pos))),
                    Some((c2, pos2)) => {
                        self.line_tokens.push(Ok((Token::Eq, pos)));
                        cs.undo((c2, pos2));
                    },
                    None => self.line_tokens.push(Ok((Token::Eq, pos))),
                }
            },
            Some(('\'', pos)) => self.line_tokens.push(Ok((Token::Apos, pos))),
            Some(('.', pos)) => {
                match cs.next() {
                    Some(('[', _)) => self.line_tokens.push(Ok((Token::DotLBracket, pos))),
                    Some((']', _)) => self.line_tokens.push(Ok((Token::DotRBracket, pos))),
                    Some(('*', _)) => self.line_tokens.push(Ok((Token::DotStar, pos))),
                    Some(('/', _)) => self.line_tokens.push(Ok((Token::DotSlash, pos))),
                    Some((c2, pos2)) => {
                        self.line_tokens.push(Ok((Token::Dot, pos)));
                        cs.undo((c2, pos2));
                    },
                    None => self.line_tokens.push(Ok((Token::Dot, pos))),
                }
            },
            Some((':', pos)) => {
                match cs.next() {
                    Some((':', _)) => self.line_tokens.push(Ok((Token::ColonColon, pos))),
                    Some((c2, pos2)) => {
                        self.line_tokens.push(Ok((Token::Colon, pos)));
                        cs.undo((c2, pos2));
                    },
                    None => self.line_tokens.push(Ok((Token::Colon, pos))),
                }
            },
            Some((',', pos)) => self.line_tokens.push(Ok((Token::Comma, pos))),
            Some((';', pos)) => self.line_tokens.push(Ok((Token::Newline, pos))),
            Some((c, pos)) if c.is_ascii_digit() => {
                cs.undo((c, pos));
                return self.read_number_token(cs);
            },
            Some((_, pos)) => {
                self.line_tokens.push(Err(Error::Parser(pos, String::from("unexpected character"))));
                self.is_stopped = true;
                return false;
            },
            None => return false,
        }
        true
    }
}
