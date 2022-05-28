use std::io::{BufRead, Lines, Write};

#[allow(dead_code)]
pub fn solve2437(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut lines = reader.lines();

    lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    let mut arr = parse_str_to_arr(lines);

    let res = get_unavailable_sum(&mut arr);
    write!(writer, "{}", res).unwrap();
}

fn parse_str_to_arr(mut lines: Lines<&mut impl BufRead>) -> Vec<i32> {
    let res: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    if res.is_empty() {
        lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    } else {
        res
    }
}

fn get_unavailable_sum(arr: &mut Vec<i32>) -> i32 {
    arr.sort_unstable();

    let mut sum = 0;
    for n in arr {
        if sum + 1 < *n {
            return sum + 1;
        }
        sum += *n;
    }
    sum + 1
}

// https://www.acmicpc.net/problem/2437
#[test]
fn test_solve2437() {
    struct TestData {
        s: String,
        want: String,
    }
    for data in std::vec![
        TestData {
            s: "7\n3 1 6 2 7 30 1".to_string(),
            want: "21".to_string(),
        },
        TestData {
            s: "7\n1 1 2 3 6 7 30".to_string(),
            want: "21".to_string(),
        },
        TestData {
            s: "1\n1".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "1\n2".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1\n3".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2\n1 1".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "2\n1 2".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "2\n1 3".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2\n1 4".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "2\n2 4".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3\n1 2 4".to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "3\n1 2 5".to_string(),
            want: "4".to_string(),
        },
    ] {
        use std::io::Cursor;
        let mut cursor = Cursor::new(data.s);
        let mut output: Vec<u8> = Vec::new();
        solve2437(&mut cursor, &mut output);

        let got = String::from_utf8(output).unwrap();
        assert_eq!(data.want, got);
    }
}
