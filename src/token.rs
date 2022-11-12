//! Module containing functions used to tokenize strings and get term frequencies.

use std::collections::BTreeMap;

use regex::Regex;


const STOP_WORDS: &[&str] = &["i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "you're", "you've", "you'll", "you'd", "your", "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "she's", "her", "hers", "herself", "it", "it's", "its", "itself", "they", "them", "their", "theirs", "themselves", "what", "which", "who", "whom", "this", "that", "that'll", "these", "those", "am", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "having", "do", "does", "did", "doing", "a", "an", "the", "and", "but", "if", "or", "because", "as", "until", "while", "of", "at", "by", "for", "with", "about", "against", "between", "into", "through", "during", "before", "after", "above", "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some", "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "don", "don't", "should", "should've", "now", "d", "ll", "m", "o", "re", "ve", "y", "ain", "aren", "aren't", "couldn", "couldn't", "didn", "didn't", "doesn", "doesn't", "hadn", "hadn't", "hasn", "hasn't", "haven", "haven't", "isn", "isn't", "ma", "mightn", "mightn't", "mustn", "mustn't", "needn", "needn't", "shan", "shan't", "shouldn", "shouldn't", "wasn", "wasn't", "weren", "weren't", "won", "won't", "wouldn", "wouldn't"];

/// Converts a `document` to sentence vector.
///
/// # Examples
///
/// ```
/// use rnltk::token;
/// 
/// let text = "Why hello there. General Kenobi!";
/// let tokens = vec!["Why hello there", "General Kenobi"];
/// let tokenized_text = token::tokenize_into_sentences(text);
///
/// assert_eq!(tokens, tokenized_text);
/// ```
pub fn tokenize_into_sentences(document: &str) -> Vec<String> {
    let quote_regex = Regex::new(r#"[\.!\?]""#).expect("Invalid regex");
    let updated_document: &str = &quote_regex.replace_all(document, "\"");

    let separator = Regex::new(r#"[\.!\?] *"#).expect("Invalid regex");
    let mut full_sentences: Vec<String> = separator.split(updated_document).map(|s| s.to_string()).collect();
    full_sentences.retain(|sentence| !sentence.is_empty());

    full_sentences
}

/// Converts `sentence` to token vector.
///
/// # Examples
///
/// ```
/// use rnltk::token;
/// 
/// let text = "Why hello there. General Kenobi!";
/// let tokens = vec!["Why", "hello", "there", "General", "Kenobi"];
/// let tokenized_text = token::tokenize_sentence(text);
///
/// assert_eq!(tokens, tokenized_text);
/// ```
pub fn tokenize_sentence(sentence: &str) -> Vec<String> {
    let punctuation = Regex::new(r#"[!"\#$%&'()*+,-./:;<=>?@\[\]^_`{|}~]+"#).expect("Invalid regex");
    let updated_sentence: &str = &punctuation.replace_all(sentence, "");

    let mut tokens: Vec<String> = updated_sentence.split(' ').map(|s| s.trim().to_string()).collect();
    tokens.retain(|token| !token.is_empty());

    tokens
}

/// Gets a count of all words from a vector of `word_tokens`.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let arg = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("leads".to_string(), 4.), ("to".to_string(), 4.), ("anger".to_string(), 2.), ("hatred".to_string(), 2.), ("conflict".to_string(), 2.), ("suffering".to_string(), 1.)]);
/// let term_frequencies = token::get_term_frequencies_from_word_vector(arg);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_term_frequencies_from_word_vector(word_tokens: Vec<&str>) -> BTreeMap<String, f64> {
    let mut word_counts: BTreeMap<String, f64> = BTreeMap::new();
    for word in word_tokens {
        let count = word_counts.entry(word.to_string()).or_insert(0.);
        *count += 1.;
    }
    word_counts
}

/// Gets a count of all words from a `sentence`.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let sentence = "fear leads to anger, anger leads to hatred, hatred leads to conflict, conflict leads to suffering.";
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("leads".to_string(), 4.), ("to".to_string(), 4.), ("anger".to_string(), 2.), ("hatred".to_string(), 2.), ("conflict".to_string(), 2.), ("suffering".to_string(), 1.)]);
/// let term_frequencies = token::get_term_frequencies_from_sentence(sentence);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_term_frequencies_from_sentence(sentence: &str) -> BTreeMap<String, f64> {
    let sentence_tokens = tokenize_sentence(sentence);
    let sentence_tokens: Vec<&str> = sentence_tokens.iter().map(|s| s.as_str()).collect();
    get_term_frequencies_from_word_vector(sentence_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_document_tokenization() {
        let text = "Why hello there. General Kenobi!";
        let tokens = vec!["Why hello there", "General Kenobi"];
        let tokenized_text = tokenize_into_sentences(text);
        assert_eq!(tokens, tokenized_text);
    }

    #[test]
    fn test_sentence_tokenization() {
        let text = "Why hello there. General Kenobi!";
        let tokens = vec!["Why", "hello", "there", "General", "Kenobi"];
        let tokenized_text = tokenize_sentence(text);
        assert_eq!(tokens, tokenized_text);
    }

    #[test]
    fn test_term_frequencies_from_str_vector() {
        let tokens = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
        let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("leads".to_string(), 4.), ("to".to_string(), 4.), ("anger".to_string(), 2.), ("hatred".to_string(), 2.), ("conflict".to_string(), 2.), ("suffering".to_string(), 1.)]);
        let term_frequencies = get_term_frequencies_from_word_vector(tokens);
        assert_eq!(word_counts, term_frequencies);
    }
}