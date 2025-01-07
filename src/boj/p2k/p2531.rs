use crate::utils::io::read_line;
use crate::{read_values_as, utils::io::read_value};
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2531(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, d, k, c) = read_values_as!(read_line(reader), usize, usize, usize, usize);

    let mut sushi = Vec::with_capacity(n);
    for _ in 0..n {
        sushi.push(read_value(read_line(reader)));
    }

    writeln!(writer, "{}", max_different_sushi(&sushi, d, k, c)).unwrap();
}

/// 회전 벨트에서 연속 k개를 골랐을 때 얻을 수 있는 최대 초밥 종류 수를 구한다.
/// 쿠폰 번호(c)에 해당하는 초밥이 포함되지 않았다면 +1을 해서 최대값을 갱신한다.
fn max_different_sushi(sushi: &[usize], d: usize, k: usize, c: usize) -> usize {
    let n = sushi.len();
    let mut eaten = vec![0; d + 1];
    let mut count = 0;

    // 초기 k개의 초밥을 먹음
    for i in 0..k {
        if eaten[sushi[i]] == 0 {
            count += 1;
        }
        eaten[sushi[i]] += 1;
    }

    // 쿠폰으로 받은 초밥이 포함되어 있지 않다면 +1
    let mut max_count = if eaten[c] == 0 { count + 1 } else { count };

    // 슬라이딩 윈도우로 나머지 경우 확인
    for i in 0..n {
        // 이전 초밥 제거
        eaten[sushi[i]] -= 1;
        if eaten[sushi[i]] == 0 {
            count -= 1;
        }

        // 새로운 초밥 추가
        let next = (i + k) % n;
        if eaten[sushi[next]] == 0 {
            count += 1;
        }
        eaten[sushi[next]] += 1;

        // 쿠폰으로 받은 초밥이 포함되어 있지 않다면 +1
        let current = if eaten[c] == 0 { count + 1 } else { count };
        max_count = max_count.max(current);
    }

    max_count
}

#[allow(dead_code)]
/// 해시맵을 이용한 풀이
fn solve2531_hashmap(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, _, k, c) = read_values_as!(read_line(reader), usize, usize, usize, usize);

    let mut sushi = Vec::with_capacity(n);
    for _ in 0..n {
        sushi.push(read_value(read_line(reader)));
    }

    writeln!(writer, "{}", max_different_sushi_hashmap(&sushi, k, c)).unwrap();
}

fn max_different_sushi_hashmap(sushi: &[usize], k: usize, c: usize) -> usize {
    let n = sushi.len();
    let mut map: HashMap<usize, usize> = HashMap::new();

    // 초기 k개의 초밥을 먹음
    for &s in sushi.iter().take(k) {
        *map.entry(s).or_insert(0) += 1;
    }

    let mut max_count = map.len() + if !map.contains_key(&c) { 1 } else { 0 };

    // 슬라이딩 윈도우로 나머지 경우 확인
    for i in 0..n {
        // 이전 초밥 제거
        *map.entry(sushi[i]).or_insert(0) -= 1;
        if *map.get(&sushi[i]).unwrap() == 0 {
            map.remove(&sushi[i]);
        }

        // 새로운 초밥 추가
        let next = (i + k) % n;
        *map.entry(sushi[next]).or_insert(0) += 1;

        let current = map.len() + if !map.contains_key(&c) { 1 } else { 0 };
        max_count = max_count.max(current);
    }

    max_count
}

// https://www.acmicpc.net/problem/2531
// 회전 초밥
#[test]
fn test_solve2531() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "8 30 4 30
7
9
7
30
2
7
9
25"
            .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "8 50 4 7
2
7
9
25
7
9
7
30"
            .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve2531(&mut reader, &mut writer);

            let got = String::from_utf8(writer).unwrap();
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }

        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve2531_hashmap(&mut reader, &mut writer);

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
}
