use crate::utils::io::{read_line, read_n_values, read_value};
use std::convert::TryFrom;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10874(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_students: usize = read_value(read_line(reader));
    let ans = (0..num_students)
        .filter_map(|i| {
            let scores: Vec<i32> = read_n_values(reader, 10);
            if scores.iter().enumerate().all(|(j, &s)| {
                let expected = i32::try_from(j % 5 + 1).unwrap();
                s == expected
            }) {
                Some((i + 1).to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/10874
// 이교수님의 시험
#[test]
fn test_solve10874() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1 1 1 1 1 1 1 1 1 1
1 2 3 4 5 1 2 3 4 5
3 2 2 1 5 1 2 2 2 2
1 2 3 4 5 1 2 3 4 5
1 2 3 4 5 1 2 3 4 5"
                .to_string(),
            want: "2
4
5"
            .to_string(),
        },
        TestData {
            s: "1
1 2 3 4 5 1 2 3 4 5"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1
5 4 3 2 1 5 4 3 2 1"
                .to_string(),
            want: "".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10874(&mut reader, &mut writer);

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
