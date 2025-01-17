use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2042(reader: &mut impl BufRead, writer: &mut impl Write) {
    // N: 배열 크기, M: 값 변경 횟수, K: 구간 합 쿼리 횟수
    let (n, m, k) = read_values_as!(read_line(reader), usize, usize, usize);

    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let v = read_value(read_line(reader));
        arr.push(v);
    }

    let mut fenwicks = vec![0_i64; n + 1];

    // pos: 1-based index, delta: 더할 값(변화량)
    fn update(fenwicks: &mut [i64], mut pos: usize, delta: i64) {
        while pos < fenwicks.len() {
            fenwicks[pos] += delta;
            pos += pos & (!pos + 1); // pos & -pos 와 동일
        }
    }

    // 1부터 pos까지의 누적합
    fn prefix_sum(fenwicks: &[i64], mut pos: usize) -> i64 {
        let mut result = 0;
        while pos > 0 {
            result += fenwicks[pos];
            pos &= pos - 1; // pos -= pos & -pos
        }
        result
    }

    // 1-based 인덱스로 펜윅 트리를 빌드
    for (i, &val) in arr.iter().enumerate() {
        update(&mut fenwicks, i + 1, val);
    }

    // 4) M+K번의 쿼리 처리
    for _ in 0..(m + k) {
        let (a, b, c) = read_values_as!(read_line(reader), u8, usize, i64);

        match a {
            1 => {
                // b번째 값을 c로 교체
                let old_val = arr[b - 1];
                let delta = c - old_val;
                arr[b - 1] = c;
                update(&mut fenwicks, b, delta);
            }
            2 => {
                // b부터 c까지 합 출력
                let left = b;
                let right = c as usize;
                let total = prefix_sum(&fenwicks, right) - prefix_sum(&fenwicks, left - 1);
                writeln!(writer, "{}", total).unwrap();
            }
            _ => panic!("invalid query"),
        }
    }
}

// https://www.acmicpc.net/problem/2042
// 구간 합 구하기
#[test]
fn test_solve2042() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "5 2 2
1
2
3
4
5
1 3 6
2 2 5
1 5 2
2 3 5"
                .to_string(),
            want: "17
12"
            .to_string(),
        },
        TestCase {
            s: "5 2 1
1
2
3
4
5
1 3 10
1 3 20
2 1 3"
                .to_string(),
            want: "23".to_string(),
        },
        TestCase {
            s: "4 4 3
10
10
10
10
1 1 11
1 2 11
2 1 4
1 1 10
2 1 4
1 1 3
2 1 4"
                .to_string(),
            want: "42
41
34"
            .to_string(),
        },
        TestCase {
            s: "4 0 1
1
2
3
4
2 1 1"
                .to_string(),
            want: "1".to_string(),
        },
        TestCase {
            s: "5 2 2
1
2
3
4
5
1 3 6
2 2 5
1 3 2
2 3 5"
                .to_string(),
            want: "17
11"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2042(&mut reader, &mut writer);

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
