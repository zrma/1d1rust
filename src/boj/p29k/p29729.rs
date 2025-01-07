use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve29729(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (initial_max, n, m) = read_values_as!(read_line(reader), usize, usize, usize);

    let mut max = initial_max;
    let mut curr = 0;

    for _ in 0..(n + m) {
        let op: usize = read_value(read_line(reader));

        if op == 1 {
            curr += 1;
            if curr > max {
                max *= 2;
            }
        } else {
            curr -= 1;
        }
    }

    writeln!(writer, "{}", max).unwrap();
}

// https://www.acmicpc.net/problem/29729
// 가변 배열
#[test]
fn test_solve29729() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 5 1
1
1
1
1
0
1"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "10 12 0
1
1
1
1
1
1
1
1
1
1
1
1"
            .to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "1 12 0
            1
            1
            1
            1
            1
            1
            1
            1
            1
            1
            1
            1"
            .to_string(),
            want: "16".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve29729(&mut reader, &mut writer);

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
