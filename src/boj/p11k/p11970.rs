use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11970(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b) = read_values_as!(read_line(reader), u32, u32);
    let (c, d) = read_values_as!(read_line(reader), u32, u32);

    let ans = if c <= b && a <= d {
        // 겹치는 경우
        b.max(d) - a.min(c)
    } else {
        // 겹치지 않는 경우
        (b - a) + (d - c)
    };
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/11970
// Fence Painting
#[test]
fn test_solve11970() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7 10
4 8"
            .to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "4 8
7 10"
                .to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "1 2
3 4"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3 4
1 2"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "0 10
4 5"
            .to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "4 5
0 10"
                .to_string(),
            want: "10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11970(&mut reader, &mut writer);

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
