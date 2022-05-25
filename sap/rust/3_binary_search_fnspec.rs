use prusti_contracts::*;

// predicate! {
//   fn is_sorted(a: &[i32]) -> bool {
//     forall(|idx1: usize, idx2: usize| 0 <= idx1 && idx1 < idx2 && idx2 < a.len() ==> a[idx1] <= a[idx2])
//   }
// }
// #[requires(is_sorted(a))]

/// Binary searches a sorted slice for a given element.
///
/// If the value is found then `Some` is returned, containing the
/// index of the matching element. If there are multiple matches, then any
/// one of the matches could be returned.
/// 
/// If the value is not found then `None` is returned.

// #[ensures(match result {
//   Some(idx) => a[idx] == key,
//   None => forall(|idx: usize| 0 <= idx && idx < a.len() ==> a[idx] != key),
// })]
pub fn search(a: &[i32], key: i32) -> Option<usize> {
  let mut low: usize = 0;
  let mut high: usize = a.len();
  while low < high {
    body_invariant!(high <= a.len());
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