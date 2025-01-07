use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9094(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let mut answers = Vec::with_capacity(num_cases);

    for _ in 0..num_cases {
        let (n, m) = read_values_as!(read_line(reader), i32, i32);
        let mut count = 0;
        for a in 1..n {
            for b in a + 1..n {
                if (a * a + b * b + m) % (a * b) == 0 {
                    count += 1;
                }
            }
        }
        answers.push(count.to_string());
    }

    writeln!(writer, "{}", answers.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/9094
// 수학적 호기심
#[test]
fn test_solve9094() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
10 1
20 3
30 4"
                .to_string(),
            want: "2
4
5"
            .to_string(),
        },
        TestData {
            s: "1
3 1"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9094(&mut reader, &mut writer);

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
