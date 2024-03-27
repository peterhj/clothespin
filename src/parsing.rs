use crate::re::{ReTrie, len_utf8};
use crate::str_util::{SafeStr, unescape_str};

use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::iter::{Peekable};
use std::mem::{replace};
use std::rc::{Rc};

#[derive(Clone, Copy)]
pub struct CharSpan {
  pub start: usize,
  pub end: usize,
}

impl Debug for CharSpan {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    if self.is_noloc() {
      write!(f, "CharSpan(.)")
    } else {
      write!(f, "CharSpan({}:{})", self.start, self.end)
    }
  }
}

impl Default for CharSpan {
  fn default() -> CharSpan {
    CharSpan{start: usize::max_value(), end: usize::max_value()}
  }
}

impl CharSpan {
  pub fn is_noloc(&self) -> bool {
    self.start == usize::max_value() && self.end == usize::max_value()
  }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Token {
  Space,
  NL,
  CR,
  IndentSpace(u32),
  CommentNL(SafeStr),
  Comma,
  Dot,
  DotDotDot,
  Semi,
  SemiSemi,
  Colon,
  ColonColon,
  Query,
  Bang,
  Dash,
  DashEq,
  Plus,
  //PlusPlus,
  PlusEq,
  Star,
  StarStar,
  StarEq,
  Slash,
  SlashEq,
  SlashSlash,
  Backslash,
  Percent,
  PercentEq,
  Amp,
  AmpEq,
  Bar,
  BarEq,
  Caret,
  Tilde,
  At,
  LShift,
  LShiftEq,
  RShift,
  RShiftEq,
  Equal,
  EqEq,
  Neq,
  Geq,
  Gt,
  Leq,
  Lt,
  XNot,
  XNeq,
  LDash,
  RDash,
  LEqual,
  REqual,
  LTilde,
  RTilde,
  LQueryDash,
  LQueryTilde,
  LBangDash,
  LBangTilde,
  LArrow,
  RArrow,
  REqArrow,
  LTildeArrow,
  RTildeArrow,
  LParen,
  RParen,
  LBrack,
  RBrack,
  LCurly,
  RCurly,
  True,
  False,
  None,
  And,
  As,
  Assert,
  Async,
  Await,
  Break,
  Case,
  Class,
  Continue,
  Def,
  Del,
  Elif,
  Else,
  Except,
  Finally,
  For,
  From,
  Global,
  Import,
  If,
  In,
  Is,
  Lambda,
  Match,
  Nonlocal,
  Not,
  Or,
  Pass,
  Raise,
  Return,
  Try,
  Type,
  Where,
  While,
  With,
  Yield,
  Place,
  Int(SafeStr),
  Lit(SafeStr),
  Ident(SafeStr),
  _Eof,
  _Bot,
}

impl Token {
  pub fn is_eof(&self) -> bool {
    match self {
      &Token::_Eof => true,
      _ => false
    }
  }

  pub fn is_space(&self) -> bool {
    match self {
      &Token::Space => true,
      _ => false
    }
  }
}

thread_local! {
  static TL_TRIE: Rc<ReTrie<Token>> = fresh_tokenizer_trie();
}

pub fn tl_tokenizer_trie() -> Rc<ReTrie<Token>> {
  TL_TRIE.with(|trie| trie.clone())
}

pub fn fresh_tokenizer_trie() -> Rc<ReTrie<Token>> {
  let mut tr = ReTrie::default();
  tr.push(r"[ \t]+", |_| Token::Space);
  tr.push(r"\#",    |_| Token::CommentNL(SafeStr::default()));
  tr.push(r"\n",    |_| Token::NL);
  tr.push(r"\r",    |_| Token::CR);
  tr.push(r"\\",    |_| Token::Backslash);
  tr.push(r",",     |_| Token::Comma);
  tr.push(r"\.",    |_| Token::Dot);
  //tr.push(r";;",    |_| Token::SemiSemi);
  tr.push(r";",     |_| Token::Semi);
  tr.push(r":\~",   |_| Token::LTilde);
  tr.push(r":\-",   |_| Token::LDash);
  tr.push(r"::",    |_| Token::ColonColon);
  tr.push(r":",     |_| Token::Colon);
  tr.push(r"\-=",   |_| Token::DashEq);
  tr.push(r"\-:",   |_| Token::RDash);
  tr.push(r"\->",   |_| Token::RArrow);
  tr.push(r"\-",    |_| Token::Dash);
  tr.push(r"\+=",   |_| Token::PlusEq);
  //tr.push(r"\+\+",  |_| Token::PlusPlus);
  tr.push(r"\+",    |_| Token::Plus);
  tr.push(r"\*=",   |_| Token::StarEq);
  tr.push(r"\*\*",  |_| Token::StarStar);
  tr.push(r"\*",    |_| Token::Star);
  tr.push(r"/=",    |_| Token::SlashEq);
  tr.push(r"//",    |_| Token::SlashSlash);
  tr.push(r"/",     |_| Token::Slash);
  tr.push(r"%=",    |_| Token::PercentEq);
  tr.push(r"%",     |_| Token::Percent);
  tr.push(r"=>",    |_| Token::REqArrow);
  tr.push(r"==",    |_| Token::EqEq);
  tr.push(r"=",     |_| Token::Equal);
  tr.push(r">>=",   |_| Token::RShiftEq);
  tr.push(r">>",    |_| Token::RShift);
  tr.push(r">=",    |_| Token::Geq);
  tr.push(r">",     |_| Token::Gt);
  tr.push(r"<\~",   |_| Token::LTildeArrow);
  tr.push(r"<\-",   |_| Token::LArrow);
  tr.push(r"<<=",   |_| Token::LShiftEq);
  tr.push(r"<<",    |_| Token::LShift);
  tr.push(r"<=",    |_| Token::Leq);
  tr.push(r"<",     |_| Token::Lt);
  tr.push(r"\~>",   |_| Token::RTildeArrow);
  tr.push(r"\~:",   |_| Token::RTilde);
  tr.push(r"\&",    |_| Token::Amp);
  tr.push(r"\|",    |_| Token::Bar);
  tr.push(r"\?\~",  |_| Token::LQueryTilde);
  tr.push(r"\?\-",  |_| Token::LQueryDash);
  tr.push(r"\?",    |_| Token::Query);
  tr.push(r"!\~",   |_| Token::LBangTilde);
  tr.push(r"!=",    |_| Token::Neq);
  tr.push(r"!\-",   |_| Token::LBangDash);
  tr.push(r"!",     |_| Token::Bang);
  tr.push(r"\^",    |_| Token::Caret);
  tr.push(r"\~",    |_| Token::Tilde);
  tr.push(r"@",     |_| Token::At);
  tr.push(r"\(",    |_| Token::LParen);
  tr.push(r"\)",    |_| Token::RParen);
  tr.push(r"\[",    |_| Token::LBrack);
  tr.push(r"\]",    |_| Token::RBrack);
  tr.push(r"\{",    |_| Token::LCurly);
  tr.push(r"\}",    |_| Token::RCurly);
  tr.push(r"True",  |_| Token::True);
  tr.push(r"False", |_| Token::False);
  tr.push(r"None",  |_| Token::None);
  tr.push(r"and",   |_| Token::And);
  tr.push(r"async", |_| Token::Async);
  tr.push(r"assert", |_| Token::Assert);
  tr.push(r"as",    |_| Token::As);
  tr.push(r"await", |_| Token::Await);
  tr.push(r"break", |_| Token::Break);
  tr.push(r"case",  |_| Token::Case);
  tr.push(r"class", |_| Token::Class);
  tr.push(r"continue", |_| Token::Continue);
  tr.push(r"def",   |_| Token::Def);
  tr.push(r"del",   |_| Token::Del);
  tr.push(r"elif",  |_| Token::Elif);
  tr.push(r"else",  |_| Token::Else);
  tr.push(r"except", |_| Token::Except);
  tr.push(r"finally", |_| Token::Finally);
  tr.push(r"from",  |_| Token::From);
  tr.push(r"for",   |_| Token::For);
  tr.push(r"global", |_| Token::Global);
  tr.push(r"import", |_| Token::Import);
  tr.push(r"if",    |_| Token::If);
  tr.push(r"in",    |_| Token::In);
  tr.push(r"is",    |_| Token::Is);
  tr.push(r"lambda", |_| Token::Lambda);
  tr.push(r"match", |_| Token::Match);
  tr.push(r"nonlocal", |_| Token::Nonlocal);
  tr.push(r"not",   |_| Token::Not);
  tr.push(r"or",    |_| Token::Or);
  tr.push(r"pass",  |_| Token::Pass);
  tr.push(r"raise", |_| Token::Raise);
  tr.push(r"return", |_| Token::Return);
  tr.push(r"try",   |_| Token::Try);
  tr.push(r"type",  |_| Token::Type);
  tr.push(r"where", |_| Token::Where);
  tr.push(r"while", |_| Token::While);
  tr.push(r"with",  |_| Token::With);
  tr.push(r"yield", |_| Token::Yield);
  tr.push(r"[0-9]+", |s| Token::Int(s.into()));
  tr.push(r"_",     |_| Token::Place);
  tr.push(r"[A-Za-z_][0-9A-Za-z_]*", |s| Token::Ident(s.into()));
  tr.into()
}

struct Buffer<S> {
  str_: S,
  off:  usize,
}

impl<S: AsRef<str>> Buffer<S> {
  pub fn as_str(&self) -> &str {
    self.str_.as_ref().get(self.off .. ).unwrap()
  }

  pub fn peek_char(&self) -> Option<char> {
    self.as_str().chars().next()
  }

  pub fn peek_char2(&self) -> Option<char> {
    let mut cs = self.as_str().chars();
    let _ = cs.next();
    cs.next()
  }

  pub fn peek_char3(&self) -> Option<char> {
    let mut cs = self.as_str().chars();
    let _ = cs.next();
    let _ = cs.next();
    cs.next()
  }

  pub fn position(&self) -> usize {
    self.off
  }

  pub fn advance(&mut self, o: usize) {
    self.off += o;
  }
}

pub struct Tokenizer<S> {
  trie: Rc<ReTrie<Token>>,
  bol:  bool,
  eof:  Option<CharSpan>,
  buf:  Buffer<S>,
}

impl<S> Tokenizer<S> {
  pub fn new(s: S) -> Tokenizer<S> {
    Tokenizer::new2(tl_tokenizer_trie(), s)
  }

  pub fn new2(trie: Rc<ReTrie<Token>>, s: S) -> Tokenizer<S> {
    Tokenizer{
      trie,
      bol:  true,
      eof:  None,
      buf:  Buffer{str_: s, off: 0},
    }
  }
}

impl<S: AsRef<str>> Iterator for Tokenizer<S> {
  type Item = (CharSpan, Token);

  fn next(&mut self) -> Option<(CharSpan, Token)> {
    if let Some(end_span) = self.eof {
      return Some((end_span, Token::_Eof));
    }
    let (mut span, mut tok) = {
      let c = self.buf.peek_char()?;
      if self.bol {
        if c == ' ' || c == '\t' {
          let mut indent = 0;
          let mut o = 0;
          for c in self.buf.as_str().chars() {
            match c {
              ' ' => {
                indent += 1;
                o += 1;
              }
              '\t' => {
                indent = (indent / 8 + 1) * 8;
                o += 1;
              }
              _ => break
            }
          }
          let start = self.buf.position();
          self.buf.advance(o);
          let end = self.buf.position();
          return Some((CharSpan{start, end}, Token::IndentSpace(indent)));
        }
        self.bol = false;
      }
      /*if c == 'f' || c == 'r' {
        let c2 = match self.buf.peek_char2() {
          None => {
            let start = self.buf.position();
            self.buf.advance(1);
            let end = self.buf.position();
            self.eof = Some(CharSpan{start: end, end});
            return Some((CharSpan{start, end}, Token::Ident(format!("{}", c).into()));
          }
          Some(c2) => c2
        };
        if c2 == '\'' || c2 == '\"' {
          // FIXME
        }
      }*/
      /*let mut doc = false;
      if c == '\"' {
        let c2 = match self.buf.peek_char2() {
          None => {
          }
          Some(c2) => c2
        };
        if c2 == '\"' {
          let c3 = match self.buf.peek_char3() {
            None => {
            }
            Some(c3) => c3
          };
          if c3 == '\"' {
            doc = true;
            // FIXME
          }
        }
      }*/
      if c == '\'' || (c == '\"' /*&& !doc*/) {
        let (s, o) = match unescape_str(self.buf.as_str(), c) {
          Err(_) => {
            //println!("DEBUG:  Tokenizer::next: eof 2");
            let end = self.buf.position();
            self.eof = Some(CharSpan{start: end, end});
            return Some((CharSpan{start: end, end}, Token::_Eof));
          }
          Ok(t) => t
        };
        //println!("DEBUG:  Tokenizer::next: lit (unescaped): {}", safe_ascii(s.as_bytes()));
        let start = self.buf.position();
        self.buf.advance(o);
        let end = self.buf.position();
        (CharSpan{start, end}, Token::Lit(s.into()))
      } else {
        let (next_tok, o) = match self.trie.match_(self.buf.as_str()) {
          None => {
            //println!("DEBUG:  Tokenizer::next: eof 3");
            let end = self.buf.position();
            self.eof = Some(CharSpan{start: end, end});
            return Some((CharSpan{start: end, end}, Token::_Eof));
          }
          Some((next_tok, next_off)) => {
            //println!("DEBUG:  Tokenizer::next: next tok={:?} off={}", next_tok, next_off);
            (next_tok, next_off)
          }
        };
        let start = self.buf.position();
        self.buf.advance(o);
        let end = self.buf.position();
        (CharSpan{start, end}, next_tok)
      }
    };
    match &mut tok {
      // FIXME: CR.
      &mut Token::NL => {
        self.bol = true;
      }
      &mut Token::CommentNL(ref mut s_) => {
        let mut s = String::new();
        loop {
          // NB: comment could end on eof.
          let c = match self.buf.peek_char() {
            None => {
              break;
            }
            Some(c) => c
          };
          if c == '\n' {
            self.bol = true;
            break;
          }
          let o = len_utf8(c as _);
          s.push(c);
          self.buf.advance(o);
        }
        span.end = self.buf.position();
        let _ = replace(s_, s.into());
      }
      _ => {}
    }
    //println!("DEBUG:  Tokenizer::next: tok={:?} off={}", tok, self.off);
    Some((span, tok))
  }
}
