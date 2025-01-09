use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1019(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: u64 = read_value(read_line(reader));
    let mut ans: Vec<u64> = vec![0; 10];

    let mut start = 1;
    let mut end = n;
    let mut point = 1;

    while start <= end {
        // end의 일의 자리를 9로 맞추기
        while end % 10 != 9 && start <= end {
            count_digits(end, point, &mut ans);
            end -= 1;
        }

        if end < start {
            break;
        }

        // start의 일의 자리를 0으로 맞추기
        while start % 10 != 0 && start <= end {
            count_digits(start, point, &mut ans);
            start += 1;
        }

        if end < start {
            break;
        }

        start /= 10;
        end /= 10;

        // 0부터 9까지 각각 point만큼 더하기
        for v in ans.iter_mut().take(10) {
            *v += (end - start + 1) * point;
        }

        point *= 10;
    }

    for v in ans {
        write!(writer, "{} ", v).unwrap();
    }
    writeln!(writer).unwrap();
}

fn count_digits(mut n: u64, point: u64, ans: &mut [u64]) {
    while n > 0 {
        ans[(n % 10) as usize] += point;
        n /= 10;
    }
}

// https://www.acmicpc.net/problem/1019
// 책 페이지
#[test]
fn test_solve1019() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "11".to_string(),
            want: "1 4 1 1 1 1 1 1 1 1".to_string(),
        },
        TestCase {
            s: "7".to_string(),
            want: "0 1 1 1 1 1 1 1 0 0".to_string(),
        },
        TestCase {
            s: "19".to_string(),
            want: "1 12 2 2 2 2 2 2 2 2".to_string(),
        },
        TestCase {
            s: "999".to_string(),
            want: "189 300 300 300 300 300 300 300 300 300".to_string(),
        },
        TestCase {
            s: "543212345".to_string(),
            want: "429904664 541008121 540917467 540117067 533117017 473117011 429904664 429904664 429904664 429904664"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1019(&mut reader, &mut writer);

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
