use std::cmp::min;
use std::ops::Add;
use ansi_term::Color::{Blue, Green, Yellow};
use spaced_list_5::{HollowRangeSpacedList, RangeSpacedList};

fn main() {
    let source =
        r"foo:
    bar: baz
    123: 456
+: >foo";

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
    let mut chars = source.chars();
    let mut start = 0;
    while let Some(char) = chars.next() {
        if char.is_whitespace() && char != '\n' {
            let mut span = 1;
            while let Some(char) = chars.next() {
                if char.is_whitespace() && char != '\n' {
                    span += 1;
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
        let end = whitespace
            .ending_at(end.position())
            .map(|option| option.into_range().0.position())
            .unwrap_or(end.position());
        if let Some(indentation) = whitespace.starting_at(start.position()) {
            let value = indentation.span();
            let start = indentation.into_range().1.position();
            lines.insert(start, end, value);
        } else {
            lines.insert(start.position(), end, 0);
        }
    }
    // endregion

    // region parse text
    let mut text = HollowRangeSpacedList::new();
    for (start, end) in lines.iter_ranges() {
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
}