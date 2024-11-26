use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10474(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut ans = vec![];
    loop {
        let (a, b) = read_values_as!(read_line(reader), i32, i32);
        if a == 0 && b == 0 {
            break;
        }

        ans.push(format!("{} {} / {}", a / b, a % b, b));
    }

    write!(writer, "{}", ans.join("\n")).expect("write! should work");
}

// https://www.acmicpc.net/problem/10474
// 분수좋아해?
#[test]
fn test_solve10474() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "27 12
2460000 98400
3 4000
0 0"
            .to_string(),
            want: "2 3 / 12
25 0 / 98400
0 3 / 4000"
                .to_string(),
        },
        TestData {
            s: "1 1
1 2
2 1
0 0"
            .to_string(),
            want: "1 0 / 1
0 1 / 2
2 0 / 1"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10474(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
