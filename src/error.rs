//! Error type for when a sentiment term already exists in the sentiment lexicon when adding a new term
//! or for when stemming a word with non-ASCII characters

use thiserror::Error;


#[derive(Error, Debug, PartialEq, Eq)]
pub enum RnltkError {
    /// An existing sentiment term could not be added to the lexicon since it was attempted
    /// without replacement
    #[error("Attempted to add existing key without replacement")]
    SentimentTermExists,
    /// Could not stem a term due to non-ASCII characters
    #[error("Could not stem term due to non-ASCII characters present")]
    StemNonAscii,
    #[error("Value 'k' must fall within 1 <= k <= n, where n is the number of columns in the TF-IDF matrix")]
    LsaOutOfBounds
}