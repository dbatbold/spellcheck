#[derive(Debug)]
pub struct Tree {
    root: Node,
}

#[derive(Debug, Clone)]
pub struct Node {
    letter: char,
    count: i64,
    nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            root: Node::new('R'),
        }
    }

    pub fn parse(self: &mut Tree, text: Vec<char>) {
        let mut node = &mut self.root;
        let mut whitespace = true;
        for n in 0..text.len() {
            let letter = text[n].to_ascii_lowercase();
            if letter.is_alphabetic() {
                whitespace = false;
            } else {
                match letter {
                    '-' => (),
                    _ => {
                        // End of a word
                        if !whitespace {
                            whitespace = true;
                            node.count += 1;
                            node = &mut self.root;
                        }
                        continue;
                    }
                }
            }
            if node.nodes.len() == 0 {
                node.nodes.push(Node::new(letter));
                let last = node.nodes.len() - 1;
                node = &mut node.nodes[last];
                continue;
            }
            for i in 0..node.nodes.len() {
                if letter == node.nodes[i].letter {
                    node = &mut node.nodes[i];
                    if n + 1 == text.len() {
                        // Last letter
                        node.count += 1;
                    }
                    break;
                }
                if i + 1 == node.nodes.len() {
                    // Last node
                    node.nodes.push(Node::new(letter));
                }
            }
        }
    }

    pub fn count(self: &mut Tree, word: &str) -> i64 {
        let text: Vec<char> = word.to_string().chars().collect();
        let mut node = &mut self.root;
        for n in 0..text.len() {
            let letter = text[n].to_ascii_lowercase();
            for i in 0..node.nodes.len() {
                if letter == node.nodes[i].letter {
                    node = &mut node.nodes[i];
                    break;
                }
            }
        }
        node.count
    }
}

impl Node {
    pub fn new(letter: char) -> Self {
        Self {
            nodes: vec![],
            letter: letter,
            count: 0,
        }
    }
}

#[test]
fn test_parse() {
    let mut tree = Tree::new();
    tree.parse("abc abc".chars().collect());
    assert_eq!(2, tree.count("abc"));
}

#[test]
fn test_uppercase() {
    let mut tree = Tree::new();
    tree.parse("Abc aBc abC".chars().collect());
    assert_eq!(3, tree.count("abc"));
}

#[test]
fn test_whitespaces() {
    let mut tree = Tree::new();
    tree.parse(" \t\r\nabc \t abc \r  abc  \t\n".chars().collect());
    assert_eq!(3, tree.count("abc"));
}
