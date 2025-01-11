use crate::read_values_as;
use crate::utils::io::read_line;
use std::collections::HashSet;
use std::io::{BufRead, Write};

const MOD1: u64 = 1_000_000_007;
const BASE1: u64 = 131;
const MOD2: u64 = 998_244_353;
const BASE2: u64 = 257;

#[allow(dead_code)]
fn solve19585(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (c, n): (usize, usize) = read_values_as!(read_line(reader), usize, usize);

    let mut line = String::new();

    // --------------------------------------------------
    // (1) 색상 이중 해시 집합
    // --------------------------------------------------
    let mut color_set = HashSet::new();
    for _ in 0..c {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let color = line.trim();
        // color 문자열의 전체 이중 해시 & 길이
        let ((h1, h2), length) = double_hash_str(color);
        color_set.insert(((h1, h2), length));
    }

    // --------------------------------------------------
    // (2) 닉네임 이중 해시 집합
    // --------------------------------------------------
    let mut name_set = HashSet::new();
    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let name = line.trim();
        // name 문자열의 전체 이중 해시 & 길이
        let ((h1, h2), length) = double_hash_str(name);
        name_set.insert(((h1, h2), length));
    }

    // --------------------------------------------------
    // (3) 질의(팀명) 처리
    // --------------------------------------------------
    line.clear();
    reader.read_line(&mut line).unwrap();
    let q: usize = line.trim().parse().unwrap();

    for _ in 0..q {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let team_name = line.trim();
        let team_len = team_name.len();

        // team_len == 0 이면 어차피 분할 불가능
        if team_len == 0 {
            writeln!(writer, "No").unwrap();
            continue;
        }

        // 팀명 prefix 해시 (이중)
        let (prefix_hashes1, prefix_hashes2) = build_prefix_double_hash(team_name);
        // BASE^i 테이블 (이중)
        let (pow_table1, pow_table2) = build_pow_table_double(team_len);

        let mut is_legend = false;

        // i 위치에서 자를 때:
        // 앞부분: [0..i-1] 길이가 i
        // 뒷부분: [i..team_len-1] 길이가 team_len - i
        // i는 최소 1부터 최대 team_len-1까지
        for i in 1..team_len {
            // (a) 앞부분 이중 해시
            let (pref_h1, pref_h2) = get_substring_double_hash(
                &prefix_hashes1,
                &prefix_hashes2,
                &pow_table1,
                &pow_table2,
                0,
                i - 1,
            );
            // (b) 뒷부분 이중 해시
            let (suf_h1, suf_h2) = get_substring_double_hash(
                &prefix_hashes1,
                &prefix_hashes2,
                &pow_table1,
                &pow_table2,
                i,
                team_len - 1,
            );

            // (c) 색상과 닉네임 집합에 각각 있는지 확인
            if color_set.contains(&((pref_h1, pref_h2), i))
                && name_set.contains(&((suf_h1, suf_h2), team_len - i))
            {
                is_legend = true;
                break;
            }
        }

        writeln!(writer, "{}", if is_legend { "Yes" } else { "No" }).unwrap();
    }
}

// --------------------------------------------------
// 문자열 전체 이중 해시 계산
// --------------------------------------------------
fn double_hash_str(s: &str) -> ((u64, u64), usize) {
    let mut h1 = 0u64;
    let mut h2 = 0u64;
    for &b in s.as_bytes() {
        h1 = (h1.wrapping_mul(BASE1)).wrapping_add(b as u64) % MOD1;
        h2 = (h2.wrapping_mul(BASE2)).wrapping_add(b as u64) % MOD2;
    }
    ((h1, h2), s.len())
}

// --------------------------------------------------
// 팀명 prefix 해시 (이중) 테이블 생성
// prefix_hashes1[i] = s[0..i]의 (mod1 기반) 해시
// prefix_hashes2[i] = s[0..i]의 (mod2 기반) 해시
// 주의: 실제 문자열 길이가 i인 prefix
// --------------------------------------------------
fn build_prefix_double_hash(s: &str) -> (Vec<u64>, Vec<u64>) {
    let mut ph1 = Vec::with_capacity(s.len() + 1);
    let mut ph2 = Vec::with_capacity(s.len() + 1);

    ph1.push(0);
    ph2.push(0);

    let mut cur1 = 0u64;
    let mut cur2 = 0u64;

    for &b in s.as_bytes() {
        cur1 = (cur1.wrapping_mul(BASE1)).wrapping_add(b as u64) % MOD1;
        cur2 = (cur2.wrapping_mul(BASE2)).wrapping_add(b as u64) % MOD2;

        ph1.push(cur1);
        ph2.push(cur2);
    }

    (ph1, ph2)
}

// --------------------------------------------------
// 거듭제곱 테이블(이중)
// pow_table1[i] = BASE1^i % MOD1
// pow_table2[i] = BASE2^i % MOD2
// --------------------------------------------------
fn build_pow_table_double(len: usize) -> (Vec<u64>, Vec<u64>) {
    let mut pw1 = vec![1u64; len + 1];
    let mut pw2 = vec![1u64; len + 1];

    for i in 1..=len {
        pw1[i] = (pw1[i - 1].wrapping_mul(BASE1)) % MOD1;
        pw2[i] = (pw2[i - 1].wrapping_mul(BASE2)) % MOD2;
    }

    (pw1, pw2)
}

// --------------------------------------------------
// prefix 해시 2개, pow_table 2개를 이용해
// s[l..r] (포함) 구간의 (hash1, hash2)를 반환
// --------------------------------------------------
fn get_substring_double_hash(
    prefix_hashes1: &[u64],
    prefix_hashes2: &[u64],
    pow_table1: &[u64],
    pow_table2: &[u64],
    l: usize,
    r: usize,
) -> (u64, u64) {
    // prefix_hashesX[r+1] = s[0..r] 해시
    // prefix_hashesX[l]   = s[0..l-1] 해시
    // => 부분 문자열 해시 = prefix_hashesX[r+1] - prefix_hashesX[l]*BASE^(r-l+1)
    let len = r - l + 1;

    // Hash1
    let left_h1 = prefix_hashes1[l];
    let right_h1 = prefix_hashes1[r + 1];
    let sub_h1 = right_h1
        .wrapping_sub((left_h1.wrapping_mul(pow_table1[len])) % MOD1)
        .wrapping_add(MOD1)
        % MOD1;

    // Hash2
    let left_h2 = prefix_hashes2[l];
    let right_h2 = prefix_hashes2[r + 1];
    let sub_h2 = right_h2
        .wrapping_sub((left_h2.wrapping_mul(pow_table2[len])) % MOD2)
        .wrapping_add(MOD2)
        % MOD2;

    (sub_h1, sub_h2)
}

// https://www.acmicpc.net/problem/19585
// 전설
#[test]
fn test_solve19585() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "4 3
red
blue
purple
orange
shift
joker
noon
5
redshift
bluejoker
purplenoon
orangeshift
whiteblue"
                .to_string(),
            want: "Yes
Yes
Yes
Yes
No"
            .to_string(),
        },
        TestCase {
            s: "1 1
a
a
2
aa
a"
            .to_string(),
            want: "Yes
No"
            .to_string(),
        },
        TestCase {
            s: "3 1
red
re
r
shift
3
redshift
reshift
rshift"
                .to_string(),
            want: "Yes
Yes
Yes"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve19585(&mut reader, &mut writer);

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
