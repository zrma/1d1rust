use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10599(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut ans = vec![];
    loop {
        let (a, b, c, d): (i32, i32, i32, i32) =
            read_values_as!(read_line(reader), i32, i32, i32, i32);
        if a == 0 && b == 0 && c == 0 && d == 0 {
            break;
        }
        ans.push(format!("{} {}", c - b, d - a));
    }

    write!(writer, "{}", ans.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/10599
// 페르시아의 왕들
#[test]
fn test_solve10599() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "100 110 180 185
-600 -575 -530 -530
-25 10 72 86
0 0 0 0"
                .to_string(),
            want: "70 85
45 70
62 111"
                .to_string(),
        },
        TestData {
            s: "-1 -1 1 1
-3 -3 -1 -1
1 1 3 3
0 0 0 0"
                .to_string(),
            want: "2 2
2 2
2 2"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10599(&mut reader, &mut writer);

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
