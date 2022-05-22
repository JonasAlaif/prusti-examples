use prusti_contracts::*;

pub fn search(a: &[i32], key: i32) -> Option<usize> {
  let mut low: usize = 0;
  let mut high: usize = a.len();
  while low < high {
    // In all loop iterations:
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
    None => (),
    Some(_idx) => ()
  }
}