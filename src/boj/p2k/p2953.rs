use crate::utils::io::read_n_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2953(reader: &mut impl BufRead, writer: &mut impl Write) {
    let ans: (i32, i32) = (1..=5)
        .map(|i| {
            let scores = read_n_values(reader, 4);
            (i, scores.iter().sum())
        })
        .max_by_key(|&(_, score)| score)
        .unwrap();

    writeln!(writer, "{} {}", ans.0, ans.1).unwrap();
}

// https://www.acmicpc.net/problem/2953
// 나는 요리사다
#[test]
fn test_solve2953() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 4 4 5
5 4 4 4
5 5 4 4
5 5 5 4
4 4 4 5"
                .to_string(),
            want: "4 19".to_string(),
        },
        TestData {
            s: "4 4 3 3
5 4 3 5
5 5 2 4
5 5 5 1
4 4 4 4"
                .to_string(),
            want: "2 17".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2953(&mut reader, &mut writer);

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
