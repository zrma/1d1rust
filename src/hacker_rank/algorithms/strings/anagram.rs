use std::cmp::Ordering;

#[allow(dead_code)]
fn anagram(s: String) -> usize {
    let mut length = s.len();
    if length < 1 || length % 2 == 1 {
        return usize::MAX;
    }

    let (s1, s2) = s.split_at(length / 2);

    let mut v1 = s1.as_bytes().to_vec();
    let mut v2 = s2.as_bytes().to_vec();
    v1.sort_unstable();
    v2.sort_unstable();

    length /= 2;
    let mut eq = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < length && j < length {
        let lhs = v1.get(i).unwrap();
        let rhs = v2.get(j).unwrap();
        match lhs.cmp(rhs) {
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
            _ => {
                eq += 1;
                i += 1;
                j += 1;
            }
        }
    }

    length - eq
}

// noinspection SpellCheckingInspection
// https://www.hackerrank.com/challenges/anagram/problem
#[test]
fn test_anagram() {
    struct TestData {
        input: String,
        expected: usize,
    }
    for data in [
        TestData { input: "".to_string(), expected: usize::MAX },
        TestData { input: "aaabbb".to_string(), expected: 3 },
        TestData { input: "ab".to_string(), expected: 1 },
        TestData { input: "abc".to_string(), expected: usize::MAX },
        TestData { input: "mnop".to_string(), expected: 2 },
        TestData { input: "xyyx".to_string(), expected: 0 },
        TestData { input: "xaxbbbxx".to_string(), expected: 1 },
        TestData { input: "hhpddlnnsjfoyxpciioigvjqzfbpllssuj".to_string(), expected: 10 },
        TestData { input: "xulkowreuowzxgnhmiqekxhzistdocbnyozmnqthhpievvlj".to_string(), expected: 13 },
        TestData { input: "dnqaurlplofnrtmh".to_string(), expected: 5 },
        TestData { input: "aujteqimwfkjoqodgqaxbrkrwykpmuimqtgulojjwtukjiqrasqejbvfbixnchzsahpnyayutsgecwvcqngzoehrmeeqlgknnb".to_string(), expected: 26 },
        TestData { input: "lbafwuoawkxydlfcbjjtxpzpchzrvbtievqbpedlqbktorypcjkzzkodrpvosqzxmpad".to_string(), expected: 15 },
        TestData { input: "drngbjuuhmwqwxrinxccsqxkpwygwcdbtriwaesjsobrntzaqbe".to_string(), expected: usize::MAX },
        TestData { input: "ubulzt".to_string(), expected: 3 },
        TestData { input: "vxxzsqjqsnibgydzlyynqcrayvwjurfsqfrivayopgrxewwruvemzy".to_string(), expected: 13 },
        TestData { input: "xtnipeqhxvafqaggqoanvwkmthtfirwhmjrbphlmeluvoa".to_string(), expected: 13 },
        TestData { input: "gqdvlchavotcykafyjzbbgmnlajiqlnwctrnvznspiwquxxsiwuldizqkkaawpyyisnftdzklwagv".to_string(), expected: usize::MAX },
    ] {
        let actual = anagram(data.input);
        assert_eq!(actual, data.expected);
    }
}
