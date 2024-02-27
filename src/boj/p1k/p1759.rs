use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1759(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (l, c) = read_values_as!(read_line(reader), usize, usize);
    let mut input = read_line(reader)
        .split_whitespace()
        .take(c)
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<char>>();
    input.sort();

    let mut params = PasswordParams {
        input: &input,
        start_index: 0,
        curr_len: 0,
        max_len: l,
        vowels: 0,
        consonants: 0,
        cur_str: String::new(),
        ans: vec![],
    };

    choose_password(&mut params);

    write!(writer, "{}", params.ans.join("\n")).unwrap();
}

struct PasswordParams<'a> {
    input: &'a [char],
    start_index: usize,
    curr_len: usize,
    max_len: usize,
    vowels: usize,
    consonants: usize,
    cur_str: String,
    ans: Vec<String>,
}

fn choose_password(params: &mut PasswordParams) {
    if params.curr_len == params.max_len {
        if params.vowels >= 1 && params.consonants >= 2 {
            params.ans.push(params.cur_str.clone());
        }
        return;
    }

    for i in params.start_index..params.input.len() {
        let s = params.input[i];
        let is_vowel = is_vowel(s);

        params.start_index = i + 1;
        params.curr_len += 1;
        params.cur_str.push(s);
        if is_vowel {
            params.vowels += 1;
        } else {
            params.consonants += 1;
        }

        choose_password(params);

        // Backtracking
        params.cur_str.pop();
        if is_vowel {
            params.vowels -= 1;
        } else {
            params.consonants -= 1;
        }
        params.curr_len -= 1;
        params.start_index = i;
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

// https://www.acmicpc.net/problem/1759
// noinspection SpellCheckingInspection
// 암호 만들기
#[test]
fn test_solve1759() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4 6
                a t c i s w"
                .to_string(),
            want: "acis
acit
aciw
acst
acsw
actw
aist
aisw
aitw
astw
cist
cisw
citw
istw"
                .to_string(),
        },
        TestData {
            s: "3 4
a c b i"
                .to_string(),
            want: "abc
bci"
            .to_string(),
        },
        TestData {
            s: "3 4
x y z i"
                .to_string(),
            want: "ixy
ixz
iyz"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1759(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
