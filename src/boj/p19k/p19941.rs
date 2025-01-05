use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve19941(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, k) = read_values_as!(read_line(reader), usize, usize);
    let s = read_line(reader);
    let chars: Vec<char> = s.chars().collect();
    let mut eaten = vec![false; n];
    let mut count = 0;

    // 각 사람에 대해
    for (i, &c) in chars.iter().enumerate() {
        if c != 'P' {
            continue;
        }

        // 가장 가까운 햄버거를 찾음 (왼쪽부터)
        let start = i.saturating_sub(k);
        let end = (i + k + 1).min(n);

        for j in start..end {
            if chars[j] == 'H' && !eaten[j] {
                eaten[j] = true;
                count += 1;
                break;
            }
        }
    }

    writeln!(writer, "{}", count).unwrap();
}

// https://www.acmicpc.net/problem/19941
// 햄버거 분배
#[test]
fn test_solve19941() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "20 1
HHPHPPHHPPHPPPHPHPHP"
                .to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "20 2
HHHHHPPPPPHPHPHPHHHP"
                .to_string(),
            want: "7".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve19941(&mut reader, &mut writer);

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
