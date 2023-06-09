use bytes::{BytesMut, Buf};

/// Converts a mutable `BytesMut` buffer into a vector of strings.
///
/// The function iterates over the buffer and splits it into individual words based on spaces.
/// It collects these words into a vector of strings, which is returned at the end.
///
/// # Arguments
///
/// * `buf` - A mutable reference to a `BytesMut` buffer to convert into a vector of strings.
pub fn buffer_to_array(buf: &mut BytesMut) -> Vec<String> {
    let mut command_words = Vec::new();
    let buffer_length = buf.len();
    let mut word = "".to_string();

    for i in 0..buffer_length {
        match buf.get_u8() {
            b' ' => {
                command_words.push(word);
                word = "".to_string();
            }
            other => {
                word.push(other as char);
                let new = word.clone();
                if i == buffer_length - 1 {
                    command_words.push(new);
                }
            }
        }
    }

    command_words
}