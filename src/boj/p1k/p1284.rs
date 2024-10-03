use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1284(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut answers = vec![];

    loop {
        let n: String = read_value(read_line(reader));
        if n == "0" {
            break;
        }

        let mut len = 1 + n.len();
        for d in n.chars() {
            len += match d {
                '1' => 2,
                '0' => 4,
                _ => 3,
            };
        }
        answers.push(len.to_string());
    }

    write!(writer, "{}", answers.join("\n")).expect("write! should work");
}

// https://www.acmicpc.net/problem/1284
// 집 주소
#[test]
fn test_solve1284() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "120
5611
100
0"
            .to_string(),
            want: "13
15
14"
            .to_string(),
        },
        TestData {
            s: "1
11
111
1111
0"
            .to_string(),
            want: "4
7
10
13"
            .to_string(),
        },
        TestData {
            s: "9
99
999
9999
0"
            .to_string(),
            want: "5
9
13
17"
            .to_string(),
        },
        TestData {
            s: "1
10
100
1000
0"
            .to_string(),
            want: "4
9
14
19"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve1284(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
