#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::HashMap;
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut m = (b'a'..=b'z')
        .map(|c| c as char)
        .map(|c| (c, 50))
        .collect::<HashMap<char, i32>>();
    for i in 0..n {
        let mut m_s = (b'a'..=b'z')
            .map(|c| c as char)
            .map(|c| (c, 0))
            .collect::<HashMap<char, i32>>();
        for j in s[i].iter() {
            let counter = m_s.entry(*j).or_insert(0);
            *counter += 1;
        }
        for k in b'a'..=b'z' {
            let m_s_value = m_s.get_mut(&(k as char)).unwrap();
            let m_value = m.get_mut(&(k as char)).unwrap();
            if m_value > m_s_value {
                *m_value = *m_s_value;
            }
        }
    }
    for i in b'a'..=b'z' {
        let val = m.get(&(i as char)).unwrap();
        if val > &0 && val != &50 {
            for _ in 0..*val {
                print!("{}", i as char);
            }
        }
    }
}
