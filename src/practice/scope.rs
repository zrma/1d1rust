pub(crate) fn scope() {
    let mut tot = 0;

    // NOTE - if borrow pointer is initialized here, compile failure has occurred
    //
    // error[E0502]: cannot borrow `tot` as immutable because it is also borrowed as mutable
    //     --> src\main.rs:13:31
    //     |
    //  9  |      let tot_ptr = &mut tot;
    //     |                    -------- mutable borrow occurs here
    //  ...
    //  12 |         *tot_ptr += i;
    //     |         ------------- mutable borrow later used here
    //  13 |         println!("{}: {}", i, tot);
    //     |                               ^^^ immutable borrow occurs here
    // let tot_ptr = &mut tot;
    for i in 1..11 {
        let tot_ptr = &mut tot;
        *tot_ptr += i;
        println!("{}: {}", i, tot);
    }

    println!("tot: {}", tot);
}
