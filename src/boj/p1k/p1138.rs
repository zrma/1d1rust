use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1138(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let bigger_counts: Vec<usize> = read_n_values(reader, n);

    // 결과를 저장할 벡터 (0으로 초기화)
    let mut result = vec![0; n];

    // 작은 키부터 (1..=n)
    for (i, &count) in bigger_counts.iter().enumerate() {
        let height = i + 1; // 키가 1인 사람 ~ n인 사람 순서
        let mut skip = 0;
        let mut pos = 0;

        while pos < n {
            if result[pos] == 0 {
                // 빈자리 발견
                if skip == count {
                    // count만큼 건너뛰고 도달했으므로 여기 배치
                    result[pos] = height;
                    break;
                }
                skip += 1;
            }
            pos += 1;
        }
    }

    // 결과 출력
    for (i, val) in result.iter().enumerate() {
        write!(writer, "{}", val).unwrap();
        if i < n - 1 {
            write!(writer, " ").unwrap();
        }
    }
    writeln!(writer).ok();
}

// https://www.acmicpc.net/problem/1138
// 한 줄로 서기
#[test]
fn test_solve1138() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
2 1 1 0"
                .to_string(),
            want: "4 2 1 3".to_string(),
        },
        TestData {
            s: "5
0 0 0 0 0"
                .to_string(),
            want: "1 2 3 4 5".to_string(),
        },
        TestData {
            s: "6
5 4 3 2 1 0"
                .to_string(),
            want: "6 5 4 3 2 1".to_string(),
        },
        TestData {
            s: "7
6 1 1 1 2 0 0"
                .to_string(),
            want: "6 2 3 4 7 5 1".to_string(),
        },
        TestData {
            s: "10
5 3 7 1 4 2 1 0 0 0"
                .to_string(),
            want: "8 4 7 2 6 1 9 5 10 3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1138(&mut reader, &mut writer);

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
