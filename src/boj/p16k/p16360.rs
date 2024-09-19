use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16360(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_cases: usize = read_value(read_line(reader));
    let suffix_mapping = vec![
        ("a", "as"),
        ("i", "ios"),
        ("y", "ios"),
        ("l", "les"),
        ("n", "anes"),
        ("ne", "anes"),
        ("o", "os"),
        ("r", "res"),
        ("t", "tas"),
        ("u", "us"),
        ("v", "ves"),
        ("w", "was"),
    ];

    let ans = (0..num_of_cases)
        .map(|_| {
            let word = read_line(reader);
            transform_word(&word, &suffix_mapping)
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("Failed to write");
}

fn transform_word(word: &str, mapping: &[(&str, &str)]) -> String {
    mapping
        .iter()
        .find_map(|&(before, after)| {
            if word.ends_with(before) {
                Some(word.rsplit_once(before).unwrap().0.to_owned() + after)
            } else {
                None
            }
        })
        .unwrap_or_else(|| word.to_owned() + "us")
}

// https://www.acmicpc.net/problem/16360
// noinspection SpellCheckingInspection
// Go Latin
#[test]
fn test_solve16360() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
toy
engine"
                .to_string(),
            want: "toios
engianes"
                .to_string(),
        },
        TestData {
            s: "3
cup
water
cappuccino"
                .to_string(),
            want: "cupus
wateres
cappuccinos"
                .to_string(),
        },
        TestData {
            s: "12
tea
safari
hey
angel
can
hello
teacher
hit
you
halv
cow
mime"
                .to_string(),
            want: "teas
safarios
heios
angeles
caanes
hellos
teacheres
hitas
yous
halves
cowas
mimeus"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve16360(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
