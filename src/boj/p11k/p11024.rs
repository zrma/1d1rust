use crate::utils::io::{read_line, read_value, read_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11024(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    (0..num_cases).for_each(|_| {
        let sum: i32 = read_values(reader).iter().sum();
        writeln!(writer, "{}", sum).expect("write! should work");
    });
}

// https://www.acmicpc.net/problem/11024
// 더하기 4
#[test]
fn test_solve11024() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
1 2 3 4 5
5 4 5 4 2 3 1 2"
                .to_string(),
            want: "15
26
"
            .to_string(),
        },
        TestData {
            s: "2
1 2 3 4 5 6 7 8 9 10
5 4 5 4 2 3 1 2"
                .to_string(),
            want: "55
26
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11024(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
