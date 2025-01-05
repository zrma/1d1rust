use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14656(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_students: usize = read_value(read_line(reader));
    let ans = read_n_values::<usize>(reader, num_students)
        .iter()
        .enumerate()
        .filter(|(i, &v)| (i + 1) != v)
        .count();

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/14656
// 조교는 새디스트야!!
#[test]
fn test_solve14656() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "5
3 2 5 4 1"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "10
2 3 4 5 6 7 8 9 10 1"
                .to_string(),
            want: "10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14656(&mut reader, &mut writer);

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
