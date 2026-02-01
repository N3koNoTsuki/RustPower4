//This files is for function only

use crate::constants;

pub fn add( a: i8, b: i8) -> i8
{
    let c : i8;
    c = a + b;
    return c;
}

pub fn display_grid(a : [[i8; constants::W]; constants::H])
{
    
    for h in 0..(2*constants::H + 1) {
        for w in 0..constants::W {

            if h%2 == 1 {
                print!("{}|", a[w][h/2] );
            }
            else {
                print!("+-");
            }
        }
        if h%2 == 1 {
            print!("\n\r");
        }
        else if h == (2*constants::H) {
            print!("+");
        }
        else {
            print!("+\n\r|");
        }
    }
}
