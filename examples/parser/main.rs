use std::cmp::min;
use std::iter::Peekable;
use std::ops::Add;
use ansi_term::Color::{Blue, Green, Yellow};
use indoc::indoc;
use itertools::Itertools;
use spaced_list_5::{HollowNestedRangeSpacedList, HollowRangeSpacedList, Position, Range, RangeSpacedList};

fn main() {
    let source =
        // indentation block from line 0 to line 6
        // indentation block from line 1 to line 6
        // indentation block from line 2 to line 5
        // indentation block from line 2 to line 3
        // indentation block from line 3 to line 3
        //
        //
        //
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
    let mut colons = HollowRangeSpacedList::new();
    for (index, char) in source.chars().enumerate() {
        if char == ':' {
            colons.insert_with_span(index, 1);
        }
    }
    // endregion

    // region parse arrows
    let mut arrows = HollowRangeSpacedList::new();
    for (index, char) in source.chars().enumerate() {
        if char == '>' {
            arrows.insert_with_span(index, 1);
        }
    }
    // endregion

    // region parse line breaks
    let mut line_breaks = HollowRangeSpacedList::new();
    for (index, char) in source.chars().enumerate() {
        if char == '\n' {
            line_breaks.insert_with_span(index, 1);
        }
    }
    // endregion

    // region parse whitespace
    let mut whitespace = HollowRangeSpacedList::new();
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
            whitespace.insert_with_span(start, span);
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
        full_lines.insert(start, line_end.position());
        start = line_start.position()
    }
    full_lines.insert(start, source.len());
    // endregion

    // region parse lines
    let mut lines = RangeSpacedList::new();
    for (start, end) in full_lines.iter_ranges() {
        let start = start.position();
        let end = end.position();
        if let Some(indentation) = whitespace.starting_at(start) {
            lines.insert(start, end, indentation.span());
        } else {
            lines.insert(start, end, 0);
        }
    }
    // endregion

    // region parse trimmed lines
    let mut trimmed_lines = RangeSpacedList::new();
    for (start, end) in lines.iter_ranges() {
        let value = *start.element().borrow();
        let start = whitespace
            .starting_at(start.position())
            .map(|option| option.into_range().1.position())
            .unwrap_or(start.position());
        let end = whitespace
            .ending_at(end.position())
            .map(|option| option.into_range().0.position())
            .unwrap_or(end.position());
        trimmed_lines.insert(start, end, value);
    }
    // endregion

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
            text.insert(start, end);
            start = end;
        }
    }
    // endregion

    // region parse indented blocks
    let mut indented_blocks = HollowNestedRangeSpacedList::new();
    // let mut lines = lines.iter_ranges().peekable();
    // TODO handle indentation of the entire source file
    indented_blocks.insert(0, source.len());

    fn search_for_indented_block(
        source: &str,
        indented_blocks: &mut HollowNestedRangeSpacedList<usize>,
        // lines: &mut Peekable<impl Iterator<Item=(Position<Range, usize, usize>, Position<Range, usize, usize>)>>,
        lines: &RangeSpacedList<usize, usize>,
        base_indentation: usize,
        from: usize,
        to: usize,
    ) {
        // Start of the search for indented blocks in this block
        // let mut lines_iter = lines.iter_ranges().skip_while(|(start, _)| start.position() < from);
        let mut lines_iter = lines.starting_at(from).unwrap().iter_next().tuples();
        while let Some((line_start, line_end)) = lines_iter.next() {
            if line_end.position() > to {
                break;
            }
            let indentation = *line_start.element().borrow();
            if indentation > base_indentation {
                // Start of an indented block
                let start = line_start.position();
                let mut end = line_end.position();
                // let mut lines_iter_ = lines.starting_after(start).unwrap().iter_next().tuples();
                while let Some((_, line_end)) = lines_iter.next() {
                    let indentation = *line_end.element().borrow();
                    if indentation <= base_indentation {
                        indented_blocks.insert(start, end);
                        break;
                    }
                    end = line_end.position();
                }
                if end == source.len() {
                    indented_blocks.insert(start, end);
                }
                // The indented block has been closed
                search_for_indented_block(source, indented_blocks, lines, indentation, start, end);
            }
        }
    }
    search_for_indented_block(source, &mut indented_blocks, &lines, 0, 0, source.len());
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