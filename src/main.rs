// Implementation of a parse tree in rust
// See parse_tree.rs for more info
// Written by Dominic Maluski

mod parse_tree;
use parse_tree::ParseTree;

fn main() {

    println!("Welcome to parse_tree tester\n");
    let string: &str = "24+8*9-6/";
    let mut tester = ParseTree::new();
    tester = tester.fill_tree(string);

    //println!("{}", ParseTree::print_tree(tester.get_root()));
    
    ParseTree::display_tree(tester.get_root());

    println!("Thank you for stopping by! :)");
}


