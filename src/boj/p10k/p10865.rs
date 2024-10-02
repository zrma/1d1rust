use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10865(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);
    let mut friends = vec![0; n];
    let mut line = String::new();
    for _ in 0..m {
        line.clear();
        reader.read_line(&mut line).expect("Failed to read");

        let (a, b) = read_values_as!(&line, usize, usize);
        friends[a - 1] += 1;
        friends[b - 1] += 1;
    }

    let ans = friends
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/10865
// 친구 친구
#[test]
fn test_solve10865() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 5
1 2
3 4
2 5
5 1
4 2"
            .to_string(),
            want: "2
3
1
2
2"
            .to_string(),
        },
        TestData {
            s: "5 2
1 2
2 3"
            .to_string(),
            want: "1
2
1
0
0"
            .to_string(),
        },
        TestData {
            s: "5 0".to_string(),
            want: "0
0
0
0
0"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10865(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
