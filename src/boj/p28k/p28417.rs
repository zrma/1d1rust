use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28417(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut ans = 0;
    for _ in 0..n {
        let scores: Vec<u32> = read_n_values(reader, 7);

        let max_run = scores[0].max(scores[1]);

        let mut max_trick1 = 0;
        let mut max_trick2 = 0;

        for &score in &scores[2..] {
            if score > max_trick1 {
                max_trick2 = max_trick1;
                max_trick1 = score;
            } else if score > max_trick2 {
                max_trick2 = score;
            }
        }

        let trick_sum = max_trick1 + max_trick2;
        ans = ans.max(max_run + trick_sum);
    }

    writeln!(writer, "{}", ans).expect("write failed");
}

// https://www.acmicpc.net/problem/28417
// 스케이트보드
#[test]
fn test_solve28417() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
1 2 4 2 3 1 1
5 5 5 5 5 5 5"
                .to_string(),
            want: "15
"
            .to_string(),
        },
        TestData {
            s: "3
100 1 1 2 3 99 98
1 100 99 98 2 3 97
1 1 1 1 1 1 1"
                .to_string(),
            want: "297
"
            .to_string(),
        },
        TestData {
            s: "3
100 1 1 2 3 99 98
1 100 100 98 2 3 100
1 1 1 1 1 1 1"
                .to_string(),
            want: "300
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve28417(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
