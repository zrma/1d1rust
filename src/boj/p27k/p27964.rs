use crate::utils::io::{read_line, read_n_values, read_value};
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27964(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_toppings: usize = read_value(read_line(reader));
    let toppings: Vec<String> = read_n_values(reader, num_of_toppings);

    let cheeses: HashSet<String> = toppings
        .iter()
        .filter(|&topping| topping.ends_with("Cheese"))
        .cloned()
        .collect();

    write!(
        writer,
        "{}",
        if cheeses.len() >= 4 { "yummy" } else { "sad" }
    )
    .expect("Failed to write");
}

// https://www.acmicpc.net/problem/27964
// noinspection SpellCheckingInspection
// 콰트로치즈피자
#[test]
fn test_solve27964() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
CheddarCheese MozzarellaCheese GoudaCheese GranaPadanoCheese"
                .to_string(),
            want: "yummy".to_string(),
        },
        TestData {
            s: "4
MozzarellaCheese MozzarellaCheese MozzarellaCheese MozzarellaCheese"
                .to_string(),
            want: "sad".to_string(),
        },
        TestData {
            s: "4
CheeseBurger CheeseBall CheeseCake CheeseRavioli"
                .to_string(),
            want: "sad".to_string(),
        },
        TestData {
            s: "7
C Chess cheese Cheesa Cheesz Cheesp Cheese"
                .to_string(),
            want: "sad".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve27964(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
