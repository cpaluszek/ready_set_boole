use std::collections::HashSet;

// A powerset of S is the set of all subsets of S
// including the empty set and S.
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut powerset = Vec::with_capacity(1 << set.len());

    for i in 0..1 << set.len() {
        let mut subset = Vec::new();
        for (j, val) in set.iter().enumerate() {
            if (i >> j) & 1 == 1 {
                subset.push(*val);
            }
        }
        powerset.push(subset);
    }

    powerset
}

pub fn has_duplicate(set: &Vec<i32>) -> bool {
    let mut seen = HashSet::with_capacity(set.len());
    for &val in set {
        if !seen.insert(val) {
            return true;
        }
    }
    false
}
