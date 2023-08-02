use std::{ptr::NonNull, fmt::Debug};

#[derive(Debug)]
pub struct LinkedList<T> {
    data: Option<T>,
    next: Option<NonNull<LinkedList<T>>>,
}


impl<T: Ord + Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList::<T> {
            data: None,
            next: None,
        }
    }

    pub fn new_with_data(data: T) -> Self {
        LinkedList::<T> {
            data: Some(data),
            next: None,
        }
    }

    #[allow(dead_code)]
    pub fn push_front(&mut self, data: T) {
        // Use box for allocate on Heap memory
        let new_node = Box::new(Self::new_with_data(data));

        let mut new_node_ptr: NonNull<LinkedList<T>> = Box::leak(new_node).into();
        match self.next {
            Some(next_node) => {
                unsafe {
                    new_node_ptr.as_mut().next = Some(next_node);
                }
                self.next = Some(new_node_ptr);
            }
            None => {}
        }
        self.next = Some(new_node_ptr);
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Self::new_with_data(data));
        let new_node_ptr: NonNull<LinkedList<T>> = Box::leak(new_node).into();

        match self.next {
            Some(exist_next) => {
                unsafe {
                    let mut child_ptr = Box::leak(Box::from(exist_next)).as_mut();
                    while let Some(mut child) = child_ptr.next {
                        child_ptr = child.as_mut();
                    }
                    child_ptr.next = Some(new_node_ptr);
                }
            }
            None => {
                self.next = Some(new_node_ptr);
            }
        }
    }

    pub fn print_content(&self) {
        let mut ptr_print = &self.next;
        while let Some(mut child) = ptr_print {
            unsafe {
                match &child.as_mut().data {
                    Some(d) => {
                        print!("{:?}, ",d);
                    },
                    None => {},
                }
                ptr_print = &child.as_mut().next;
            }
        }
    }
}
