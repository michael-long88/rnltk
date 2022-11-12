//! Module containing functions to retrieve sample data for
//! use in the main modules.

use crate::sentiment::CustomWords;
use crate::document::GenericMatrix;
use nalgebra::DMatrix;


pub fn get_sample_custom_word_dict() -> CustomWords {
    let custom_word_dict = r#"
    {
        "abduction": {
            "word": "abduction",
            "stem": "abduct",
            "avg": [2.76, 5.53],
            "std": [2.06, 2.43]
        },
        "betrayed": {
            "word": "betrayed",
            "stem": "betrai",
            "avg": [2.57, 7.24],
            "std": [1.83, 2.06]
        },
        "bees": {
            "word": "bees",
            "stem": "bee",
            "avg": [3.2, 6.51],
            "std": [2.07, 2.14]
        }
    }"#;

    serde_json::from_str(custom_word_dict).unwrap()
}

pub fn get_term_frequencies() -> GenericMatrix {
    DMatrix::from_row_slice(11, 4, &[1., 0., 0., 0.,
        0., 1., 0., 0.,
        0., 0., 1., 1.,
        1., 0., 0., 0.,
        1., 0., 0., 0.,
        2., 0., 0., 0.,
        0., 0., 0., 1.,
        0., 1., 0., 0.,
        0., 0., 0., 1.,
        0., 0., 1., 0.,
        1., 0., 0., 0.,])
}