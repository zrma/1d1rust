use std::io::{BufRead, Write};

#[allow(dead_code)]
pub(crate) fn solve2437(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();

    let arr = line
        .split_whitespace()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect();

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
#[test]
fn test_solve2437() {
    struct TestData {
        s: String,
        want: String,
    }
    for data in vec![
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
    ] {
        use std::io::Cursor;
        let mut cursor = Cursor::new(data.s);
        let mut output: Vec<u8> = Vec::new();
        solve2437(&mut cursor, &mut output);

        let got = String::from_utf8(output).unwrap();
        assert_eq!(data.want, got);
    }
}
