mod filter;
mod recursion;
mod scope;
mod sum;

use filter::filter;
use recursion::recursion;
use scope::scope;
use sum::sum;

pub fn practice() {
    println!("Hello, world!");

    sum();
    scope();
    recursion();
    filter();
}
