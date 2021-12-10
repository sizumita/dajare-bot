pub mod rulebase;
use std::path::Path;
use std::sync::Arc;
use futures::lock::Mutex;

use lindera::tokenizer::{Tokenizer, TokenizerConfig};
use lindera_core::viterbi::Mode;


pub struct Dajare {

}

pub struct DajareSearcher {
    pub(crate) tokenizer: Arc<Mutex<Tokenizer>>
}

impl DajareSearcher {
    pub fn new(dic_path: &str) -> Self {
        let config = TokenizerConfig {
            dict_path: Some(&Path::new(dic_path)),
            mode: Mode::Normal,
            ..TokenizerConfig::default()
        };
        return DajareSearcher {
            tokenizer: Arc::new(Mutex::new(Tokenizer::with_config(config).unwrap()))
        }
    }
}


pub fn tokenize(text: &str) -> () {
    let config = TokenizerConfig {
        dict_path: Some(&Path::new("./lindera-ipadic")),
        mode: Mode::Normal,
        ..TokenizerConfig::default()
    };
    let mut tokenizer = Tokenizer::with_config(config).unwrap();
    let tokens = tokenizer.tokenize(text).unwrap();
    for token in tokens {
        println!("{}", token.text);
    }
}

// pub fn search_dajare(text: &str) -> Vec<Dajare> {
//     return vec![]
// }
