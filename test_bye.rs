use bye::Bye;
mod bye;



fn main(){
  let bye = Bye;
  bye.say();
  bye.yell();
}

#[test]
fn test_say(){
  let bye = Bye;
  assert!(&"bye" == bye.say());
}

#[test]
fn test_yell(){
  let bye = Bye;
  assert!(&"GOOD BYE!" == bye.yell());
}
