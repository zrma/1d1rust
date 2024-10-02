use crate::utils::io::{read_line, read_value};
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1283(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_phrases: usize = read_value(read_line(reader));
    let mut shortcut_set = HashSet::new();
    let mut answers = Vec::with_capacity(num_phrases);

    for _ in 0..num_phrases {
        let phrase = read_line(reader);
        let phrase_with_shortcut = insert_shortcut(&phrase, &mut shortcut_set);
        answers.push(phrase_with_shortcut);
    }

    write!(writer, "{}", answers.join("\n")).unwrap();
}

fn insert_shortcut(phrase: &str, shortcuts: &mut HashSet<char>) -> String {
    let words = phrase.split_whitespace().collect::<Vec<_>>();
    for (i, word) in words.iter().enumerate() {
        if let Some(new_phrase) = try_insert_shortcut_for_word(word, i, &words, shortcuts) {
            return new_phrase;
        }
    }

    try_insert_shortcut_anywhere_in_phrase(phrase, shortcuts).unwrap_or_else(|| phrase.to_owned())
}

fn try_insert_shortcut_for_word(
    word: &str,
    word_index: usize,
    words: &[&str],
    shortcuts: &mut HashSet<char>,
) -> Option<String> {
    let first_char = word.chars().next()?;
    if !shortcuts.contains(&first_char.to_ascii_lowercase())
        && !shortcuts.contains(&first_char.to_ascii_uppercase())
    {
        shortcuts.insert(first_char);
        let modified_word = format!("[{}]{}", first_char, &word[1..]);
        let modified_words = [
            &words[..word_index],
            &[&modified_word],
            &words[word_index + 1..],
        ]
        .concat();
        Some(modified_words.join(" "))
    } else {
        None
    }
}

fn try_insert_shortcut_anywhere_in_phrase(
    phrase: &str,
    shortcuts: &mut HashSet<char>,
) -> Option<String> {
    for (i, c) in phrase.char_indices() {
        if c == ' ' {
            continue;
        }
        if !shortcuts.contains(&c.to_ascii_lowercase())
            && !shortcuts.contains(&c.to_ascii_uppercase())
        {
            shortcuts.insert(c);
            return Some(format!("{}[{}]{}", &phrase[..i], c, &phrase[i + 1..]));
        }
    }
    None
}

// https://www.acmicpc.net/problem/1283
// noinspection SpellCheckingInspection
// 단축키 지정
#[test]
fn test_solve1283() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
New
Open
Save
Save As
Save All"
                .to_string(),
            want: "[N]ew
[O]pen
[S]ave
Save [A]s
Sa[v]e All"
                .to_string(),
        },
        TestData {
            s: "8
New window
New file
Copy
Undo
Format
Font
Cut
Paste"
                .to_string(),
            want: "[N]ew window
New [f]ile
[C]opy
[U]ndo
F[o]rmat
Fon[t]
Cut
[P]aste"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1283(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
