// Change "Settings > Prusti Assistant > Build Channel > LatestDev"
// Add "Settings > Prusti Assistant > Extra Prusti Env > PRUSTI_ENABLE_TYPE_INVARIANTS = true"

use prusti_contracts::*;
use std::cmp::{Ord, Ordering::{self, Equal, Less, Greater}};

pub fn main() {
    let mut tree = Tree::Empty;
    tree.insert(0);
    // The tree shold contain the value (not be Empty)
    // and shouldn't contain any other values, thus the
    // root should be equal to the value we put in!
    if let Tree::Node(value, _, _) = tree {
        let zero = 0;
        assert!(value.cmp(&zero).is_eq());
    } else {
        unreachable!()
    }
}

#[invariant(self.is_bst())]
pub enum Tree<T: Ord> {
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
    Empty,
}

impl<T: Ord> Tree<T> {
    // Contains function that is efficient but only
    // works properly if the input tree is a BST
    #[pure]
    pub fn contains(&self, find_value: &T) -> bool {
        if let Tree::Node(value, left, right) = self {
            match find_value.cmp(value) {
                Equal => true,
                Less => left.contains(find_value),
                Greater => right.contains(find_value),
            }
        } else { false }
    }

    // Prusti perdicate which checks if a given tree
    // is a BST (used in the invariant)
    predicate! {
    pub fn is_bst(&self) -> bool {
        if let Tree::Node(value, left, right) = self {
            forall(|i: &T| left.contains(i) == (i.cmp(value).is_lt() && self.contains(i))) &&
            forall(|i: &T| right.contains(i) == (i.cmp(value).is_gt() && self.contains(i)))
        } else { true }
    }
    }

    // An insert function which puts the `new_value` in
    // the correct place as expected by a BST
    #[ensures(self.contains(&new_value))]
    #[ensures(
        forall(|i: &T|
            !new_value.cmp(i).is_eq()
                ==> self.contains(i) == old(self).contains(i)
        )
    )]
    pub fn insert(&mut self, new_value: T) {
        if let Tree::Node(value, left, right) = self {
            match new_value.cmp(value) {
                Equal => (),
                Less => left.insert(new_value),
                Greater => right.insert(new_value),
            }
        } else {
            *self = Tree::Node(
                new_value,
                Box::new(Tree::Empty),
                Box::new(Tree::Empty)
            )
        }
    }








    // An example of `get_left` for a BST
    #[requires(matches!(self, Tree::Node(..)))]
    #[assert_on_expiry(
        // Must hold before result can expire
        if let Tree::Node(value, _, _) = old(self) {
            forall(|i: &T| result.contains(i) ==> i.cmp(value).is_lt())
        } else { false },
        // A postcondition of `get_left` after result expires
        if let Tree::Node(old_value, _, old_right) = old(&*self) {
            if let Tree::Node(value, left, right) = &*self {
                old_value.cmp(value).is_eq() &&
                forall(|i: &T| before_expiry(result).contains(i) == left.contains(i)) &&
                forall(|i: &T| old_right.contains(i) == right.contains(i))
            } else { false }
        } else { false }
    )]
    pub fn get_left(&mut self) -> &mut Self {
        if let Tree::Node(_, left, _) = self { left } else { panic!() }
    }
}

// Specify relationship between `Ordering` and rust integer operators
#[extern_spec]
impl Ord for i32 {
    #[pure]
    #[ensures(result.is_eq() == (*self == *other))]
    #[ensures(result.is_lt() == (*self < *other))]
    #[ensures(result.is_gt() == (*self > *other))]
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
