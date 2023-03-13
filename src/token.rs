//! Module containing functions used to tokenize strings and get term frequencies.

use std::collections::BTreeMap;

use regex::Regex;

use crate::stem;

pub fn get_stop_words() -> Vec<String> {
    ["i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "you're", "you've", "you'll", "you'd", "your", "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "she's", "her", "hers", "herself", "it", "it's", "its", "itself", "they", "them", "their", "theirs", "themselves", "what", "which", "who", "whom", "this", "that", "that'll", "these", "those", "am", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "having", "do", "does", "did", "doing", "a", "an", "the", "and", "but", "if", "or", "because", "as", "until", "while", "of", "at", "by", "for", "with", "about", "against", "between", "into", "through", "during", "before", "after", "above", "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some", "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "don", "don't", "should", "should've", "now", "d", "ll", "m", "o", "re", "ve", "y", "ain", "aren", "aren't", "couldn", "couldn't", "didn", "didn't", "doesn", "doesn't", "hadn", "hadn't", "hasn", "hasn't", "haven", "haven't", "isn", "isn't", "ma", "mightn", "mightn't", "mustn", "mustn't", "needn", "needn't", "shan", "shan't", "shouldn", "shouldn't", "wasn", "wasn't", "weren", "weren't", "won", "won't", "wouldn", "wouldn't"]
        .map(String::from)
        .to_vec()
}

#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub stem: bool,
    pub remove_stop_words: bool,
    pub stop_words: Vec<String>,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            stem: true,
            remove_stop_words: true,
            stop_words: get_stop_words(),
        }
    }
}

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

/// Converts `sentence` to token vector without stop words.
///
/// # Examples
///
/// ```
/// use rnltk::token;
/// 
/// let text = "Why hello there. General Kenobi!";
/// let tokens = vec!["hello", "general", "kenobi"];
/// let stop_words = token::get_stop_words();
/// let tokenized_text = token::tokenize_sentence_without_stop_words(text, stop_words);
///
/// assert_eq!(tokens, tokenized_text);
/// ```
pub fn tokenize_sentence_without_stop_words(sentence: &str, stop_words: Vec<String>) -> Vec<String> {
    let punctuation = Regex::new(r#"[!"\#$%&'()*+,-./:;<=>?@\[\]^_`{|}~]+"#).expect("Invalid regex");
    let updated_sentence: &str = &punctuation.replace_all(sentence, "");

    let mut tokens: Vec<String> = updated_sentence
        .split(' ')
        .map(|s| s.trim().to_ascii_lowercase())
        .collect();
    tokens.retain(|token| !token.is_empty() && !stop_words.contains(token));

    tokens
}

/// Converts `sentence` to stemmed token vector.
///
/// # Examples
///
/// ```
/// use rnltk::token;
/// 
/// let text = "Why hello there. General Kenobi!";
/// let tokens = vec!["why", "hello", "there", "gener", "kenobi"];
/// let tokenized_text = token::tokenize_stemmed_sentence(text);
///
/// assert_eq!(tokens, tokenized_text);
/// ```
pub fn tokenize_stemmed_sentence(sentence: &str) -> Vec<String> {
    let punctuation = Regex::new(r#"[!"\#$%&'()*+,-./:;<=>?@\[\]^_`{|}~]+"#).expect("Invalid regex");
    let updated_sentence: &str = &punctuation.replace_all(sentence, "");

    let tokens: Vec<String> = updated_sentence
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| stem::get(s).unwrap_or_else(|_| s.to_string()))
        .collect();
    
    tokens
}

/// Converts `sentence` to stemmed token vector without stop words.
///
/// # Examples
///
/// ```
/// use rnltk::token;
/// 
/// let text = "Why hello there. General Kenobi!";
/// let tokens = vec!["hello", "gener", "kenobi"];
/// let stop_words = token::get_stop_words();
/// let tokenized_text = token::tokenize_stemmed_sentence_without_stop_words(text, stop_words);
///
/// assert_eq!(tokens, tokenized_text);
/// ```
pub fn tokenize_stemmed_sentence_without_stop_words(sentence: &str, stop_words: Vec<String>) -> Vec<String> {
    let punctuation = Regex::new(r#"[!"\#$%&'()*+,-./:;<=>?@\[\]^_`{|}~]+"#).expect("Invalid regex");
    let updated_sentence: &str = &punctuation.replace_all(sentence, "");

    let tokens: Vec<String> = updated_sentence
        .split(' ')
        .map(|token| token.trim().to_ascii_lowercase())
        .filter(|token| !token.is_empty() && !stop_words.contains(&token.to_string()))
        .map(|token| stem::get(&token).unwrap_or_else(|_| token.to_string()))
        .collect();

    tokens
}

/// Tokenize sentence based on a given configuration.
/// 
/// This function will be deprecated in the future once `rnltk` hits version 1.0
/// and functionality will be moved to `tokenize_sentence`.
/// 
/// # Examples
///
/// ```
/// use rnltk::token;
/// 
/// let token_config = token::TokenConfig::default();
/// let text = "Why hello there. General Kenobi!";
/// let tokens = vec!["hello", "gener", "kenobi"];
/// let tokenized_text = token::tokenize_sentence_configurable(text, token_config);
///
/// assert_eq!(tokens, tokenized_text);
/// ```
pub fn tokenize_sentence_configurable(sentence: &str, config: TokenConfig) -> Vec<String> {
    if config.remove_stop_words && config.stem {
        tokenize_stemmed_sentence_without_stop_words(sentence, config.stop_words)
    } else if config.remove_stop_words {
        tokenize_sentence_without_stop_words(sentence, config.stop_words)
    } else if config.stem {
        tokenize_stemmed_sentence(sentence)
    } else {
        tokenize_sentence(sentence)
    }
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

/// Gets a count of all words from a vector of `word_tokens` without stop words.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let arg = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("leads".to_string(), 4.), ("anger".to_string(), 2.), ("hatred".to_string(), 2.), ("conflict".to_string(), 2.), ("suffering".to_string(), 1.)]);
/// let stop_words = token::get_stop_words();
/// let term_frequencies = token::get_term_frequencies_from_word_vector_without_stop_words(arg, stop_words);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_term_frequencies_from_word_vector_without_stop_words(word_tokens: Vec<&str>, stop_words: Vec<String>) -> BTreeMap<String, f64> {
    let mut word_counts: BTreeMap<String, f64> = BTreeMap::new();
    for word in word_tokens {
        if !stop_words.contains(&word.to_string()) {
            let count = word_counts.entry(word.to_string()).or_insert(0.);
            *count += 1.;
        }
    }
    word_counts
}

/// Gets a count of all stemmed words from a vector of `word_tokens`.
/// 
/// If a word cannot be stemmed, it will get a frequency of the original word.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let arg = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("lead".to_string(), 4.), ("to".to_string(), 4.), ("anger".to_string(), 2.), ("hatr".to_string(), 2.), ("conflict".to_string(), 2.), ("suffer".to_string(), 1.)]);
/// let term_frequencies = token::get_stemmed_term_frequencies_from_word_vector(arg);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_stemmed_term_frequencies_from_word_vector(word_tokens: Vec<&str>) -> BTreeMap<String, f64> {
    let mut word_counts: BTreeMap<String, f64> = BTreeMap::new();
    for word in word_tokens {
        let count = word_counts.entry(stem::get(word).unwrap_or_else(|_| word.to_string())).or_insert(0.);
        *count += 1.;
    }
    word_counts
}

/// Gets a count of all stemmed words from a vector of `word_tokens` without stop words.
/// 
/// If a word cannot be stemmed, it will get a frequency of the original word.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let arg = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("lead".to_string(), 4.), ("anger".to_string(), 2.), ("hatr".to_string(), 2.), ("conflict".to_string(), 2.), ("suffer".to_string(), 1.)]);
/// let stop_words = token::get_stop_words();
/// let term_frequencies = token::get_stemmed_term_frequencies_from_word_vector_without_stop_words(arg, stop_words);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_stemmed_term_frequencies_from_word_vector_without_stop_words(word_tokens: Vec<&str>, stop_words: Vec<String>) -> BTreeMap<String, f64> {
    let mut word_counts: BTreeMap<String, f64> = BTreeMap::new();
    for word in word_tokens {
        if !stop_words.contains(&word.to_string()) {
            let count = word_counts.entry(stem::get(word).unwrap_or_else(|_| word.to_string())).or_insert(0.);
            *count += 1.;
        }
    }
    word_counts
}

/// Gets a count of all words from a vector of `word_tokens` based on a given configuration.
/// 
/// This function will be deprecated in the future once `rnltk` hits version 1.0
/// and functionality will be moved to `get_term_frequencies_from_word_vector`.
/// 
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let token_config = token::TokenConfig::default();
/// let arg = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("lead".to_string(), 4.), ("anger".to_string(), 2.), ("hatr".to_string(), 2.), ("conflict".to_string(), 2.), ("suffer".to_string(), 1.)]);
/// let term_frequencies = token::get_term_frequencies_from_word_vector_configurable(arg, token_config);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_term_frequencies_from_word_vector_configurable(word_tokens: Vec<&str>, config: TokenConfig) -> BTreeMap<String, f64> {
    if config.remove_stop_words && config.stem {
        get_stemmed_term_frequencies_from_word_vector_without_stop_words(word_tokens, config.stop_words)
    } else if config.remove_stop_words {
        get_term_frequencies_from_word_vector_without_stop_words(word_tokens, config.stop_words)
    } else if config.stem {
        get_stemmed_term_frequencies_from_word_vector(word_tokens)
    } else {
        get_term_frequencies_from_word_vector(word_tokens)
    }
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

/// Gets a count of all words from a `sentence` without `stop_words`.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let sentence = "fear leads to anger, anger leads to hatred, hatred leads to conflict, conflict leads to suffering.";
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("leads".to_string(), 4.), ("anger".to_string(), 2.), ("hatred".to_string(), 2.), ("conflict".to_string(), 2.), ("suffering".to_string(), 1.)]);
/// let stop_words = token::get_stop_words();
/// let term_frequencies = token::get_term_frequencies_from_sentence_without_stop_words(sentence, stop_words);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_term_frequencies_from_sentence_without_stop_words(sentence: &str, stop_words: Vec<String>) -> BTreeMap<String, f64> {
    let sentence_tokens = tokenize_sentence(sentence);
    let sentence_tokens: Vec<&str> = sentence_tokens.iter().map(|s| s.as_str()).collect();
    get_term_frequencies_from_word_vector_without_stop_words(sentence_tokens, stop_words)
}

/// Gets a count of all stemmed words from a `sentence`.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let sentence = "fear leads to anger, anger leads to hatred, hatred leads to conflict, conflict leads to suffering.";
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("lead".to_string(), 4.), ("to".to_string(), 4.), ("anger".to_string(), 2.), ("hatr".to_string(), 2.), ("conflict".to_string(), 2.), ("suffer".to_string(), 1.)]);
/// let term_frequencies = token::get_stemmed_term_frequencies_from_sentence(sentence);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_stemmed_term_frequencies_from_sentence(sentence: &str) -> BTreeMap<String, f64> {
    let sentence_tokens = tokenize_sentence(sentence);
    let sentence_tokens: Vec<&str> = sentence_tokens.iter().map(|s| s.as_str()).collect();
    get_stemmed_term_frequencies_from_word_vector(sentence_tokens)
}

/// Gets a count of all stemmed words from a `sentence` without `stop_words`.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let sentence = "fear leads to anger, anger leads to hatred, hatred leads to conflict, conflict leads to suffering.";
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("lead".to_string(), 4.), ("anger".to_string(), 2.), ("hatr".to_string(), 2.), ("conflict".to_string(), 2.), ("suffer".to_string(), 1.)]);
/// let stop_words = token::get_stop_words();
/// let term_frequencies = token::get_stemmed_term_frequencies_from_sentence_without_stop_words(sentence, stop_words);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_stemmed_term_frequencies_from_sentence_without_stop_words(sentence: &str, stop_words: Vec<String>) -> BTreeMap<String, f64> {
    let sentence_tokens = tokenize_sentence(sentence);
    let sentence_tokens: Vec<&str> = sentence_tokens.iter().map(|s| s.as_str()).collect();
    get_stemmed_term_frequencies_from_word_vector_without_stop_words(sentence_tokens, stop_words)
}

/// Gets a count of all words from a `sentence` based on a given configuration.
/// 
/// This function will be deprecated in the future once `rnltk` hits version 1.0
/// and functionality will be moved to `get_term_frequencies_from_sentence`.
/// 
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let token_config = token::TokenConfig::default();
/// let sentence = "fear leads to anger, anger leads to hatred, hatred leads to conflict, conflict leads to suffering.";
/// let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("lead".to_string(), 4.), ("anger".to_string(), 2.), ("hatr".to_string(), 2.), ("conflict".to_string(), 2.), ("suffer".to_string(), 1.)]);
/// let term_frequencies = token::get_term_frequencies_from_sentence_configurable(sentence, token_config);
///
/// assert_eq!(word_counts, term_frequencies);
/// ```
pub fn get_term_frequencies_from_sentence_configurable(sentence: &str, config: TokenConfig) -> BTreeMap<String, f64> {
    if config.remove_stop_words && config.stem {
        get_stemmed_term_frequencies_from_sentence_without_stop_words(sentence, config.stop_words)
    } else if config.remove_stop_words {
        get_term_frequencies_from_sentence_without_stop_words(sentence, config.stop_words)
    } else if config.stem {
        get_stemmed_term_frequencies_from_sentence(sentence)
    } else {
        get_term_frequencies_from_sentence(sentence)
    }
}

/// Gets a count of all words from a vector of `sentence`s.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
/// let word_counts1 = BTreeMap::from([
///     ("fear".to_string(), 1.), ("leads".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatred".to_string(), 0.), ("conflict".to_string(), 0.), ("suffering".to_string(), 0.)
/// ]);
/// let word_counts2 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatred".to_string(), 1.), ("conflict".to_string(), 0.), ("suffering".to_string(), 0.)
/// ]);
/// let word_counts3 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatred".to_string(), 1.), ("conflict".to_string(),1.), ("suffering".to_string(), 0.)
/// ]);
/// let word_counts4 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatred".to_string(), 0.), ("conflict".to_string(), 1.), ("suffering".to_string(), 1.)
/// ]);
/// let term_frequencies = token::get_term_frequencies_from_sentences(&sentences);
///
/// assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
/// ```
pub fn get_term_frequencies_from_sentences(sentences: &[&str]) -> Vec<BTreeMap<String, f64>> {
    let mut total_terms: Vec<String> = vec![];
    let mut term_frequencies: Vec<BTreeMap<String, f64>> = sentences.iter().map(|sentence| {
        let frequencies = get_term_frequencies_from_sentence(sentence);
        total_terms.extend(frequencies.keys().cloned().collect::<Vec<String>>());
        frequencies
    }).collect();
    for frequency_counts in &mut term_frequencies {
        for term in &total_terms {
            if !frequency_counts.contains_key(term) {
                frequency_counts.insert(term.to_string(), 0.);
            }
        }
    }
    term_frequencies
}

/// Gets a count of all words from a vector of `sentence`s without `stop_words`.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
/// let stop_words = token::get_stop_words();
/// let word_counts1 = BTreeMap::from([
///     ("fear".to_string(), 1.), ("leads".to_string(), 1.), ("anger".to_string(), 1.), ("hatred".to_string(), 0.), ("conflict".to_string(), 0.), ("suffering".to_string(), 0.)
/// ]);
/// let word_counts2 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("anger".to_string(), 1.), ("hatred".to_string(), 1.), ("conflict".to_string(), 0.), ("suffering".to_string(), 0.)
/// ]);
/// let word_counts3 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("anger".to_string(), 0.), ("hatred".to_string(), 1.), ("conflict".to_string(),1.), ("suffering".to_string(), 0.)
/// ]);
/// let word_counts4 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("anger".to_string(), 0.), ("hatred".to_string(), 0.), ("conflict".to_string(), 1.), ("suffering".to_string(), 1.)
/// ]);
/// let term_frequencies = token::get_term_frequencies_from_sentences_without_stop_words(&sentences, stop_words);
///
/// assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
/// ```
pub fn get_term_frequencies_from_sentences_without_stop_words(sentences: &[&str], stop_words: Vec<String>) -> Vec<BTreeMap<String, f64>> {
    let mut total_terms: Vec<String> = vec![];
    let mut term_frequencies: Vec<BTreeMap<String, f64>> = sentences.iter().map(|sentence| {
        let frequencies = get_term_frequencies_from_sentence_without_stop_words(sentence, stop_words.clone());
        total_terms.extend(frequencies.keys().cloned().collect::<Vec<String>>());
        frequencies
    }).collect();
    for frequency_counts in &mut term_frequencies {
        for term in &total_terms {
            if !frequency_counts.contains_key(term) {
                frequency_counts.insert(term.to_string(), 0.);
            }
        }
    }
    term_frequencies
}

/// Gets a count of all stemmed words from a vector of `sentence`s.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
/// let word_counts1 = BTreeMap::from([
///     ("fear".to_string(), 1.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 0.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
/// ]);
/// let word_counts2 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 1.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
/// ]);
/// let word_counts3 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 1.), ("conflict".to_string(),1.), ("suffer".to_string(), 0.)
/// ]);
/// let word_counts4 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 0.), ("conflict".to_string(), 1.), ("suffer".to_string(), 1.)
/// ]);
/// let term_frequencies = token::get_stemmed_term_frequencies_from_sentences(&sentences);
///
/// assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
/// ```
pub fn get_stemmed_term_frequencies_from_sentences(sentences: &[&str]) -> Vec<BTreeMap<String, f64>> {
    let mut total_terms: Vec<String> = vec![];
    let mut term_frequencies: Vec<BTreeMap<String, f64>> = sentences.iter().map(|sentence| {
        let frequencies = get_stemmed_term_frequencies_from_sentence(sentence);
        total_terms.extend(frequencies.keys().cloned().collect::<Vec<String>>());
        frequencies
    }).collect();
    for frequency_counts in &mut term_frequencies {
        for term in &total_terms {
            if !frequency_counts.contains_key(term) {
                frequency_counts.insert(term.to_string(), 0.);
            }
        }
    }
    term_frequencies
}


/// Gets a count of all stemmed words from a vector of `sentence`s without `stop_words`.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
/// let stop_words = token::get_stop_words();
/// let word_counts1 = BTreeMap::from([
///     ("fear".to_string(), 1.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 0.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
/// ]);
/// let word_counts2 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 1.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
/// ]);
/// let word_counts3 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 1.), ("conflict".to_string(),1.), ("suffer".to_string(), 0.)
/// ]);
/// let word_counts4 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 0.), ("conflict".to_string(), 1.), ("suffer".to_string(), 1.)
/// ]);
/// let term_frequencies = token::get_stemmed_term_frequencies_from_sentences(&sentences);
///
/// assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
/// ```
pub fn get_stemmed_term_frequencies_from_sentences_without_stop_words(sentences: &[&str], stop_words: Vec<String>) -> Vec<BTreeMap<String, f64>> {
    let mut total_terms: Vec<String> = vec![];
    let mut term_frequencies: Vec<BTreeMap<String, f64>> = sentences.iter().map(|sentence| {
        let frequencies = get_stemmed_term_frequencies_from_sentence_without_stop_words(sentence, stop_words.clone());
        total_terms.extend(frequencies.keys().cloned().collect::<Vec<String>>());
        frequencies
    }).collect();
    for frequency_counts in &mut term_frequencies {
        for term in &total_terms {
            if !frequency_counts.contains_key(term) {
                frequency_counts.insert(term.to_string(), 0.);
            }
        }
    }
    term_frequencies
}

/// Gets a count of all words from a vector of `word_tokens` based on a given configuration.
/// 
/// This function will be deprecated in the future once `rnltk` hits version 1.0
/// and functionality will be moved to `get_term_frequencies_from_word_vector`.
/// 
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use rnltk::token;
/// 
/// let token_config = token::TokenConfig::default();
/// let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
/// let word_counts1 = BTreeMap::from([
///     ("fear".to_string(), 1.), ("lead".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 0.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
/// ]);
/// let word_counts2 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 1.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
/// ]);
/// let word_counts3 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 1.), ("conflict".to_string(),1.), ("suffer".to_string(), 0.)
/// ]);
/// let word_counts4 = BTreeMap::from([
///     ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 0.), ("conflict".to_string(), 1.), ("suffer".to_string(), 1.)
/// ]);
/// let term_frequencies = token::get_term_frequencies_from_sentences_configurable(&sentences, token_config);
///
/// assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
/// ```
pub fn get_term_frequencies_from_sentences_configurable(sentences: &[&str], config: TokenConfig) -> Vec<BTreeMap<String, f64>> {
    if config.remove_stop_words && config.stem {
        get_stemmed_term_frequencies_from_sentences_without_stop_words(sentences, config.stop_words)
    } else if config.remove_stop_words {
        get_term_frequencies_from_sentences_without_stop_words(sentences, config.stop_words)
    } else if config.stem {
        get_stemmed_term_frequencies_from_sentences(sentences)
    } else {
        get_term_frequencies_from_sentences(sentences)
    }
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
    fn test_sentence_tokenization_without_stop_words() {
        let stop_words = get_stop_words();
        let text = "Why hello there. General Kenobi!";
        let tokens = vec!["hello", "general", "kenobi"];
        let tokenized_text = tokenize_sentence_without_stop_words(text, stop_words);
        assert_eq!(tokens, tokenized_text);
    }

    #[test]
    fn test_sentence_tokenization_with_stemming() {
        let text = "Why hello there. General Kenobi!";
        let tokens = vec!["why", "hello", "there", "gener", "kenobi"];
        let tokenized_text = tokenize_stemmed_sentence(text);
        assert_eq!(tokens, tokenized_text);
    }

    #[test]
    fn test_sentence_tokenization_with_stemming_without_stop_words() {
        let stop_words = get_stop_words();
        let text = "Why hello there. General Kenobi!";
        let tokens = vec!["hello", "gener", "kenobi"];
        let tokenized_text = tokenize_stemmed_sentence_without_stop_words(text, stop_words);
        assert_eq!(tokens, tokenized_text);
    }

    #[test]
    fn test_sentence_tokenization_configurable() {
        let token_config = TokenConfig::default();
        let text = "Why hello there. General Kenobi!";
        let tokens = vec!["hello", "gener", "kenobi"];
        let tokenized_text = tokenize_sentence_configurable(text, token_config);
        assert_eq!(tokens, tokenized_text);
    }

    #[test]
    fn test_term_frequencies_from_str_vector() {
        let tokens = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
        let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("leads".to_string(), 4.), ("to".to_string(), 4.), ("anger".to_string(), 2.), ("hatred".to_string(), 2.), ("conflict".to_string(), 2.), ("suffering".to_string(), 1.)]);
        let term_frequencies = get_term_frequencies_from_word_vector(tokens);
        assert_eq!(word_counts, term_frequencies);
    }

    #[test]
    fn test_term_frequencies_from_str_vector_without_stop_words() {
        let stop_words = get_stop_words();
        let tokens = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
        let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("leads".to_string(), 4.), ("anger".to_string(), 2.), ("hatred".to_string(), 2.), ("conflict".to_string(), 2.), ("suffering".to_string(), 1.)]);
        let term_frequencies = get_term_frequencies_from_word_vector_without_stop_words(tokens, stop_words);
        assert_eq!(word_counts, term_frequencies);
    }

    #[test]
    fn test_term_frequencies_from_str_vector_with_stemming() {
        let tokens = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
        let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("lead".to_string(), 4.), ("to".to_string(), 4.), ("anger".to_string(), 2.), ("hatr".to_string(), 2.), ("conflict".to_string(), 2.), ("suffer".to_string(), 1.)]);
        let term_frequencies = get_stemmed_term_frequencies_from_word_vector(tokens);
        assert_eq!(word_counts, term_frequencies);
    }

    #[test]
    fn test_term_frequencies_from_str_vector_with_stemming_without_stop_words() {
        let stop_words = get_stop_words();
        let tokens = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
        let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("lead".to_string(), 4.), ("anger".to_string(), 2.), ("hatr".to_string(), 2.), ("conflict".to_string(), 2.), ("suffer".to_string(), 1.)]);
        let term_frequencies = get_stemmed_term_frequencies_from_word_vector_without_stop_words(tokens, stop_words);
        assert_eq!(word_counts, term_frequencies);
    }

    #[test]
    fn test_term_frequencies_from_str_vector_configurable() {
        let token_config = TokenConfig::default();
        let tokens = vec!["fear", "leads", "to", "anger", "anger", "leads", "to", "hatred", "hatred", "leads", "to", "conflict", "conflict", "leads", "to", "suffering"];
        let word_counts = BTreeMap::from([("fear".to_string(), 1.), ("lead".to_string(), 4.), ("anger".to_string(), 2.), ("hatr".to_string(), 2.), ("conflict".to_string(), 2.), ("suffer".to_string(), 1.)]);
        let term_frequencies = get_term_frequencies_from_word_vector_configurable(tokens, token_config);
        assert_eq!(word_counts, term_frequencies);
    }

    #[test]
    fn test_term_frequencies_from_sentences() {
        let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
        let word_counts1 = BTreeMap::from([
            ("fear".to_string(), 1.), ("leads".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatred".to_string(), 0.), ("conflict".to_string(), 0.), ("suffering".to_string(), 0.)
        ]);
        let word_counts2 = BTreeMap::from([
            ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatred".to_string(), 1.), ("conflict".to_string(), 0.), ("suffering".to_string(), 0.)
        ]);
        let word_counts3 = BTreeMap::from([
            ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatred".to_string(), 1.), ("conflict".to_string(), 1.), ("suffering".to_string(), 0.)
        ]);
        let word_counts4 = BTreeMap::from([
            ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatred".to_string(), 0.), ("conflict".to_string(), 1.), ("suffering".to_string(), 1.)
        ]);
        let term_frequencies = get_term_frequencies_from_sentences(&sentences);
        
        assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
    }

    #[test]
    fn test_term_frequencies_from_sentences_without_stop_words() {
        let stop_words = get_stop_words();
        let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
        let word_counts1 = BTreeMap::from([
            ("fear".to_string(), 1.), ("leads".to_string(), 1.), ("anger".to_string(), 1.), ("hatred".to_string(), 0.), ("conflict".to_string(), 0.), ("suffering".to_string(), 0.)
        ]);
        let word_counts2 = BTreeMap::from([
            ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("anger".to_string(), 1.), ("hatred".to_string(), 1.), ("conflict".to_string(), 0.), ("suffering".to_string(), 0.)
        ]);
        let word_counts3 = BTreeMap::from([
            ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("anger".to_string(), 0.), ("hatred".to_string(), 1.), ("conflict".to_string(), 1.), ("suffering".to_string(), 0.)
        ]);
        let word_counts4 = BTreeMap::from([
            ("fear".to_string(), 0.), ("leads".to_string(), 1.), ("anger".to_string(), 0.), ("hatred".to_string(), 0.), ("conflict".to_string(), 1.), ("suffering".to_string(), 1.)
        ]);
        let term_frequencies = get_term_frequencies_from_sentences_without_stop_words(&sentences, stop_words);
        
        assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
    }

    #[test]
    fn test_term_frequencies_from_sentences_with_stemming() {
        let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
        let word_counts1 = BTreeMap::from([
            ("fear".to_string(), 1.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 0.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
        ]);
        let word_counts2 = BTreeMap::from([
            ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 1.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
        ]);
        let word_counts3 = BTreeMap::from([
            ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 1.), ("conflict".to_string(),1.), ("suffer".to_string(), 0.)
        ]);
        let word_counts4 = BTreeMap::from([
            ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("to".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 0.), ("conflict".to_string(), 1.), ("suffer".to_string(), 1.)
        ]);
        let term_frequencies = get_stemmed_term_frequencies_from_sentences(&sentences);
        
        assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
    }

    #[test]
    fn test_term_frequencies_from_sentences_with_stemming_without_stop_words() {
        let stop_words = get_stop_words();
        let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
        let word_counts1 = BTreeMap::from([
            ("fear".to_string(), 1.), ("lead".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 0.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
        ]);
        let word_counts2 = BTreeMap::from([
            ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 1.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
        ]);
        let word_counts3 = BTreeMap::from([
            ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 1.), ("conflict".to_string(), 1.), ("suffer".to_string(), 0.)
        ]);
        let word_counts4 = BTreeMap::from([
            ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 0.), ("conflict".to_string(), 1.), ("suffer".to_string(), 1.)
        ]);
        let term_frequencies = get_stemmed_term_frequencies_from_sentences_without_stop_words(&sentences, stop_words);
        
        assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
    }

    #[test]
    fn test_term_frequencies_from_sentences_configurable() {
        let token_config = TokenConfig::default();
        let sentences = vec!["fear leads to anger", "anger leads to hatred", "hatred leads to conflict", "conflict leads to suffering."];
        let word_counts1 = BTreeMap::from([
            ("fear".to_string(), 1.), ("lead".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 0.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
        ]);
        let word_counts2 = BTreeMap::from([
            ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("anger".to_string(), 1.), ("hatr".to_string(), 1.), ("conflict".to_string(), 0.), ("suffer".to_string(), 0.)
        ]);
        let word_counts3 = BTreeMap::from([
            ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 1.), ("conflict".to_string(), 1.), ("suffer".to_string(), 0.)
        ]);
        let word_counts4 = BTreeMap::from([
            ("fear".to_string(), 0.), ("lead".to_string(), 1.), ("anger".to_string(), 0.), ("hatr".to_string(), 0.), ("conflict".to_string(), 1.), ("suffer".to_string(), 1.)
        ]);
        let term_frequencies = get_term_frequencies_from_sentences_configurable(&sentences, token_config);
        
        assert_eq!(vec![word_counts1, word_counts2, word_counts3, word_counts4], term_frequencies);
    }
}