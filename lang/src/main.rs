use std::{env, fs};

use lang::{eval, parser};

fn main2() {
    let f = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let ast = parser::parse(&f).unwrap();
    eprintln!("{}", ast.pretty());
    let (res, cost) = eval::evaluate(&ast).unwrap();
    println!("result: {}\ncost: {}", res, cost);
}

fn main() {
    const STACK_SIZE: usize = 2048 * 1024 * 1024;
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(main2)
        .unwrap()
        .join()
        .unwrap();
}
