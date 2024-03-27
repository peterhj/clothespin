use clothespin::parsing::{Token, Tokenizer};

use std::fs::{File};
use std::io::{Read};

#[test]
fn test_tokenizer_data_1() {
  let mut f = File::open("test_data/parser-1.txt").unwrap();
  let mut s = String::new();
  f.read_to_string(&mut s).unwrap();
  let toks0 = vec![
    Token::Def,
    Token::Space,
    Token::Ident("hello".into()),
    Token::LParen,
    Token::Ident("x".into()),
    Token::RParen,
    Token::Colon,
    Token::NL,
    Token::IndentSpace(4),
    Token::Return,
    Token::Space,
    Token::Lit("world".into()),
    Token::NL,
  ];
  let toks = Tokenizer::new(&s);
  for ((_, tok), tok0) in toks.zip(toks0.into_iter()) {
    //println!("DEBUG:  test: {:?}", tok);
    assert_eq!(tok, tok0);
  }
}

#[test]
fn test_tokenizer_data_2() {
  let mut f = File::open("test_data/parser-2.txt").unwrap();
  let mut s = String::new();
  f.read_to_string(&mut s).unwrap();
  let toks0 = vec![
    Token::Def,
    Token::Space,
    Token::Ident("hello".into()),
    Token::LParen,
    Token::Ident("x".into()),
    Token::RParen,
    Token::Colon,
    Token::NL,
    Token::IndentSpace(4),
    Token::CommentNL(" nonsense".into()),
    Token::NL,
    Token::IndentSpace(4),
    Token::Where,
    Token::Space,
    Token::Ident("y".into()),
    Token::Space,
    Token::LDash,
    Token::Space,
    Token::Ident("x".into()),
    Token::NL,
    Token::IndentSpace(4),
    Token::Return,
    Token::Space,
    Token::Lit("world".into()),
    Token::NL,
  ];
  let toks = Tokenizer::new(&s);
  for ((_, tok), tok0) in toks.zip(toks0.into_iter()) {
    //println!("DEBUG:  test: {:?}", tok);
    assert_eq!(tok, tok0);
  }
}
