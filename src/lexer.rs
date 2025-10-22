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
use crate::doc::*;
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
    Ques,
    Star,
    Slash,
    Plus,
    Minus,
    DotStar,
    DotSlash,
    DotPlus,
    DotMinus,
    Lt,
    GtEq,
    Gt,
    LtEq,
    Eq,
    EqEq,
    ExEq,
    Apos,
    Dot,
    Colon,
    ColonColon,
    Comma,
    Newline,
    And,
    Break,
    By,
    Continue,
    Else,
    End,
    False,
    Fill,
    For,
    Function,
    If,
    In,
    Inf,
    Module,
    Nan,
    None,
    Not,
    Or,
    Quit,
    Return,
    Root,
    To,
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
    is_stopped: bool,
    keywords: HashMap<String, Token>,
    doc: Option<Option<String>>,
}

impl<'a> Lexer<'a>
{
    pub fn new_with_doc_flag(path: Arc<String>, reader: &'a mut dyn BufRead, is_doc: bool) -> Self
    {
        let mut keywords: HashMap<String, Token> = HashMap::new();
        keywords.insert(String::from("and"), Token::And);
        keywords.insert(String::from("break"), Token::Break);
        keywords.insert(String::from("by"), Token::By);
        keywords.insert(String::from("continue"), Token::Continue);
        keywords.insert(String::from("else"), Token::Else);
        keywords.insert(String::from("end"), Token::End);
        keywords.insert(String::from("false"), Token::False);
        keywords.insert(String::from("fill"), Token::Fill);
        keywords.insert(String::from("for"), Token::For);
        keywords.insert(String::from("function"), Token::Function);
        keywords.insert(String::from("if"), Token::If);
        keywords.insert(String::from("in"), Token::In);
        keywords.insert(String::from("inf"), Token::Inf);
        keywords.insert(String::from("module"), Token::Module);
        keywords.insert(String::from("nan"), Token::Nan);
        keywords.insert(String::from("none"), Token::None);
        keywords.insert(String::from("not"), Token::Not);
        keywords.insert(String::from("or"), Token::Or);
        keywords.insert(String::from("quit"), Token::Quit);
        keywords.insert(String::from("return"), Token::Return);
        keywords.insert(String::from("root"), Token::Root);
        keywords.insert(String::from("to"), Token::To);
        keywords.insert(String::from("true"), Token::True);
        keywords.insert(String::from("while"), Token::While);
        let doc = if is_doc {
            Some(None)
        } else {
            None
        };
        Lexer {
            path,
            line: 1,
            eol_column: 0,
            reader,
            line_tokens: Vec::new(),
            is_stopped: false,
            keywords,
            doc,
        }
    }

    pub fn new(path: Arc<String>, reader: &'a mut dyn BufRead) -> Self
    { Self::new_with_doc_flag(path, reader, true) }
    
    pub fn path(&self) -> &Arc<String>
    { &self.path }
    
    fn read_line_tokens(&mut self)
    {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => self.is_stopped = true,
            Ok(_) => {
                let path = self.path.clone();
                let line_count = self.line;
                let line_without_crnl = str_without_crnl(line.as_str());
                let mut cs = line_without_crnl.chars().enumerate().map(|p| (p.1, Pos::new(path.clone(), line_count, p.0 + 1)));
                let cs2: &mut dyn Iterator<Item = (char, Pos)> = &mut cs;
                let mut cs3 = PushbackIter::new(cs2);
                self.line_tokens.clear();
                self.eol_column = line_without_crnl.chars().count() + 1;
                while self.read_token(&mut cs3) {}
                self.line_tokens.push(Ok((Token::Newline, Pos::new(path, line_count, self.eol_column))));
                self.line_tokens.reverse();
                self.line += 1;
            },
            Err(err) => {
                self.line_tokens = vec![Err(Error::ParserIo(self.path.clone(), err))];
                self.is_stopped = true;
            },
        }
    }
    
    fn skip_spaces(&self, cs: &mut PushbackIter<&mut dyn Iterator<Item = (char, Pos)>>)
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
    
    fn read_one_or_more_digits(&mut self, cs: &mut PushbackIter<&mut dyn Iterator<Item = (char, Pos)>>, s: &mut String, s_pos: Option<&mut Pos>) -> bool
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
                self.line_tokens.push(Err(Error::Parser(Pos::new(self.path.clone(), self.line, self.eol_column), String::from("no decimal digits"))));
                self.is_stopped = true;
                false
            },
        }
    }

    fn read_one_or_more_hexdigits(&mut self, cs: &mut PushbackIter<&mut dyn Iterator<Item = (char, Pos)>>, s: &mut String, s_pos: Option<&mut Pos>) -> bool
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
                self.line_tokens.push(Err(Error::Parser(Pos::new(self.path.clone(), self.line, self.eol_column), String::from("no hexadecimal digits"))));
                self.is_stopped = true;
                false
            },
        }
    }
    
    fn read_number_token(&mut self, cs: &mut PushbackIter<&mut dyn Iterator<Item = (char, Pos)>>) -> bool
    {
        let mut s = String::new(); 
        let mut s_pos = Pos::new(self.path.clone(), self.line, 1);
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

    fn read_string_token(&mut self, cs: &mut PushbackIter<&mut dyn Iterator<Item = (char, Pos)>>) -> bool
    {
        match cs.next() {
            Some(('"', pos)) => {
                let mut s = String::new();
                loop {
                    match cs.next() {
                        Some(('"', _)) => break,
                        Some(('\\', pos2)) => {
                            match cs.next() {
                                Some(('a', _)) => s.push('\x07'),
                                Some(('b', _)) => s.push('\x08'),
                                Some(('t', _)) => s.push('\t'),
                                Some(('n', _)) => s.push('\n'),
                                Some(('v', _)) => s.push('\x0b'),
                                Some(('f', _)) => s.push('\x0c'),
                                Some(('r', _)) => s.push('\r'),
                                Some((c3 @ ('U'| 'u'), _)) => {
                                    let mut t = String::new();
                                    let n = if c3 == 'U' { 6 } else { 4 };
                                    for _ in 0..n {
                                        match cs.next() {
                                            Some((c4, _)) if c4.is_ascii_hexdigit() => t.push(c4),
                                            _ => {
                                                self.line_tokens.push(Err(Error::Parser(pos2, String::from("invalid unicode escape"))));
                                                self.is_stopped = true;
                                                return false;
                                            }
                                        }
                                    }
                                    match u32::from_str_radix(t.as_str(), 16) {
                                        Ok(code) => {
                                            match char::from_u32(code) {
                                                Some(esc_c) => s.push(esc_c),
                                                None => {
                                                    self.line_tokens.push(Err(Error::Parser(pos2, String::from("invalid unicode escape"))));
                                                    self.is_stopped = true;
                                                    return false;
                                                },
                                            }
                                        },
                                        Err(_) => {
                                            self.line_tokens.push(Err(Error::Parser(pos2, String::from("invalid unicode escape"))));
                                            self.is_stopped = true;
                                            return false;
                                        },
                                    }
                                },
                                Some((c3 @ ('0'..='7'), _)) => {
                                    let mut t = String::new();
                                    t.push(c3);
                                    for _ in 0..2 {
                                        match cs.next() {
                                            Some((c4 @ ('0'..='7'), _)) => t.push(c4),
                                            Some((c4, pos4)) => {
                                                cs.undo((c4, pos4));
                                                break;
                                            },
                                            None => {
                                                self.line_tokens.push(Err(Error::Parser(pos2, String::from("unclosed string"))));
                                                self.is_stopped = true;
                                                return false;
                                            }
                                        }
                                    }
                                    match u32::from_str_radix(t.as_str(), 8) {
                                        Ok(code) => {
                                            match char::from_u32(code) {
                                                Some(esc_c) => s.push(esc_c),
                                                None => {
                                                    self.line_tokens.push(Err(Error::Parser(pos2, String::from("invalid octal escape"))));
                                                    self.is_stopped = true;
                                                    return false;
                                                },
                                            }
                                        },
                                        Err(_) => {
                                            self.line_tokens.push(Err(Error::Parser(pos2, String::from("invalid octal escape"))));
                                            self.is_stopped = true;
                                            return false;
                                        },
                                    }
                                },
                                Some((c3, _)) => s.push(c3),
                                None => {
                                    self.line_tokens.push(Err(Error::Parser(pos, String::from("unclosed string"))));
                                    self.is_stopped = true;
                                    return false;
                                },
                            }
                        },
                        Some((c2, _)) => s.push(c2),
                        None => {
                            self.line_tokens.push(Err(Error::Parser(pos, String::from("unclosed string"))));
                            self.is_stopped = true;
                            return false;
                        },
                    }
                }
                self.line_tokens.push(Ok((Token::String(s), pos)));
                true
            },
            Some((_, pos)) => {
                self.line_tokens.push(Err(Error::Parser(pos, String::from("invalid string"))));
                self.is_stopped = true;
                false
            },
            None => {
                self.line_tokens.push(Err(Error::Parser(Pos::new(self.path.clone(), self.line, self.eol_column), String::from("invalid string"))));
                self.is_stopped = true;
                false
            },
        }
    }
    
    fn read_ident_chars(&mut self, cs: &mut PushbackIter<&mut dyn Iterator<Item = (char, Pos)>>, s: &mut String)
    {
        loop {
            match cs.next() {
                Some((c, _)) if c.is_alphanumeric() || c == '_' => s.push(c),
                Some((c, pos)) => {
                    cs.undo((c, pos));
                    break;
                },
                None => break,
            }
        }
    }
    
    fn read_keyword_or_ident_token(&mut self, cs: &mut PushbackIter<&mut dyn Iterator<Item = (char, Pos)>>) -> bool
    {
        match cs.next() {
            Some((c, pos)) if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                s.push(c);
                self.read_ident_chars(cs, &mut s);
                match self.keywords.get(&s) {
                    Some(keyword) => self.line_tokens.push(Ok((keyword.clone(), pos))),
                    None => self.line_tokens.push(Ok((Token::Ident(s), pos))),
                }
                true
            },
            Some((_, pos)) => {
                self.line_tokens.push(Err(Error::Parser(pos, String::from("invalid keyword or identifier"))));
                self.is_stopped = true;
                false
            },
            None => {
                self.line_tokens.push(Err(Error::Parser(Pos::new(self.path.clone(), self.line, self.eol_column), String::from("invalid keyword or identifier"))));
                self.is_stopped = true;
                false
            },
        }
    }
    
    fn read_token(&mut self, cs: &mut PushbackIter<&mut dyn Iterator<Item = (char, Pos)>>) -> bool
    {
        self.skip_spaces(cs);
        match cs.next() {
            Some(('#' | '%', _)) => {
                match cs.next() {
                    Some(('#' | '%', _)) => {
                        match &mut self.doc {
                            Some(doc) => {
                                match cs.next() {
                                    Some((c3, _)) if c3.is_whitespace() => (),
                                    Some((c3, pos3)) => cs.undo((c3, pos3)),
                                    None => (),
                                }
                                let mut doc_line: String = cs.map(|p| p.0).collect();
                                doc_line.push('\n');
                                match doc {
                                    Some(doc) => doc.push_str(doc_line.as_str()),
                                    None => *doc = Some(doc_line),
                                }
                            },
                            None => (),
                        }
                    },
                    _ => (),
                }
                return false
            },
            Some(('(', pos)) => self.line_tokens.push(Ok((Token::LParen, pos))),
            Some((')', pos)) => self.line_tokens.push(Ok((Token::RParen, pos))),
            Some(('[', pos)) => self.line_tokens.push(Ok((Token::LBracket, pos))),
            Some((']', pos)) => self.line_tokens.push(Ok((Token::RBracket, pos))),
            Some(('{', pos)) => self.line_tokens.push(Ok((Token::LBrace, pos))),
            Some(('}', pos)) => self.line_tokens.push(Ok((Token::RBrace, pos))),
            Some(('?', pos)) => self.line_tokens.push(Ok((Token::Ques, pos))),
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
            Some(('!', pos)) => {
                match cs.next() {
                    Some(('=', _)) => self.line_tokens.push(Ok((Token::ExEq, pos))),
                    _ => {
                        self.line_tokens.push(Err(Error::Parser(pos, String::from("unexpected character"))));
                        self.is_stopped = true;
                        return false;
                    },
                }
            },
            Some(('\'', pos)) => self.line_tokens.push(Ok((Token::Apos, pos))),
            Some(('.', pos)) => {
                match cs.next() {
                    Some(('[', _)) => self.line_tokens.push(Ok((Token::DotLBracket, pos))),
                    Some((']', _)) => self.line_tokens.push(Ok((Token::DotRBracket, pos))),
                    Some(('*', _)) => self.line_tokens.push(Ok((Token::DotStar, pos))),
                    Some(('/', _)) => self.line_tokens.push(Ok((Token::DotSlash, pos))),
                    Some(('+', _)) => self.line_tokens.push(Ok((Token::DotPlus, pos))),
                    Some(('-', _)) => self.line_tokens.push(Ok((Token::DotMinus, pos))),
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
            Some((c @ '"', pos)) => {
                cs.undo((c, pos));
                return self.read_string_token(cs);
            },
            Some((c, pos)) if c.is_ascii_digit() => {
                cs.undo((c, pos));
                return self.read_number_token(cs);
            },
            Some((c, pos)) if c.is_alphabetic() || c == '_' => {
                cs.undo((c, pos));
                return self.read_keyword_or_ident_token(cs);
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

impl<'a> Iterator for Lexer<'a>
{
    type Item = Result<(Token, Pos)>;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.line_tokens.is_empty() {
            if !self.is_stopped {
                self.read_line_tokens();
            } else {
                return None;
            }
        }
        match self.line_tokens.pop() {
            Some(Ok(token)) => Some(Ok(token)),
            Some(Err(err)) => {
                self.line_tokens.clear();
                Some(Err(err))
            },
            None => None,
        }
    }
}

impl<'a> DocIterator for Lexer<'a>
{
    fn take_doc(&mut self) -> Option<String>
    {
        match &mut self.doc {
            Some(doc) => doc.take(),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
