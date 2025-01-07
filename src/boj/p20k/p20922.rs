use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values};
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20922(reader: &mut impl BufRead, writer: &mut impl Write) {
    // 입력 받기
    let (n, k) = read_values_as!(read_line(reader), usize, usize);
    let arr: Vec<usize> = read_n_values(reader, n);

    // 슬라이딩 윈도우 알고리즘
    let mut freq = HashMap::new();
    let (mut left, mut answer) = (0, 0);

    for right in 0..n {
        // 오른쪽 포인터 확장
        let count = freq.entry(arr[right]).or_insert(0);
        *count += 1;

        // 만약 어떤 수의 빈도가 k를 초과하면, left를 옮기며 조정
        while *freq.get(&arr[right]).unwrap() > k {
            let left_val = arr[left];
            *freq.entry(left_val).or_insert(0) -= 1;
            left += 1;
        }

        // 최댓길이 갱신
        answer = answer.max(right - left + 1);
    }

    writeln!(writer, "{}", answer).unwrap();
}

// https://www.acmicpc.net/problem/20922
// 겹치는 건 싫어
#[test]
fn test_solve20922() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9 2
3 2 5 5 6 4 4 5 7"
                .to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "10 1
1 2 3 4 5 6 7 8 9 10"
                .to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "5 2
1 1 1 1 1"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20922(&mut reader, &mut writer);

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
