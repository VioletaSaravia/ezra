use rusqlite::{Connection, Result};
use std::error::Error;

pub struct Ezra {
    db: String,
    pub initial_message: String,
    user_history: Vec<String>,
    keywords: Vec<Keyword>,
}

struct Keyword {
    name: String,
    priority: i32,
    memory: i32,
}

struct Transformation {
    from_word: String,
    to_word: String,
}

struct FamilyWord {
    family: String,
    word: String,
}

struct Decomposition {
    // TODO impl Eq para keyword
    keyword: Keyword,
    value: String,
    recompositions: Vec<Recomposition>,
}

struct Recomposition {
    value: String,
    history: i32,
}

impl Ezra {
    pub fn new(db: &str) -> Result<Self, Box<dyn Error>> {
        let db = format!(":/data/{}.db", db);
        let kw_query = Connection::open(db.clone())?;

        let mut kw_query = kw_query.prepare("SELECT * FROM Keywords ORDER BY priority DESC;")?;

        let kw_query = kw_query.query_map([], |row| {
            Ok(Keyword {
                name: row.get(1)?,
                priority: row.get(2)?,
                memory: row.get(3)?,
            })
        })?;

        let mut keywords = vec![];
        for kw in kw_query {
            keywords.push(kw?);
        }

        Ok(Ezra {
            db,
            initial_message: "Hi! What would you like to talk about?".into(),
            user_history: vec![],
            keywords,
        })
    }

    fn transform(&self, mut user_reply: String) -> Result<String, Box<dyn Error>> {
        user_reply = user_reply.to_lowercase();

        let conn = Connection::open(format!(":/data/{}.db", self.db))?;

        let mut trans_query = conn.prepare("SELECT * FROM Transformations")?;

        let trans_query = trans_query.query_map([], |row| {
            Ok(Transformation {
                from_word: row.get(0)?,
                to_word: row.get(1)?,
            })
        })?;

        for transf in trans_query {
            let transf = transf?;
            user_reply = user_reply.replace(&transf.from_word, &transf.to_word);
        }

        Ok(user_reply)
    }

    pub fn reply(&mut self, mut user_reply: String) -> Result<String, Box<dyn Error>> {
        self.user_history.push(user_reply.clone());
        user_reply = self.transform(user_reply.to_lowercase())?;

        // TODO loopear oraciones *dentro* de keywords ordenados por prioridad
        let user_reply = user_reply.split('.').find(|&sentence| {
            for kw in &self.keywords {
                match sentence.contains(&kw.name) {
                    true => return true,
                    false => continue,
                };
            }
            false
        });

        Err("Pasó algo".into())
    }
}
