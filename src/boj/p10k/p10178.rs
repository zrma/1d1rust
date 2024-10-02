use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10178(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let ans = (0..num_cases)
        .map(|_| {
            let (candy, children) = read_values_as!(read_line(reader), i32, i32);
            let (pieces_per_child, dad_pieces) = (candy / children, candy % children);
            format!(
                "You get {} piece(s) and your dad gets {} piece(s).",
                pieces_per_child, dad_pieces
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/10178
// 할로윈의 사탕
#[test]
fn test_solve10178() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
22 3
15 5
99 8
7 4
101 5"
                .to_string(),
            want: "You get 7 piece(s) and your dad gets 1 piece(s).
You get 3 piece(s) and your dad gets 0 piece(s).
You get 12 piece(s) and your dad gets 3 piece(s).
You get 1 piece(s) and your dad gets 3 piece(s).
You get 20 piece(s) and your dad gets 1 piece(s)."
                .to_string(),
        },
        TestData {
            s: "1
1 1"
            .to_string(),
            want: "You get 1 piece(s) and your dad gets 0 piece(s).".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10178(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
