use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28289(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut ans = [0; 4];

    for _ in 0..n {
        let (grade, class, _) = read_values_as!(read_line(reader), usize, usize, usize);
        let index = match (grade, class) {
            (1, _) => 3,
            (_, 1 | 2) => 0,
            (_, 3) => 1,
            (_, 4) => 2,
            _ => unreachable!("invalid class"),
        };
        ans[index] += 1;
    }

    for &count in &ans {
        writeln!(writer, "{}", count).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/28289
// 과 조사하기
#[test]
fn test_solve28289() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1 3 5
2 1 10
2 2 12
2 4 8
3 3 10"
                .to_string(),
            want: "2
1
1
1
"
            .to_string(),
        },
        TestData {
            s: "12
1 1 1
1 2 2
1 3 3
1 4 4
2 1 1
2 2 2
2 3 3
2 4 4
3 1 1
3 2 2
3 3 3
3 4 4"
                .to_string(),
            want: "4
2
2
4
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve28289(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
