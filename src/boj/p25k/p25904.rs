use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25904(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, mut x) = read_values_as!(read_line(reader), usize, usize);
    let limits: Vec<usize> = read_n_values(reader, n);

    for (i, &limit) in limits.iter().cycle().enumerate() {
        if x > limit {
            write!(writer, "{}", i % n + 1).expect("write! should work");
            return;
        }
        x += 1;
    }
}

#[allow(dead_code)]
fn solve25904_cycle(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, mut curr) = read_values_as!(read_line(reader), usize, usize);
    let limits: Vec<usize> = read_n_values(reader, n);

    let res = limits.iter().cycle().enumerate().find(|(_, &limit)| {
        if limit < curr {
            true
        } else {
            curr += 1;
            false
        }
    });

    write!(writer, "{}", res.expect("should have a result").0 % n + 1).expect("write! should work");
}

// https://www.acmicpc.net/problem/25904
// 안녕 클레오파트라 세상에서 제일가는 포테이토칩
#[test]
fn test_solve25904() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 3
8 6 5"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "4 5
4 10 9 8"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve25904(&mut reader, &mut writer);

            let got = String::from_utf8(writer).expect("writer should be a valid string");
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }

        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve25904_cycle(&mut reader, &mut writer);

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
}
