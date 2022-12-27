//! Functionality for performing matrix operations on document term frequencies.

use nalgebra::{Matrix, Dynamic, VecStorage};

use crate::error::RnltkError;

pub type GenericMatrix = Matrix<f64, Dynamic, Dynamic, VecStorage<f64, Dynamic, Dynamic>>;

/// Struct for holding the matrix of `document_term_frequencies`
#[derive(Debug, Clone)]
pub struct DocumentTermFrequencies {
    pub document_term_frequencies: GenericMatrix
}

/// Struct for holding the resulting `tfidf_matrix`
/// from [`DocumentTermFrequencies::get_tfidf_from_term_frequencies`]
#[derive(Debug, Clone)]
pub struct TfidfMatrix {
    tfidf_matrix: GenericMatrix
}

/// Struct for holding the resulting `cosine_similarity_matrix`
/// from [`TfidfMatrix::get_cosine_similarity_from_tfidf`]
#[derive(Debug, Clone)]
pub struct CosineSimilarityMatrix {
    cosine_similarity_matrix: GenericMatrix
}

/// Struct for holding the resulting `cosine_similarity_matrix`
/// from [`TfidfMatrix::get_cosine_similarity_from_tfidf`]
#[derive(Debug, Clone)]
pub struct LsaCosineSimilarityMatrix {
    lsa_cosine_similarity_matrix: GenericMatrix
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
    /// let tfidf_matrix = document_term_frequencies.get_tfidf_from_term_frequencies();
    /// ```
    pub fn get_tfidf_from_term_frequencies(&self) -> TfidfMatrix {
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
    /// Gets the TF-IDF matrix that was created from [`DocumentTermFrequencies::get_tfidf_from_term_frequencies`].
    /// 
    /// This ensures the user can't instantiate their own instance of [`TfidfMatrix`] and must use the 
    /// formatted, normalized matrix.
    pub fn get_tfidf_matrix(&self) -> &GenericMatrix {
        &self.tfidf_matrix
    }

    /// Gets the cosine similarity matrix from the [`TfidfMatrix`]'s `tfidf_matrix`.
    /// 
    /// Normally, calculating the cosine similarity of two document vectors would look like
    /// \\(\cos \theta = \frac{D_i \cdot D_j}{|D_i| |D_j|}\\). Since the TF-IDF matrix returned
    /// from [`DocumentTermFrequencies::get_tfidf_from_term_frequencies`] is already normalized, this simplifies
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
    /// let tfidf_matrix = document_term_frequencies.get_tfidf_from_term_frequencies();
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

    /// Gets the Latent Semantic Analysis (LSA) cosine similarity matrix from the [`TfidfMatrix`]'s `tfidf_matrix`.
    /// 
    /// Singular Value Decomposition (SVD) is applied to the \\(m \times n\\) `tfidf_matrix` to reduce dimensionality.
    /// The k largest singular values are chosen to produce a reduced \\({V_k}^T\\) matrix, with 
    /// \\(1 \le v \le n\\). Each document column in the \\({V_k}^T\\) matrix is normalized and then we 
    /// dot product them together. To shift the resulting dot product from a range of [-1...-1] to 
    /// [0...1], we add 1 to the dot product and then divide by 2 (\\(\frac{1 + \cos(\theta)}{2}\\)).
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
    /// let tfidf_matrix = document_term_frequencies.get_tfidf_from_term_frequencies();
    /// let lsa_cosine_similarity_matrix = tfidf_matrix.get_lsa_cosine_similarity_from_tfidf(2).unwrap();
    /// ```
    pub fn get_lsa_cosine_similarity_from_tfidf(&self, k: usize) -> Result<LsaCosineSimilarityMatrix, RnltkError> {
        if k > self.tfidf_matrix.ncols() {
            return Err(RnltkError::LsaOutOfBounds);
        }
        let svd_matrix = self.tfidf_matrix.clone().svd(true, true);
        let mut v_t = svd_matrix.v_t.unwrap();

        let mut v_tk = v_t.slice_mut((0, 0), (k, v_t.ncols()));

        for mut column in v_tk.column_iter_mut() {
            let normalized = column.normalize();
            column.copy_from(&normalized);
        }

        let num_cols = v_tk.ncols();
        let mut lsa_cosine_similarity_matrix: GenericMatrix = GenericMatrix::zeros(num_cols, num_cols);
        for col_index in 0..num_cols {
            for inner_col_index in 0..num_cols {
                if col_index == inner_col_index {
                    lsa_cosine_similarity_matrix[(col_index, inner_col_index)] = 1.
                } else {
                    let mut dot_product = v_tk.column(col_index).dot(&v_tk.column(inner_col_index));
                    if dot_product.is_nan() {
                        dot_product = 0.;
                    }
                    let shifted_dot_product = (dot_product + 1.) / 2.;
                    lsa_cosine_similarity_matrix[(col_index, inner_col_index)] = shifted_dot_product
                }
            }
        }

        Ok(LsaCosineSimilarityMatrix {
            lsa_cosine_similarity_matrix
        })
        
    }
}

impl CosineSimilarityMatrix {
    /// Gets the cosine similarity matrix that was created 
    /// from [`TfidfMatrix::get_cosine_similarity_from_tfidf`].
    /// 
    /// This ensures the user can't instantiate their own instance of [`CosineSimilarityMatrix`] and must use the 
    /// formatted matrix returned from [`TfidfMatrix::get_cosine_similarity_from_tfidf`].
    pub fn get_cosine_similarity_matrix(&self) -> &GenericMatrix {
        &self.cosine_similarity_matrix
    }
}

impl LsaCosineSimilarityMatrix {
    /// Gets the LSA cosine similarity matrix that was created 
    /// from [`TfidfMatrix::get_lsa_cosine_similarity_from_tfidf`].
    /// 
    /// This ensures the user can't instantiate their own instance of [`LsaCosineSimilarityMatrix`] and must use the 
    /// formatted matrix returned from [`TfidfMatrix::get_lsa_cosine_similarity_from_tfidf`].
    pub fn get_lsa_cosine_similarity_matrix(&self) -> &GenericMatrix {
        &self.lsa_cosine_similarity_matrix
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
        let output = document_term_frequencies.get_tfidf_from_term_frequencies();
        assert_eq!(output.tfidf_matrix, tfidf_matrix);
    }

    #[test]
    fn cosine_similarity() {
        let document_term_frequencies: DocumentTermFrequencies = DocumentTermFrequencies::new(sample_data::get_term_frequencies());
        let tfidf_matrix = document_term_frequencies.get_tfidf_from_term_frequencies();
        let cosine_similarity_matrix = DMatrix::from_row_slice(4, 4, &[1., 0., 0., 0.,
                                                                                            0., 1., 0., 0.,
                                                                                            0., 0., 1., 0.149071198499986,
                                                                                            0., 0., 0.149071198499986, 1.,]);
        let output = tfidf_matrix.get_cosine_similarity_from_tfidf();
        assert_eq!(output.cosine_similarity_matrix, cosine_similarity_matrix);
    }

    #[test]
    fn lsa_cosine_similarity() {
        let document_term_frequencies: DocumentTermFrequencies = DocumentTermFrequencies::new(sample_data::get_term_frequencies());
        let tfidf_matrix = document_term_frequencies.get_tfidf_from_term_frequencies();
        let lsa_cosine_similarity_matrix = DMatrix::from_row_slice(4, 4, &[1., 0.5, 0.5, 0.5,
                                                                                            0.5, 1., 0.5, 0.5,
                                                                                            0.5, 0.5, 1., 1.,
                                                                                            0.5, 0.5, 1., 1.,]);
        let output = tfidf_matrix.get_lsa_cosine_similarity_from_tfidf(2).unwrap();
        assert_eq!(output.lsa_cosine_similarity_matrix, lsa_cosine_similarity_matrix);
    }
}