//Implementation of a parse tree in rust



fn main() {

    println!("Welcome to parse_tree tester\n");
    let string: &str = "24+8*";
    let mut tester = ParseTree::new();
    tester = tester.fill_tree(string);

    println!("{}", ParseTree::print_tree(tester.root));

    println!("Thank you for stopping by! :)");
}

//pub mod parse_tree {
    
    type Link = Option<Box<Node>>;

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

    pub struct ParseTree {
        root: Link,
    }

    impl ParseTree {
        pub fn new() -> Self {
            ParseTree { root: None }
        }

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
            println!("Finished for");

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
    }

    pub fn is_operator(value: char) -> bool {
        if value == '-' || value == '+' || value == '/' || value == '*' {
            return true;
        } else {
            return false;
        }
    }
//}
