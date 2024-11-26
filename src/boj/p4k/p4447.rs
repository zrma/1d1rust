use crate::utils::io::read_line;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4447(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();

    for _ in 0..n {
        let s = read_line(reader);
        let mut g = 0;
        let mut b = 0;
        for ch in s.chars() {
            match ch {
                'g' | 'G' => g += 1,
                'b' | 'B' => b += 1,
                _ => {}
            }
        }
        let ans = match g.cmp(&b) {
            Greater => " is GOOD",
            Less => " is A BADDY",
            Equal => " is NEUTRAL",
        };
        writeln!(writer, "{}{}", s, ans).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/4447
// 좋은놈 나쁜놈
// noinspection SpellCheckingInspection
#[test]
fn test_solve4447() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "8
Algorithm Crunching Man
Green Lantern
Boba Fett
Superman
Batman
Green Goblin
Barney
Spider Pig"
                .to_string(),
            want: "Algorithm Crunching Man is GOOD
Green Lantern is GOOD
Boba Fett is A BADDY
Superman is NEUTRAL
Batman is A BADDY
Green Goblin is GOOD
Barney is A BADDY
Spider Pig is GOOD
"
            .to_string(),
        },
        TestData {
            s: "1
Algorithm Crunching Man"
                .to_string(),
            want: "Algorithm Crunching Man is GOOD
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4447(&mut reader, &mut writer);

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
