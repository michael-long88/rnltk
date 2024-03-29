//! # Natural Language Toolkit for Rust
//! 
//! This project aims to create a general tooklit for natural language processing, a current deficiency in the Rust ecosystem.
//! 
//! Right now, RNLTK supports basic functionality for tokenization, stemming, and sentiment analysis. The goal is to eventually 
//! incorporate topic clustering, term-document frequency matrices, and sentiment negation.
//! 
//! ## Getting Started
//!
//! To start using RNLTK simply add the following to your Cargo.toml file:
//! ```toml
//! [dependencies]
//! rnltk = "0.4.0"
//! ```
//! 
//! While this project provides the basic framework for natural language processing, it does require you to provide
//! your own sentiment lexicon. While this may seem like a disadvantage at first, it allows for much greater flexibilty
//! since you aren't constrained to any sentiment terms the project provides. This was also a decision that was made
//! after finding a lexicon that could be used non-commercially, but required licensing for commercial use. Since 
//! this project was designed to be open source, I decided against including it in order to maintain proper licensing 
//! across the project. That being said, RNLTK does expect user-provided lexicons to follow a specific format since 
//! the sentiment analysis is based on arousal and valence levels.
//! 
//! \* For anyone interested in a sentiment lexicon for non-commercial use, please check out the work of 
//! [Warriner et al., 2013](https://link.springer.com/article/10.3758/s13428-012-0314-x) for an expanded ANEW lexicon. The
//! data can be found in the "Electronic supplementary material" section of the paper. 
//! 
//! For example, this code takes a JSON string and then creates a [`CustomWords`] type from that serialized data. The CustomWords
//! type is then used to instantiate the [`SentimentModel`] struct.
//! 
//! [`CustomWords`]: ./sentiment/type.CustomWords.html
//! [`SentimentModel`]: ./sentiment/struct.SentimentModel.html
//! ```
//! use rnltk::sentiment::{SentimentModel, CustomWords};
//! 
//! let custom_word_dict = r#"
//! {
//!     "abduction": {
//!         "word": "abduction",
//!         "stem": "abduct",
//!         "avg": [2.76, 5.53],
//!         "std": [2.06, 2.43]
//!     }
//! }"#;
//! 
//! let custom_words_sentiment_hashmap: CustomWords = serde_json::from_str(custom_word_dict).unwrap();
//! 
//! let sentiment = SentimentModel::new(custom_words_sentiment_hashmap);
//! ```
//! 
//! Checkout the examples folder in the github project repository for more comprehensive examples.
//! 

pub mod token;
pub mod sentiment;
pub mod stem;
pub mod error;
pub mod sample_data;
pub mod document;