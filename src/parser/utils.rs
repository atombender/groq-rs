pub trait TokenTests {
  fn is_digit(&self) -> bool;
  fn is_letter(&self) -> bool;
  fn is_identifier(&self) -> bool;
  fn is_leading_identifier(&self) -> bool;
  fn is_operator(&self) -> bool;
}

impl TokenTests for char {
  fn is_digit(&self) -> bool {
    *self >= '0' && *self <= '9'
  }

  fn is_letter(&self) -> bool {
    (*self >= 'a' && *self <= 'z') || (*self >= 'A' && *self <= 'Z')
  }

  fn is_identifier(&self) -> bool {
    self.is_letter() || self.is_digit() || *self == '_'
  }

  fn is_leading_identifier(&self) -> bool {
    self.is_letter() || *self == '_' || *self == '$'
  }

  // TODO: Need to special case '^' right now because two consecutive ^'s gets joined into
  // a '^^' operator. Need to fix
  fn is_operator(&self) -> bool {
    *self == '<'
      || *self == '>'
      || *self == '|'
      || *self == '='
      || *self == '&'
      || *self == '!'
      || *self == '*'
      || *self == '?'
      || *self == '-'
  }
}
