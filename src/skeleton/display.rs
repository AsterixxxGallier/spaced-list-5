#![allow(unused)]

use std::fmt::{Display, Formatter, Write};
use std::iter::repeat;
use std::ops::{Bound, RangeBounds};

use crate::skeleton::display::Cell::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Horizontal { highlighted: bool },
    Vertical { highlighted: bool },
    Connector { connectivity: Connectivity },
    Char(char),
}

impl Cell {
    fn horizontal_connectivity(&self) -> Connectivity {
        match *self {
            Horizontal { highlighted } =>
                if highlighted { Connectivity::Highlighted } else { Connectivity::Normal },
            Connector { connectivity } => connectivity,
            _ => Connectivity::None
        }
    }

    fn vertical_connectivity(&self) -> Connectivity {
        match *self {
            Vertical { highlighted } =>
                if highlighted { Connectivity::Highlighted } else { Connectivity::Normal },
            Connector { connectivity } => connectivity,
            _ => Connectivity::None
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Connectivity {
    None,
    Normal,
    Highlighted,
}

struct Grid(Vec<Vec<Cell>>);

impl Grid {
    fn new() -> Self {
        Self(vec![])
    }

    fn cell(&self, row: usize, column: usize) -> Option<&Cell> {
        self.0.get(row)?.get(column)
    }

    fn cell_mut(&mut self, row: usize, column: usize) -> Option<&mut Cell> {
        self.0.get_mut(row)?.get_mut(column)
    }

    fn row(&self, row: usize) -> Option<&Vec<Cell>> {
        self.0.get(row)
    }

    fn row_mut(&mut self, row: usize) -> Option<&mut Vec<Cell>> {
        self.0.get_mut(row)
    }

    fn iter_row(&self, row: usize) -> impl Iterator<Item=&Cell> {
        self.row(row).into_iter().flatten()
    }

    fn iter_mut_row(&mut self, row: usize) -> impl Iterator<Item=&mut Cell> {
        self.row_mut(row).into_iter().flatten()
    }

    fn iter_column(&self, column: usize) -> impl Iterator<Item=&Cell> {
        ColumnIterator::new(self, column)
    }

    fn iter_mut_column(&mut self, column: usize) -> impl Iterator<Item=&mut Cell> {
        ColumnIteratorMut::new(self, column)
    }

    fn expand_to_fit(&mut self, row: usize, column: usize) {
        if let Some(row) = self.row_mut(row) {
            if column >= row.len() {
                row.extend(repeat(Empty).take(column - row.len() + 1));
            }
        } else {
            self.0.extend(repeat(vec![]).take(row - self.0.len() + 1));
            self.expand_to_fit(row, column);
        }
    }

    const fn required_max(start_bound: Bound<&usize>, end_bound: Bound<&usize>) -> usize {
        match (start_bound, end_bound) {
            (Bound::Unbounded, Bound::Unbounded) => 0,
            (_, Bound::Excluded(x)) => *x - 1,
            (_, Bound::Included(x)) => *x,
            (Bound::Excluded(x), _) => *x - 1,
            (Bound::Included(x), _) => *x,
        }
    }

    fn horizontal_line(&mut self, row: usize, range: impl RangeBounds<usize>, highlighted: bool) {
        let start_bound = range.start_bound();
        let end_bound = range.end_bound();
        self.expand_to_fit(row, 0);
        let row_cells = self.row(row).unwrap();
        let start = match start_bound {
            Bound::Included(x) => *x,
            Bound::Excluded(x) => x + 1,
            Bound::Unbounded => 0
        };
        // inclusive
        let end = match end_bound {
            Bound::Included(x) => *x,
            Bound::Excluded(x) => x - 1,
            Bound::Unbounded => row_cells.len() - 1
        };
        self.expand_to_fit(row, end);
        let row_cells = self.row_mut(row).unwrap();
        for cell in row_cells[start..=end].iter_mut() {
            *cell = Horizontal { highlighted }
        }
    }

    fn vertical_line(&mut self, column: usize, range: impl RangeBounds<usize>, highlighted: bool) {
        let start_bound = range.start_bound();
        let end_bound = range.end_bound();
        let start = match start_bound {
            Bound::Included(x) => *x,
            Bound::Excluded(x) => x + 1,
            Bound::Unbounded => 0
        };
        // inclusive
        let end = match end_bound {
            Bound::Included(x) => *x,
            Bound::Excluded(x) => x - 1,
            Bound::Unbounded => self.0.len() - 1
        };
        for row in 0..=end {
            self.expand_to_fit(row, column);
        }
        for cell in self.iter_mut_column(column).skip(start).take(end - start + 1) {
            *cell = Vertical { highlighted }
        }
    }

    fn empty(&mut self, row: usize, column: usize) {
        self.expand_to_fit(row, column);
        *self.cell_mut(row, column).unwrap() = Empty
    }

    fn connector(&mut self, row: usize, column: usize) {
        self.expand_to_fit(row, column);
        *self.cell_mut(row, column).unwrap() = Connector { connectivity: Connectivity::None }
    }

    fn connective_connector(&mut self, row: usize, column: usize) {
        self.expand_to_fit(row, column);
        *self.cell_mut(row, column).unwrap() = Connector { connectivity: Connectivity::Normal }
    }

    fn char(&mut self, row: usize, column: usize, char: char) {
        self.expand_to_fit(row, column);
        *self.cell_mut(row, column).unwrap() = Char(char)
    }

    fn string(&mut self, row: usize, column: usize, string: &str) {
        self.expand_to_fit(row, column + string.len() - 1);
        for (index, char) in string.chars().enumerate() {
            *self.cell_mut(row, column + index).unwrap() = Char(char)
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (row, cells) in self.0.iter().enumerate() {
            for (column, cell) in cells.iter().enumerate() {
                match cell {
                    Empty => {
                        f.write_char(' ')?;
                    }
                    Cell::Horizontal { highlighted } => {
                        if *highlighted {
                            f.write_char('━')?;
                        } else {
                            f.write_char('─')?;
                        }
                    }
                    Cell::Vertical { highlighted } => {
                        if *highlighted {
                            f.write_char('┃')?;
                        } else {
                            f.write_char('│')?;
                        }
                    }
                    Char(char) => {
                        f.write_char(*char)?;
                    }
                    Connector { .. } => {
                        let left =
                            if column == 0 {
                                Connectivity::None
                            } else if let Some(cell) = self.cell(row, column - 1) {
                                cell.horizontal_connectivity()
                            } else {
                                Connectivity::None
                            };
                        let right =
                            if let Some(cell) = self.cell(row, column + 1) {
                                cell.horizontal_connectivity()
                            } else {
                                Connectivity::None
                            };
                        let top =
                            if row == 0 {
                                Connectivity::None
                            } else if let Some(cell) = self.cell(row - 1, column) {
                                cell.vertical_connectivity()
                            } else {
                                Connectivity::None
                            };
                        let bottom =
                            if let Some(cell) = self.cell(row + 1, column) {
                                cell.vertical_connectivity()
                            } else {
                                Connectivity::None
                            };
                        use Connectivity::*;
                        // @formatter:off
                        #[rustfmt::skip]
                        f.write_char(match (top, right, bottom, left) {
                            ///   ╴ ╸
                            /// ╵ ┘ ┙
                            /// ╹ ┚ ┛
                            /// ╶ ─ ╾
                            /// └ ┴ ┵
                            /// ┖ ┸ ┹
                            /// ╺ ╼ ━
                            /// ┕ ┶ ┷
                            /// ┗ ┺ ┻
                            /// ╷ ┐ ┑
                            /// │ ┤ ┥
                            /// ╿ ┦ ┩
                            /// ┌ ┬ ┭
                            /// ├ ┼ ┽
                            /// ┞ ╀ ╃
                            /// ┍ ┮ ┯
                            /// ┝ ┾ ┿
                            /// ┡ ╄ ╇
                            /// ╻ ┒ ┓
                            /// ╽ ┧ ┪
                            /// ┃ ┨ ┫
                            /// ┎ ┰ ┱
                            /// ┟ ╁ ╅
                            /// ┠ ╂ ╉
                            /// ┏ ┲ ┳
                            /// ┢ ╆ ╈
                            /// ┣ ╊ ╋
                            (None,          None,        None,        None)        => '╳',
                            (Normal,        None,        None,        None)        => '╵',
                            (Highlighted,   None,        None,        None)        => '╹',
                            (None,          Normal,      None,        None)        => '╶',
                            (Normal,        Normal,      None,        None)        => '└',
                            (Highlighted,   Normal,      None,        None)        => '┖',
                            (None,          Highlighted, None,        None)        => '╺',
                            (Normal,        Highlighted, None,        None)        => '┕',
                            (Highlighted,   Highlighted, None,        None)        => '┗',
                            (None,          None,        Normal,      None)        => '╷',
                            (Normal,        None,        Normal,      None)        => '│',
                            (Highlighted,   None,        Normal,      None)        => '╿',
                            (None,          Normal,      Normal,      None)        => '┌',
                            (Normal,        Normal,      Normal,      None)        => '├',
                            (Highlighted,   Normal,      Normal,      None)        => '┞',
                            (None,          Highlighted, Normal,      None)        => '┍',
                            (Normal,        Highlighted, Normal,      None)        => '┝',
                            (Highlighted,   Highlighted, Normal,      None)        => '┡',
                            (None,          None,        Highlighted, None)        => '╻',
                            (Normal,        None,        Highlighted, None)        => '╽',
                            (Highlighted,   None,        Highlighted, None)        => '┃',
                            (None,          Normal,      Highlighted, None)        => '┎',
                            (Normal,        Normal,      Highlighted, None)        => '┟',
                            (Highlighted,   Normal,      Highlighted, None)        => '┠',
                            (None,          Highlighted, Highlighted, None)        => '┏',
                            (Normal,        Highlighted, Highlighted, None)        => '┢',
                            (Highlighted,   Highlighted, Highlighted, None)        => '┣',
                            (None,          None,        None,        Normal)      => '╴',
                            (Normal,        None,        None,        Normal)      => '┘',
                            (Highlighted,   None,        None,        Normal)      => '┚',
                            (None,          Normal,      None,        Normal)      => '─',
                            (Normal,        Normal,      None,        Normal)      => '┴',
                            (Highlighted,   Normal,      None,        Normal)      => '┸',
                            (None,          Highlighted, None,        Normal)      => '╼',
                            (Normal,        Highlighted, None,        Normal)      => '┶',
                            (Highlighted,   Highlighted, None,        Normal)      => '┺',
                            (None,          None,        Normal,      Normal)      => '┐',
                            (Normal,        None,        Normal,      Normal)      => '┤',
                            (Highlighted,   None,        Normal,      Normal)      => '┦',
                            (None,          Normal,      Normal,      Normal)      => '┬',
                            (Normal,        Normal,      Normal,      Normal)      => '┼',
                            (Highlighted,   Normal,      Normal,      Normal)      => '╀',
                            (None,          Highlighted, Normal,      Normal)      => '┮',
                            (Normal,        Highlighted, Normal,      Normal)      => '┾',
                            (Highlighted,   Highlighted, Normal,      Normal)      => '╄',
                            (None,          None,        Highlighted, Normal)      => '┒',
                            (Normal,        None,        Highlighted, Normal)      => '┧',
                            (Highlighted,   None,        Highlighted, Normal)      => '┨',
                            (None,          Normal,      Highlighted, Normal)      => '┰',
                            (Normal,        Normal,      Highlighted, Normal)      => '╁',
                            (Highlighted,   Normal,      Highlighted, Normal)      => '╂',
                            (None,          Highlighted, Highlighted, Normal)      => '┲',
                            (Normal,        Highlighted, Highlighted, Normal)      => '╆',
                            (Highlighted,   Highlighted, Highlighted, Normal)      => '╊',
                            (None,          None,        None,        Highlighted) => '╸',
                            (Normal,        None,        None,        Highlighted) => '┙',
                            (Highlighted,   None,        None,        Highlighted) => '┛',
                            (None,          Normal,      None,        Highlighted) => '╾',
                            (Normal,        Normal,      None,        Highlighted) => '┵',
                            (Highlighted,   Normal,      None,        Highlighted) => '┹',
                            (None,          Highlighted, None,        Highlighted) => '━',
                            (Normal,        Highlighted, None,        Highlighted) => '┷',
                            (Highlighted,   Highlighted, None,        Highlighted) => '┻',
                            (None,          None,        Normal,      Highlighted) => '┑',
                            (Normal,        None,        Normal,      Highlighted) => '┥',
                            (Highlighted,   None,        Normal,      Highlighted) => '┩',
                            (None,          Normal,      Normal,      Highlighted) => '┭',
                            (Normal,        Normal,      Normal,      Highlighted) => '┽',
                            (Highlighted,   Normal,      Normal,      Highlighted) => '╃',
                            (None,          Highlighted, Normal,      Highlighted) => '┯',
                            (Normal,        Highlighted, Normal,      Highlighted) => '┿',
                            (Highlighted,   Highlighted, Normal,      Highlighted) => '╇',
                            (None,          None,        Highlighted, Highlighted) => '┓',
                            (Normal,        None,        Highlighted, Highlighted) => '┪',
                            (Highlighted,   None,        Highlighted, Highlighted) => '┫',
                            (None,          Normal,      Highlighted, Highlighted) => '┱',
                            (Normal,        Normal,      Highlighted, Highlighted) => '╅',
                            (Highlighted,   Normal,      Highlighted, Highlighted) => '╉',
                            (None,          Highlighted, Highlighted, Highlighted) => '┳',
                            (Normal,        Highlighted, Highlighted, Highlighted) => '╈',
                            (Highlighted,   Highlighted, Highlighted, Highlighted) => '╋'
                        })?;
                        // @formatter:on
                    }
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

struct ColumnIterator<'a> {
    grid: &'a Grid,
    column: usize,
    row: usize,
}

impl<'a> ColumnIterator<'a> {
    fn new(grid: &'a Grid, column: usize) -> Self {
        Self {
            grid,
            column,
            row: 0,
        }
    }
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.row({
            self.row += 1;
            self.row - 1
        })?.get(self.column)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rows_coming = self.grid.0.len() - self.row;
        (rows_coming, Some(rows_coming))
    }
}

struct ColumnIteratorMut<'a> where Self: 'a {
    grid: &'a mut Grid,
    column: usize,
    row: usize,
}

impl<'a> ColumnIteratorMut<'a> {
    fn new(grid: &'a mut Grid, column: usize) -> Self {
        Self {
            grid,
            column,
            row: 0,
        }
    }
}

impl<'a> Iterator for ColumnIteratorMut<'a> {
    type Item = &'a mut Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let row = {
            self.row += 1;
            self.row - 1
        };
        // SAFETY: We own the grid, therefore, nobody else can keep any mutable references to any
        // part of it. Here, we want to give out references to distinct cells of the grid, but the
        // compiler cannot prove that they are really distinct. Therefore, it understandably
        // complains that we can't just give out multiple mutable references to parts of a single
        // grid. However, we *can* prove that this is safe: Because next always increments self.row,
        // and we only ever take cells from the row at the index of self.row, we can guarantee that
        // we will always return references to different parts of the grid, different cells.
        unsafe {
            let row = self.grid.0.get_mut(row)?;
            if self.column < row.len() {
                Some(&mut *(row.as_mut_slice() as *mut [Cell]).get_unchecked_mut(self.column))
            } else {
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rows_coming = self.grid.0.len() - self.row;
        (rows_coming, Some(rows_coming))
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    #[ignore]
    fn test() {
        let mut grid = Grid::new();
        grid.connective_connector(0, 0);
        grid.horizontal_line(0, (1 + 0)..64, false);
        grid.connective_connector(0, 64);
        grid.connective_connector(1, 0);
        grid.horizontal_line(1, (1 + 0)..32, false);
        grid.connective_connector(1, 32);
        grid.connective_connector(2, 0);
        grid.horizontal_line(2, (1 + 0)..16, false);
        grid.connective_connector(2, 16);
        grid.connective_connector(2, 32);
        grid.horizontal_line(2, (1 + 32)..48, false);
        grid.connective_connector(2, 48);
        grid.connective_connector(3, 0);
        grid.horizontal_line(3, (1 + 0)..8, false);
        grid.connective_connector(3, 8);
        grid.connective_connector(3, 16);
        grid.horizontal_line(3, (1 + 16)..24, false);
        grid.connective_connector(3, 24);
        grid.connective_connector(3, 32);
        grid.horizontal_line(3, (1 + 32)..40, false);
        grid.connective_connector(3, 40);
        grid.connective_connector(3, 48);
        grid.horizontal_line(3, (1 + 48)..56, false);
        grid.connective_connector(3, 56);
        grid.connective_connector(4, 0);
        grid.horizontal_line(4, (1 + 0)..4, false);
        grid.connective_connector(4, 4);
        grid.connective_connector(4, 8);
        grid.horizontal_line(4, (1 + 8)..12, false);
        grid.connective_connector(4, 12);
        grid.connective_connector(4, 16);
        grid.horizontal_line(4, (1 + 16)..20, false);
        grid.connective_connector(4, 20);
        grid.connective_connector(4, 24);
        grid.horizontal_line(4, (1 + 24)..28, false);
        grid.connective_connector(4, 28);
        grid.connective_connector(4, 32);
        grid.horizontal_line(4, (1 + 32)..36, false);
        grid.connective_connector(4, 36);
        grid.connective_connector(4, 40);
        grid.horizontal_line(4, (1 + 40)..44, false);
        grid.connective_connector(4, 44);
        grid.connective_connector(4, 48);
        grid.horizontal_line(4, (1 + 48)..52, false);
        grid.connective_connector(4, 52);
        grid.connective_connector(4, 56);
        grid.horizontal_line(4, (1 + 56)..60, false);
        grid.connective_connector(4, 60);
        grid.vertical_line(64, 1..4, false);
        grid.connective_connector(4, 64);
        grid.connective_connector(5, 0);
        grid.connective_connector(5, 4);
        grid.connective_connector(5, 8);
        grid.connective_connector(5, 12);
        println!("{}", grid);
    }
}