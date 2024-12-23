use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2304(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n_line = read_line(reader);
    let n: usize = n_line.trim().parse().expect("N must be a number");

    // 위치(1~1000)마다 기둥 높이를 저장할 배열
    let mut pillars = [0usize; 1001];
    let mut left_most = 1001;
    let mut right_most = 0;
    let mut max_height = 0;
    let mut max_index = 0;

    // 입력 처리
    for _ in 0..n {
        let (l, h) = read_values_as!(read_line(reader), usize, usize);
        pillars[l] = h;

        if l < left_most {
            left_most = l;
        }
        if l > right_most {
            right_most = l;
        }
        if h > max_height {
            max_height = h;
            max_index = l;
        }
    }

    // 왼쪽 -> 최고 높이 기둥까지 누적 최대 높이
    let mut area = 0;
    let mut current_left_height = pillars[left_most];
    for &height in &pillars[left_most..max_index] {
        if height > current_left_height {
            current_left_height = height;
        }
        area += current_left_height;
    }

    // 오른쪽 -> 최고 높이 기둥까지 누적 최대 높이
    let mut current_right_height = pillars[right_most];
    for &height in pillars[max_index + 1..=right_most].iter().rev() {
        if height > current_right_height {
            current_right_height = height;
        }
        area += current_right_height;
    }

    // 중앙(최고) 기둥 높이
    area += max_height;

    // 결과 출력
    writeln!(writer, "{}", area).expect("Failed to write output");
}

// https://www.acmicpc.net/problem/2304
// 창고 다각형
#[test]
fn test_solve2304() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
2 4
11 4
15 8
4 6
5 3
8 10
13 6"
                .to_string(),
            want: "98".to_string(),
        },
        TestData {
            s: "6
0 1
1 1
3 2
4 2
5 1
6 1"
            .to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "3
0 3
1 2
2 3"
            .to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "4
1 3
2 2
3 3
4 3"
            .to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "5
4 3
6 5
9 5
11 3
13 4"
                .to_string(),
            want: "42".to_string(),
        },
        TestData {
            s: "5
13 4
6 5
4 3
11 3
9 5"
            .to_string(),
            want: "42".to_string(),
        },
        TestData {
            s: "4
1000 5
999 5
998 5
997 4"
                .to_string(),
            want: "19".to_string(),
        },
        TestData {
            s: "6
5 1
1 1
4 2
2 2
3 3
6 10"
                .to_string(),
            want: "22".to_string(),
        },
        TestData {
            s: "5
1 1
2 1
3 1
4 1
5 1"
            .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "8
1 10
2 50
3 40
4 30
5 40
6 30
7 50
8 100"
                .to_string(),
            want: "410".to_string(),
        },
        TestData {
            s: "1
1 1"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2304(&mut reader, &mut writer);

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
