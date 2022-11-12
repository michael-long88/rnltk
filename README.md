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

## TF-IDF
Term frequency–inverse document frequency (TF-IDF) is an algorithm used to find document similarity. Creating a TF-IDF matrix takes place over two steps:
1. Apply a weight, $w_{i,j}$, for every term, $t_i$, in the document, $D_j$. $w_{i,j}$ is defined as $tf_{i,j} \times idf_i$, where $tf_{i,j}$ is the number of occurrences of $t_i$ in $D_j$, and $idf_i$ is the log of inverse fraction of documents $n_i$ that contain at least one occurrence of $t_i, idf_i = ln(\frac{n}{n_i})$.
1. Take the weighted matrix and then normalize each document vector in order to remove the influence of document length.

The weighted, normalized matrix can then be used to find the cosine similarity between documents. 
Normally, calculating the cosine similarity of two document vectors would look like $\cos \theta = \frac{D_i \cdot D_j}{|D_i| |D_j|}$. Since the matrix is already normalized, this simplifies to $\cos \theta = D_i \cdot D_j$. 

The resulting $MxM$ matrix, where $M$ is the number of columns from the TF-IDF matrix, has 1's along the diagonal since the similarity of a document with itself is 1. The intersections of rows and columns, $M_{i,j}$, is the cosine similarity value between $D_i$ and $D_j$.

## Roadmap
* article summary (based on term frequency)
* topic clustering
* term-document frequency matrices
* sentiment negation