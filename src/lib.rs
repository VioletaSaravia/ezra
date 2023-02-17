use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::error::Error;

// struct BotDB {}

pub struct Ezra {
    pub initial_message: String,
    user_history: Vec<String>,
    substitutions: HashMap<String, String>, // TODO hacer BiMap?
    keywords: HashMap<String, usize>,
    decompositions: HashMap<String, Vec<String>>,
    recompositions: HashMap<String, Vec<String>>,
}

impl Ezra {
    pub fn new(_db: &str) -> Result<Self, Box<dyn Error>> {
        let mut keywords = HashMap::new();
        keywords.insert("I".into(), 0);

        let mut substitutions: HashMap<String, String> = HashMap::new();
        substitutions.insert("i".into(), "you".into());
        substitutions.insert("you".into(), "i".into());
        substitutions.insert("yourself".into(), "myself".into());
        substitutions.insert("myself".into(), "yourself".into());

        let mut decompositions = HashMap::new();
        decompositions.insert(
            "i".to_owned(),
            vec![r"(?P<first>\w*)I are(?P<second>\w*)".to_owned()],
        );

        decompositions.insert("sorry".to_owned(), vec![r"Sorry\*".to_owned()]);

        let mut recompositions = HashMap::new();
        recompositions.insert(
            decompositions["i".into()][0].clone(),
            vec!["What makes you think I am $second".to_owned()],
        );

        recompositions.insert(
            decompositions["sorry".into()][0].clone(),
            vec![
                "Please don't apologize".to_owned(),
                "Apologies are not necessary.".to_owned(),
                "What feeling do you have when you apologize?".to_owned(),
                "I've told you that apologies are not required.".to_owned(),
            ],
        );

        Ok(Ezra {
            initial_message: "What's up".into(),
            substitutions,
            user_history: vec![],
            keywords,
            decompositions,
            recompositions,
        })
    }

    pub fn reply(&mut self, user_reply: String) -> Result<String, Box<dyn Error>> {
        self.user_history.push(user_reply.clone());
        let user_reply = user_reply.to_lowercase();

        let transform = user_reply
            .split_whitespace()
            .map_while(|w| match w {
                // borra x pero no debería
                // va antes
                x if x.ends_with(",") || x.ends_with(".") => None,
                x => self.substitutions.get(x).cloned(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let mut keystock = VecDeque::new();
        let mut precedence = 0 as usize;
        for word in transform.split_whitespace() {
            match self.keywords.get(word).cloned() {
                Some(x) => {
                    // no hace sorting
                    if x > precedence {
                        keystock.push_front(word.to_owned());
                        precedence = x;
                    } else {
                        keystock.push_back(word.to_owned());
                    }
                }
                None => continue,
            }
        }

        for kw in keystock {
            for decomposition in &self.decompositions[&kw] {
                let regex = Regex::new(r"{decomposition})").unwrap();
                if regex.is_match(&transform) {
                    // agregar uno al count de esta recomposicion
                    regex.replace_all(&transform, self.recompositions[decomposition][0].clone());
                    return Ok(transform);
                }
            }
        }

        Err("Pasó algo".into())
    }
}
