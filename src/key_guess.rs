

const KEY_VALUES: &[u8; 2932] = include_bytes!("..\\keys.txt");

pub struct KeyGuess<'a> {

  values: Vec<&'a [u8]>,

}

impl<'a> Default for KeyGuess<'a> {
  fn default() -> Self {
    let mut values = Vec::<&[u8]>::default();
    
    for i in 0..733 {
      values.push(&KEY_VALUES[i * 4..]);
    }

    Self {
      values
    }
  }
}

impl<'a> KeyGuess<'a> {

  pub fn add_key(&mut self, id: u8, value: u8) {
    if id > 3 { return; }

    self.values.retain(|v| {
      if v[id as usize] == value {
        return true;
      }

      println!("Removed {}{}{}{}", v[0] as char, v[1] as char, v[2] as char, v[3] as char);

      return false;
    });
  }

  pub fn len(&self) -> usize {
    
    self.values.len()
  
  }

  pub fn get_list(&self) -> &Vec<&[u8]> {
    
    &self.values
  
  }

}


#[cfg(test)]
mod tests {
    use super::KeyGuess;

  #[test]
  fn base_test() {
    let mut key_guess = KeyGuess::default();

    key_guess.add_key(0, b'w');
    key_guess.add_key(1, b'a');
    key_guess.add_key(2, b'r');
    key_guess.add_key(3, b'm');

    let list = key_guess.get_list();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0][0..4], *"warm".as_bytes());
  }

  #[test]
  fn test_2_solutions() {
    let mut key_guess = KeyGuess::default();

    key_guess.add_key(0, b'w');

    key_guess.add_key(2, b'r');
    key_guess.add_key(3, b'm');

    let list = key_guess.get_list();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0][0..4], *"warm".as_bytes());
    assert_eq!(list[1][0..4], *"worm".as_bytes());
  }

}