use bytes::Bytes;
use hashbrown::HashMap;

pub struct Db {
    entries: HashMap<String, Bytes>,
}

impl Db {
    pub fn new() -> Db {
        Db { entries: HashMap::new() }
    }

    pub fn write(&mut self, command_words: &Vec<String>) -> Result<&str, &'static str> {
        let key = &command_words[0];
        let value = &command_words[1];

        // FIXME: the key is not updated if alrdy exists
        let result = &self.entries.insert(key.to_string(), Bytes::from(value.clone()));

        match result {
            Some(_) => Ok("r OK"),
            None => Ok("Ok")
        }
    }
}