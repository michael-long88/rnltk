//! Create and add sentiment lexicon
//! The data* contained in `BRM-emot-submit.csv` can be found at https://link.springer.com/article/10.3758/s13428-012-0314-x
//! After downloading the data, simply update the file path in the below example
//! 
//! \* The data referenced here is only permitted to be used non-commercially

use std::collections::HashMap;
use rnltk::sentiment::{SentimentModel, CustomWords, SentimentDictValue};
use rnltk::stem;


fn main() {
    // lexicon data pulled from https://link.springer.com/article/10.3758/s13428-012-0314-x
    let mut reader = csv::Reader::from_path("examples/BRM-emot-submit.csv").unwrap();
    let mut custom_words: CustomWords = HashMap::new();
    for record in reader.records() {
        let record = record.unwrap();
        let word = record[1].to_owned();
        let stemmed_word = stem::get(&word).unwrap();
        if &word == "betrayal" {
            println!("{:?}", &stemmed_word)
        }
        let avg = vec![record[2].parse::<f64>().unwrap(), record[5].parse::<f64>().unwrap()];
        let std = vec![record[3].parse::<f64>().unwrap(), record[6].parse::<f64>().unwrap()];
        let sentiment_dict = SentimentDictValue::new(word, stemmed_word, avg, std);
        custom_words.insert(record[1].to_owned(), sentiment_dict);
    }
    
    let sentiment = SentimentModel::new(custom_words);
    println!("{:?}", sentiment.get_arousal_for_single_term("abduction"));
}
