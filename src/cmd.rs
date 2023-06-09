pub enum Command {
    Get,
    Set,
    Invalid,
}

impl Command {
    /// Parses a string into a corresponding `Command` enum variant.
    ///
    /// This function takes a string reference and matches it against known command strings to determine
    /// the corresponding `Command` enum variant. Returns `Invalid` if nothing is matched.
    ///
    /// # Arguments
    ///
    /// * `str` - A reference to a string to be parsed into a `Command` enum variant.
    ///
    /// # Returns
    ///
    /// The corresponding `Command` enum variant based on the input string.
    pub fn get_command(str: &String) -> Command {
        match str.as_bytes() {
            b"get" => Command::Get,
            b"set" => Command::Set,
            _ => Command::Invalid,
        }
    }
}