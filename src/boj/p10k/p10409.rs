use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10409(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, t) = read_values_as!(read_line(reader), usize, i32);

    let ans = read_n_values::<i32>(reader, n)
        .iter()
        .scan(0, |acc, &v| {
            *acc += v;
            Some(*acc)
        })
        .take_while(|&v| v <= t)
        .count()
        .to_string();

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/10409
// 서버
#[test]
fn test_solve10409() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6 180
45 30 55 20 80 20"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "10 60
20 7 10 8 10 27 2 3 10 5"
                .to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10409(&mut reader, &mut writer);

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
