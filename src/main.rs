mod sum;

use sum::plus;

fn main() {
    let mut tot = 0;
    for i in 1..11 {
        tot += i;
        println!("{}: {}", i, tot);
    }
    println!("tot: {}", tot);

    println!("Hello, world!");
    println!("{}", plus(3, 5));
}
