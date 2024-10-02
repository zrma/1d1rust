use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1475(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut n: usize = read_value(read_line(reader));
    if n == 0 {
        write!(writer, "1").expect("Failed to write");
        return;
    }

    let mut counts = [0; 10];
    count_digits(&mut n, &mut counts);

    let ans = compute_max_set_needed(&counts);
    write!(writer, "{}", ans).expect("write! should work");
}

fn count_digits(n: &mut usize, counts: &mut [i32; 10]) {
    while *n > 0 {
        counts[*n % 10] += 1;
        *n /= 10;
    }
}

fn compute_max_set_needed(counts: &[i32; 10]) -> i32 {
    let mut ans = 0;
    for (i, &count) in counts.iter().enumerate() {
        if i == 6 || i == 9 {
            continue;
        }
        ans = ans.max(count);
    }
    let six_nine = (counts[6] + counts[9] + 1) / 2;
    ans.max(six_nine)
}

// https://www.acmicpc.net/problem/1475
// 방 번호
#[test]
fn test_solve9999() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9999".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "122".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "123456789".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "12635".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "999966".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "888888".to_string(),
            want: "6".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1475(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
