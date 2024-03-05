use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2720(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let t = line.trim().parse::<i64>().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let c = line.trim().parse::<u32>().unwrap();

        let mut res = String::new();
        res.push_str(&format!("{} ", c / 25));
        res.push_str(&format!("{} ", (c % 25) / 10));
        res.push_str(&format!("{} ", ((c % 25) % 10) / 5));
        res.push_str(&format!("{}", ((c % 25) % 10) % 5));
        writeln!(writer, "{}", res).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/2720
// 세탁소 사장 동혁
#[test]
fn test_solve2720() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "3
124
25
194"
        .to_string(),
        want: "4 2 0 4
1 0 0 0
7 1 1 4
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2720(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
