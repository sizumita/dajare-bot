use std::path::Path;

use lindera::tokenizer::{Tokenizer, TokenizerConfig};
use lindera_core::viterbi::Mode;


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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
