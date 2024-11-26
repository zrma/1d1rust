use crate::read_values_as;
use crate::utils::io::read_line;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve30454(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (num_horses, _) = read_values_as!(read_line(reader), usize, usize);

    let mut max_segment_count = 0;
    let mut horses_with_max_segments = 0;

    for _ in 0..num_horses {
        let line = read_line(reader);
        let current_segment_count = line
            .split('0')
            .filter(|segment| !segment.is_empty())
            .count();

        match current_segment_count.cmp(&max_segment_count) {
            Greater => {
                max_segment_count = current_segment_count;
                horses_with_max_segments = 1;
            }
            Equal => {
                horses_with_max_segments += 1;
            }
            Less => {}
        }
    }

    write!(writer, "{} {}", max_segment_count, horses_with_max_segments).expect("Failed to write");
}

// https://www.acmicpc.net/problem/30454
// noinspection SpellCheckingInspection
// 얼룩말을 찾아라!
#[test]
fn test_solve30454() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 9
110010101
101010100
000011111
011011010
100100101"
                .to_string(),
            want: "4 3".to_string(),
        },
        TestData {
            s: "5 5
11001
10101
00001
01101
10010"
                .to_string(),
            want: "3 1".to_string(),
        },
        TestData {
            s: "5 5
11001
10001
00001
10000
10010"
                .to_string(),
            want: "2 3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve30454(&mut reader, &mut writer);

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
