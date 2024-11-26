use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::collections::BTreeMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14464(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (num_chickens, num_cows) = read_values_as!(read_line(reader), usize, usize);

    let mut chicken_times = BTreeMap::new();
    for _ in 0..num_chickens {
        let time: i32 = read_value(read_line(reader));
        *chicken_times.entry(time).or_insert(0) += 1;
    }

    let mut cow_intervals = (0..num_cows)
        .map(|_| read_values_as!(read_line(reader), i32, i32))
        .collect::<Vec<_>>();

    cow_intervals.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

    let mut total_crossings = 0;
    for (start, end) in cow_intervals {
        if let Some((&time, _)) = chicken_times.range(start..=end).next() {
            let count = chicken_times.get_mut(&time).unwrap();
            *count -= 1;
            if *count == 0 {
                chicken_times.remove(&time);
            }
            total_crossings += 1;
        }
    }

    write!(writer, "{}", total_crossings).expect("Failed to write");
}

// https://www.acmicpc.net/problem/14464
// 소가 길을 건너간 이유 4
#[test]
fn test_solve14464() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 4
7
8
6
2
9
2 5
4 9
0 3
8 13"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "2 2
1
2
2 3
1 10"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2 2
4
6
1 6
4 5"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2 2
2
4
1 5
2 3"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2 2
4
4
1 5
2 3"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2 2
4
4
1 5
1 5"
            .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14464(&mut reader, &mut writer);

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
