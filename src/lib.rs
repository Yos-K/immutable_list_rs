use std::ops::Index;
use std::iter::Iterator;
use std::slice::Iter;

/// `ImmutableList` represents an immutable singly linked list.
#[derive(Debug, PartialEq, Clone)]
struct ImmutableList<T> {
    value: Option<T>,
    next: Option<Box<ImmutableList<T>>>
}

impl<T> ImmutableList<T> where T: Clone {
    pub fn new() -> Self {
        ImmutableList { value: None, next: None }
    }

    pub fn len(&self) -> usize {
        match &self.next {
            Some(next) => 1 + next.len(),
            None => match &self.value {
                Some(_) => 1,
                None => 0,
            },
        }
    }

    pub fn index(&self, index: usize) -> Option<&T> {
        self.iter().enumerate()
        .find(|(i, _)| i == &index)
        .map(
            |(_, v)| v
        )
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn prepend(&self, value: T) -> Self {
        match self.is_empty() {
            true => {
                ImmutableList { 
                    value: Some(value), 
                    next: None
                }
            },
            false => {
                ImmutableList { 
                    value: Some(value), 
                    next: Some(Box::new(self.clone())),
                }
            },
        }
    }

    pub fn iter(&self) -> IterList<T> {
        IterList { current: Some(self) }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct IterList<'a, T>{
    current: Option<&'a ImmutableList<T>>,
}

impl<'a, T> Iterator for IterList<'a, T> where T: Clone {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(v) => {
                self.current = v.next.as_deref();
                v.value.as_ref()
            },
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prepend() {
        let im_list = ImmutableList::new();
        let new_list = im_list.prepend(1);
        assert_eq!(new_list, ImmutableList{value: Some(1), next: None});
    }

    #[test]
    fn test_len() {
        let im_list = ImmutableList::new();
        let new_list = im_list.prepend(1);
        assert_eq!(new_list.len(), 1);

        let new_list = new_list.prepend(2);
        assert_eq!(new_list.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let im_list = ImmutableList::new();
        assert!(im_list.is_empty());

        let new_list = im_list.prepend(1);
        assert!(!new_list.is_empty());
    }

    #[test]
    fn test_iter() {
        let im_list = ImmutableList::new();
        let new_list = im_list.prepend(1);
        let new_list = new_list.prepend(2);
        let mut iter = new_list.iter();
        dbg!(&new_list);
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        dbg!(&new_list);
    }

    #[test]
    fn test_index() {
        let im_list = ImmutableList::new();
        assert_eq!(im_list.index(0), None);

        let new_list = im_list.prepend(2);
        dbg!(&new_list);
        assert_eq!(new_list.index(0), Some(&2));
    }
}