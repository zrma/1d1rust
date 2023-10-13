use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2684(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    for _ in 0..n {
        let s = read_line(reader);
        let arr = s.as_bytes();
        let mut ans = vec![0; 8];
        for i in 0..s.len() - 2 {
            let mut num = 0;
            for j in 0..3 {
                num <<= 1;
                num += if arr[i + j] == b'H' { 1 } else { 0 };
            }
            ans[num] += 1;
        }

        let res = ans
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(writer, "{}", res).unwrap();
    }
}

// https://www.acmicpc.net/problem/2684
// 동전 게임
// noinspection SpellCheckingInspection
#[test]
fn test_solve2684() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT
HHTTTHHTTTHTHHTHHTTHTTTHHHTHTTHTTHTTTHTH
HTHTHHHTHHHTHTHHHHTTTHTTTTTHHTTTTHTHHHHT"
                .to_string(),
            want: "0 0 0 0 0 0 0 38
38 0 0 0 0 0 0 0
4 7 6 4 7 4 5 1
6 3 4 5 3 6 5 6
"
            .to_string(),
        },
        TestData {
            s: "1
HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH"
                .to_string(),
            want: "0 0 0 0 0 0 0 38
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2684(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
