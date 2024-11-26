use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13222(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, w, h) = read_values_as!(read_line(reader), u32, u32, u32);
    let area_limit = w * w + h * h;

    for _ in 0..n {
        let curr = read_value::<u32>(read_line(reader)).pow(2);
        let ans = if curr <= area_limit { "YES" } else { "NO" };
        writeln!(writer, "{}", ans).expect("writeln! should work");
    }
}

// https://www.acmicpc.net/problem/13222
// Matches
#[test]
fn test_solve13222() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 3 4
3
4
5
6
7"
            .to_string(),
            want: "YES
YES
YES
NO
NO
"
            .to_string(),
        },
        TestData {
            s: "2 12 17
21
20"
            .to_string(),
            want: "NO
YES
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13222(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
