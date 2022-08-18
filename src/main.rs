mod spell;

use crate::spell::Tree;

fn main() {
    let mut tree = Tree::new();
    tree.parse("abc abc".chars().collect());
    println!("{}", tree.count("abc"));
}
