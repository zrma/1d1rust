use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9501(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let results: Vec<String> = (0..num_cases)
        .map(|_| {
            let (num_spaceships, dist) = read_values_as!(read_line(reader), i32, i32);
            let count = (0..num_spaceships)
                .filter(|_| {
                    let (v, f, c) = read_values_as!(read_line(reader), i32, i32, i32);
                    v * f >= dist * c
                })
                .count();
            count.to_string()
        })
        .collect();

    write!(writer, "{}", results.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/9501
// 꿍의 우주여행
#[test]
fn test_solve9501() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
3 100
52 75 10
88 13 44
56 9 5
2 920368
950 950 1
943 976 1"
                .to_string(),
            want: "2
1"
            .to_string(),
        },
        TestData {
            s: "1
3 100
10 10 1
10 10 2
10 20 2"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9501(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
