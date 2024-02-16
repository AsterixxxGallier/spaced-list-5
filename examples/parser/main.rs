use std::ops::RangeInclusive;
use ansi_term::Color::{Blue, Green, Yellow};
use indoc::indoc;
use itertools::Itertools;
use spaced_list_5::{HollowRangeSpacedList, NestedRangeSpacedList, RangeSpacedList};

fn main() {
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
    let mut colons = HollowRangeSpacedList::new();
    for (index, char) in source.chars().enumerate() {
        if char == ':' {
            colons.try_insert_with_span(index, 1).unwrap();
        }
    }
    // endregion

    // region parse arrows
    let mut arrows = HollowRangeSpacedList::new();
    for (index, char) in source.chars().enumerate() {
        if char == '>' {
            arrows.try_insert_with_span(index, 1).unwrap();
        }
    }
    // endregion

    // region parse line breaks
    let mut line_breaks = HollowRangeSpacedList::new();
    for (index, char) in source.chars().enumerate() {
        if char == '\n' {
            line_breaks.try_insert_with_span(index, 1).unwrap();
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
        let value = *start.element().borrow();
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
            text.try_insert(start, end).unwrap();
            start = end;
        }
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
            .map(|(index, (start, _))| (*start.element().borrow(), index))
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