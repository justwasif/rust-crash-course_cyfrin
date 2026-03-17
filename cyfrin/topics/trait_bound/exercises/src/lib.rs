use std::cmp::PartialOrd;

pub fn min<T:PartialOrd>(x:T, y: T) -> T {
    if x <= y { x } else { y }
}

pub fn zip<B:Copy,A:Copy>(a: Vec<B>, b: Vec<A>) -> Vec<(B, A)> {
    let mut v = vec![];
    let len = min(a.len(), b.len());

    for i in 0..len {
        v.push((a[i], b[i]));
    }
    v
}
