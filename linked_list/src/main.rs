use std::{env::current_exe, path::Component::CurDir};

fn main() {
    println!("Linked List Implementation");

    // New instance of LL
    let mut my_list = LinkedList::new();
    my_list.push_front(10);
    my_list.push_back(20);
    my_list.push_back(30);
    my_list.print_linked_list();

    println!("Contains 20 ?? {}", my_list.contains(20));
    println!("Deleting 20");
    my_list.delete(20);
    my_list.print_linked_list();
}


// Node struct representing each element
struct Node {
    value: i32, 
    // Every Node can have a sub or not (Option type)
    next: Option<Box<Node>>,
}


// linked list with a head pointer
struct LinkedList {
    head: Option<Box<Node>>,
}

// implementations
impl LinkedList {

    fn new() -> Self {
        Self {
            head: None
        }
    }

    fn push_front(&mut self, val: i32) {

        // a new instance of node
        let new_node = Box::new(
            Node {
                value: val,
                next: self.head.take(),
            }
        );

        self.head = Some(new_node);
    }

    fn push_back(&mut self, val: i32) {

        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(
            Box::new(
                Node {
                    value: val,
                    next: None
                }
            )
        )
    }


    fn delete(&mut self, val: i32) -> bool {
        let mut current = &mut self.head;
        loop {
            match current {
                None => return false,
                Some(node) if node.value == val => {
                    *current = node.next.take();
                    return true;
                },
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }

    fn contains(&self, val: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == val {
                return true;
            }
            current = &node.next;
        }

        false
    }

    fn print_linked_list(&self) {
        let mut current = &self.head;
        print!("List: ");
        while let Some(node) = current {
            print!("{}-> ", node.value);
            current = &node.next;
        }

        println!("None");
    }
}


// Implement Display trait for Linked
impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = &self.head;
        while let Some(node) = current {
            write!(f, "{}-> ", node.value)?;
            current = &node.next;
        }
        write!(f, "None")
    }
}