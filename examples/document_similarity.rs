//! Create a document similarity matrix from four documents

use rnltk::{document, token};
use nalgebra::{DMatrix};


fn main() {
    let document1 = "It is a far, far better thing I do, than I have ever done";
    let document2 = "Call me Ishmael";
    let document3 = "Is this a dagger I see before me?";
    let document4 = "O happy dagger";

    let documents = vec![document1, document2, document3, document4];

    let documents_term_frequencies = token::get_term_frequencies_from_sentences(&documents);

    let mut all_term_frequencies: Vec<f64> = vec![];

    documents_term_frequencies.iter().for_each(|term_frequencies| {
        all_term_frequencies.extend(term_frequencies.values().into_iter());
    });

    let nrows = documents_term_frequencies[0].values().len();
    let ncols = documents.len();

    let document_term_frequencies = DMatrix::from_vec(nrows, ncols, all_term_frequencies);

    let document_term_frequency_matrix = document::DocumentTermFrequencies::new(document_term_frequencies);
    let tfidf_matrix = document_term_frequency_matrix.get_tfidf_from_term_frequencies();

    let cosine_similarity = tfidf_matrix.get_cosine_similarity_from_tfidf();
    let cosine_similarity_matrix = cosine_similarity.get_cosine_similarity_matrix();

    println!("COSINE SIMILARITY MATRIX");
    for row_index in 0..ncols {
        println!(
            "Document {}          {:.2}          {:.2}          {:.2}          {:.2}",
            row_index + 1,
            &cosine_similarity_matrix[(row_index, 0)],
            &cosine_similarity_matrix[(row_index, 1)],
            &cosine_similarity_matrix[(row_index, 2)],
            &cosine_similarity_matrix[(row_index, 3)]
        )
    }
    println!("              Document 1    Document 2    Document 3    Document 4");

    println!("\n-----------------------------\n");

    let lsa_cosine_similarity = tfidf_matrix.get_lsa_cosine_similarity_from_tfidf(2).unwrap();
    let lsa_cosine_similarity_matrix = lsa_cosine_similarity.get_lsa_cosine_similarity_matrix();

    println!("LSA COSINE SIMILARITY MATRIX");
    for row_index in 0..ncols {
        println!(
            "Document {}          {:.2}          {:.2}          {:.2}          {:.2}",
            row_index + 1,
            &lsa_cosine_similarity_matrix[(row_index, 0)],
            &lsa_cosine_similarity_matrix[(row_index, 1)],
            &lsa_cosine_similarity_matrix[(row_index, 2)],
            &lsa_cosine_similarity_matrix[(row_index, 3)]
        )
    }
    println!("              Document 1    Document 2    Document 3    Document 4");
}
