use std::{env, fs};

use lang::{eval, parser};

fn main() {
    let f = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let ast = parser::parse(&f).unwrap();
    let (res, cost) = eval::evaluate(&ast).unwrap();
    println!("result: {}\ncost: {}", res, cost);
}
