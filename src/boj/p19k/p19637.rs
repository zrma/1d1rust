use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve19637(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (num_titles, num_powers) = read_values_as!(read_line(reader), usize, usize);

    // 칭호와 상한값 읽기
    let mut titles = Vec::with_capacity(num_titles);
    for _ in 0..num_titles {
        let (name, power) = read_values_as!(read_line(reader), String, u32);
        titles.push((name, power));
    }

    // 각 캐릭터 전투력에 대해 이진 탐색 수행
    for _ in 0..num_powers {
        let q = read_value(read_line(reader));
        let mut low = 0;
        let mut high = titles.len();
        while low < high {
            let mid = (low + high) / 2;
            if titles[mid].1 >= q {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        writeln!(writer, "{}", titles[low].0).unwrap();
    }
}

// https://www.acmicpc.net/problem/19637
// IF문 좀 대신 써줘
#[test]
fn test_solve19637() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 8
WEAK 10000
NORMAL 100000
STRONG 1000000
0
9999
10000
10001
50000
100000
500000
1000000"
                .to_string(),
            want: "WEAK
WEAK
WEAK
NORMAL
NORMAL
NORMAL
STRONG
STRONG"
                .to_string(),
        },
        TestData {
            s: "3 5
B 100
A 100
C 1000
99
100
101
500
1000"
                .to_string(),
            want: "B
B
C
C
C"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve19637(&mut reader, &mut writer);

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
