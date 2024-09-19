use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6603(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = read_line(reader);
    let mut res = vec![];

    while line != "0" {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        let mut ans = vec![];
        choose_lotto(&numbers, &mut ans, 0, 6, vec![]);

        res.push(ans.join("\n"));
        line = read_line(reader);
    }

    write!(writer, "{}", res.join("\n\n")).unwrap();
}

fn choose_lotto(
    arr: &[i32],
    ans: &mut Vec<String>,
    start_index: usize,
    remaining: usize,
    cur: Vec<i32>,
) {
    if remaining == 0 {
        ans.push(
            cur.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );
        return;
    }

    for i in start_index..arr.len() {
        let mut next = cur.clone();
        next.push(arr[i]);
        choose_lotto(arr, ans, i + 1, remaining - 1, next);
    }
}

// https://www.acmicpc.net/problem/6603
// 로또
#[test]
fn test_solve6603() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7 1 2 3 4 5 6 7
8 1 2 3 5 8 13 21 34
0"
            .to_string(),
            want: "1 2 3 4 5 6
1 2 3 4 5 7
1 2 3 4 6 7
1 2 3 5 6 7
1 2 4 5 6 7
1 3 4 5 6 7
2 3 4 5 6 7

1 2 3 5 8 13
1 2 3 5 8 21
1 2 3 5 8 34
1 2 3 5 13 21
1 2 3 5 13 34
1 2 3 5 21 34
1 2 3 8 13 21
1 2 3 8 13 34
1 2 3 8 21 34
1 2 3 13 21 34
1 2 5 8 13 21
1 2 5 8 13 34
1 2 5 8 21 34
1 2 5 13 21 34
1 2 8 13 21 34
1 3 5 8 13 21
1 3 5 8 13 34
1 3 5 8 21 34
1 3 5 13 21 34
1 3 8 13 21 34
1 5 8 13 21 34
2 3 5 8 13 21
2 3 5 8 13 34
2 3 5 8 21 34
2 3 5 13 21 34
2 3 8 13 21 34
2 5 8 13 21 34
3 5 8 13 21 34"
                .to_string(),
        },
        TestData {
            s: "6 1 2 3 4 5 6
0"
            .to_string(),
            want: "1 2 3 4 5 6".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6603(&mut reader, &mut writer);
        let ans = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(ans, data.want, "case: {}", i);
    }
}
