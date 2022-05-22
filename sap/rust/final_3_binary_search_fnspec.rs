use prusti_contracts::*;

predicate! {
  fn is_sorted(a: &[i32]) -> bool {
    forall(|idx1: usize, idx2: usize| 0 <= idx1 && idx1 < idx2 && idx2 < a.len() ==> a[idx1] <= a[idx2])
  }
}

#[requires(is_sorted(a))]
#[ensures(match result {
  // If we returned `None` the `key` must not be in the slice:
  None => forall(|idx: usize| 0 <= idx && idx < a.len() ==> a[idx] != key),
  // If we returned `Some(idx)` then `idx` must point to an element equal to `key`:
  Some(idx) => a[idx] == key,
})]
pub fn search(a: &[i32], key: i32) -> Option<usize> {
  let mut low: usize = 0;
  let mut high: usize = a.len();
  while low < high {
    body_invariant!(high <= a.len());
    // All elements left of `low` must be smaller than `key`:
    body_invariant!(forall(|idx: usize| 0 <= idx && idx < low ==> a[idx] < key));
    // All elements right of `high` must be larger than `key`:
    body_invariant!(forall(|idx: usize| high <= idx && idx < a.len() ==> a[idx] > key));
    let mid: usize = low + (high-low)/2;
    let mid_val: i32 = a[mid];
    if mid_val < key {
      low = mid + 1;
    } else if mid_val > key {
      high = mid;
    } else {
      return Some(mid);
    }
  }
  None
}

pub fn main() {
  let a = [4, 8, 15, 16, 23, 42];
  match search(&a, 42) {
    // Panic if `search` implemented wrong
    None => panic!(),
    Some(idx) => {
      // Panic if `search` implemented wrong
      assert!(idx == 5);
      // Out of bounds if `idx >= a.len()`
      let the_answer = a[idx];
      assert!(the_answer == 42);
    }
  }
}