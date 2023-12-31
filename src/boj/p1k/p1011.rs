use crate::read_values;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1011(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));
    let mut ans = Vec::with_capacity(t);

    for _ in 0..t {
        let (x, y) = read_values!(read_line(reader), i64, i64);
        let distance = (y - x).abs();
        ans.push(calculate_steps(distance).to_string());
    }

    write!(writer, "{}", ans.join("\n")).unwrap();
}

fn calculate_steps(distance: i64) -> i64 {
    let n = (distance as f64).sqrt() as i64;
    let mut steps = 2 * n - 1;
    let remaining = distance - n * n;

    if remaining > 0 {
        steps += 1; // 한 번 더 이동
        if remaining > n {
            steps += 1; // 남은 거리가 n보다 크면 한 번 더 이동
        }
    }
    steps
}

// https://www.acmicpc.net/problem/1011
// Fly me to the Alpha Centauri
#[test]
fn test_solve1011() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
0 3
1 5
45 50"
                .to_string(),
            want: "3
3
4"
            .to_string(),
        },
        TestData {
            s: "1
0 1"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "15
0 15
20 23
0 2147483647
1 2147483647
2 2147483647
41706 2147483647
41707 2147483647
2147483643 2147483647
2147483644 2147483647
2147483645 2147483647
2147483646 2147483647
0 1
0 2
0 3
0 4"
            .to_string(),
            want: "7
3
92681
92681
92681
92681
92680
3
3
2
1
1
2
3
3"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1011(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
