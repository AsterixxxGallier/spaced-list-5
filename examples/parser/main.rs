use std::ops::Range as IntRange;

use spaced_list_5::{RangeSpacedList, SpacedList, HollowRangeSpacedList, Position, Range};

struct Source {
    content: String,
    listeners: Vec<&'static dyn Fn(&str, Change)>,
}

enum Change {
    /// Contains the index range of the added text
    Addition(IntRange<usize>),
    /*/// Contains the index range of the deleted text
    Deletion(IntRange<usize>),*/
}

impl Source {
    fn new(content: String) -> Self {
        Self {
            content,
            listeners: vec![],
        }
    }

    fn listen<Listener: 'static + Fn(&str, Change)>(&mut self, listener: &'static Listener) {
        self.listeners.push(listener)
    }

    fn add(&mut self, index: usize, string: &str) {
        self.content.insert_str(index, string);
        let range = index..index + string.len();
        for listener in &self.listeners {
            listener(self.content.as_str(), Change::Addition(range.clone()));
        }
    }

    /*fn delete(&mut self, range: IntRange<usize>) {
        for listener in &self.listeners {
            listener(self.content.as_str(), Change::Deletion(range.clone()));
        }
        self.content.drain(range);
    }*/
}

/*struct Pattern {
    initial: &'static dyn Fn(Parser),
    on_change: &'static dyn Fn(Parser, Change),
}

impl Pattern {
    fn new<Initial, OnChange>(initial: &'static Initial, on_change: &'static OnChange) -> Self
        where Initial: 'static + Fn(Parser),
              OnChange: 'static + Fn(Parser, Change) {
        Self {
            initial,
            on_change,
        }
    }

    fn initial(&self, parser: Parser) {
        (self.initial)(parser)
    }

    fn on_change(&self, parser: Parser, change: Change) {
        (self.on_change)(parser, change)
    }
}

struct Parser<'source> {
    source: &'source Source,
    opening_parens: HollowRangeSpacedList<usize>,
    closing_parens: HollowRangeSpacedList<usize>,
}

impl<'source> Parser<'source> {
    fn new(source: &'source mut Source, patterns: Vec<Pattern>) -> Self {
        let parser: Parser<'source> = Self {
            source,
            opening_parens: HollowRangeSpacedList::<usize>::new(),
            closing_parens: HollowRangeSpacedList::<usize>::new(),
        };
        for pattern in &patterns {
            pattern.initial(parser);
        }
        for pattern in patterns {
            source.listen(
                &|source, change|
                    pattern.on_change(parser, change)
            );
        }
        parser
    }
}*/

#[derive(Debug, Eq, PartialEq)]
enum Paren {
    Opening,
    Closing
}

fn main() {
    // let mut source = Source::new("(print hello world)".into());
    let mut source = Source::new("(()(()))".into());
    let mut parens = RangeSpacedList::new();
    for (index, char) in source.content.char_indices() {
        if char == '(' {
            parens.insert_with_span(index, 1, Paren::Opening);
        } else if char == ')' {
            parens.insert_with_span(index, 1, Paren::Closing);
        }
    }
    /*let mut paren_pairs = SpacedList::new();
    let mut opening_paren_stack = vec![];
    for (start, end) in parens.iter_ranges() {
        match *start.element().borrow() {
            Paren::Opening => {
                opening_paren_stack.push(start.clone());
            }
            Paren::Closing => {
                paren_pairs.insert(start.position(), (Paren::Opening, start.clone(), end.clone()));
                paren_pairs.insert(end.position(), (Paren::Closing, start.clone(), end.clone()));
                opening_paren_stack.pop();
            }
        }
    }*/
    let tree = build_tree(&mut parens.iter_ranges());
    println!("{:#?}", tree);
}

#[derive(Debug)]
struct TreeNode {
    opening: Position<Range, usize, Paren>,
    closing: Position<Range, usize, Paren>,
    children: Vec<TreeNode>
}

#[derive(Debug)]
enum TreeNodeOrClosingParen {
    TreeNode(TreeNode),
    ClosingParen(Position<Range, usize, Paren>)
}

fn build_tree(iterator: &mut impl Iterator<Item=(Position<Range, usize, Paren>, Position<Range, usize, Paren>)>) -> Option<TreeNodeOrClosingParen> {
    let (opening, _) = iterator.next()?;
    match *opening.element().borrow() {
        Paren::Opening => {
            let mut children = vec![];
            loop {
                match build_tree(iterator).unwrap() {
                    TreeNodeOrClosingParen::TreeNode(child) => children.push(child),
                    TreeNodeOrClosingParen::ClosingParen(closing) => break Some(TreeNodeOrClosingParen::TreeNode(TreeNode {
                        opening,
                        closing,
                        children
                    }))
                }
            }
        }
        Paren::Closing => {
            Some(TreeNodeOrClosingParen::ClosingParen(opening))
        }
    }
}