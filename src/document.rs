//! Functionality for performing matrix operations on document term frequencies.

use nalgebra::{Matrix, Dynamic, VecStorage};

pub type GenericMatrix = Matrix<f64, Dynamic, Dynamic, VecStorage<f64, Dynamic, Dynamic>>;

/// Struct for holding the matrix of `document_term_frequencies`
pub struct DocumentTermFrequencies {
    pub document_term_frequencies: GenericMatrix
}

/// Struct for holding the resulting `tfidf_matrix`
/// from [`DocumentTermFrequencies::get_tfidf`]
pub struct TfidfMatrix {
    pub tfidf_matrix: GenericMatrix
}

/// Struct for holding the resulting `cosine_similarity_matrix`
/// from [`TfidfMatrix::get_cosine_similarity_from_tfidf`]
pub struct CosineSimilarityMatrix {
    pub cosine_similarity_matrix: GenericMatrix
}

impl DocumentTermFrequencies {
    /// Creates new instance of DocumentTermFrequencies from a [`DMatrix`].
    /// 
    /// [`DMatrix`]: nalgebra::DMatrix
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::document::DocumentTermFrequencies;
    /// use nalgebra::DMatrix;
    /// 
    /// let term_frequencies = DMatrix::from_row_slice(11, 4, &[1., 0., 0., 0.,
    ///     0., 1., 0., 0.,
    ///     0., 0., 1., 1.,
    ///     1., 0., 0., 0.,
    ///     1., 0., 0., 0.,
    ///     2., 0., 0., 0.,
    ///     0., 0., 0., 1.,
    ///     0., 1., 0., 0.,
    ///     0., 0., 0., 1.,
    ///     0., 0., 1., 0.,
    ///     1., 0., 0., 0.,]);
    /// 
    /// let document_term_frequencies: DocumentTermFrequencies = DocumentTermFrequencies::new(term_frequencies);
    /// ```
    pub fn new(document_term_frequencies: GenericMatrix) -> Self {
        DocumentTermFrequencies {
            document_term_frequencies
        }
    }

    /// Gets the Term Frequency–Inverse Document Frequency (TF-IDF) matrix of the 
    /// [`DocumentTermFrequencies`]'s `document_term_frequencies`.
    /// 
    /// Creating a TF-IDF matrix takes place over two steps. 
    /// The first step is applying a weight, \\(w_{i,j}\\), for every term, \\(t_i\\), 
    /// in the document, \\(D_j\\). \\(w_{i,j}\\) is defined as \\(tf_{i,j} \times idf_i\\), 
    /// where \\(tf_{i,j}\\) is the number of occurrences of \\(t_i\\) in \\(D_j\\), and 
    /// \\(idf_i\\) is the log of inverse fraction of documents \\(n_i\\) that contain at least one 
    /// occurrence of \\(t_i, idf_i = ln(n / n_i)\\).
    /// The second step takes the weighted matrix and then normalizes each document vector in order
    /// to remove the influence of document length.
    ///
    /// # Examples
    /// 
    /// ```
    /// use rnltk::document::DocumentTermFrequencies;
    /// use rnltk::sample_data;
    /// 
    /// let document_term_frequencies: DocumentTermFrequencies = DocumentTermFrequencies::new(sample_data::get_term_frequencies());
    /// let tfidf_matrix = document_term_frequencies.get_tfidf();
    /// ```
    pub fn get_tfidf(&self) -> TfidfMatrix {
        let mut document_term_frequencies = self.document_term_frequencies.clone();
        for row_index in 0..document_term_frequencies.nrows() {
            let term_count: f64 = document_term_frequencies.row(row_index).iter().fold(0., |acc, frequency| {
                if frequency > &0. {
                    acc + 1.
                } else {
                    acc
                }
            });
            for col_index in 0..document_term_frequencies.ncols() {
                let term_frequency = &document_term_frequencies[(row_index, col_index)];
                let inverse_document_frequency = (document_term_frequencies.ncols() as f64 / term_count).ln();
                document_term_frequencies[(row_index, col_index)] = term_frequency * inverse_document_frequency;
            }
        }
    
        for mut column in document_term_frequencies.column_iter_mut() {
            let normalized = column.normalize();
            column.copy_from(&normalized);
        }
    
        TfidfMatrix {
            tfidf_matrix: document_term_frequencies
        }
    }
}

impl TfidfMatrix {
    /// Gets the cosine similarity matrix from the [`TfidfMatrix`]'s `tfidf_matrix`.
    /// 
    /// Normally, calculating the cosine similarity of two document vectors would look like
    /// \\(\cos \theta = \frac{D_i \cdot D_j}{|D_i| |D_j|}\\). Since the TF-IDF matrix returned
    /// from [`DocumentTermFrequencies::get_tfidf`] is already normalized, this simplifies
    /// to \\(\cos \theta = D_i \cdot D_j\\). 
    /// 
    /// The resulting matrix has 1's along the diagonal since the similarity of a document
    /// with itself is 1. The intersections of rows and columns, \\(M_{i,j}\\), is the cosine 
    /// similarity value between \\(D_i\\) and \\(D_j\\).
    ///
    /// # Examples
    /// 
    /// ```
    /// use rnltk::document::DocumentTermFrequencies;
    /// use rnltk::sample_data;
    /// 
    /// let document_term_frequencies: DocumentTermFrequencies = DocumentTermFrequencies::new(sample_data::get_term_frequencies());
    /// let tfidf_matrix = document_term_frequencies.get_tfidf();
    /// let cosine_similarity_matrix = tfidf_matrix.get_cosine_similarity_from_tfidf();
    /// ```
    pub fn get_cosine_similarity_from_tfidf(&self) -> CosineSimilarityMatrix {
        let num_cols = self.tfidf_matrix.ncols();
        let mut cosine_similarity_matrix: GenericMatrix = GenericMatrix::zeros(num_cols, num_cols);
        for col_index in 0..num_cols {
            for inner_col_index in 0..num_cols {
                if col_index == inner_col_index {
                    cosine_similarity_matrix[(col_index, inner_col_index)] = 1.
                } else {
                    let dot_product = self.tfidf_matrix.column(col_index).dot(&self.tfidf_matrix.column(inner_col_index));
                    cosine_similarity_matrix[(col_index, inner_col_index)] = dot_product
                }
            }
        }
    
        CosineSimilarityMatrix {
            cosine_similarity_matrix
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_1_SQRT_2;
    use nalgebra::{DMatrix};
    use crate::sample_data;
    
    #[test]
    fn tfidf() {
        let document_term_frequencies: DocumentTermFrequencies = DocumentTermFrequencies::new(sample_data::get_term_frequencies());
        let tfidf_matrix= DMatrix::from_row_slice(11, 4, &[0.3535533905932738, 0., 0., 0.,
                                                                            0., FRAC_1_SQRT_2, 0., 0.,
                                                                            0., 0., 0.447213595499958, 0.33333333333333337,
                                                                            0.3535533905932738, 0., 0., 0.,
                                                                            0.3535533905932738, 0., 0., 0.,
                                                                            FRAC_1_SQRT_2, 0., 0., 0.,
                                                                            0., 0., 0., 0.6666666666666667,
                                                                            0., FRAC_1_SQRT_2, 0., 0.,
                                                                            0., 0., 0., 0.6666666666666667,
                                                                            0., 0., 0.894427190999916, 0.,
                                                                            0.3535533905932738, 0., 0., 0.,]);
        let output = document_term_frequencies.get_tfidf();
        assert_eq!(output.tfidf_matrix, tfidf_matrix);
    }

    #[test]
    fn cosine_similarity() {
        let document_term_frequencies: DocumentTermFrequencies = DocumentTermFrequencies::new(sample_data::get_term_frequencies());
        let tfidf_matrix = document_term_frequencies.get_tfidf();
        let cosine_similarity_matrix = DMatrix::from_row_slice(4, 4, &[1., 0., 0., 0.,
                                                                                            0., 1., 0., 0.,
                                                                                            0., 0., 1., 0.149071198499986,
                                                                                            0., 0., 0.149071198499986, 1.,]);
        let output = tfidf_matrix.get_cosine_similarity_from_tfidf();
        assert_eq!(output.cosine_similarity_matrix, cosine_similarity_matrix);
    }
}