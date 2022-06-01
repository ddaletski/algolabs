#[macro_use]
extern crate rust_config;
//use rust_config.configurable;

#[configurable]
struct Example {
    x: i32,
    y: String,
}

struct Item {
    x: i32,
}

fn main() {
    let x = vec![1, 2, 3];

    let x: Vec<i32> = x
        .into_iter()
        .map(|it| it)
        .filter(|&it| it > 3)
        .map(|x| x)
        .collect();
}
