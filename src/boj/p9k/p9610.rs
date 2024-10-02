use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9610(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));
    let (mut q1, mut q2, mut q3, mut q4, mut axis) = (0, 0, 0, 0, 0);

    for _ in 0..n {
        let (x, y) = read_values_as!(read_line(reader), i32, i32);
        match (x, y) {
            (0, _) | (_, 0) => axis += 1,
            (x, y) if x > 0 && y > 0 => q1 += 1,
            (x, y) if x < 0 && y > 0 => q2 += 1,
            (x, y) if x < 0 && y < 0 => q3 += 1,
            (x, y) if x > 0 && y < 0 => q4 += 1,
            _ => {}
        }
    }

    writeln!(writer, "Q1: {}", q1).expect("Failed to write");
    writeln!(writer, "Q2: {}", q2).expect("Failed to write");
    writeln!(writer, "Q3: {}", q3).expect("Failed to write");
    writeln!(writer, "Q4: {}", q4).expect("Failed to write");
    write!(writer, "AXIS: {}", axis).expect("Failed to write");
}

// https://www.acmicpc.net/problem/9610
// 사분면
#[test]
fn test_solve9610() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
0 0
0 1
1 1
3 -3
2 2"
            .to_string(),
            want: "Q1: 2
Q2: 0
Q3: 0
Q4: 1
AXIS: 2"
                .to_string(),
        },
        TestData {
            s: "10
1 0
0 1
-1 0
0 -1
1 1
-1 1
-1 -1
2 2
-2 2
3 3"
            .to_string(),
            want: "Q1: 3
Q2: 2
Q3: 1
Q4: 0
AXIS: 4"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve9610(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
