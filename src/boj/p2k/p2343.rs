use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2343(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (num_lessons, num_blue_rays) = read_values_as!(read_line(reader), usize, usize);
    let lesson_durations: Vec<usize> = {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        s.split_whitespace()
            .take(num_lessons)
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let mut lower_bound = *lesson_durations.iter().max().unwrap();
    let mut upper_bound = lesson_durations.iter().sum();
    while lower_bound < upper_bound {
        let mid_size = (lower_bound + upper_bound) / 2;
        let mut blueray_count = 1;
        let mut current_sum = 0;
        for &lesson_duration in &lesson_durations {
            if current_sum + lesson_duration > mid_size {
                blueray_count += 1;
                current_sum = 0;
            }
            current_sum += lesson_duration;
        }
        if blueray_count > num_blue_rays {
            lower_bound = mid_size + 1;
        } else {
            upper_bound = mid_size;
        }
    }
    writeln!(writer, "{}", lower_bound).unwrap();
}

// https://www.acmicpc.net/problem/2343
// 기타 레슨
#[test]
fn test_solve2343() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9 3
1 2 3 4 5 6 7 8 9"
                .to_string(),
            want: "17".to_string(),
        },
        TestData {
            s: "8 3
1 2 3 4 5 6 7 8"
                .to_string(),
            want: "15".to_string(),
        },
        TestData {
            s: "4 2
3 2 1 4"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "5 4
1 5 9 3 10"
                .to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "9 4
9 9 9 9 9 9 9 9 9"
                .to_string(),
            want: "27".to_string(),
        },
        TestData {
            s: "7 7
5 9 6 8 7 7 5"
                .to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "4 3
99 1 99 1"
                .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "10 5
10000 10000 10000 10000 10000 10000 10000 10000 10000 10000"
                .to_string(),
            want: "20000".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2343(&mut reader, &mut writer);

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
