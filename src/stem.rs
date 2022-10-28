use std::str;

/// Member b is a vector of bytes holding a word to be stemmed.
/// The letters are in b[0], b[1] ... ending at b[z->k]. Member k is readjusted
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
/// 
/// let b = "pencils";
/// let res = stem::get(b);
/// assert_eq!(res, Ok("pencil".to_string()));
///```
struct Stemmer {
    bytes: Vec<u8>,
    bytes_length: usize,
    offset: usize,
}

impl Stemmer {
    fn new(word: &str) -> Result<Stemmer, &'static str> {
        if !word.is_ascii() {
            Err("Only supports English words with ASCII characters")
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

    /// stem.is_consonant(i) is true <=> stem[i] is a consonant
    #[inline]
    fn is_consonant(&self, i: usize) -> bool {
        match self.bytes[i] {
            b'a' | b'e' | b'i' | b'o' | b'u' => false,
            b'y' => {
                if i == 0 {
                    true
                } else {
                    !self.is_consonant(i - 1)
                }
            }
            _ => true,
        }
    }

    /// stem.measure() measures the number of consonant sequences in [0, j).
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
        let mut i = 0;
        let j = self.offset;
        loop {
            if i >= j {
                return n;
            }
            if !self.is_consonant(i) {
                break;
            }
            i += 1;
        }
        i += 1;
        loop {
            loop {
                if i >= j {
                    return n;
                }
                if self.is_consonant(i) {
                    break;
                }
                i += 1;
            }
            i += 1;
            n += 1;
            loop {
                if i >= j {
                    return n;
                }
                if !self.is_consonant(i) {
                    break;
                }
                i += 1;
            }
            i += 1;
        }
    }

    /// stem.has_vowel() is TRUE <=> [0, j-1) contains a vowel
    fn has_vowel(&self) -> bool {
        for i in 0..self.offset {
            if !self.is_consonant(i) {
                return true;
            }
        }
        false
    }

    /// stem.double_consonant(i) is TRUE <=> i,(i-1) contain a double consonant.
    #[inline]
    fn double_consonant(&self, i: usize) -> bool {
        if i < 1 || self.bytes[i] != self.bytes[i - 1] {
            false
        } else {
            self.is_consonant(i)
        }
    }

    /// cvc(z, i) is TRUE <=> i-2,i-1,i has the form consonant - vowel - consonant
    /// and also if the second c is not w,x or y. this is used when trying to
    /// restore an e at the end of a short word. e.g.
    ///
    /// ~~~notrust
    ///    cav(e), lov(e), hop(e), crim(e), but
    ///    snow, box, tray.
    /// ~~~
    fn cvc(&self, i: usize) -> bool {
        if i < 2 || !self.is_consonant(i) || self.is_consonant(i - 1) || !self.is_consonant(i - 2) {
            false
        } else {
            !matches!(self.bytes[i], b'w' | b'x' | b'y')
        }
    }

    /// stem.ends(s) is true <=> [0, k) ends with the string s.
    fn ends(&mut self, _s: &str) -> bool {
        let s = _s.as_bytes();
        let len = s.len();
        if len > self.bytes_length {
            false
        } else if &self.bytes[self.bytes_length - len..self.bytes_length] == s {
            self.offset = self.bytes_length - len;
            true
        } else {
            false
        }
    }

    /// stem.setto(s) sets [j,k) to the characters in the string s,
    /// readjusting k.
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
                self.bytes_length -= 2;
            } else if self.ends("ies") {
                self.set_to("i");
            } else if self.bytes[self.bytes_length - 2] != b's' {
                self.bytes_length -= 1;
            }
        }
        if self.ends("eed") {
            if self.measure() > 0 {
                self.bytes_length -= 1
            }
        } else if (self.ends("ed") || self.ends("ing")) && self.has_vowel() {
            self.bytes_length = self.offset;
            if self.ends("at") {
                self.set_to("ate");
            } else if self.ends("bl") {
                self.set_to("ble");
            } else if self.ends("iz") {
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

    /// stem.step1c() turns terminal y to i when there is another vowel in the stem.
    fn step1c(&mut self) {
        if self.ends("y") && self.has_vowel() {
            self.bytes[self.bytes_length - 1] = b'i';
        }
    }

    /// stem.step2() maps double suffices to single ones. so -ization ( = -ize
    /// plus -ation) maps to -ize etc. note that the string before the suffix
    /// must give m(z) > 0.
    fn step2(&mut self) {
        match self.bytes[self.bytes_length - 2] {
            b'a' => {
                if self.ends("ational") {
                    self.replace("ate");
                    return;
                }
                if self.ends("tional") {
                    self.replace("tion");
                    return;
                }
            }
            b'c' => {
                if self.ends("enci") {
                    self.replace("ence");
                    return;
                }
                if self.ends("anci") {
                    self.replace("ance");
                    return;
                }
            }
            b'e' => {
                if self.ends("izer") {
                    self.replace("ize");
                    return;
                }
            }
            b'l' => {
                if self.ends("bli") {
                    self.replace("ble");
                    return;
                } /*-DEPARTURE-*/

                /* To match the published algorithm, replace this line with
                'l' => {
                    if self.ends("abli") { self.replace("able"); return } */

                if self.ends("alli") {
                    self.replace("al");
                    return;
                }
                if self.ends("entli") {
                    self.replace("ent");
                    return;
                }
                if self.ends("eli") {
                    self.replace("e");
                    return;
                }
                if self.ends("ousli") {
                    self.replace("ous");
                    return;
                }
            }
            b'o' => {
                if self.ends("ization") {
                    self.replace("ize");
                    return;
                }
                if self.ends("ation") {
                    self.replace("ate");
                    return;
                }
                if self.ends("ator") {
                    self.replace("ate");
                    return;
                }
            }
            b's' => {
                if self.ends("alism") {
                    self.replace("al");
                    return;
                }
                if self.ends("iveness") {
                    self.replace("ive");
                    return;
                }
                if self.ends("fulness") {
                    self.replace("ful");
                    return;
                }
                if self.ends("ousness") {
                    self.replace("ous");
                    return;
                }
            }
            b't' => {
                if self.ends("aliti") {
                    self.replace("al");
                    return;
                }
                if self.ends("iviti") {
                    self.replace("ive");
                    return;
                }
                if self.ends("biliti") {
                    self.replace("ble");
                    return;
                }
            }
            b'g' => {
                if self.ends("logi") {
                    self.replace("log");
                    return;
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
                    self.replace("ic");
                    return;
                }
                if self.ends("ative") {
                    self.replace("");
                    return;
                }
                if self.ends("alize") {
                    self.replace("al");
                    return;
                }
            }
            b'i' => {
                if self.ends("iciti") {
                    self.replace("ic");
                    return;
                }
            }
            b'l' => {
                if self.ends("ical") {
                    self.replace("ic");
                    return;
                }
                if self.ends("ful") {
                    self.replace("");
                    return;
                }
            }
            b's' => {
                if self.ends("ness") {
                    self.replace("");
                    return;
                }
            }
            _ => (),
        }
    }

    /// stem.step4() takes off -ant, -ence etc., in context <c>vcvc<v>.
    fn step4(&mut self) {
        match self.bytes[self.bytes_length - 2] {
            b'a' => {
                if self.ends("al") {
                } else {
                    return;
                }
            }
            b'c' => {
                if self.ends("ance") || self.ends("ence") {
                } else {
                    return;
                }
            }
            b'e' => {
                if self.ends("er") {
                } else {
                    return;
                }
            }
            b'i' => {
                if self.ends("ic") {
                } else {
                    return;
                }
            }
            b'l' => {
                if self.ends("able") || self.ends("ible") {
                } else {
                    return;
                }
            }
            b'n' => {
                if self.ends("ant") || self.ends("ement") || self.ends("ment") || self.ends("ent") {
                } else {
                    return;
                }
            }
            b'o' => {
                if self.ends("ion") && (self.bytes[self.offset - 1] == b's' || self.bytes[self.offset - 1] == b't') || self.ends("ou") {
                } else {
                    return;
                }
                /* takes care of -ous */
            }
            b's' => {
                if self.ends("ism") {
                } else {
                    return;
                }
            }
            b't' => {
                if self.ends("ate") || self.ends("iti") {
                } else {
                    return;
                }
            }
            b'u' => {
                if self.ends("ous") {
                } else {
                    return;
                }
            }
            b'v' => {
                if self.ends("ive") {
                } else {
                    return;
                }
            }
            b'z' => {
                if self.ends("ize") {
                } else {
                    return;
                }
            }
            _ => return,
        }
        if self.measure() > 1 {
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

pub fn get(word: &str) -> Result<String, &str> {
    if word.len() > 2 {
        match Stemmer::new(word) {
            Ok(w) => {
                let mut mw = w;
                mw.step1ab();
                mw.step1c();
                mw.step2();
                mw.step3();
                mw.step4();
                mw.step5();
                Ok(mw.get())
            }
            Err(e) => Err(e),
        }
    } else {
        Ok(word.to_owned())
    }
}

#[cfg(test)]
mod test_stem {
    use super::get;
    use std::ops::Deref;

    pub static INPUT: &str = include_str!("../test-data/voc.txt");
    pub static RESULT: &str = include_str!("../test-data/output.txt");

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