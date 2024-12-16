use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values};
use std::cmp::Ordering::{Equal, Greater};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve21921(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, x) = read_values_as!(read_line(reader), usize, usize);
    let visitors: Vec<usize> = read_n_values(reader, n);

    // 초기 윈도우의 합 계산
    let mut window_sum: usize = visitors.iter().take(x).sum();
    let mut max_visitors = window_sum;
    let mut count = if window_sum > 0 { 1 } else { 0 };

    // 슬라이딩 윈도우로 나머지 구간 확인
    for i in x..n {
        window_sum = window_sum + visitors[i] - visitors[i - x];
        match window_sum.cmp(&max_visitors) {
            Greater => {
                max_visitors = window_sum;
                count = 1;
            }
            Equal if window_sum > 0 => {
                count += 1;
            }
            _ => {}
        }
    }

    if max_visitors == 0 {
        writeln!(writer, "SAD").unwrap();
    } else {
        writeln!(writer, "{}", max_visitors).unwrap();
        writeln!(writer, "{}", count).unwrap();
    }
}

// https://www.acmicpc.net/problem/21921
// 블로그
#[test]
fn test_solve21921() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 2
1 4 2 5 1"
                .to_string(),
            want: "7
1"
            .to_string(),
        },
        TestData {
            s: "7 5
1 1 1 1 1 5 1"
                .to_string(),
            want: "9
2"
            .to_string(),
        },
        TestData {
            s: "5 3
0 0 0 0 0"
                .to_string(),
            want: "SAD".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve21921(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("valid utf8 string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
