pub fn rot13(text: &String) -> String {
  let mut result = String::with_capacity(text.len());
  let first_char_numeric_value = 'a' as u8;
  let shift = 13;
  let number_of_characters = 26;

  for ch in text.chars() {
      let current_char_numeric_value = ch as u8;
      let next_char = (((
          (current_char_numeric_value - first_char_numeric_value) + shift) 
          % number_of_characters)
          + first_char_numeric_value)
      as char;

      result.push(next_char);
  }

  return result;
}
