use std::{collections::HashMap, borrow::Cow};
use std::f64::consts::PI;

use serde::{Serialize, Deserialize};

use crate::stem;

type AnewWords = HashMap<String, SentimentDictValue>;
type AnewStems = HashMap<String, SentimentDictValue>;
type HapiWords = HashMap<String, SentimentDictValue>;
type CustomWords = HashMap<String, SentimentDictValue>;
type CustomStems = HashMap<String, SentimentDictValue>;

#[derive(Serialize, Deserialize, Debug)]
pub struct SentimentDictValue {
    dict: Dict,
    word: String,
    stem: String,
    avg: Vec<f64>,
    std: Vec<f64>,
    fq: i64,
}

impl SentimentDictValue {
    pub fn new(dict: Dict, word: String, stem: String, avg: Vec<f64>, std: Vec<f64>, fq: i64) -> Self {
        SentimentDictValue {
            dict,
            word,
            stem,
            avg,
            std,
            fq
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Dict {
    #[serde(rename = "anew")]
    Anew,
    #[serde(rename = "anew-ex")]
    AnewEx,
    #[serde(rename = "happiness")]
    Happiness,
    #[serde(rename = "custom")]
    Custom,
}

fn get_anew_words() -> AnewWords {
    let anew_word_dict = include_str!("../data/anew_word.json");
    let anew_words_sentiment_hashmap: AnewWords = serde_json::from_str(anew_word_dict).unwrap();

    anew_words_sentiment_hashmap
}

fn get_anew_stems() -> AnewStems {
    let anew_stem_dict = include_str!("../data/anew_stem.json");
    let anew_stems_sentiment_hashmap: AnewStems = serde_json::from_str(anew_stem_dict).unwrap();

    anew_stems_sentiment_hashmap
}

fn get_hapi_words() -> HapiWords {
    let hapi_word_dict = include_str!("../data/hapi_word.json");
    let hapi_words_sentiment_hashmap: HapiWords = serde_json::from_str(hapi_word_dict).unwrap();

    hapi_words_sentiment_hashmap
}

pub struct SentimentModel {
    anew_words: AnewWords,
    anew_stems: AnewStems,
    hapi_words: HapiWords,
    custom_words: CustomWords,
    custom_stems: CustomStems,
}

impl Default for SentimentModel {
    fn default() -> Self {
        Self::new()
    }
}

impl SentimentModel {
    pub fn new() -> Self {
        let custom_words_dict = SentimentDictValue::new(Dict::Custom, "".to_string(), "".to_string(), vec![0.0, 0.0], vec![0.0, 0.0], 0);
        let custom_words = HashMap::from([("".to_string(), custom_words_dict)]);
        let custom_stems_dict = SentimentDictValue::new(Dict::Custom, "".to_string(), "".to_string(), vec![0.0, 0.0], vec![0.0, 0.0], 0);
        let custom_stems = HashMap::from([("".to_string(), custom_stems_dict)]);
        
        SentimentModel {
            anew_words: get_anew_words(),
            anew_stems: get_anew_stems(),
            hapi_words: get_hapi_words(),
            custom_words,
            custom_stems,
        }
    }

    /// Checks if a term exists in the sentiment dictionaries
    /// 
    /// # Arguments
    /// 
    /// * `term` - &str representation of the word token 
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// if sentiment.does_term_exist("abduction") {
    ///     println!("abduction exists");
    /// }
    /// ```
    pub fn does_term_exist(&self, term: &str) -> bool {
        self.anew_words.contains_key(term) || self.anew_stems.contains_key(term) || self.hapi_words.contains_key(term) || self.custom_words.contains_key(term) || self.custom_stems.contains_key(term)
    }

    /// Gets the raw arousal values (average, standard deviation) for a given term
    /// 
    /// # Arguments
    /// 
    /// * `term` - &str representation of the word token 
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let arousal = sentiment.get_raw_arousal("abduction");
    /// let correct_arousal = vec![5.53, 2.43];
    /// 
    /// assert_eq!(arousal, correct_arousal);
    /// ```
    pub fn get_raw_arousal(&self, term: &str) -> Vec<f64> {
        let mut average = 0.0;
        let mut std_dev = 0.0; 

        if !self.does_term_exist(term) {
            return vec![average, std_dev];
        } else if self.anew_words.contains_key(term) {
            let sentiment_info = self.anew_words.get(term).unwrap();
            average = sentiment_info.avg[1];
            std_dev = sentiment_info.std[1];
        } else if self.anew_stems.contains_key(term) {
            let sentiment_info = self.anew_stems.get(term).unwrap();
            average = sentiment_info.avg[1];
            std_dev = sentiment_info.std[1];
        } else if self.hapi_words.contains_key(term) {
            let sentiment_info = self.hapi_words.get(term).unwrap();
            average = sentiment_info.avg[1];
            std_dev = sentiment_info.std[1];
        } else if self.custom_words.contains_key(term) {
            let sentiment_info = self.custom_words.get(term).unwrap();
            average = sentiment_info.avg[1];
            std_dev = sentiment_info.std[1];
        } else if self.custom_stems.contains_key(term) {
            let sentiment_info = self.custom_stems.get(term).unwrap();
            average = sentiment_info.avg[1];
            std_dev = sentiment_info.std[1];
        }
        vec![average, std_dev]
    }

    /// Gets the raw valence values (average, standard deviation) for a given term
    /// 
    /// # Arguments
    /// 
    /// * `term` - &str representation of the word token 
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let valence = sentiment.get_raw_valence("abduction");
    /// let correct_valence = vec![2.76, 2.06];
    /// 
    /// assert_eq!(valence, correct_valence);
    /// ```
    pub fn get_raw_valence(&self, term: &str) -> Vec<f64> {
        let mut average = 0.0;
        let mut std_dev = 0.0; 

        if !self.does_term_exist(term) {
            return vec![average, std_dev];
        } else if self.anew_words.contains_key(term) {
            let sentiment_info = self.anew_words.get(term).unwrap();
            average = sentiment_info.avg[0];
            std_dev = sentiment_info.std[0];
        } else if self.anew_stems.contains_key(term) {
            let sentiment_info = self.anew_stems.get(term).unwrap();
            average = sentiment_info.avg[0];
            std_dev = sentiment_info.std[0];
        } else if self.hapi_words.contains_key(term) {
            let sentiment_info = self.hapi_words.get(term).unwrap();
            average = sentiment_info.avg[0];
            std_dev = sentiment_info.std[0];
        } else if self.custom_words.contains_key(term) {
            let sentiment_info = self.custom_words.get(term).unwrap();
            average = sentiment_info.avg[0];
            std_dev = sentiment_info.std[0];
        } else if self.custom_stems.contains_key(term) {
            let sentiment_info = self.custom_stems.get(term).unwrap();
            average = sentiment_info.avg[0];
            std_dev = sentiment_info.std[0];
        }
        vec![average, std_dev]
    }

    /// Gets the arousal value for a given term
    /// 
    /// # Arguments
    /// 
    /// * `term` - &str representation of the word token 
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let arousal = sentiment.get_arousal_for_single_term("abduction");
    /// let correct_arousal = 5.53;
    /// 
    /// assert_eq!(arousal, correct_arousal);
    /// ```
    pub fn get_arousal_for_single_term(&self, term: &str) -> f64 {
        self.get_raw_arousal(term)[0]
    }

    /// Gets the valence value for a given term
    /// 
    /// # Arguments
    /// 
    /// * `term` - &str representation of the word token 
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let valence = sentiment.get_valence_for_single_term("abduction");
    /// let correct_valence = 2.76;
    /// 
    /// assert_eq!(valence, correct_valence);
    /// ```
    pub fn get_valence_for_single_term(&self, term: &str) -> f64 {
        self.get_raw_valence(term)[0]
    }

    /// Gets the arousal value for a given vector of terms
    /// 
    /// # Arguments
    /// 
    /// * `terms` - &Vec<&str> representation of the word tokens
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let arousal = sentiment.get_arousal_for_term_vector(&vec!["I", "betrayed", "the", "bees"]);
    /// let correct_arousal = 6.881952380952381;
    /// 
    /// assert_eq!(arousal, correct_arousal);
    /// ```
    pub fn get_arousal_for_term_vector(&self, terms: &Vec<&str>) -> f64 {
        let c = 2.0 * PI;
        let mut prob: Vec<f64> = vec![];
        let mut prob_sum = 0.0;
        let mut arousal_means: Vec<f64> = vec![];

        for term in terms {
            if self.does_term_exist(term) {
                let raw_arousal = self.get_raw_arousal(term);
                
                let p = 1.0 / (c * raw_arousal[1].powi(2)).sqrt();
                prob.push(p);
                prob_sum += p;

                arousal_means.push(raw_arousal[0]);
            }
        }
        let mut arousal = 0.0;
        for index in 0..arousal_means.len() {
            arousal += prob[index] / prob_sum * arousal_means[index];
        }

        arousal
    }

    /// Gets the valence value for a given vector of terms
    /// 
    /// # Arguments
    /// 
    /// * `terms` - &Vec<&str> representation of the word tokens
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let valence = sentiment.get_valence_for_term_vector(&vec!["I", "betrayed", "the", "bees"]);
    /// let correct_valence = 2.865615384615385;
    /// 
    /// assert_eq!(valence, correct_valence);
    /// ```
    pub fn get_valence_for_term_vector(&self, terms: &Vec<&str>) -> f64 {
        let c = 2.0 * PI;
        let mut prob: Vec<f64> = vec![];
        let mut prob_sum = 0.0;
        let mut valence_means: Vec<f64> = vec![];

        for term in terms {
            if self.does_term_exist(term) {
                let raw_valence = self.get_raw_valence(term);
                
                let p = 1.0 / (c * raw_valence[1].powi(2)).sqrt();
                prob.push(p);
                prob_sum += p;

                valence_means.push(raw_valence[0]);
            }
        }
        let mut valence = 0.0;
        for index in 0..valence_means.len() {
            valence += prob[index] / prob_sum * valence_means[index];
        }

        valence
    }

    /// Gets the valence, arousal sentiment for a term
    /// 
    /// # Arguments
    /// 
    /// * `term` - &str representation of the word token
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let sentiment_info = sentiment.get_sentiment_for_term("abduction");
    /// let sentiment_map = HashMap::from([("valence", 2.76), ("arousal", 5.53)]);
    /// 
    /// assert_eq!(sentiment_info, sentiment_map);
    /// ```
    pub fn get_sentiment_for_term(&self, term: &str) -> HashMap<&str, f64> {
        let mut sentiment: HashMap<&str, f64>  = HashMap::new();
        sentiment.insert("valence", self.get_valence_for_single_term(term));
        sentiment.insert("arousal", self.get_arousal_for_single_term(term));

        sentiment
    }

    /// Gets the valence, arousal sentiment for a vector of terms
    /// 
    /// # Arguments
    /// 
    /// * `terms` - &Vec<&str> representation of the word tokens
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let sentiment_info = sentiment.get_sentiment_for_term_vector(&vec!["I", "betrayed", "the", "bees"]);
    /// let sentiment_map = HashMap::from([("valence", 2.865615384615385), ("arousal", 6.881952380952381)]);
    /// 
    /// assert_eq!(sentiment_info, sentiment_map);
    /// ```
    pub fn get_sentiment_for_term_vector(&self, terms: &Vec<&str>) -> HashMap<&str, f64> {
        let mut sentiment: HashMap<&str, f64>  = HashMap::new();
        sentiment.insert("valence", self.get_valence_for_term_vector(terms));
        sentiment.insert("arousal", self.get_arousal_for_term_vector(terms));

        sentiment
    }

    /// Gets the Russel-like description given a valence and arousal score
    /// 
    /// # Arguments
    /// 
    /// * `valence` - &f64 valence score
    /// * `arousal` - &f64 arousal score
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let sentiment_description = sentiment.get_sentiment_description(&2.76, &5.53);
    /// let description = "upset";
    /// 
    /// assert_eq!(sentiment_description, description);
    /// ```
    pub fn get_sentiment_description(&self, valence: &f64, arousal: &f64) -> Cow<'static, str> {
        if !(1.0..=9.0).contains(valence) || !(1.0..=9.0).contains(arousal) {
            println!("Valence and arousal must be bound between 1 and 9 (inclusive)");
            return Cow::from("unknown");
        } 

        // Center of circumplex (5,5) will give an r=0, div by zero error, so handle explicitly
        if *valence == 5.0 && *arousal == 5.0 {
            return Cow::from("average");
        }

        // Angular cutoffs for different emotional states (same on top and bottom)
        let angular_cutoffs = vec![0.0, 18.43, 45.0, 71.57, 90.0, 108.43, 135.0, 161.57, 180.0];

        // Terms to return for bottom, top half of circumplex
        let lower_term = vec![
            "contented", "serene", "relaxed", "calm",
            "bored", "lethargic", "depressed", "sad"
        ];
        let upper_term = vec![
            "happy", "elated", "excited", "alert",
            "tense", "nervous", "stressed", "upset"
        ];

        // Normalize valence and arousal, use polar coordinates to get angle
        // clockwise along bottom, counterclockwise along top
        let normalized_valence = ((valence - 1.0) - 4.0) / 4.0;
        let normalized_arousal = ((arousal - 1.0) - 4.0) / 4.0;
        let mut radius = (normalized_valence.powi(2).abs() + normalized_arousal.powi(2).abs()).sqrt();
        let direction = (normalized_valence / radius).acos().to_degrees();

        //  Normalize radius for "strength" of emotion
        if direction <= 45.0 || direction >= 135.0 {
            radius /= (normalized_arousal.powi(2).abs() + 1.0).sqrt();
        } else {
            radius /= (normalized_valence.powi(2).abs() + 1.0).sqrt();
        }

        let mut modify = "";
        
        if radius <= 0.25 {
            modify = "slightly ";
        } else if radius <= 0.5 {
            modify = "moderately ";
        } else if radius > 0.75 {
            modify = "very ";
        }

        // Use normalized arousal to determine if we're on bottom of top of circumplex
        let mut term = lower_term;
        if normalized_arousal > 0.0 {
            term = upper_term;
        }

        let description;

        // Walk along angular boundaries until we determine which "slice"
        // our valence and arousal point lies in, return corresponding term
        for index in 0..term.len() {
            if direction >= angular_cutoffs[index] && direction <= angular_cutoffs[index + 1] {
                description = format!("{}{}", modify, term[index]);
                return Cow::from(description);
            }
        }

        println!("unexpected angle {} did not match any term", normalized_arousal);
        Cow::from("unknown")
    }

    /// Gets the Russel-like description given a term
    /// 
    /// # Arguments
    /// 
    /// * `term` - &str representation of the word token
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let sentiment_description = sentiment.get_term_description("abduction");
    /// let description = "upset";
    /// 
    /// assert_eq!(sentiment_description, description);
    /// ```
    pub fn get_term_description(&self, term: &str) -> Cow<'static, str> {
        let sentiment = self.get_sentiment_for_term(term);
        if sentiment.get("arousal").unwrap() == &0.0 {
            return Cow::from("unknown");
        }
        self.get_sentiment_description(sentiment.get("valence").unwrap(), sentiment.get("arousal").unwrap())
    }

    /// Gets the Russel-like description given a vector of terms
    /// 
    /// # Arguments
    /// 
    /// * `terms` - &Vec<&str> representation of the word tokens
    ///
    /// # Examples
    ///
    /// ```
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let sentiment = SentimentModel::new();
    /// let sentiment_description = sentiment.get_term_vector_description(&vec!["I", "betrayed", "the", "bees"]);
    /// let description = "stressed";
    /// 
    /// assert_eq!(sentiment_description, description);
    /// ```
    pub fn get_term_vector_description(&self, terms: &Vec<&str>) -> Cow<'static, str> {
        let sentiment = self.get_sentiment_for_term_vector(terms);
        if sentiment.get("arousal").unwrap() == &0.0 {
            return Cow::from("unknown");
        }
        self.get_sentiment_description(sentiment.get("valence").unwrap(), sentiment.get("arousal").unwrap())
    }

    /// Adds a new term to the sentiment lexicons. If the term does not already exist, it 
    /// will be added to the custom sentiment lexicon.
    /// 
    /// # Arguments
    /// 
    /// * `term` - &str representation of the word token
    /// * `valence` - &f64 valence value
    /// * `arousal` - &f64 arousal value
    /// 
    /// # Errors
    /// 
    /// If the term already exists, an error variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let mut sentiment = SentimentModel::new();
    /// let sentiment_return_value = sentiment.add_term_without_replacement("squanch", &2.0, &8.5);
    /// match sentiment_return_value {
    ///     Ok(_) => {
    ///         let sentiment_info = sentiment.get_sentiment_for_term("squanch");
    ///         let sentiment_map = HashMap::from([("valence", 2.0), ("arousal", 8.5)]);
    /// 
    ///         assert_eq!(sentiment_info, sentiment_map);
    ///     },
    ///     Err(error_msg) => assert_eq!(error_msg, "Term already exists"),
    /// }
    /// ```
    pub fn add_term_without_replacement(&mut self, term: &'static str, valence: &f64, arousal: &f64) -> Result<(), Cow<'static, str>>{
        if self.does_term_exist(term) {
            return Err(Cow::from("Term already exists"));
        } else {
            let stemmed_word = stem::get(term);
            match stemmed_word {
                Ok(stem) => {
                    let dict = Dict::Custom;
                    let word = term.to_string();
                    let stemmed_word = stem.clone();
                    let avg = vec![*valence, *arousal];
                    let std = vec![1.0, 1.0];
                    let fq = 1;
                    let word_dict_value = SentimentDictValue {
                        dict,
                        word,
                        stem: stemmed_word,
                        avg,
                        std,
                        fq
                    };
                    let dict = Dict::Custom;
                    let word = stem.clone();
                    let stem = stem;
                    let avg = vec![*valence, *arousal];
                    let std = vec![1.0, 1.0];
                    let fq = 1;
                    let stem_dict_value = SentimentDictValue {
                        dict,
                        word,
                        stem,
                        avg,
                        std,
                        fq
                    };
                    self.custom_words.insert(term.to_string(), word_dict_value);
                    self.custom_stems.insert(term.to_string(), stem_dict_value);
                },
                Err(error_msg) => {
                    return Err(Cow::from(error_msg));
                },
            }
        }
        Ok(())
    }
    
    /// Adds a new term to the sentiment lexicons. If this terms already exists, the term will be updated
    /// with the new valence and arousal values. If the term does not already exist, the term will be
    /// stemmed and added to the custom sentiment lexicon. 
    /// 
    /// # Arguments
    /// 
    /// * `term` - &str representation of the word token
    /// * `valence` - &f64 valence value
    /// * `arousal` - &f64 arousal value
    ///
    /// # Errors
    /// 
    /// In the event that the term being stemmed contains non-ASCII characters (like hopè), an error will be returned.
    /// 
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use rnltk::sentiment::SentimentModel;
    /// 
    /// let mut sentiment = SentimentModel::new();
    /// let sentiment_return_value = sentiment.add_term_with_replacement("abduction", &8.0, &8.5);
    /// match sentiment_return_value {
    ///     Ok(_) => {
    ///         let sentiment_info = sentiment.get_sentiment_for_term("abduction");
    ///         let sentiment_map = HashMap::from([("valence", 8.0), ("arousal", 8.5)]);
    /// 
    ///         assert_eq!(sentiment_info, sentiment_map);
    ///     },
    ///     Err(error_msg) => assert_eq!(error_msg, "Only supports English words with ASCII characters"),
    /// }
    /// ```
    pub fn add_term_with_replacement(&mut self, term: &'static str, valence: &f64, arousal: &f64) -> Result<(), Cow<'static, str>>{
        if self.anew_words.contains_key(term) {
            let dict_value = self.anew_words.get_mut(term).unwrap();
            dict_value.avg[0] = *valence;
            dict_value.avg[1] = *arousal;
        } else if self.anew_stems.contains_key(term) {
            let dict_value = self.anew_stems.get_mut(term).unwrap();
            dict_value.avg[0] = *valence;
            dict_value.avg[1] = *arousal;
        } else if self.hapi_words.contains_key(term) {
            let dict_value = self.hapi_words.get_mut(term).unwrap();
            dict_value.avg[0] = *valence;
            dict_value.avg[1] = *arousal;
        } else if self.custom_words.contains_key(term) {
            let dict_value = self.custom_words.get_mut(term).unwrap();
            dict_value.avg[0] = *valence;
            dict_value.avg[1] = *arousal;
        } else if self.custom_stems.contains_key(term) {
            let dict_value = self.custom_stems.get_mut(term).unwrap();
            dict_value.avg[0] = *valence;
            dict_value.avg[1] = *arousal;
        } else {
            let stemmed_word = stem::get(term);
            match stemmed_word {
                Ok(stem) => {
                    let dict = Dict::Custom;
                    let word = term.to_string();
                    let stemmed_word = stem.clone();
                    let avg = vec![*valence, *arousal];
                    let std = vec![1.0, 1.0];
                    let fq = 1;
                    let word_dict_value = SentimentDictValue {
                        dict,
                        word,
                        stem: stemmed_word,
                        avg,
                        std,
                        fq
                    };
                    let dict = Dict::Custom;
                    let word = stem.clone();
                    let stem = stem;
                    let avg = vec![*valence, *arousal];
                    let std = vec![1.0, 1.0];
                    let fq = 1;
                    let stem_dict_value = SentimentDictValue {
                        dict,
                        word,
                        stem,
                        avg,
                        std,
                        fq
                    };
                    self.custom_words.insert(term.to_string(), word_dict_value);
                    self.custom_stems.insert(term.to_string(), stem_dict_value);
                },
                Err(error_msg) => {
                    return Err(Cow::from(error_msg));
                },
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anew_words() {
        let anew_words = get_anew_words();

        let mut stemmed_word = "";
        if let Some(sentiment_info) = anew_words.get("abduction") {
            stemmed_word = &sentiment_info.stem;
        };

        assert_eq!(stemmed_word, "abduct");
    }

    #[test]
    fn anew_stems() {
        let anew_stems = get_anew_stems();

        let mut full_word = "";
        if let Some(sentiment_info) = anew_stems.get("abduct") {
            full_word = &sentiment_info.word;
        };

        assert_eq!(full_word, "abduction");
    }

    #[test]
    fn hapi_words() {
        let hapi_words = get_hapi_words();

        let mut stemmed_word = "";
        if let Some(sentiment_info) = hapi_words.get("laughs") {
            stemmed_word = &sentiment_info.stem;
        };

        assert_eq!(stemmed_word, "laugh");
    }

    #[test]
    fn raw_arousal() {
        let sentiment = SentimentModel::new();
        let arousal = sentiment.get_raw_arousal("abduction");
        let correct_arousal = vec![5.53, 2.43];

        assert_eq!(arousal, correct_arousal);
    }

    #[test]
    fn raw_valence() {
        let sentiment = SentimentModel::new();
        let valence = sentiment.get_raw_valence("abduction");
        let correct_valence = vec![2.76, 2.06];

        assert_eq!(valence, correct_valence);
    }

    #[test]
    fn valence() {
        let sentiment = SentimentModel::new();
        let valence = sentiment.get_valence_for_single_term("abduction");
        let correct_valence = 2.76;

        assert_eq!(valence, correct_valence);
    }

    #[test]
    fn arousal() {
        let sentiment = SentimentModel::new();
        let arousal = sentiment.get_arousal_for_single_term("abduction");
        let correct_arousal = 5.53;

        assert_eq!(arousal, correct_arousal);
    }
    
    #[test]
    fn arousal_vector() {
        let sentiment = SentimentModel::new();
        let arousal = sentiment.get_arousal_for_term_vector(&vec!["I", "betrayed", "the", "bees"]);
        let correct_arousal = 6.881952380952381;

        assert_eq!(arousal, correct_arousal);
    }

    #[test]
    fn valence_vector() {
        let sentiment = SentimentModel::new();
        let valence = sentiment.get_valence_for_term_vector(&vec!["I", "betrayed", "the", "bees"]);
        let correct_valence = 2.865615384615385;

        assert_eq!(valence, correct_valence);
    }

    #[test]
    fn term_sentiment() {
        let sentiment = SentimentModel::new();
        let sentiment_info = sentiment.get_sentiment_for_term("abduction");
        let sentiment_map = HashMap::from([("valence", 2.76), ("arousal", 5.53)]);

        assert_eq!(sentiment_info, sentiment_map);
    }

    #[test]
    fn term_vector_sentiment() {
        let sentiment = SentimentModel::new();
        let sentiment_info = sentiment.get_sentiment_for_term_vector(&vec!["I", "betrayed", "the", "bees"]);
        let sentiment_map = HashMap::from([("valence", 2.865615384615385), ("arousal", 6.881952380952381)]);

        assert_eq!(sentiment_info, sentiment_map);
    }

    #[test]
    fn sentiment_description() {
        let sentiment = SentimentModel::new();
        let sentiment_description = sentiment.get_sentiment_description(&2.76, &5.53);
        let description = "upset";

        assert_eq!(sentiment_description, description);
    }

    #[test]
    fn term_description() {
        let sentiment = SentimentModel::new();
        let sentiment_description = sentiment.get_term_description("abduction");
        let description = "upset";

        assert_eq!(sentiment_description, description);
    }

    #[test]
    fn term_vector_description() {
        let sentiment = SentimentModel::new();
        let sentiment_description = sentiment.get_term_vector_description(&vec!["I", "betrayed", "the", "bees"]);
        let description = "stressed";

        assert_eq!(sentiment_description, description);
    }

    #[test]
    fn replace_term() {
        let mut sentiment = SentimentModel::new();
        sentiment.add_term_with_replacement("abduction", &8.0, &8.5).unwrap();
        let sentiment_info = sentiment.get_sentiment_for_term("abduction");
        let sentiment_map = HashMap::from([("valence", 8.0), ("arousal", 8.5)]);

        assert_eq!(sentiment_info, sentiment_map);
    }

    #[test]
    fn non_ascii_error_replace_term() {
        let mut sentiment = SentimentModel::new();
        let add_sentiment_error = sentiment.add_term_with_replacement("hopè", &8.0, &8.5).unwrap_err();
        assert_eq!(add_sentiment_error, "Only supports English words with ASCII characters");
    }

    #[test]
    fn term_exists_error() {
        let mut sentiment = SentimentModel::new();
        let add_sentiment_error = sentiment.add_term_without_replacement("abduction", &8.0, &8.5).unwrap_err();
        assert_eq!(add_sentiment_error, "Term already exists");
    }

    #[test]
    fn add_term() {
        let mut sentiment = SentimentModel::new();
        sentiment.add_term_without_replacement("squanch", &2.0, &8.5).unwrap();
        let sentiment_info = sentiment.get_sentiment_for_term("squanch");
        let sentiment_map = HashMap::from([("valence", 2.0), ("arousal", 8.5)]);

        assert_eq!(sentiment_info, sentiment_map);
    }

}
