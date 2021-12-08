//! Iterator implementation for Japanese syllabograms
//!
//! This lifted and adapted from the unicode-segmentation crate:
//!     https://github.com/unicode-rs/unicode-segmentation
//!
//! TODO: Implement the rest of this with proper error handling.
const SMALL_KANA: &str = "ゃゅょャュョァィゥェォ";


pub struct Syllabograms<'a> {
    string: &'a str,
    cursor: SyllabogramCursor,
    cursor_back: SyllabogramCursor,
}

impl <'a> Syllabograms<'a> {
    #[inline]
    pub fn as_str(&self) -> &'a str {
        &self.string[self.cursor.cur_cursor()..self.cursor_back.cur_cursor()]
    }
}

impl<'a> Iterator for Syllabograms<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        let start = self.cursor.cur_cursor();
        if start == self.cursor_back.cur_cursor() {
            return None;
        }
        let next = self.cursor.next_boundary(self.string, 0).unwrap().unwrap();
        Some(&self.string[start..next])
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let slen = self.cursor_back.cur_cursor() - self.cursor.cur_cursor();
        (std::cmp::min(slen, 1), Some(slen))
    }
}

#[inline]
pub fn new_syllabograms(s: &str) -> Syllabograms {
    let len = s.len();
    Syllabograms {
        string: s,
        cursor: SyllabogramCursor::new(0, len),
        cursor_back: SyllabogramCursor::new(len, len),
    }
}

pub struct SyllabogramCursor {
    offset: usize,
    len: usize,
}

impl SyllabogramCursor {

    #[inline]
    pub fn new(offset: usize, len: usize) -> SyllabogramCursor {
        SyllabogramCursor{ offset, len }
    }

    #[inline]
    pub fn cur_cursor(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn next_boundary(&mut self, chunk: &str, chunk_start: usize) -> Result<Option<usize>, ()> {
        if self.offset == self.len {
            return Ok(None)
        }
        let mut iter = chunk[self.offset - chunk_start..].chars();
        let ch = iter.next().unwrap();
        self.offset += ch.len_utf8();
        if let Some(next_ch) = iter.next() {
            if SMALL_KANA.contains(next_ch) {
                self.offset += next_ch.len_utf8();
            }
        }
        Ok(Some(self.offset))
    }
}
