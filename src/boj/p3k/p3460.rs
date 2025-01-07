use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3460(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_value(read_line(reader));

    let mut res = Vec::with_capacity(t);
    for _ in 0..t {
        let mut n: i32 = read_value(read_line(reader));
        let mut idx = 0;
        let mut r = vec![];
        while n > 0 {
            if n % 2 == 1 {
                r.push(idx.to_string());
            }
            n /= 2;
            idx += 1;
        }
        res.push(r.join(" "));
    }

    writeln!(writer, "{}", res.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/3460
// 이진수
#[test]
fn test_solve3460() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
13"
            .to_string(),
            want: "0 2 3".to_string(),
        },
        TestData {
            s: "3
1
2
3"
            .to_string(),
            want: "0
1
0 1"
            .to_string(),
        },
        TestData {
            s: "5
4
5
6
7
8"
            .to_string(),
            want: "2
0 2
1 2
0 1 2
3"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve3460(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
