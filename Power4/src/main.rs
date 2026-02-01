mod function;
mod constants;

use crate::function::add;
use crate::function::display_grid;


fn main() {

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let a : i8;
    let b : i8;
    let c : i8 ;
    let test : [[i8; constants::W];constants::H];

    a = 13;
    b = 2;
    c = add(a, b);
    test =  [[1,2,3,4,5],
             [6,7,8,9,0],
             [1,2,3,4,5],
             [6,7,8,9,0],
             [6,7,8,9,0],];
             
    print!("Hello, world! {}\n\r", c);
    if c == 14 {
        print!("Meow\n\r");
    }
    else {
        print!("woaf\n\r");
    }
    println!("");
    display_grid(test);
}

