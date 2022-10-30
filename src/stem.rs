//! Module containing function used to stem strings.

use std::str;
use crate::error::RnltkError;

/// `word` is a vector of bytes holding a word to be stemmed.
/// The letters are in word[0], word[1] ... ending at word[z->`bytes_length`]. `bytes_length` is readjusted
/// downwards as the stemming progresses. Zero termination is not in fact used
/// in the algorithm.
///
/// Note that only lower case sequences are stemmed. get(...) automatically
/// lowercases the string before processing.
///
///
/// Typical usage is:
/// 
///```
/// use rnltk::stem;
/// # use rnltk::error::RnltkError;
/// 
/// # fn main() -> Result<(), RnltkError> {
/// let word = "pencils";
/// let stemmed_word = stem::get(word)?;
/// assert_eq!(stemmed_word, "pencil".to_string());
/// #
/// #   Ok(())
/// # }
///```
struct Stemmer {
    bytes: Vec<u8>,
    bytes_length: usize,
    offset: usize,
}

impl Stemmer {
    fn new(word: &str) -> Result<Stemmer, RnltkError> {
        if !word.is_ascii() {
            Err(RnltkError::StemNonAscii)
        } else {
            let bytes = word.to_ascii_lowercase().into_bytes();
            let bytes_length = bytes.len();
            Ok(Stemmer { 
                bytes, 
                bytes_length, 
                offset: 0 
            })
        }
    }

    /// stem.is_consonant(index) is true <=> stem[index] is a consonant
    #[inline]
    fn is_consonant(&self, index: usize) -> bool {
        match self.bytes[index] {
            b'a' | b'e' | b'i' | b'o' | b'u' => false,
            b'y' => {
                if index == 0 {
                    true
                } else {
                    !self.is_consonant(index - 1)
                }
            }
            _ => true,
        }
    }

    /// stem.measure() measures the number of consonant sequences in [0, offset).
    /// if c is a consonant sequence and v a vowel sequence, and <..> indicates
    /// arbitrary presence,
    ///
    /// ~~~notrust
    ///    <c><v>       gives 0
    ///    <c>vc<v>     gives 1
    ///    <c>vcvc<v>   gives 2
    ///    <c>vcvcvc<v> gives 3
    ///    ....
    /// ~~~
    fn measure(&self) -> usize {
        let mut n = 0;
        let mut index = 0;
        let offset = self.offset;
        loop {
            if index >= offset {
                return n;
            }
            if !self.is_consonant(index) {
                break;
            }
            index += 1;
        }
        index += 1;
        loop {
            loop {
                if index >= offset {
                    return n;
                }
                if self.is_consonant(index) {
                    break;
                }
                index += 1;
            }
            index += 1;
            n += 1;
            loop {
                if index >= offset {
                    return n;
                }
                if !self.is_consonant(index) {
                    break;
                }
                index += 1;
            }
            index += 1;
        }
    }

    /// stem.has_vowel() is TRUE <=> [0, offset-1) contains a vowel
    fn has_vowel(&self) -> bool {
        for index in 0..self.offset {
            if !self.is_consonant(index) {
                return true;
            }
        }
        false
    }

    /// stem.double_consonant(index) is TRUE <=> index,(index-1) contain a double consonant.
    #[inline]
    fn double_consonant(&self, index: usize) -> bool {
        if index < 1 || self.bytes[index] != self.bytes[index - 1] {
            false
        } else {
            self.is_consonant(index)
        }
    }

    /// cvc(z, index) is TRUE <=> index-2,index-1,index has the form consonant - vowel - consonant
    /// and also if the second c is not w,x or y. this is used when trying to
    /// restore an e at the end of a short word. e.g.
    ///
    /// ~~~notrust
    ///    cav(e), lov(e), hop(e), crim(e), but
    ///    snow, box, tray.
    /// ~~~
    fn cvc(&self, index: usize) -> bool {
        if index < 2 || !self.is_consonant(index) || self.is_consonant(index - 1) || !self.is_consonant(index - 2) {
            false
        } else {
            !matches!(self.bytes[index], b'w' | b'x' | b'y')
        }
    }

    /// stem.ends(s) is true <=> [0, bytes_length) ends with the string s.
    fn ends(&mut self, _s: &str) -> bool {
        let s = _s.as_bytes();
        let len = s.len();
        if len > self.bytes_length {
            false
        } else { 
            &self.bytes[self.bytes_length - len..self.bytes_length] == s 
        }
    }

    fn update_offset(&mut self, _s: &str) {
        let s = _s.as_bytes();
        let len = s.len();
        self.offset = self.bytes_length - len;
    }

    /// stem.setto(s) sets [offset, bytes_length) to the characters in the string s,
    /// readjusting bytes_length.
    fn set_to(&mut self, s: &str) {
        let s = s.as_bytes();
        let len = s.len();
        for (index, byte) in s.iter().enumerate() {
            self.bytes[self.offset + index] = *byte;
        }
        self.bytes_length = self.offset + len;
    }

    /// self.replace(s) is used further down.
    #[inline]
    fn replace(&mut self, s: &str) {
        if self.measure() > 0 {
            self.set_to(s);
        }
    }

    /// stem.step1ab() gets rid of plurals and -ed or -ing. e.g.
    ///
    /// ~~~~notrust
    ///     caresses  ->  caress
    ///     ponies    ->  poni
    ///     ties      ->  ti
    ///     caress    ->  caress
    ///     cats      ->  cat
    ///
    ///     feed      ->  feed
    ///     agreed    ->  agree
    ///     disabled  ->  disable
    ///
    ///     matting   ->  mat
    ///     mating    ->  mate
    ///     meeting   ->  meet
    ///     milling   ->  mill
    ///     messing   ->  mess
    ///
    ///     meetings  ->  meet
    /// ~~~~
    fn step1ab(&mut self) {
        if self.bytes[self.bytes_length - 1] == b's' {
            if self.ends("sses") {
                self.update_offset("sses");
                self.bytes_length -= 2;
            } else if self.ends("ies") {
                self.update_offset("ies");
                self.set_to("i");
            } else if self.bytes[self.bytes_length - 2] != b's' {
                self.bytes_length -= 1;
            }
        }
        if self.ends("eed") {
            self.update_offset("eed");
            if self.measure() > 0 {
                self.bytes_length -= 1
            }
        } else if self.ends("ed") || self.ends("ing") {
            if self.ends("ed") {
                self.update_offset("ed");
            } else if self.ends("ing") {
                self.update_offset("ing");
            }
            if self.has_vowel() {
                self.bytes_length = self.offset;
                if self.ends("at") {
                    self.update_offset("at");
                    self.set_to("ate");
                } else if self.ends("bl") {
                    self.update_offset("bl");
                    self.set_to("ble");
                } else if self.ends("iz") {
                    self.update_offset("iz");
                    self.set_to("ize");
                } else if self.double_consonant(self.bytes_length - 1) {
                    self.bytes_length -= 1;
                    match self.bytes[self.bytes_length - 1] {
                        b'l' | b's' | b'z' => self.bytes_length += 1,
                        _ => (),
                    }
                } else if self.measure() == 1 && self.cvc(self.bytes_length - 1) {
                    self.set_to("e");
                }
            }
        }
    }

    /// stem.step1c() turns terminal y to i when there is another vowel in the stem.
    fn step1c(&mut self) {
        if self.ends("y") {
            self.update_offset("y");
            if self.has_vowel() {
                self.bytes[self.bytes_length - 1] = b'i';
            }
        }
    }

    /// stem.step2() maps double suffices to single ones. so -ization ( = -ize
    /// plus -ation) maps to -ize etc. note that the string before the suffix
    /// must give m(z) > 0.
    fn step2(&mut self) {
        match self.bytes[self.bytes_length - 2] {
            b'a' => {
                if self.ends("ational") {
                    self.update_offset("ational");
                    self.replace("ate");
                } else if self.ends("tional") {
                    self.update_offset("tional");
                    self.replace("tion");
                }
            }
            b'c' => {
                if self.ends("enci") {
                    self.update_offset("enci");
                    self.replace("ence");
                } else if self.ends("anci") {
                    self.update_offset("anci");
                    self.replace("ance");
                }
            }
            b'e' => {
                if self.ends("izer") {
                    self.update_offset("izer");
                    self.replace("ize");
                }
            }
            b'l' => {
                if self.ends("bli") {
                    self.update_offset("bli");
                    self.replace("ble");
                } /*-DEPARTURE-*/

                /* To match the published algorithm, replace this line with
                'l' => {
                    if self.ends("abli") { self.replace("able"); return } */
                else if self.ends("alli") {
                    self.update_offset("alli");
                    self.replace("al");
                } else if self.ends("entli") {
                    self.update_offset("entli");
                    self.replace("ent");
                } else if self.ends("eli") {
                    self.update_offset("eli");
                    self.replace("e");
                } else if self.ends("ousli") {
                    self.update_offset("ousli");
                    self.replace("ous");
                }
            }
            b'o' => {
                if self.ends("ization") {
                    self.update_offset("ization");
                    self.replace("ize");
                } else if self.ends("ation") {
                    self.update_offset("ation");
                    self.replace("ate");
                } else if self.ends("ator") {
                    self.update_offset("ator");
                    self.replace("ate");
                }
            }
            b's' => {
                if self.ends("alism") {
                    self.update_offset("alism");
                    self.replace("al");
                } else if self.ends("iveness") {
                    self.update_offset("iveness");
                    self.replace("ive");
                } else if self.ends("fulness") {
                    self.update_offset("fulness");
                    self.replace("ful");
                } else if self.ends("ousness") {
                    self.update_offset("ousness");
                    self.replace("ous");
                }
            }
            b't' => {
                if self.ends("aliti") {
                    self.update_offset("aliti");
                    self.replace("al");
                } else if self.ends("iviti") {
                    self.update_offset("iviti");
                    self.replace("ive");
                } else if self.ends("biliti") {
                    self.update_offset("biliti");
                    self.replace("ble");
                }
            }
            b'g' => {
                if self.ends("logi") {
                    self.update_offset("logi");
                    self.replace("log");
                }
            } /*-DEPARTURE-*/
            /* To match the published algorithm, delete this line */
            _ => (),
        }
    }

    /// stem.step3() deals with -ic-, -full, -ness etc. similar strategy to step2.
    fn step3(&mut self) {
        match self.bytes[self.bytes_length - 1] {
            b'e' => {
                if self.ends("icate") {
                    self.update_offset("icate");
                    self.replace("ic");
                } else if self.ends("ative") {
                    self.update_offset("ative");
                    self.replace("");
                } else if self.ends("alize") {
                    self.update_offset("alize");
                    self.replace("al");
                }
            }
            b'i' => {
                if self.ends("iciti") {
                    self.update_offset("iciti");
                    self.replace("ic");
                }
            }
            b'l' => {
                if self.ends("ical") {
                    self.update_offset("ical");
                    self.replace("ic");
                } else if self.ends("ful") {
                    self.update_offset("ful");
                    self.replace("");
                }
            }
            b's' => {
                if self.ends("ness") {
                    self.update_offset("ness");
                    self.replace("");
                }
            }
            _ => (),
        }
    }

    /// stem.step4() takes off -ant, -ence etc., in context <c>vcvc<v>.
    fn step4(&mut self) {
        let mut byte_was_matched = false;
        match self.bytes[self.bytes_length - 2] {
            b'a' => {
                if self.ends("al") {
                    self.update_offset("al");
                    byte_was_matched = true;
                }
            }
            b'c' => {
                if self.ends("ance") {
                    self.update_offset("ance");
                    byte_was_matched = true;
                } else if self.ends("ence") {
                    self.update_offset("ence");
                    byte_was_matched = true;
                }
            }
            b'e' => {
                if self.ends("er") {
                    self.update_offset("er");
                    byte_was_matched = true;
                }
            }
            b'i' => {
                if self.ends("ic") {
                    self.update_offset("ic");
                    byte_was_matched = true;
                }
            }
            b'l' => {
                if self.ends("able") {
                    self.update_offset("able");
                    byte_was_matched = true;
                } else if self.ends("ible") {
                    self.update_offset("ible");
                    byte_was_matched = true;
                }
            }
            b'n' => {
                if self.ends("ant") {
                    self.update_offset("ant");
                    byte_was_matched = true;
                } else if self.ends("ement") {
                    self.update_offset("ement");
                    byte_was_matched = true;
                } else if self.ends("ment") {
                    self.update_offset("ment");
                    byte_was_matched = true;
                } else if self.ends("ent") {
                    self.update_offset("ent");
                    byte_was_matched = true;
                }
            }
            b'o' => {
                if self.ends("ion") {
                    self.update_offset("ion");
                    if self.offset > 0 && (self.bytes[self.offset - 1] == b's' || self.bytes[self.offset - 1] == b't') {
                        byte_was_matched = true;
                    }
                } else if self.ends("ou") {
                    self.update_offset("ou");
                    byte_was_matched = true;
                }
                /* takes care of -ous */
            }
            b's' => {
                if self.ends("ism") {
                    self.update_offset("ism");
                    byte_was_matched = true;
                }
            }
            b't' => {
                if self.ends("ate") {
                    self.update_offset("ate");
                    byte_was_matched = true;
                } else if self.ends("iti") {
                    self.update_offset("iti");
                    byte_was_matched = true;
                }
            }
            b'u' => {
                if self.ends("ous") {
                    self.update_offset("ous");
                    byte_was_matched = true;
                }
            }
            b'v' => {
                if self.ends("ive") {
                    self.update_offset("ive");
                    byte_was_matched = true;
                }
            }
            b'z' => {
                if self.ends("ize") {
                    self.update_offset("ize");
                    byte_was_matched = true;
                }
            }
            _ => return,
        }
        if byte_was_matched && self.measure() > 1 {
            self.bytes_length = self.offset
        }
    }

    /// stem.step5() removes a final -e if self.measure() > 0, and changes -ll
    /// to -l if self.measure() > 1.
    fn step5(&mut self) {
        self.offset = self.bytes_length;
        if self.bytes[self.bytes_length - 1] == b'e' {
            let a = self.measure();
            if a > 1 || a == 1 && !self.cvc(self.bytes_length - 2) {
                self.bytes_length -= 1
            }
        }
        if self.bytes[self.bytes_length - 1] == b'l' && self.double_consonant(self.bytes_length - 1) && self.measure() > 1 {
            self.bytes_length -= 1;
        }
    }

    #[inline]
    fn get(&self) -> String {
        unsafe { str::from_utf8_unchecked(&self.bytes[..self.bytes_length]).to_owned() }
    }
}

pub fn get(word: &str) -> Result<String, RnltkError> {
    if word.len() > 2 {
        let mut mw = Stemmer::new(word)?;
        mw.step1ab();
        mw.step1c();
        mw.step2();
        mw.step3();
        mw.step4();
        mw.step5();
        Ok(mw.get())
    } else {
        Ok(word.to_owned())
    }
}

#[cfg(test)]
mod test_stem {
    use super::get;
    use std::ops::Deref;

    pub static INPUT: &str = include_str!("../test_data/voc.txt");
    pub static RESULT: &str = include_str!("../test_data/output.txt");

    fn test_loop<I0: Iterator<Item = T>, I1: Iterator<Item = T>, T: Deref<Target = str>>(
        tests: I0,
        results: I1,
    ) {
        for (test, expect) in tests.zip(results) {
            let test = test.trim_end();
            let expect = expect.trim_end();
            let stemmed = get(test.trim_end());

            assert!(stemmed.is_ok(), "[FAILED] Expected stem for '{}'", test);
            assert_eq!(stemmed.unwrap().trim_end(), expect);
        }
    }

    #[test]
    fn lexicon() {
        let input_s = INPUT.split('\n');
        let result_s = RESULT.split('\n');

        test_loop(input_s, result_s);
    }
}