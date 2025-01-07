use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3613(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let arr = s.as_bytes();
    let mut ans = String::new();
    let mut is_java = false;
    let mut is_cpp = false;
    let mut is_error = false;
    let mut is_first = false;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i == 0 || is_cpp {
                is_error = true;
                break;
            }
            is_java = true;
            ans.push('_');
            ans.push(c.to_ascii_lowercase());
        } else if c == '_' {
            if is_java || i == 0 || i == s.len() - 1 || arr[i + 1] == b'_' {
                is_error = true;
                break;
            }
            is_cpp = true;
            is_java = false;
            is_first = true;
        } else if is_first {
            is_first = false;
            ans.push(c.to_ascii_uppercase());
        } else {
            ans.push(c);
        }
    }
    if is_error || is_java && arr[0].is_ascii_uppercase() {
        writeln!(writer, "Error!").unwrap();
    } else {
        writeln!(writer, "{}", ans).unwrap();
    }
}

// https://www.acmicpc.net/problem/3613
// Java vs C++
#[test]
fn test_solve3613() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "long_and_mnemonic_identifier".to_string(),
            want: "longAndMnemonicIdentifier".to_string(),
        },
        TestData {
            s: "anotherExample".to_string(),
            want: "another_example".to_string(),
        },
        TestData {
            s: "longAndMnemonicIdentifier".to_string(),
            want: "long_and_mnemonic_identifier".to_string(),
        },
        TestData {
            s: "another_example".to_string(),
            want: "anotherExample".to_string(),
        },
        TestData {
            s: "i".to_string(),
            want: "i".to_string(),
        },
        TestData {
            s: "if".to_string(),
            want: "if".to_string(),
        },
        TestData {
            s: "long_and_Mnemonic_identifier".to_string(),
            want: "Error!".to_string(),
        },
        TestData {
            s: "longAndMnemonic_identifier".to_string(),
            want: "Error!".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3613(&mut reader, &mut writer);

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
