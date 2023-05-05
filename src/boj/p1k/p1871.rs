use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1871(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let n = line.trim().parse::<usize>().unwrap();
    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).unwrap();

        let (left, right) = {
            let mut iter = line.split('-');
            (iter.next().unwrap(), iter.next().unwrap())
        };

        let left_value = left
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| ((c as u8 - b'A') as usize) * 26usize.pow(i as u32))
            .sum::<usize>();

        let right_value = right.trim().parse::<usize>().unwrap();

        let diff = left_value as i32 - right_value as i32;
        let result = if (-100..=100).contains(&diff) {
            "nice"
        } else {
            "not nice"
        };

        writeln!(writer, "{}", result).unwrap();
    }
}

// https://www.acmicpc.net/problem/1871
#[test]
fn test_solve1871() {
    struct TestData {
        s: String,
        want: String,
    }
    for data in std::vec![
        TestData {
            s: "2
ABC-0123
AAA-9999"
                .to_string(),
            want: "nice
not nice
"
            .to_string(),
        },
        TestData {
            s: "1
ABC-0123"
                .to_string(),
            want: "nice
"
            .to_string(),
        },
        TestData {
            s: "1
AAA-9999"
                .to_string(),
            want: "not nice
"
            .to_string(),
        },
    ] {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1871(&mut reader, &mut writer);
        assert_eq!(String::from_utf8(writer).unwrap(), data.want);
    }
}
