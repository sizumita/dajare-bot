use crate::{Dajare, DajareSearcher};
use std::path::Path;
use futures::StreamExt;
use lindera::tokenizer::{Tokenizer, TokenizerConfig, Token as LinderaToken};
use lindera_core::viterbi::Mode;


#[derive(Debug, Clone, PartialEq)]
pub enum Pos {
    Noun, // 名詞
    Verb, // 動詞
    Particle, // 助詞
    Sign, // 記号
    AuxVerb, // 助動詞
    Interjection, // 感動詞
}


impl Pos {
    pub fn from_string(name: String) -> Self {
        match &*name {
            "名詞" => { Self::Noun }
            "動詞" => { Self::Verb }
            "助詞" => { Self::Particle }
            "記号" => { Self::Sign }
            "助動詞" => { Self::AuxVerb }
            "感動詞" => { Self::Interjection }
            _ => {
                panic!(format!("unexpected name {}", name))
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct Token {
    pub text: String, // 読み方
    pub pos: Pos, // 品詞
    pub sub_pos: Vec<String>, // 品詞細分類
    pub conjugation_type: String, // 活用形
    pub conjugation_form: String, // 活用型
    pub base_form: String, // 原形
    pub reading: String, // 読み方
    pub pronunciation: String, // 発音
}

impl Token {
    pub fn from_lindera_token(token: LinderaToken) -> Option<Self> {
        match token.detail.as_slice() {
            [pos, pos1, pos2, pos3, conjugation_type, conjugation_form, base_form, reading, pronunciation] => {
                Some(
                    Token {
                        text: token.text.to_string(),
                        pos: Pos::from_string(pos.clone()),
                        sub_pos: vec![pos1.clone(), pos2.clone(), pos3.clone()],
                        conjugation_type: conjugation_type.clone(),
                        conjugation_form: conjugation_form.clone(),
                        base_form: base_form.clone(),
                        reading: reading.clone(),
                        pronunciation: pronunciation.clone(),
                    }
                )
            }
            _ => {
                None
            }
        }
    }
}

pub async fn tokenize(tokenizer: &mut Tokenizer, text: &str) -> Vec<Token> {
    tokenizer.tokenize(text).unwrap().into_iter().map(Token::from_lindera_token).filter(
        |x| x.is_some() && x.clone().unwrap().pos != Pos::Sign
    ).map(|x| x.unwrap()).collect()
}

pub async fn search_dajare(searcher: DajareSearcher, text: &str) -> Vec<Dajare> {
    let mut tokenizer = searcher.tokenizer.lock().await;
    let tokens = tokenize(&mut tokenizer, text).await;
    for token in tokens {
        println!("{}: {}", token.text, token.reading)
    }
    return vec![]
}

#[cfg(test)]
mod tests {
    use crate::DajareSearcher;

    #[tokio::test]
    async fn it_works() {
        let s = DajareSearcher::new("./lindera-ipadic");
        super::search_dajare(s, "私の名前はすみどらです。やーい。ああ。").await;
        assert_eq!(2 + 2, 4);
    }
}
