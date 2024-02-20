// this example is unfinished, but that's okay for now
#![allow(dead_code, unused_variables)]

use std::ops::RangeInclusive;
use ansi_term::Color::{Blue, Green, Yellow};
use indoc::indoc;
use itertools::Itertools;
use spaced_list_5::{RangeSpacedList, NestedRangeSpacedList, HollowRangeSpacedList, HollowIndex, Index, Range, NestedRange};

fn main() {
    // let mut list = SpacedList::new();
    // let position = list.try_push(2, 42).unwrap();
    // let mut element_ref = position.element_mut();
    // *element_ref += 3;
    // list.insert(1, 43);
    // println!("{}", *element_ref);

    let source =
        indoc! {r"
            foo:
                bar:
                   hello
                    world
                 baz
                 123
                000"
        };

    // region parse colons
    let mut colons = HollowRangeSpacedList::<usize>::new();
    for (index, char) in source.chars().enumerate() {
        if char == ':' {
            colons.try_insert_with_span(index, 1).unwrap();
        }
    }
    // endregion

    // region parse arrows
    let mut arrows = HollowRangeSpacedList::<usize>::new();
    for (index, char) in source.chars().enumerate() {
        if char == '>' {
            arrows.try_insert_with_span(index, 1).unwrap();
        }
    }
    // endregion

    // region parse slashes
    let mut slashes = HollowRangeSpacedList::<usize>::new();
    for (index, char) in source.chars().enumerate() {
        if char == '/' {
            slashes.try_insert_with_span(index, 1).unwrap();
        }
    }
    // endregion

    // region parse line breaks
    let mut line_breaks = HollowRangeSpacedList::<usize>::new();
    for (index, char) in source.chars().enumerate() {
        if char == '\n' {
            line_breaks.try_insert_with_span(index, 1).unwrap();
        }
    }
    // endregion

    // region parse whitespace
    let mut whitespace = HollowRangeSpacedList::<usize>::new();
    let mut chars = source.chars().peekable();
    let mut start = 0;
    while let Some(char) = chars.next() {
        if char.is_whitespace() && char != '\n' {
            let mut span = 1;
            while let Some(char) = chars.peek() {
                if char.is_whitespace() && *char != '\n' {
                    span += 1;
                    chars.next();
                } else {
                    break;
                }
            }
            whitespace.try_insert_with_span(start, span).unwrap();
            start += span;
        } else {
            start += 1;
        }
    }
    // endregion

    // region parse full lines
    let mut full_lines = HollowRangeSpacedList::new();
    let mut start = 0;
    for (line_end, line_start) in line_breaks.iter_ranges() {
        full_lines.try_insert(start, line_end.position()).unwrap();
        start = line_start.position()
    }
    full_lines.try_insert(start, source.len()).unwrap();
    // endregion

    // region parse lines
    let mut lines = RangeSpacedList::new();
    for (start, end) in full_lines.iter_ranges() {
        let start = start.position();
        let end = end.position();
        if let Some(indentation) = whitespace.starting_at(start) {
            lines.try_insert(start, end, indentation.span()).unwrap();
        } else {
            lines.try_insert(start, end, 0).unwrap();
        }
    }
    // endregion

    // region parse trimmed lines
    let mut trimmed_lines = RangeSpacedList::new();
    for (start, end) in lines.iter_ranges() {
        let value = *start.element();
        let start = whitespace
            .starting_at(start.position())
            .map(|option| option.into_range().1.position())
            .unwrap_or(start.position());
        let end = whitespace
            .ending_at(end.position())
            .map(|option| option.into_range().0.position())
            .unwrap_or(end.position());
        trimmed_lines.try_insert(start, end, value).unwrap();
    }
    // endregion

    let mut inline_expressions = NestedRangeSpacedList::<usize, InlineExpression>::new();
    let mut expressions = NestedRangeSpacedList::<usize, Expression>::new();

    // region parse text
    let mut text = HollowRangeSpacedList::new();
    for (start, end) in trimmed_lines.iter_ranges() {
        let mut start = start.position();
        while start < end.position() {
            if let Some(colon) = colons.starting_at(start) {
                start += colon.span();
                continue;
            }
            if let Some(arrow) = arrows.starting_at(start) {
                start += arrow.span();
                continue;
            }
            if let Some(slash) = slashes.starting_at(start) {
                start += slash.span();
                continue;
            }
            if let Some(whitespace) = whitespace.starting_at(start) {
                start += whitespace.span();
                continue;
            }
            let mut end = end.position();
            if let Some(colon) = colons.starting_after(start) {
                if colon.position() < end {
                    end = whitespace
                        .ending_at(colon.position())
                        .map(|option| option.into_range().0.position())
                        .unwrap_or(colon.position());
                }
            }
            if let Some(arrow) = arrows.starting_after(start) {
                if arrow.position() < end {
                    end = whitespace
                        .ending_at(arrow.position())
                        .map(|option| option.into_range().0.position())
                        .unwrap_or(arrow.position());
                }
            }
            if let Some(slash) = slashes.starting_after(start) {
                if slash.position() < end {
                    end = whitespace
                        .ending_at(slash.position())
                        .map(|option| option.into_range().0.position())
                        .unwrap_or(slash.position());
                }
            }
            let position = text.try_insert(start, end).unwrap();
            let inline_value = InlineExpression::Text(Text {
                range: position.index().into_range()
            });
            inline_expressions.try_insert(start, end, inline_value.clone()).unwrap();
            expressions.try_insert(start, end, Expression::Inline(inline_value)).unwrap();
            start = end;
        }
    }
    // endregion

    // region parse paths
    /*
    foo/bar/baz
    foo/bar/>baz
    foo/>bar/baz
    foo/>bar/>baz
    >foo/bar/baz
    >foo/bar/>baz
    >foo/>bar/baz
    >foo/>bar/>baz

    (foo/bar)/baz
    (foo/bar)/>baz
    foo/>(bar/baz)
    foo/>(bar/>baz)
    >((foo/bar)/baz)
    >((foo/bar)/>baz)
    >(foo/>(bar/baz))
    >(foo/>(bar/>baz))

    from left to right iterating over all slashes
     */

    let paths = NestedRangeSpacedList::<usize, (InlineExpression, InlineExpression)>::new();
    let mut expected_inline_expression_before_slash_errors = HollowRangeSpacedList::new();

    for (start, end) in slashes.iter_ranges() {
        let left_position = whitespace
            .ending_at(start.position())
            .map(|option| option.into_range().0.position())
            .unwrap_or(start.position());
        let right_position = whitespace
            .starting_at(end.position())
            .map(|option| option.into_range().1.position())
            .unwrap_or(end.position());
        let left =
            inline_expressions
                .ending_at(left_position)
                .map(|position| position.element().clone())
                .unwrap_or_else(|| {
                    // let error_start = left_position - 1;
                    let previous_whitespace =
                        whitespace
                            .ending_before(left_position)
                            .map(|position| position.position());
                    let line_start = trimmed_lines
                        // TODO long term Here, an "at or before but if at then return None" could
                        //  have been useful maybe, consider when rewriting SpacedList
                        .starting_at_or_before(left_position)
                        .unwrap().position();
                        // .map(|position| position.position())
                        // .filter(|position| *position < left_position);
                    let (start, end) =
                        match previous_whitespace {
                            None => {
                                if line_start == left_position {
                                    (left_position, left_position + 1)
                                } else {
                                    (line_start, left_position)
                                }
                            }
                            Some(previous_whitespace) => {
                                if previous_whitespace >= line_start {
                                    (previous_whitespace, left_position)
                                } else if line_start == left_position {
                                    (left_position, left_position + 1)
                                } else {
                                    (line_start, left_position)
                                }
                            }
                        };
                    let position = expected_inline_expression_before_slash_errors.try_insert(start, end).unwrap();
                    let inline_value = InlineExpression::Error(Error::ExpectedInlineExpressionBeforeSlash {
                        range: position.index().into_range()
                    });
                    inline_expressions.try_insert(start, end, inline_value.clone()).unwrap();
                    expressions.try_insert(start, end, Expression::Inline(inline_value.clone())).unwrap();
                    inline_value
                });
        let right =
            inline_expressions
                .ending_at(left_position)
                .map(|position| position.element().clone())
                .unwrap_or_else(|| {
                    // let error_start = left_position - 1;
                    let previous_whitespace =
                        whitespace
                            .ending_before(left_position)
                            .map(|position| position.position());
                    let line_start = trimmed_lines
                        // TODO long term Here, an "at or before but if at then return None" could
                        //  have been useful maybe, consider when rewriting SpacedList
                        .starting_at_or_before(left_position)
                        .unwrap().position();
                        // .map(|position| position.position())
                        // .filter(|position| *position < left_position);
                    let (start, end) =
                        match previous_whitespace {
                            None => {
                                if line_start == left_position {
                                    (left_position, left_position + 1)
                                } else {
                                    (line_start, left_position)
                                }
                            }
                            Some(previous_whitespace) => {
                                if previous_whitespace >= line_start {
                                    (previous_whitespace, left_position)
                                } else if line_start == left_position {
                                    (left_position, left_position + 1)
                                } else {
                                    (line_start, left_position)
                                }
                            }
                        };
                    let position = expected_inline_expression_before_slash_errors.try_insert(start, end).unwrap();
                    let inline_value = InlineExpression::Error(Error::ExpectedInlineExpressionBeforeSlash {
                        range: position.index().into_range()
                    });
                    inline_expressions.try_insert(start, end, inline_value.clone()).unwrap();
                    expressions.try_insert(start, end, Expression::Inline(inline_value.clone())).unwrap();
                    inline_value
                });
    }
    // endregion

    // region parse indented blocks
    let mut indented_blocks = NestedRangeSpacedList::new();

    let vec =
        lines
            .iter_ranges()
            .map(|(start, end)| (start.position(), end.position()))
            .collect_vec();

    let mut map =
        lines
            .iter_ranges()
            .enumerate()
            .map(|(index, (start, _))| (*start.element(), index))
            .into_grouping_map()
            .collect::<Vec<_>>();

    let mut indices_elimination = (0..vec.len()).collect_vec();

    fn consecutive_ranges(data: &[usize]) -> impl Iterator<Item=RangeInclusive<usize>> + '_ {
        let mut slice_start = 0;
        (1..=data.len()).flat_map(move |i| {
            if i == data.len() || data[i - 1] + 1 != data[i] {
                let begin = slice_start;
                slice_start = i;
                Some(data[begin]..=data[i - 1])
            } else {
                None
            }
        })
    }

    for baseline in 0.. {
        let indices_to_eliminate = map.remove(&baseline);
        if let Some(indices_to_eliminate) = indices_to_eliminate {
            for range in consecutive_ranges(indices_elimination.as_slice()) {
                if indices_to_eliminate.iter().any(|it| range.contains(it)) {
                    indented_blocks.try_insert(vec[*range.start()].0, vec[*range.end()].1, baseline).unwrap();
                }
            }
            for index in &indices_to_eliminate {
                indices_elimination.remove(indices_elimination.binary_search(index).unwrap());
            }
            if indices_elimination.is_empty() {
                break;
            }
        }
    }
    // endregion

    /**/
    // region print syntax highlighted source code
    let mut colored_source = String::new();
    let mut start = 0;
    loop {
        if let Some(colon) = colons.starting_at(start) {
            let string: String = source.chars().skip(start).take(colon.span()).collect();
            colored_source += Blue.paint(string).to_string().as_str();
            start += colon.span();
            continue;
        }
        if let Some(arrow) = arrows.starting_at(start) {
            let string: String = source.chars().skip(start).take(arrow.span()).collect();
            colored_source += Green.paint(string).to_string().as_str();
            start += arrow.span();
            continue;
        }
        if let Some(text) = text.starting_at(start) {
            let string: String = source.chars().skip(start).take(text.span()).collect();
            colored_source += Yellow.paint(string).to_string().as_str();
            start += text.span();
            continue;
        }
        if let Some(whitespace) = whitespace.starting_at(start) {
            let string: String = source.chars().skip(start).take(whitespace.span()).collect();
            colored_source += string.as_str();
            start += whitespace.span();
            continue;
        }
        if let Some(line_break) = line_breaks.starting_at(start) {
            let string: String = source.chars().skip(start).take(line_break.span()).collect();
            colored_source += string.as_str();
            start += line_break.span();
            continue;
        }
        break;
    }
    println!("{}", colored_source);
    // endregion
    /**/
}

#[derive(Clone)]
enum Expression {
    Inline(InlineExpression),
    Scope(Scope),
    ScopeExtendedInline(ScopeExtendedInline),
}

#[derive(Clone)]
enum InlineExpression {
    Text(Text),
    Path(Path),
    Reference(Reference),
    Error(Error),
}

#[derive(Clone)]
enum Error {
    ExpectedInlineExpressionBeforeSlash {
        range: (HollowIndex<Range, usize>, HollowIndex<Range, usize>)
    }
}

#[derive(Clone)]
struct Text {
    range: (HollowIndex<Range, usize>, HollowIndex<Range, usize>),
}

#[derive(Clone)]
struct Path {
    range: (HollowIndex<Range, usize>, HollowIndex<Range, usize>),
    left: Box<InlineExpression>,
    right: Box<InlineExpression>,
}

#[derive(Clone)]
struct Reference {
    range: (HollowIndex<Range, usize>, HollowIndex<Range, usize>),
    path: Box<InlineExpression>,
}

#[derive(Clone)]
struct Scope {
    range: (Index<NestedRange, usize, usize>, Index<NestedRange, usize, usize>),
    associations: Vec<Association>,
}

#[derive(Clone)]
struct Association {
    left: Option<InlineExpression>,
    right: Expression,
}

#[derive(Clone)]
struct ScopeExtendedInline {
    inline: InlineExpression,
    scope: Scope,
}
