use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1522(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let chars: Vec<char> = s.chars().collect();

    writeln!(writer, "{}", min_swaps_to_group(&chars)).unwrap();
}

/// 문자열에서 'a'를 모두 연속되게 만들기 위한 최소 교환 횟수를 구한다.
/// 순환 문자열이므로 'a'가 문자열의 끝과 처음에 걸쳐서 연속될 수 있다.
fn min_swaps_to_group(chars: &[char]) -> usize {
    let n = chars.len();
    let a_count = chars.iter().filter(|&&c| c == 'a').count();

    if a_count == 0 || a_count == n {
        return 0;
    }

    let mut min_swaps = n;
    let extended: Vec<char> = chars.iter().chain(chars.iter()).copied().collect();

    // 슬라이딩 윈도우로 모든 가능한 위치를 확인
    for i in 0..n {
        let window = &extended[i..i + a_count];
        let b_count = window.iter().filter(|&&c| c == 'b').count();
        min_swaps = min_swaps.min(b_count);
    }

    min_swaps
}

// https://www.acmicpc.net/problem/1522
// 문자열 교환
#[test]
fn test_solve1522() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "abababababababa".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "ba".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "aaaabbbbba".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "abab".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "aabbaaabaaba".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "aaaa".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1522(&mut reader, &mut writer);

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
