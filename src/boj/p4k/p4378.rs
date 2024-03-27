use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4378(reader: &mut impl BufRead, writer: &mut impl Write) {
    let qwerty = "`1234567890-=QWERTYUIOP[]\\ASDFGHJKL;'ZXCVBNM,./";

    loop {
        let mut s = String::new();
        if reader.read_line(&mut s).is_err() || s.is_empty() {
            break;
        }

        let result = s
            .chars()
            .map(|c| {
                qwerty
                    .find(c)
                    .and_then(|index| index.checked_sub(1))
                    .and_then(|new_index| qwerty.chars().nth(new_index))
                    .unwrap_or(c)
            })
            .collect::<String>();

        write!(writer, "{}", result).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/4378
// noinspection SpellCheckingInspection
// 트ㅏㅊ;
#[test]
fn test_solve4378() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "O S, GOMR YPFSU/".to_string(),
            want: "I AM FINE TODAY.".to_string(),
        },
        TestData {
            s: "O S, GOMR YPFSU/
JR;;P. EPT;F/"
                .to_string(),
            want: "I AM FINE TODAY.
HELLO, WORLD."
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4378(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
