// Change "Settings > Prusti Assistant > Build Channel > LatestDev"
// Add "Settings > Prusti Assistant > Extra Prusti Env > PRUSTI_ENABLE_TYPE_INVARIANTS = true"

use prusti_contracts::*;
use std::cmp::{Ord, Ordering::{self, Equal, Less, Greater}};

pub fn main() {
    let mut tree = Tree::Empty;
    tree.insert(0);
    if let Tree::Node(value, _, _) = tree {
        let zero = 0;
        assert!(value.cmp(&zero).is_eq());
    } else {
        unreachable!()
    }
}

pub enum Tree<T: Ord> {
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
    Empty,
}

impl<T: Ord> Tree<T> {
    pub fn contains(&self, find_value: &T) -> bool {
        if let Tree::Node(value, left, right) = self {
            match find_value.cmp(value) {
                Equal => todo!(),
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }

    pub fn is_bst(&self) -> bool {
        if let Tree::Node(value, left, right) = self {
            todo!()
        } else {
            todo!()
        }
    }

    pub fn insert(&mut self, new_value: T) {
        if let Tree::Node(value, left, right) = self {
            match new_value.cmp(value) {
                Equal => todo!(),
                Less => todo!(),
                Greater => todo!(),
            }
        } else {
            // Tree::Node(new_value, Box::new(Tree::Empty), Box::new(Tree::Empty))
            todo!()
        }
    }
}




























// Specify relationship between `Ordering` and rust integer operators
#[extern_spec]
impl Ord for i32 {
    #[pure]
    #[ensures(matches!(result, Equal) == (*self == *other))]
    #[ensures(matches!(result, Less) == (*self < *other))]
    #[ensures(matches!(result, Greater) == (*self > *other))]
    fn cmp(&self, other: &Self) -> Ordering;
}

// Specify Symmetry and Transitivity of `ord`
#[extern_spec]
trait Ord {
    #[pure]
    #[ensures(match (result, other.cmp(self)) {
        (Equal, Equal) |
        (Less, Greater) |
        (Greater, Less) => true,
        _ => false,
    })]
    #[ensures(forall(|x: &Self| match (result, other.cmp(x), self.cmp(x)) {
        (Equal, Equal, Equal) => true,
        (Equal, Less, Less) => true,
        (Equal, Greater, Greater) => true,
        (Less, Equal, Less) => true,
        (Less, Less, Less) => true,
        (Less, Greater, _) => true,
        (Greater, Equal, Greater) => true,
        (Greater, Less, _) => true,
        (Greater, Greater, Greater) => true,
        _ => false,
    }))]
    fn cmp(&self, other: &Self) -> Ordering;
}

// Helper functions for working with Ordering
#[extern_spec]
impl Ordering {
    #[pure]
    #[ensures(result == matches!(self, Equal))]
    fn is_eq(self) -> bool;
    #[pure]
    #[ensures(result == matches!(self, Less))]
    fn is_lt(self) -> bool;
    #[pure]
    #[ensures(result == matches!(self, Greater))]
    fn is_gt(self) -> bool;
}
