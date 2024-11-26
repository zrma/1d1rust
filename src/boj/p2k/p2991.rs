use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2991(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c, d) = read_values_as!(read_line(reader), i32, i32, i32, i32);
    let (p, m, n) = read_values_as!(read_line(reader), i32, i32, i32);

    fn is_aggressive(t: i32, aggressive: i32, rest: i32) -> bool {
        let cycle = aggressive + rest;
        let time_in_cycle = (t - 1) % cycle + 1;
        time_in_cycle <= aggressive
    }

    fn num_dogs_attacking(t: i32, a: i32, b: i32, c: i32, d: i32) -> i32 {
        let mut count = 0;
        if is_aggressive(t, a, b) {
            count += 1;
        }
        if is_aggressive(t, c, d) {
            count += 1;
        }
        count
    }

    writeln!(writer, "{}", num_dogs_attacking(p, a, b, c, d)).unwrap();
    writeln!(writer, "{}", num_dogs_attacking(m, a, b, c, d)).unwrap();
    write!(writer, "{}", num_dogs_attacking(n, a, b, c, d)).unwrap();
}

// https://www.acmicpc.net/problem/2991
// 사나운 개
#[test]
fn test_solve2991() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 2 3 3
1 3 4"
                .to_string(),
            want: "2
1
0"
            .to_string(),
        },
        TestData {
            s: "2 3 4 5
4 9 5"
                .to_string(),
            want: "1
0
0"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2991(&mut reader, &mut writer);

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
