use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9295(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let mut res = Vec::with_capacity(num_cases);
    for i in 0..num_cases {
        let (a, b) = read_values_as!(read_line(reader), i32, i32);
        res.push(format!("Case {}: {}", i + 1, a + b));
    }

    writeln!(writer, "{}", res.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/9295
// 주사위
#[test]
fn test_solve9295() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1 2
1 3
3 5
2 6
3 4"
            .to_string(),
            want: "Case 1: 3
Case 2: 4
Case 3: 8
Case 4: 8
Case 5: 7"
                .to_string(),
        },
        TestData {
            s: "2
1 1
6 6"
            .to_string(),
            want: "Case 1: 2
Case 2: 12"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9295(&mut reader, &mut writer);

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
