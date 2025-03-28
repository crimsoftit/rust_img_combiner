fn get_nth_arg (n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct ImgArgs {
  pub img_1: String,
  pub img_2: String,
  
  pub feedback: String,
}

impl ImgArgs {
  pub fn new() -> Self {
      ImgArgs {
          // img_1: String::new(),
          img_1: get_nth_arg(1),
          img_2: get_nth_arg(2),
          
          feedback: get_nth_arg(3),
      }
  }
}