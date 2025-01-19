use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10867(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_values = read_value(read_line(reader));
    let mut values: Vec<i32> = read_n_values(reader, num_values);

    values.sort_unstable();
    values.dedup();

    for value in values {
        write!(writer, "{} ", value).unwrap();
    }
}

// https://www.acmicpc.net/problem/10867
// 중복 빼고 정렬하기
#[test]
fn test_solve10867() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "10
1 4 2 3 1 4 2 3 1 2"
                .to_string(),
            want: "1 2 3 4".to_string(),
        },
        TestCase {
            s: "10
9 9 9 9 9 9 9 9 9 9"
                .to_string(),
            want: "9".to_string(),
        },
        TestCase {
            s: "10
1 2 3 4 5 6 7 8 9 10"
                .to_string(),
            want: "1 2 3 4 5 6 7 8 9 10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10867(&mut reader, &mut writer);

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
