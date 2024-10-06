use crate::read_values_as;
use crate::utils::io::read_line;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12756(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a_atk, a_hp) = read_values_as!(read_line(reader), i32, i32);
    let (b_atk, b_hp) = read_values_as!(read_line(reader), i32, i32);

    let a_surv_turns = (a_hp + b_atk - 1) / b_atk;
    let b_surv_turns = (b_hp + a_atk - 1) / a_atk;

    let ans = match a_surv_turns.cmp(&b_surv_turns) {
        Less => "PLAYER B",
        Equal => "DRAW",
        Greater => "PLAYER A",
    };

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/12756
// 고급 여관
#[test]
fn test_solve12756() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "4 12
5 5"
            .to_string(),
            want: "PLAYER A".to_string(),
        },
        TestData {
            s: "4 12
12 4"
                .to_string(),
            want: "DRAW".to_string(),
        },
        TestData {
            s: "4 12
4 13"
                .to_string(),
            want: "PLAYER B".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve12756(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
