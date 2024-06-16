use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9550(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans = (0..num_cases)
        .map(|_| {
            let (n, k) = read_values_as!(read_line(reader), usize, usize);
            read_n_values::<usize>(reader, n)
                .iter()
                .map(|&x| x / k)
                .sum::<usize>()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n");

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/9550
// 아이들은 사탕을 좋아해
#[test]
fn test_solve9550() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
3 2
4 5 7
3 8
4 5 7"
                .to_string(),
            want: "7
0"
            .to_string(),
        },
        TestData {
            s: "1
5 2
4 5 7 8 9"
                .to_string(),
            want: "15".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];

        solve9550(&mut reader, &mut writer);
        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
