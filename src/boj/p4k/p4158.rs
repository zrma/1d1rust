use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4158(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut results = Vec::new();
    let mut hash_set = std::collections::HashSet::new();
    let mut buffer = String::new();

    loop {
        let (n, m) = read_values_as!(read_line(reader), usize, usize);
        if n == 0 && m == 0 {
            break;
        }

        hash_set.clear();
        for _ in 0..n {
            reader.read_line(&mut buffer).expect("Failed to read");
            let num = buffer.trim().parse::<i32>().expect("Failed to parse");
            buffer.clear();
            hash_set.insert(num);
        }

        let mut overlap_count = 0;
        for _ in 0..m {
            reader.read_line(&mut buffer).expect("Failed to read");
            let num = buffer.trim().parse::<i32>().expect("Failed to parse");
            buffer.clear();

            if hash_set.contains(&num) {
                overlap_count += 1;
            }
        }

        results.push(overlap_count.to_string());
    }

    write!(writer, "{}", results.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/4158
// noinspection SpellCheckingInspection
// CD
#[test]
fn test_solve4158() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 3
1
2
3
1
2
4
0 0"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "10 4
1
3
5
7
9
11
13
15
17
19
7
8
9
10
3 3
1
2
3
4
5
6
3 3
1
2
3
2
3
4
0 0"
            .to_string(),
            want: "2
0
2"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4158(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
