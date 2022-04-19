use std::ops::Range;

use spaced_list_5::{FilledRangeSpacedList, FilledSpacedList, HollowRangeSpacedList, Position};

struct Source {
    content: String,
    listeners: Vec<&'static dyn Fn(&str, Change)>,
}

enum Change {
    /// Contains the index range of the added text
    Addition(Range<usize>),
    /*/// Contains the index range of the deleted text
    Deletion(Range<usize>),*/
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

    /*fn delete(&mut self, range: Range<usize>) {
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

enum Paren {
    Opening,
    Closing
}

fn main() {
    let mut source = Source::new("(print hello world)".into());
    let mut parens = FilledRangeSpacedList::new();
    for (index, char) in source.content.char_indices() {
        if char == '(' {
            parens.insert_range(index, index + 1, Paren::Opening);
        } else if char == ')' {
            parens.insert_range(index, index + 1, Paren::Closing);
        }
    }
    let mut paren_pairs = FilledSpacedList::new();
    let mut opening_paren_stack = vec![];
    for (start, end) in parens.iter() {
        match parens.element(&start) {
            Paren::Opening => {
                opening_paren_stack.push(start);
            }
            Paren::Closing => {
                paren_pairs.insert_element(start.position(), (Paren::Opening, start.clone(), end.clone()));
                paren_pairs.insert_element(end.position(), (Paren::Closing, start, end));
                opening_paren_stack.pop();
            }
        }
    }
}