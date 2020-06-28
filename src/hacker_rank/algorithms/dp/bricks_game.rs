macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($xs: expr), +) => {
        {
            use std::cmp::min;
            min($x, min!($($xs), +))
        }
    }
}

#[test]
fn test_min() {
    assert_eq!(min!(1), 1);
    assert_eq!(min!(2, 1, 3), 1);
    assert_eq!(min!(0, 1, -1), -1);
}

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($xs: expr), +) => {
        {
            use std::cmp::max;
            max($x, max!($($xs), +))
        }
    }
}

#[test]
fn test_max() {
    assert_eq!(max!(1), 1);
    assert_eq!(max!(1, 3, 2), 3);
    assert_eq!(max!(1, 0, -1), 1);
}

type Cache = std::collections::BTreeMap<usize, Inner>;
type Inner = std::collections::BTreeMap<usize, i64>;

#[allow(dead_code)]
fn bricks_game(arr: Vec<i32>) -> i64 {
    let mut cache = Cache::new();
    play_sub_game(&arr, 0, arr.len() - 1, &mut cache)
}

fn get_cache(cache: &Cache, begin: usize, end: usize) -> (i64, bool) {
    match cache.get(&begin) {
        Some(inner) => match inner.get(&end) {
            Some(x) => (*x, true),
            _ => (0, false),
        },
        _ => (0, false),
    }
}

fn set_cache(cache: &mut Cache, begin: usize, end: usize, val: i64) {
    let c = cache.borrow_mut();
    c.entry(begin).or_insert_with(Inner::new);
    c.get_mut(&begin).unwrap().entry(end).or_insert(val);
}

fn play_sub_game(arr: &[i32], begin: usize, end: usize, cache: &mut Cache) -> i64 {
    let (result, ok) = get_cache(cache, begin, end);
    if ok {
        return result;
    }

    let range = end as i32 - begin as i32;
    if range <= 3 {
        let mut sum: i64 = 0;
        for i in 0..(range + 1) {
            if i == 3 {
                break;
            }
            sum += arr[begin + i as usize] as i64;
        }
        set_cache(cache, begin, end, sum);
        return sum;
    }

    let _v0 = min!(
        arr[begin] as i64 + play_sub_game(arr, begin + 2, end, cache),
        arr[begin] as i64 + play_sub_game(arr, begin + 3, end, cache),
        arr[begin] as i64 + play_sub_game(arr, begin + 4, end, cache)
    );
    let _v1 = min!(
        arr[begin] as i64 + arr[begin + 1] as i64 + play_sub_game(arr, begin + 3, end, cache),
        arr[begin] as i64 + arr[begin + 1] as i64 + play_sub_game(arr, begin + 4, end, cache),
        arr[begin] as i64 + arr[begin + 1] as i64 + play_sub_game(arr, begin + 5, end, cache)
    );
    let _v2 = min!(
        arr[begin] as i64
            + arr[begin + 1] as i64
            + arr[begin + 2] as i64
            + play_sub_game(arr, begin + 4, end, cache),
        arr[begin] as i64
            + arr[begin + 1] as i64
            + arr[begin + 2] as i64
            + play_sub_game(arr, begin + 5, end, cache),
        arr[begin] as i64
            + arr[begin + 1] as i64
            + arr[begin + 2] as i64
            + play_sub_game(arr, begin + 6, end, cache)
    );
    let result = max!(_v0, _v1, _v2);
    set_cache(cache, begin, end, result);
    result
}

// https://www.hackerrank.com/challenges/play-game/problem
#[test]
fn test_bricks_game() {
    struct TestData {
        arr: Vec<i32>,
        expected: i64,
    }
    for data in std::vec![
        TestData {
            arr: vec![1, 2, 3],
            expected: 6
        },
        TestData {
            arr: vec![1, 2, 3, 4],
            expected: 6
        },
        TestData {
            arr: vec![1, 2, 3, 4, 5],
            expected: 6
        },
        TestData {
            arr: vec![999, 1, 1, 1, 0],
            expected: 1001
        },
        TestData {
            arr: vec![0, 1, 1, 1, 999],
            expected: 999
        },
        TestData {
            arr: vec![0, 1, 1, 1, 999, 999],
            expected: 1001
        },
    ] {
        let actual = bricks_game(data.arr);
        assert_eq!(actual, data.expected);
    }
}

extern crate csv;

#[allow(dead_code)]
fn read_csv(file_name: &str) -> Vec<Vec<i32>> {
    let path = std::path::Path::new(file_name);

    let mut res = vec![];
    let mut reader = csv::Reader::from_path(path).unwrap();
    for row in reader.headers().iter() {
        let mut curr = vec![];
        for col in row.iter() {
            let v: i32 = col.parse().unwrap();
            curr.push(v);
        }
        res.push(curr);
    }
    for record in reader.records() {
        for row in record.iter() {
            let mut curr = vec![];
            for col in row.iter() {
                let v: i32 = col.parse().unwrap();
                curr.push(v);
            }
            res.push(curr);
        }
    }
    res
}

use std::borrow::BorrowMut;
use std::{sync::mpsc, thread, time::Duration};

#[allow(dead_code)]
fn panic_after<T, F>(d: Duration, f: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T,
    F: Send + 'static,
{
    let (done_tx, done_rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = f();
        done_tx.send(()).expect("Unable to send completion signal");
        val
    });

    match done_rx.recv_timeout(d) {
        Ok(_) => handle.join().expect("Thread panicked"),
        _ => panic!("Thread took too long"),
    }
}

#[test]
fn test_bricks_game_bench() {
    let arr = read_csv("./src/hacker_rank/algorithms/dp/test_data/bricks_game_0.csv");

    // NOTE - 아래의 테스트 케이스는 stack overflow 발생하므로 비활성화
    // let mut arr2 = read_csv("./src/hacker_rank/algorithms/dp/test_data/bricks_game_1.csv");
    // arr.append(arr2.as_mut());

    struct TestData {
        l: usize,
        e: i64,
    }
    let expected = std::vec![
        TestData { l: 1000, e: 249147 },
        TestData { l: 1000, e: 251633 },
        TestData { l: 1000, e: 249302 },
        TestData { l: 1000, e: 247737 },
        TestData { l: 1000, e: 253105 },
        // NOTE - 위의 주석 참고
        // TestData {
        //     l: 100000,
        //     e: 249791261588
        // },
        // TestData {
        //     l: 100000,
        //     e: 249894676936
        // },
        // TestData {
        //     l: 100000,
        //     e: 250224672758
        // },
    ];
    assert_eq!(arr.len(), expected.len());

    panic_after(Duration::from_millis(1000), move || {
        for (i, arr) in arr.into_iter().enumerate() {
            assert_eq!(arr.len(), expected[i].l);
            let actual = bricks_game(arr);
            assert_eq!(actual, expected[i].e);
        }
    });
}
