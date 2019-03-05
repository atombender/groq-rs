mod parser;

fn main() {
  let groq = "\n  // foo!\n* { abc -> ^.c }";
  println!("begin");
  // let chars = &mut groq.chars();
  let mut scanner = parser::Scanner::new(groq);
  loop {
    let tk = scanner.next();
    match tk {
      parser::Token::EOF => break,
      _ => {
        println!("tk {} {:?}", scanner.pos(), tk);
      }
    }
  }
}
