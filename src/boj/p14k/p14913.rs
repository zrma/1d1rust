use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14913(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (first_term, common_diff, target) = read_values_as!(read_line(reader), i64, i64, i64);
    let ans = match (target - first_term).checked_rem(common_diff) {
        Some(0) if (target - first_term) / common_diff >= 0 => {
            ((target - first_term) / common_diff + 1).to_string()
        }
        _ => "X".to_string(),
    };

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/14913
// 등차수열에서 항 번호 찾기
#[test]
fn test_solve14913() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1 2 9".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "1 4 13".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "2 2 9".to_string(),
            want: "X".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14913(&mut reader, &mut writer);

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
