use clothespin::parsing::{Token, Tokenizer};
use time::{get_time};

use std::fs::{File};
use std::io::{Read};

fn bench_tokenizer() {
  let mut f = File::open("test_data/gravity.py").unwrap();
  let mut s = String::new();
  f.read_to_string(&mut s).unwrap();
  drop(f);
  let n = 1000;
  let t0 = get_time();
  for _ in 0 .. n {
    let toks = Tokenizer::new(&s);
    for _ in toks {
      //println!("DEBUG:  test: {:?}", tok);
    }
  }
  let t1 = get_time();
  let dt = (t1-t0).to_f64();
  let r = (s.len() * n) as f64 / dt;
  println!("DEBUG: bench: tokenizer: len = {}, elapsed = {:.06} s, n = {}, rate = {}",
      s.len(), dt, n, r as i64);
}

pub fn main() {
  bench_tokenizer();
}
