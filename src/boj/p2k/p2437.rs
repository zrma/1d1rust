use crate::utils::io::{read_line, read_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2437(reader: &mut impl BufRead, writer: &mut impl Write) {
    read_line(reader);
    let arr = read_values::<i32>(reader);
    let res = get_unavailable_sum(arr);
    write!(writer, "{}", res).unwrap();
}

fn get_unavailable_sum(mut arr: Vec<i32>) -> i32 {
    arr.sort_unstable();

    let mut sum = 0;
    for n in arr {
        if sum + 1 < n {
            return sum + 1;
        }
        sum += n;
    }
    sum + 1
}

// https://www.acmicpc.net/problem/2437
// 저울
#[test]
fn test_solve2437() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
3 1 6 2 7 30 1"
                .to_string(),
            want: "21".to_string(),
        },
        TestData {
            s: "7
1 1 2 3 6 7 30"
                .to_string(),
            want: "21".to_string(),
        },
        TestData {
            s: "1
1"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "1
2"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1
3"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2
1 1"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "2
1 2"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "2
1 3"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2
1 4"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2
2 4"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3
1 2 4"
                .to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "3
1 2 5"
                .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2437(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
