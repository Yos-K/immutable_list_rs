use std::ops::Index;

/// `ImmutableList` represents an immutable singly linked list.
#[derive(Debug, PartialEq, Clone)]
struct ImmutableLst<T> {
    value: Option<T>,
    next: Option<Box<ImmutableLst<T>>>
}

impl<T> ImmutableLst<T> where T: Clone {
    pub fn new() -> Self {
        ImmutableLst { value: None, next: None }
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn prepend(&self, value: T) -> Self {
        match self.is_empty() {
            true => {
                ImmutableLst { 
                    value: Some(value), 
                    next: None
                }
            },
            false => {
                ImmutableLst { 
                    value: Some(value), 
                    next: Some(Box::new(self.clone())),
                }
            },
        }
    }
}

impl<T> Index<usize> for ImmutableLst<T> where T: Clone {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prepend() {
        let im_list = ImmutableLst::new();
        let new_list = im_list.prepend(1);
        assert_eq!(new_list, ImmutableLst{value: Some(1), next: None});
    }

    #[test]
    fn test_len() {
        let im_list = ImmutableLst::new();
        let new_list = im_list.prepend(1);
        assert_eq!(new_list.len(), 1);

        let new_list = new_list.prepend(2);
        assert_eq!(new_list.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let im_list = ImmutableLst::new();
        assert!(im_list.is_empty());

        let new_list = im_list.prepend(1);
        assert!(!new_list.is_empty());
    }
}