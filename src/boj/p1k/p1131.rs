use crate::utils::io::read_line;
use std::io::{BufRead, Write};

const MAX_SIZE: i64 = 1_000_001;
const ARR: [[i64; 16]; 7] = [
    [0; 16],                                          // K = 0 (placeholder)
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0], // K = 1
    [1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // K = 2
    [
        1, 55, 136, 153, 160, 370, 371, 407, 919, 0, 0, 0, 0, 0, 0, 0,
    ], // K = 3
    [
        1, 1138, 1634, 2178, 8208, 9474, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // K = 4
    [
        1, 244, 4150, 4151, 8294, 8299, 9044, 9045, 10933, 24584, 54748, 58618, 89883, 92727,
        93084, 194979,
    ], // K = 5
    [
        1, 17148, 63804, 93531, 239459, 282595, 548834, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // K = 6
];

#[allow(dead_code)]
fn solve1131(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, k) = read_values!(read_line(reader), i64, i64, i64);
    let mut dp = vec![0; MAX_SIZE as usize];
    for &val in ARR[k as usize].iter().take_while(|&&x| x != 0) {
        let mut n = val;
        loop {
            if n < MAX_SIZE {
                dp[n as usize] = val;
            }
            n = calc_s(k, n);
            if n == val {
                break;
            }
        }
    }

    let sum: i64 = (a..=b).map(|n| calc_min(k, n, &mut dp)).sum();
    write!(writer, "{}", sum).unwrap();
}

fn calc_s(k: i64, n: i64) -> i64 {
    let mut res = 0;
    let mut n = n;
    while n > 0 {
        let mut v = 1;
        for _ in 0..k {
            v *= n % 10;
        }
        res += v;
        n /= 10;
    }
    res
}

fn calc_min(k: i64, n: i64, dp: &mut [i64]) -> i64 {
    if n >= MAX_SIZE {
        return calc_min(k, calc_s(k, n), dp);
    }
    if dp[n as usize] > 0 {
        return dp[n as usize];
    }
    dp[n as usize] = calc_min(k, calc_s(k, n), dp).min(n);
    dp[n as usize]
}

// https://www.acmicpc.net/problem/1131
// 숫자
#[test]
fn test_solve1131() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 5 2".to_string(),
            want: "14".to_string(),
        },
        TestData {
            s: "10 99 1".to_string(),
            want: "450".to_string(),
        },
        TestData {
            s: "535 538 3".to_string(),
            want: "820".to_string(),
        },
        TestData {
            s: "100000 400000 6".to_string(),
            want: "5169721292".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve1131(reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}

extern crate rayon;
use crate::read_values;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

fn find_cycle_min(k: i64, n: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let mut seen: HashMap<i64, i64> = HashMap::new();
    let mut sequence: Vec<i64> = Vec::new();
    let mut x = n;
    let mut step = 0;

    loop {
        if let Some(&result) = memo.get(&x) {
            return result;
        }

        if let Some(prev_step) = seen.get(&x) {
            let min_cycle = sequence[*prev_step as usize..]
                .iter()
                .min()
                .unwrap()
                .to_owned();
            memo.insert(n, min_cycle);
            return min_cycle;
        }
        seen.insert(x, step);
        sequence.push(x);
        x = calc_s(k, x);
        step += 1;
    }
}

#[allow(dead_code)]
fn find_min_values(k: i64, n: i64) -> Vec<i64> {
    let unique_cycle_min_set = Mutex::new(HashSet::new());

    (1..=n).into_par_iter().for_each(|i| {
        let mut memo = HashMap::new();
        let cycle_min = find_cycle_min(k, i, &mut memo);
        let mut unique_minimums = unique_cycle_min_set.lock().unwrap();
        unique_minimums.insert(cycle_min);
    });

    let mut sorted_unique_minimums: Vec<i64> = unique_cycle_min_set
        .lock()
        .unwrap()
        .iter()
        .cloned()
        .collect();
    sorted_unique_minimums.sort();
    sorted_unique_minimums
}

#[test]
fn test_find_circle_minimal() {
    // [1, 2, 3, 4, 5, 6, 7, 8, 9]

    let got = find_min_values(1, 10);
    assert_eq!(got, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
#[ignore]
fn test_find_circle() {
    // [1, 2, 3, 4, 5, 6, 7, 8, 9]
    // [1, 4]
    // [1, 55, 136, 153, 160, 370, 371, 407, 919]
    // [1, 1138, 1634, 2178, 8208, 9474]
    // [1, 244, 4150, 4151, 8294, 8299, 9044, 9045, 10933, 24584, 54748, 58618, 89883, 92727, 93084, 194979]
    // [1, 17148, 63804, 93531, 239459, 282595, 548834]

    let k_values: Vec<i64> = vec![1, 2, 3, 4, 5, 6];
    let n: i64 = 1_000_000;

    let results: Vec<Vec<i64>> = k_values
        .par_iter()
        .map(|&k| find_min_values(k, n))
        .collect();

    for (k, min_values) in k_values.iter().zip(results.iter()) {
        println!("k = {}, min_values = {:?}", k, min_values);
    }

    assert_eq!(results[0], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(results[1], vec![1, 4]);
    assert_eq!(results[2], vec![1, 55, 136, 153, 160, 370, 371, 407, 919]);
    assert_eq!(results[3], vec![1, 1138, 1634, 2178, 8208, 9474]);
    assert_eq!(
        results[4],
        vec![
            1, 244, 4150, 4151, 8294, 8299, 9044, 9045, 10933, 24584, 54748, 58618, 89883, 92727,
            93084, 194979
        ]
    );
    assert_eq!(
        results[5],
        vec![1, 17148, 63804, 93531, 239459, 282595, 548834]
    );
}
