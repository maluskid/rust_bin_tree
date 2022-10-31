// Module for parse tree written by Dominic Maluski
// my first Rust program, and first attempt at real documentation
// the end goal is to build a module that can perform actual useful
// operations with a binary tree, specifically build one from a string
// and output it in a way that shows all the links using UTF-8
// characters. Enjoy!

//constants for readability in printing function
const VLINE: &str = "\u{2502}";
const DASH_VLINE: &str = "\u{2506}";
const HLINE: &str = "\u{2500}";
const DASH_HLINE: &str = "\u{2504}";
const ELBOW: &str = "\u{2514}";
const T_INT: &str = "\u{251C}";


//renaming wrapped Node pointer for readability
type Link = Option<Box<Node>>;


// Node struct for use in tree
    pub struct Node {
        left: Link,
        right: Link,
        value: char,
    }

    impl Node {
        pub fn new(value: char) -> Self {
            Node {
                left: None,
                right: None,
                value,
            }
        }
    }

    //Tree struct holds pointer to root node
    pub struct ParseTree {
        root: Link,
    }

    impl ParseTree {

        pub fn new() -> Self {
            ParseTree { root: None }
        }

        pub fn get_root(self) -> Link { self.root }

        pub fn fill_tree(self, data: &str) -> Self {
            let mut stack = Vec::new();
            let mut input = false;
            for i in data.chars() {
                print!("{}", i);
                if i != ' ' {
                    input = true;

                    if !is_operator(i) {
                        let new_node = Box::new(Node::new(i));
                        stack.push(new_node);
                    } else {
                        let new_node = Box::new(Node {
                            value: i,
                            right: stack.pop(),
                            left: stack.pop(),
                        });

                        stack.push(new_node);
                    }
                }
            }

            if input {
                return ParseTree { root: stack.pop() };
            } else {
                return ParseTree { root: None };
            }
        }

        pub fn print_tree(root: Link) -> String {
            
            match root {
                None => "".to_string(),
                Some(node) => {
                    let mut out = String::new();
                    out.push_str("/\n");
                    out.push_str(Self::print_tree(node.left).as_str());
                    out.push_str(Self::print_tree(node.right).as_str());
                    out.push_str(node.value.to_string().as_str());
                    out
                }
            }
        }

        // pub fn dispaly_tree(root: Link, counter: u32) -> () {

        //     let mut buffer = String::new();
        //     if let Some(node) = root {
        //         if counter != 0 {
        //             buffer.push_str(node.value.to_string().as_str());
        //             buffer.push_str("\n");
        //             buffer.push_str(VLINE);
        //         }
        //         else {
        //             buffer
        //         }
        //     }
        // }
    }

    pub fn is_operator(value: char) -> bool {
        if value == '-' || value == '+' || value == '/' || value == '*' {
            return true;
        } else {
            return false;
        }
    }