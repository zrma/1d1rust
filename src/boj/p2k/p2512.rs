use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2512(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cities = read_value(read_line(reader));
    let city_budgets: Vec<u32> = {
        let s = read_line(reader);
        s.split_whitespace()
            .take(num_cities)
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let total_budget = read_value(read_line(reader));

    let mut lower_bound = 0;
    let mut upper_bound = city_budgets.iter().max().unwrap() + 1;
    while lower_bound + 1 < upper_bound {
        let mid = (lower_bound + upper_bound) / 2;
        let total: u32 = city_budgets
            .iter()
            .map(|&budget| if budget > mid { mid } else { budget })
            .sum();
        if total <= total_budget {
            lower_bound = mid;
        } else {
            upper_bound = mid;
        }
    }

    write!(writer, "{}", lower_bound).expect("write! should work");
}

// https://www.acmicpc.net/problem/2512
// 예산
#[test]
fn test_solve2512() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
120 110 140 150
485"
            .to_string(),
            want: "127".to_string(),
        },
        TestData {
            s: "5
70 80 30 40 100
450"
            .to_string(),
            want: "100".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2512(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
