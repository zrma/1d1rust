use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10811(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);

    let mut arr = (1..=n).collect::<Vec<_>>();

    for _ in 0..m {
        let (i, j) = read_values_as!(read_line(reader), usize, usize);

        arr[i - 1..j].reverse();
    }

    let output = arr
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    write!(writer, "{}", output).expect("Failed to write");
}

// https://www.acmicpc.net/problem/10811
// 바구니 뒤집기
#[test]
fn test_solve10811() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "5 4
1 2
3 4
1 4
2 2"
        .to_string(),
        want: "3 4 1 2 5".to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10811(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
