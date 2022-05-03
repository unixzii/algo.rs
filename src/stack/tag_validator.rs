//! Tag Validator
//!
//! # Link
//!
//! [591. Tag Validator](https://leetcode.com/problems/tag-validator/)

#[derive(Debug)]
struct Cursor<'a> {
    content: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(content: &'a str) -> Self {
        content.len();
        Self { content, pos: 0 }
    }

    pub fn next(&mut self) -> Option<char> {
        let pos = self.pos;
        if pos >= self.content.len() {
            return None;
        }
        let cur = self.content.chars().nth(pos).unwrap();
        self.pos += 1;
        Some(cur)
    }

    pub fn peek(&self) -> Option<char> {
        let pos = self.pos;
        if pos >= self.content.len() {
            return None;
        }
        self.content.chars().nth(pos)
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        return self.pos >= self.content.len();
    }
}

impl<'a> Clone for Cursor<'a> {
    fn clone(&self) -> Self {
        Self {
            content: self.content,
            pos: self.pos,
        }
    }
}

#[derive(Debug)]
enum ParseResult<'a, T> {
    /// Succeeded parsing an AST element.
    Ok(T, Cursor<'a>),
    /// Failed to parse such AST element.
    Fail,
    /// Unrecoverable error.
    Err,
}

pub struct TagValidator;

impl TagValidator {
    pub fn is_valid(code: String) -> bool {
        let cursor = Cursor::new(&code);
        match Self::parse_node(cursor) {
            ParseResult::Ok(_, next_cur) => {
                if !next_cur.is_eof() {
                    // The input string must be exhausted.
                    return false;
                }
                return true;
            }
            _ => false,
        }
    }

    fn parse_node<'a>(cur: Cursor<'a>) -> ParseResult<'a, ()> {
        let mut cur = cur;
        let tag: String;
        match Self::parse_tag(cur, false) {
            ParseResult::Ok(_tag, next_cur) => {
                cur = next_cur;
                tag = _tag;
            }
            ParseResult::Fail => return ParseResult::Fail,
            ParseResult::Err => return ParseResult::Err,
        };

        loop {
            match Self::parse_tag(cur.clone(), true) {
                ParseResult::Ok(_tag, next_cur) => {
                    if _tag != tag {
                        return ParseResult::Err;
                    }
                    return ParseResult::Ok((), next_cur);
                }
                ParseResult::Err => {
                    return ParseResult::Err;
                }
                _ => {}
            };

            if let ParseResult::Ok(_, next_cur) = Self::parse_until(cur.clone(), "<") {
                cur = next_cur;
            } else {
                return ParseResult::Err;
            }

            match Self::parse_node(cur.clone()) {
                ParseResult::Ok(_, next_cur) => {
                    cur = next_cur;
                }
                ParseResult::Err => {
                    return ParseResult::Err;
                }
                _ => {}
            };

            match Self::parse_cdata(cur.clone()) {
                ParseResult::Ok(_, next_cur) => {
                    cur = next_cur;
                }
                ParseResult::Err => {
                    return ParseResult::Err;
                }
                _ => {}
            }
        }
    }

    fn parse_cdata<'a>(cur: Cursor<'a>) -> ParseResult<'a, String> {
        let mut cur = cur;

        // Match "<!" first, because token start with "<!" and not followed by "[CDATA[" is
        // not valid. Don't parse the whole token as an atomic operation, thus we can
        // recognize such error.
        if let ParseResult::Ok(_tag, next_cur) = Self::parse_token(cur, "<!") {
            cur = next_cur;
        } else {
            return ParseResult::Fail;
        }

        if let ParseResult::Ok(_tag, next_cur) = Self::parse_token(cur, "[CDATA[") {
            cur = next_cur;
        } else {
            // This is the error!
            return ParseResult::Err;
        }

        match Self::parse_until(cur, "]]>") {
            ParseResult::Ok(s, next_cur) => ParseResult::Ok(s, next_cur),
            ParseResult::Fail => ParseResult::Err,
            ParseResult::Err => ParseResult::Err,
        }
    }

    fn parse_tag<'a>(cur: Cursor<'a>, close: bool) -> ParseResult<'a, String> {
        let mut cur = cur;
        if let ParseResult::Ok(_, next_cur) = Self::parse_token(cur, "<") {
            cur = next_cur;
        } else {
            return ParseResult::Fail;
        }

        if close {
            if let ParseResult::Ok(_, next_cur) = Self::parse_token(cur, "/") {
                cur = next_cur;
            } else {
                return ParseResult::Fail;
            }
        } else {
            // Open tag cannot have "/" after "<".
            let cur_c = cur.peek();
            if cur_c == Some('/') || cur_c == Some('!') {
                return ParseResult::Fail;
            }
        }

        let mut tag = String::new();
        loop {
            match cur.next() {
                Some(c) => {
                    if c == '>' {
                        break;
                    }
                    tag.push(c);
                }
                None => return ParseResult::Err,
            }
        }

        if !Self::validate_tag_name(&tag) {
            return ParseResult::Err;
        }

        ParseResult::Ok(tag, cur)
    }

    fn parse_token<'a>(cur: Cursor<'a>, token: &str) -> ParseResult<'a, ()> {
        let mut cur = cur;
        for c in token.chars() {
            if let Some(cc) = cur.next() {
                if c != cc {
                    return ParseResult::Fail;
                }
            } else {
                return ParseResult::Fail;
            }
        }
        ParseResult::Ok((), cur)
    }

    fn parse_until<'a>(cur: Cursor<'a>, end_seq: &str) -> ParseResult<'a, String> {
        let mut cur = cur;
        let mut content = String::new();
        loop {
            let _cur = cur.clone();
            if let ParseResult::Ok(_, next_cur) = Self::parse_token(_cur, end_seq) {
                return ParseResult::Ok(content, cur);
            }
            if let Some(c) = cur.next() {
                content.push(c);
            } else {
                return ParseResult::Fail;
            }
        }
    }

    fn validate_tag_name(tag_name: &str) -> bool {
        let len = tag_name.len();
        if len == 0 || len > 9 {
            return false;
        }
        return tag_name.chars().all(|c| c.is_ascii_uppercase());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(
            super::TagValidator::is_valid(
                "<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_owned()
            ),
            true
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            super::TagValidator::is_valid(
                "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_owned()
            ),
            true
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            super::TagValidator::is_valid("<A>  <B> </A>   </B>".to_owned()),
            false
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            super::TagValidator::is_valid("<DIV>  div tag is not closed  <DIV>".to_owned()),
            false
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            super::TagValidator::is_valid("<DIV>  unmatched <  </DIV>".to_owned()),
            false
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            super::TagValidator::is_valid(
                "<DIV> closed tags with invalid tag name  <b>123</b> </DIV>".to_owned()
            ),
            false
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            super::TagValidator::is_valid("<A></A><B></B>".to_owned()),
            false
        );
    }

    #[test]
    fn test8() {
        assert_eq!(super::TagValidator::is_valid("<A<></A<>".to_owned()), false);
    }

    #[test]
    fn test9() {
        assert_eq!(
            super::TagValidator::is_valid("<A><A></A></ A>".to_owned()),
            false
        );
    }
}
