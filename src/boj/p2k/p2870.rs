use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2870(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_line(reader).parse().unwrap();

    let mut nums = Vec::new();
    for _ in 0..n {
        let line = read_line(reader);
        let extracted_nums = extract_numbers(&line);
        nums.extend(extracted_nums);
    }

    nums.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            a.len().cmp(&b.len())
        }
    });

    for num in nums {
        writeln!(writer, "{}", num).expect("Failed to write");
    }
}

fn extract_numbers(line: &str) -> Vec<String> {
    let mut nums = Vec::new();
    let mut num = String::new();
    for c in line.chars() {
        if c.is_ascii_digit() {
            num.push(c);
        } else if !num.is_empty() {
            num = trim_leading_zeros(num);
            nums.push(num);
            num = String::new();
        }
    }
    if !num.is_empty() {
        num = trim_leading_zeros(num);
        nums.push(num);
    }
    nums
}

fn trim_leading_zeros(num: String) -> String {
    if num.len() > 1 && num.starts_with('0') {
        let trimmed = num.trim_start_matches('0').to_string();
        if trimmed.is_empty() {
            "0".to_string()
        } else {
            trimmed
        }
    } else {
        num
    }
}

// https://www.acmicpc.net/problem/2870
// 수학숙제
#[test]
fn test_solve2870() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
lo3za4
01"
            .to_string(),
            want: "1
3
4
"
            .to_string(),
        },
        TestData {
            s: "4
43silos0
zita002
le2sim
231233"
                .to_string(),
            want: "0
2
2
43
231233
"
            .to_string(),
        },
        TestData {
            s: "4
01bond
02james007
03bond
04austinpowers000"
                .to_string(),
            want: "0
1
2
3
4
7
"
            .to_string(),
        },
        TestData {
            s: "4
12345678901234567890
9
1
91234567890123456789"
                .to_string(),
            want: "1
9
12345678901234567890
91234567890123456789
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2870(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
