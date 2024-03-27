pub use crate::re::{len_utf8};

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub fn safe_ascii(s: &[u8]) -> String {
  let mut buf = String::new();
  for &x in s.iter() {
    if x <= 0x20 {
      buf.push(' ');
    } else if x < 0x7f {
      buf.push(x.try_into().unwrap());
    } else {
      buf.push('?');
    }
  }
  buf.into()
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SafeStr {
  raw:  String,
}

impl From<String> for SafeStr {
  fn from(s: String) -> SafeStr {
    SafeStr{raw: s.into()}
  }
}

impl<'a> From<&'a str> for SafeStr {
  fn from(s: &'a str) -> SafeStr {
    SafeStr{raw: s.into()}
  }
}

impl SafeStr {
  pub fn len(&self) -> usize {
    self.raw.len()
  }

  pub fn is_empty(&self) -> bool {
    self.raw.is_empty()
  }

  pub fn as_raw_str(&self) -> &str {
    self.raw.as_str()
  }

  pub fn set_raw_str<S: Into<String>>(&mut self, s: S) {
    self.raw = s.into();
  }
}

impl Debug for SafeStr {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{:?}", safe_ascii(self.raw.as_bytes()))
  }
}

impl Display for SafeStr {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", safe_ascii(self.raw.as_bytes()))
  }
}

// NB: json-like string (un-)escape via rustc_serialize.

fn decode_hex_escape<I: Iterator<Item=char>>(src: &mut I, off: &mut usize) -> Result<u16, ()> {
  let mut o = 0;
  let mut i = 0;
  let mut n = 0;
  while i < 4 {
    let c = match src.next() {
      Some(c) => c,
      None => return Err(())
    };
    o += len_utf8(c as _);
    n = match c {
      c @ '0' ..= '9' => n * 16 + ((c as u16) - ('0' as u16)),
      c @ 'a' ..= 'f' => n * 16 + (10 + (c as u16) - ('a' as u16)),
      c @ 'A' ..= 'F' => n * 16 + (10 + (c as u16) - ('A' as u16)),
      _ => return Err(())
    };
    i += 1;
  }
  *off += o;
  Ok(n)
}

pub fn unescape_str(src: &str, delim: char) -> Result<(String, usize), ()> {
  let mut src = src.chars();
  let mut off = 0;

  let c = match src.next() {
    None => {
      return Err(());
    }
    Some(c) => c
  };
  if c != delim {
    return Err(());
  }
  off += len_utf8(c as _);

  let mut res = String::new();
  let mut escape = false;

  loop {
    let c = match src.next() {
      None => {
        return Err(());
      }
      Some(c) => c
    };
    off += len_utf8(c as _);

    if escape {
      match c {
        '"' => res.push('"'),
        '\\' => res.push('\\'),
        '/' => res.push('/'),
        'b' => res.push('\x08'),
        'f' => res.push('\x0c'),
        'n' => res.push('\n'),
        'r' => res.push('\r'),
        't' => res.push('\t'),
        'u' => match decode_hex_escape(&mut src, &mut off)? {
          0xDC00 ..= 0xDFFF => {
            //return self.error(LoneLeadingSurrogateInHexEscape)
            return Err(());
          }

          // Non-BMP characters are encoded as a sequence of
          // two hex escapes, representing UTF-16 surrogates.
          n1 @ 0xD800 ..= 0xDBFF => {
            match (src.next(), src.next()) {
              (Some('\\'), Some('u')) => (),
              //_ => return self.error(UnexpectedEndOfHexEscape),
              _ => return Err(())
            }
            off += 2;

            let n2 = decode_hex_escape(&mut src, &mut off)?;
            if n2 < 0xDC00 || n2 > 0xDFFF {
              //return self.error(LoneLeadingSurrogateInHexEscape)
              return Err(());
            }
            let c = (((n1 - 0xD800) as u32) << 10 |
                 (n2 - 0xDC00) as u32) + 0x1_0000;
            res.push(char::from_u32(c).unwrap());
          }

          n => match char::from_u32(n as u32) {
            Some(c) => res.push(c),
            //None => return self.error(InvalidUnicodeCodePoint),
            None => return Err(())
          },
        },
        //_ => return self.error(InvalidEscape),
        _ => return Err(())
      }
      escape = false;
    } else if c == '\\' {
      escape = true;
    } else {
      if c == delim {
        return Ok((res, off));
      } else if c <= '\u{1F}' {
        //return self.error(ControlCharacterInString),
        return Err(());
      } else {
        res.push(c);
      }
    }
  }
}
