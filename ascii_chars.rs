use std::io;

fn main(){
let mut string = String::new();
io::stdin().read_line(&mut string).expect("Erro ao ler a linha.");
let trim_string = string.trim();
for i in 0..trim_string.chars().count(){
  if let Some(c) = trim_string.chars().nth(i){
      println!("{}", c as u32);
  }
 }
}
