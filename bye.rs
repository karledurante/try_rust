pub struct Bye;

//
// implementation for MyStruct
//
impl Bye {
  pub fn say(&self) -> &str{
    let out = "bye";
    println(out);
    out
  }
}

impl Bye {
  pub fn yell(&self) -> &str {
    let out = "GOOD BYE!";
    println(out);
    out
  }
}

fn main() {
  let bye = Bye;
  bye.say();
  bye.yell();
}
