use prusti_contracts::*;
use std::cmp::{Ord, Ordering::{self, Equal, Less, Greater}};

pub fn main() {
    let mut tree = Tree::Empty;
    tree.insert(0);
    
    let left = tree.get_left();
    left.insert(5);

    let five = 5;
    assert!(tree.contains(&five));
}

pub enum Tree<T: Ord> {
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
    Empty,
}

impl<T: Ord> Tree<T> {
    pub fn get_left(&mut self) -> &mut Self {
        todo!()
    }


    #[pure]
    pub fn contains(&self, find_value: &T) -> bool {
        if let Tree::Node(value, left, right) = self {
            match find_value.cmp(value) {
                Equal => true,
                _ => left.contains(find_value)
                    || right.contains(find_value),
            }
        } else { false }
    }

    #[ensures(self.contains(&new_value))]
    #[ensures(
        forall(|i: &T|
            !matches!(new_value.cmp(i), Equal)
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
}















#[extern_spec]
impl Ord for i32 {
    #[pure]
    #[ensures(matches!(result, Equal) == (*self == *other))]
    #[ensures(matches!(result, Less) == (*self < *other))]
    #[ensures(matches!(result, Greater) == (*self > *other))]
    fn cmp(&self, other: &Self) -> Ordering;
}

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
