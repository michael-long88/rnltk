//! Error type for when a sentiment term already exists in the sentiment lexicon when adding a new term
//! or for when stemming a word with non-ASCII characters

#[derive(Debug, PartialEq, Eq)]
pub enum RnltkError {
    /// An existing sentiment term could not be added to the lexicon since it was attempted
    /// without replacement
    SentimentTermExists,
    /// Could not stem a term due to non-ASCII characters
    StemNonAscii
}