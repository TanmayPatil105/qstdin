extern crate qstdin;

use qstdin::{is, Stdin};

fn main() {
    println!("input? {}", is(Stdin::Input));
    println!("file? {}", is(Stdin::File));
    println!("directory? {}", is(Stdin::Directory));
}
