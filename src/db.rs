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
        let key = &command_words[1];
        let value = &command_words[2];

        println!("set key '{}' to '{}'", key, value);
        let result = &self.entries.insert(key.to_string(), Bytes::from(value.clone()));

        match result {
            Some(_) => Ok("r Ok"),
            None => Ok("Ok")
        }
    }

    pub fn read(&mut self, command_words: &Vec<String>) -> Result<&Bytes, &'static str> {
        let key = &command_words[1];
        let result = &self.entries.get(key);

        match result {
            Some(value) => Ok(value),
            None => {
                eprintln!("Key '{}' not found", command_words[0]);
                Err("No key found")
            }
        }
    }
}