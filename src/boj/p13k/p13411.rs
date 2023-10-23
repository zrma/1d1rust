use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13411(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();

    let n = s.trim().parse::<usize>().unwrap();

    struct Entry {
        id: usize,
        time: f64,
    }

    let mut entries = Vec::with_capacity(n);
    for i in 0..n {
        s.clear();
        reader.read_line(&mut s).unwrap();

        let mut iter = s.split_whitespace();
        let (x, y, v) = (
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
        );
        let time = (x * x + y * y) / (v * v);
        entries.push(Entry { id: i + 1, time });
    }

    entries.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    s.clear();
    for entry in entries {
        s.push_str(&format!("{}\n", entry.id));
    }

    write!(writer, "{}", s).unwrap();
}

// https://www.acmicpc.net/problem/13411
// 하늘에서 정의가 빗발친다!
#[test]
fn test_solve13411() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
3 3 1
-3 2 3
-3 -3 1
4 -4 4"
                .to_string(),
            want: "2
4
1
3
"
            .to_string(),
        },
        TestData {
            s: "3
3 3 1
-3 -3 1
4 -4 4"
                .to_string(),
            want: "3
1
2
"
            .to_string(),
        },
        TestData {
            s: "2
3 4 5
1 0 1"
                .to_string(),
            want: "1
2
"
            .to_string(),
        },
        TestData {
            s: "2
5 0 5
2 0 1"
                .to_string(),
            want: "1
2
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13411(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}
