use crate::utils::io::read_line;
use std::io::{BufRead, Write};

const MAX_SIZE: usize = 1_000_001;
const CYCLE_TABLE: [[usize; 16]; 7] = [
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
    let (a, b, k) = read_values_as!(read_line(reader), usize, usize, usize);
    let mut dp = vec![0; MAX_SIZE];
    for &val in CYCLE_TABLE[k].iter().take_while(|&&x| x != 0) {
        let mut n = val;
        loop {
            if n < MAX_SIZE {
                dp[n] = val;
            }
            n = calc_s(k, n);
            if n == val {
                break;
            }
        }
    }

    let ans: usize = (a..=b).map(|n| calc_min(k, n, &mut dp)).sum();
    write!(writer, "{}", ans).expect("write! should work");
}

fn calc_s(k: usize, n: usize) -> usize {
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

fn calc_min(k: usize, n: usize, dp: &mut [usize]) -> usize {
    if n >= MAX_SIZE {
        return calc_min(k, calc_s(k, n), dp);
    }
    if dp[n] > 0 {
        return dp[n];
    }
    dp[n] = calc_min(k, calc_s(k, n), dp).min(n);
    dp[n]
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

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}

extern crate rayon;
use crate::read_values_as;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

fn find_min_cycle(k: usize, n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let mut seen: HashMap<usize, usize> = HashMap::new();
    let mut sequence: Vec<usize> = Vec::new();
    let mut x = n;
    let mut step = 0;

    loop {
        if let Some(&result) = memo.get(&x) {
            return result;
        }

        if let Some(prev_step) = seen.get(&x) {
            let min_cycle = sequence[*prev_step..].iter().min().unwrap().to_owned();
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
fn find_min_cycles(k: usize, n: usize) -> Vec<usize> {
    let unique_cycle_min_set = Mutex::new(HashSet::new());

    (1..=n).into_par_iter().for_each(|i| {
        let mut memo = HashMap::new();
        let cycle_min = find_min_cycle(k, i, &mut memo);
        let mut unique_minimums = unique_cycle_min_set.lock().unwrap();
        unique_minimums.insert(cycle_min);
    });

    let mut sorted_unique_minimums: Vec<usize> = unique_cycle_min_set
        .lock()
        .unwrap()
        .iter()
        .cloned()
        .collect();
    sorted_unique_minimums.sort();
    sorted_unique_minimums
}

#[test]
fn test_find_min_cycles() {
    // [1, 2, 3, 4, 5, 6, 7, 8, 9]

    let got = find_min_cycles(1, 10);
    assert_eq!(got, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
#[ignore]
fn test_find_whole_min_cycles() {
    // [1, 2, 3, 4, 5, 6, 7, 8, 9]
    // [1, 4]
    // [1, 55, 136, 153, 160, 370, 371, 407, 919]
    // [1, 1138, 1634, 2178, 8208, 9474]
    // [1, 244, 4150, 4151, 8294, 8299, 9044, 9045, 10933, 24584, 54748, 58618, 89883, 92727, 93084, 194979]
    // [1, 17148, 63804, 93531, 239459, 282595, 548834]

    let k_values: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
    let n: usize = 1_000_000;

    let whole_min_cycles: Vec<Vec<usize>> = k_values
        .par_iter()
        .map(|&k| find_min_cycles(k, n))
        .collect();

    for (k, min_cycles) in k_values.iter().zip(whole_min_cycles.iter()) {
        println!("k = {}, min_values = {:?}", k, min_cycles);
    }

    assert_eq!(whole_min_cycles[0], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(whole_min_cycles[1], vec![1, 4]);
    assert_eq!(
        whole_min_cycles[2],
        vec![1, 55, 136, 153, 160, 370, 371, 407, 919]
    );
    assert_eq!(whole_min_cycles[3], vec![1, 1138, 1634, 2178, 8208, 9474]);
    assert_eq!(
        whole_min_cycles[4],
        vec![
            1, 244, 4150, 4151, 8294, 8299, 9044, 9045, 10933, 24584, 54748, 58618, 89883, 92727,
            93084, 194979
        ]
    );
    assert_eq!(
        whole_min_cycles[5],
        vec![1, 17148, 63804, 93531, 239459, 282595, 548834]
    );
}
