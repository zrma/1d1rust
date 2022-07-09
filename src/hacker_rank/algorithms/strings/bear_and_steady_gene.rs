#[allow(dead_code)]
fn steady_gene(s: &str) -> i32 {
    let gene = s.as_bytes();
    let length = gene.len() as i32;
    let cnt = length / 4;
    let mut arr = vec![-cnt, -cnt, -cnt, -cnt];

    for &c in gene {
        arr = increase(arr, c as char);
    }

    let mut begin: i32 = 0;
    let mut end: i32 = 0;
    let mut min = i32::MAX;
    while end < length - 1 {
        arr = decrease(arr, gene[end as usize] as char);
        end += 1;

        while arr.iter().all(|&num| num <= 0) {
            min = std::cmp::min(min, end - begin);
            arr = increase(arr, gene[begin as usize] as char);
            begin += 1;
        }
    }
    min
}

fn increase(arr: Vec<i32>, c: char) -> Vec<i32> {
    acc(arr, c, 1)
}

fn decrease(arr: Vec<i32>, c: char) -> Vec<i32> {
    acc(arr, c, -1)
}

fn acc(mut arr: Vec<i32>, c: char, step: i32) -> Vec<i32> {
    match c {
        'A' => arr[0] += step,
        'C' => arr[1] += step,
        'T' => arr[2] += step,
        _ => arr[3] += step,
    };
    arr
}

//noinspection SpellCheckingInspection
// https://www.hackerrank.com/challenges/bear-and-steady-gene/problem
#[test]
fn test_steady_gene() {
    {
        //noinspection SpellCheckingInspection
        let actual = steady_gene("GAAATAAA");
        let expected: i32 = 5;
        assert_eq!(actual, expected);
    }

    {
        //noinspection SpellCheckingInspection
        let actual = steady_gene("TGATGCCGTCCCCTCAACTTGAGTGCTCCTAATGCGTTGC");
        let expected: i32 = 5;
        assert_eq!(actual, expected);
    }
}
