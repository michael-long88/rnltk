use rnltk::sentiment::{SentimentModel, CustomWords};
use rnltk::token;
use rnltk::stem;


struct Setup {
    custom_words: CustomWords
}

impl Setup {
    fn new() -> Self {
        let custom_word_dict: &str = include_str!("../test_data/test.json");
        let custom_words: CustomWords = serde_json::from_str(custom_word_dict).unwrap();
        Setup {
            custom_words
        }
    }
}

#[test]
fn sentiment_from_tokenized_sentence() {
    let setup = Setup::new();
    let sentiment = SentimentModel::new(setup.custom_words);

    let text = "I betrayed the bees!";
    let tokenized_text = token::tokenize_sentence(text);
    let tokens: Vec<&str> = tokenized_text.iter().map(|token| &**token).collect();

    let sentiment_description = sentiment.get_term_vector_description(&tokens);
    let description = "stressed";
    assert_eq!(sentiment_description, description);
}

#[test]
fn stems_from_tokenized_sentence() {
    let text = "I betrayed the bees!";
    let tokenized_text = token::tokenize_sentence(text);
    let tokens: Vec<&str> = tokenized_text.iter().map(|token| &**token).collect();

    let stems: Vec<String> = tokens.iter().map(|token| stem::get(token).unwrap()).collect();
    let stems: Vec<&str> = stems.iter().map(|stem| &**stem).collect();
    let stemmed_tokens = vec!["I", "betrai", "the", "bee"];

    assert_eq!(stems, stemmed_tokens);
}
