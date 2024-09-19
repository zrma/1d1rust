use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2121(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();
    let (want_w, want_h) = read_values_as!(read_line(reader), i32, i32);
    let mut points_set = std::collections::HashSet::new();

    for _ in 0..n {
        points_set.insert(read_point(reader));
    }

    let res = points_set
        .iter()
        .filter(|&&(x, y)| {
            points_set.contains(&(x + want_w, y))
                && points_set.contains(&(x, y + want_h))
                && points_set.contains(&(x + want_w, y + want_h))
        })
        .count();

    write!(writer, "{}", res).expect("Failed to write");
}

fn read_point(reader: &mut impl BufRead) -> (i32, i32) {
    read_values_as!(read_line(reader), i32, i32)
}

// https://www.acmicpc.net/problem/2121
// 넷이 놀기
#[test]
fn test_solve2121() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6
2 3
0 0
2 0
2 3
0 3
4 0
4 3"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "6
1 1
0 0
1 0
0 1
1 1
1 2
2 3"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2121(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
