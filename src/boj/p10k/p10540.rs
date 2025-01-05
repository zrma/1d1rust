use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10540(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for _ in 0..n {
        let (x, y) = read_values_as!(read_line(reader), i32, i32);
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let max_area = (max_x - min_x).max(max_y - min_y).pow(2);
    writeln!(writer, "{}", max_area).unwrap();
}

// https://www.acmicpc.net/problem/10540
// KLOPKA
#[test]
fn test_solve10540() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
3 4
5 7
4 3"
            .to_string(),
            want: "16".to_string(),
        },
        TestData {
            s: "4
1 5
5 1
10 5
5 10"
                .to_string(),
            want: "81".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10540(&mut reader, &mut writer);

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
