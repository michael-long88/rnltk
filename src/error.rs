#[derive(Debug, PartialEq, Eq)]
pub enum RnltkError {
    SentimentTermExists,
    StemNonAscii
}