# rnltk
This crate is designed to create a general tooklit for natural language processing, a current deficiency in the Rust ecosystem.  

Project can be found on [crates.io](https://crates.io/crates/rnltk).

## Examples
Check out the examples folder to see how to create a sentiment lexicon and get the arousal level for a term.

## Sentiment
The sentiment analysis was originally designed by [Dr. Christopher Healey](https://www.csc.ncsu.edu/people/healey) and then ported
to Rust for the purpose of this project.

## Token
Basic tokenization is supported right now (string to sentences, string to tokens, term frequencies), but there are plans to expand 
this to include stop word removal as well.

## Stem
Stemming currently uses modified code from [rust-stem](https://github.com/minhnhdo/rust-stem), but this may switch to the [rust-stemmers](https://crates.io/crates/rust-stemmers) crate after further research.

More information on the stemming algorithm can be found [here](https://tartarus.org/martin/PorterStemmer/).

## Roadmap
* article summary (based on term frequency)
* topic clustering
* term-document frequency matrices
* sentiment negation