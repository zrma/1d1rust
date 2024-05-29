mod filter;
mod networking;
mod recursion;
mod scope;
mod sum;

use filter::filter;
use recursion::recursion;
use scope::scope;
use sum::accumulate;

#[allow(dead_code)]
pub(crate) fn practice() {
    println!("Hello, world!");

    accumulate();
    scope();
    recursion();
    filter();
}
