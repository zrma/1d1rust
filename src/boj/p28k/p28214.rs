use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28214(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, k, p) = read_values_as!(read_line(reader), usize, usize, usize);
    let threshold = k.saturating_sub(p);

    let ans = read_n_values(reader, n * k)
        .chunks(k)
        .filter(|chunk| chunk.iter().sum::<usize>() > threshold)
        .count();

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/28214
// 크림빵
#[test]
fn test_solve28214() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 3 2
1 1 0 1 0 0"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3 2 1
1 1 0 0 1 1"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve28214(&mut reader, &mut writer);

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
