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
#[allow(clippy::eq_op)]
fn test_min() {
    assert_eq!(min!(1), 1);
    assert_eq!(min!(2, 1, 3), 1);
    assert_eq!(min!(0, 1, -1), -1);
}

#[allow(dead_code)]
fn bricks_game(arr: Vec<i32>) -> i64 {
    let size = arr.len();
    if size <= 3 {
        return arr.iter().sum::<i32>() as i64;
    }

    let mut res = vec![0; size];
    let mut tot: i64 = 0;

    let last_idx = size - 1;

    for i in 0..3 {
        tot += arr[last_idx - i] as i64;
        res[last_idx - i] = tot;
    }

    for i in 3..(last_idx + 1) {
        tot += arr[last_idx - i] as i64;
        res[last_idx - i] = tot
            - min!(
                res[last_idx - i + 1],
                res[last_idx - i + 2],
                res[last_idx - i + 3]
            );
    }
    res[0]
}

// https://www.hackerrank.com/challenges/play-game/problem
#[test]
fn test_bricks_game() {
    struct TestData {
        arr: Vec<i32>,
        expected: i64,
    }
    for data in [
        TestData {
            arr: vec![1, 2, 3],
            expected: 6,
        },
        TestData {
            arr: vec![1, 2, 3, 4],
            expected: 6,
        },
        TestData {
            arr: vec![1, 2, 3, 4, 5],
            expected: 6,
        },
        TestData {
            arr: vec![999, 1, 1, 1, 0],
            expected: 1001,
        },
        TestData {
            arr: vec![0, 1, 1, 1, 999],
            expected: 999,
        },
        TestData {
            arr: vec![0, 1, 1, 1, 999, 999],
            expected: 1001,
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
    if let Ok(row) = reader.headers() {
        let mut curr = vec![];
        for col in row.iter() {
            let v: i32 = col.parse().unwrap();
            curr.push(v);
        }
        res.push(curr);
    }
    for row in reader.records().flatten() {
        let mut curr = vec![];
        for col in row.iter() {
            let v: i32 = col.parse().unwrap();
            curr.push(v);
        }
        res.push(curr);
    }
    res
}

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
    let mut arr = read_csv("./src/hacker_rank/algorithms/dp/test_data/bricks_game_0.csv");
    let mut arr2 = read_csv("./src/hacker_rank/algorithms/dp/test_data/bricks_game_1.csv");
    arr.append(arr2.as_mut());

    struct TestData {
        l: usize,
        e: i64,
    }
    let expected = vec![
        TestData { l: 1000, e: 249147 },
        TestData { l: 1000, e: 251633 },
        TestData { l: 1000, e: 249302 },
        TestData { l: 1000, e: 247737 },
        TestData { l: 1000, e: 253105 },
        TestData {
            l: 100000,
            e: 249791261588,
        },
        TestData {
            l: 100000,
            e: 249894676936,
        },
        TestData {
            l: 100000,
            e: 250224672758,
        },
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
