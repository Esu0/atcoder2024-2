use proconio::input;

fn cartesian_tree_with<T>(
    arr: &[T],
    mut f: impl FnMut(&T, &T) -> std::cmp::Ordering,
) -> Vec<usize> {
    let mut parents = vec![usize::MAX; arr.len()];
    if arr.is_empty() {
        return parents;
    }
    let mut stack = Vec::with_capacity(arr.len());
    stack.push(0);
    for (i, elem) in arr.iter().enumerate().skip(1) {
        use std::cmp::Ordering::*;
        let top = *stack.last().unwrap();
        if let Less = f(elem, &arr[top]) {
            let mut cur = top;
            stack.pop();
            while let Some(&top) = stack.last() {
                if let Less = f(elem, &arr[top]) {
                    stack.pop();
                    cur = top;
                } else {
                    break;
                }
            }
            parents[i] = parents[cur];
            parents[cur] = i;
        } else {
            parents[i] = top;
        }
        stack.push(i);
    }
    parents
}

// fn cartesian_tree<T: Ord>(arr: &[T]) -> Vec<usize> {
//     cartesian_tree_with(arr, Ord::cmp)
// }

fn cartesian_tree_with_key<T, K: Ord>(arr: &[T], mut f: impl FnMut(&T) -> K) -> Vec<usize> {
    cartesian_tree_with(arr, |a, b| f(a).cmp(&f(b)))
}

fn solve(tree: &[(usize, usize)], p: usize, a: &[i64]) -> (i64, Vec<i64>) {
    if p == usize::MAX {
        return (0, vec![]);
    }
    let (l, r) = tree[p];
    let mut v1 = solve(tree, l, a);
    let mut v2 = solve(tree, r, a);
    if v1.1.len() > v2.1.len() {
        std::mem::swap(&mut v1, &mut v2);
    }
    let (s1, v1) = v1;
    let (s2, mut v2) = v2;
    for (&v1i, dst) in v1.iter().zip(&mut v2) {
        *dst += v1i + a[p];
    }

    let d = v1.len();
    if v2.len() == d {
        v2.push(a[p] - s1 - s2);
    } else {
        v2[d] += a[p] - s1;
        v2.push(-s2);
    }
    for _ in 0..d {
        v2.push(-a[p]);
    }
    (a[p], v2)
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let t = cartesian_tree_with_key(&a, |&x| -x);
    // eprintln!("{t:?}");
    let mut tree = vec![(usize::MAX, usize::MAX); n];
    let mut r = usize::MAX;
    for (i, &pi) in t.iter().enumerate() {
        if pi == usize::MAX {
            r = i;
            continue;
        }
        if i < pi {
            tree[pi].0 = i;
        } else {
            tree[pi].1 = i;
        }
    }
    let (_, mut ans) = solve(&tree, r, &a);
    // eprintln!("{:?}", tree[1]);
    // eprintln!("{:?}", solve(&tree, 2, &a));
    for i in 0..n - 1 {
        ans[i + 1] += ans[i];
    }
    for &ai in &ans {
        println!("{ai}");
    }
}
