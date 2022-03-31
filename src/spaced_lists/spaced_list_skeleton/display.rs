use std::default::default;
use std::fmt::{Debug, Display, Formatter, Write};

use crate::{SpacedListSkeleton, SpacedList, Spacing};

#[derive(Clone)]
pub struct SkeletonFormatOptions {
	pub show_link_lengths: bool,
	pub show_positions: bool,
	pub show_sublists: bool,
	pub spacing: usize,
	pub highlighted_links: Vec<usize>,
	pub highlighted_nodes: Vec<usize>,
}

impl Default for SkeletonFormatOptions {
	fn default() -> Self {
		Self {
			show_link_lengths: true,
			show_positions: true,
			show_sublists: true,
			spacing: 2,
			highlighted_links: Vec::new(),
			highlighted_nodes: Vec::new(),
		}
	}
}

impl<S: Spacing + Display, Sub: SpacedList<S>> SpacedListSkeleton<S, Sub> {
	pub fn format(&self,
				  show_link_lengths: bool,
				  show_positions: bool,
				  show_sublists: bool,
				  spacing: usize,
				  highlighted_links: Vec<usize>,
				  highlighted_nodes: Vec<usize>,
	) -> CustomFormat<S, Sub> {
		CustomFormat {
			skeleton: self,
			options: SkeletonFormatOptions {
				show_link_lengths,
				show_positions,
				show_sublists,
				spacing,
				highlighted_links,
				highlighted_nodes,
			}
		}
	}

	pub fn default_format(&self) -> CustomFormat<S, Sub> {
		CustomFormat {
			skeleton: self,
			options: default()
		}
	}

	fn format_lines(&self, options: SkeletonFormatOptions) -> Iter<S, Sub> {
		Iter::new(self, options, false)
	}
}

pub struct CustomFormat<'a, S: Spacing + Display, Sub: SpacedList<S>> {
	skeleton: &'a SpacedListSkeleton<S, Sub>,
	options: SkeletonFormatOptions
}

struct Iter<'a, S: Spacing + Display, Sub: SpacedList<S>> {
	skeleton: &'a SpacedListSkeleton<S, Sub>,
	options: SkeletonFormatOptions,
	node_index: usize,
	continuing_links: bool,
	is_sublist: bool,
	sub_iterator: Option<Box<Iter<'a, S, Sub>>>,
}

impl<'a, S: Spacing + Display, Sub: SpacedList<S>> Iter<'a, S, Sub> {
	fn new(skeleton: &'a SpacedListSkeleton<S, Sub>, options: SkeletonFormatOptions, is_sublist: bool) -> Self {
		Self {
			skeleton,
			options,
			node_index: 0,
			continuing_links: false,
			is_sublist,
			sub_iterator: None,
		}
	}
}

impl<'a, S: Spacing + Display, Sub: SpacedList<S>> Iterator for Iter<'a, S, Sub> {
	type Item = String;

	/// ┌┬┬┬0
	/// │││╰1
	/// ││╰┬2
	/// ││ ╰3
	/// │╰┬┬4
	/// │ │╰5
	/// │ ╰┬6
	/// │  ╰7
	/// ╰───8

	/// ┌─┬─┬─┬─0
	/// │ │ │ ╰─1
	/// │ │ ╰─┬─2
	/// │ │   ╰─3
	/// │ ╰─┬─┬─4
	/// │   │ ╰─5
	/// │   ╰─┬─6
	/// │     ╰─7
	/// ╰───────8

	/// ┌──┬──┬──┬──0
	/// │  │  │  │0
	/// │  │  │  ╰──1
	/// │  │  │   1
	/// │  │  ╰──┬──2
	/// │  │     │2
	/// │  │     ╰──3
	/// │  │      3
	/// │  ╰──┬──┬──4
	/// │     │  │4
	/// │     │  ╰──5
	/// │     │   5
	/// │     ╰──┬──6
	/// │        │6
	/// │        ╰──7
	/// │         7
	/// ╰───────────8

	/// ┌──┬──┬──┬──
	/// │  │  │  │
	/// │  │  │  ╰──
	/// │  │  │
	/// │  │  ╰──┬──
	/// │  │     │
	/// │  │     ╰──
	/// │  │
	/// │  ╰──┬──┬──
	/// │     │  │
	/// │     │  ╰──
	/// │     │
	/// │     ╰──┬──
	/// │        │
	/// │        ╰──
	/// │
	/// ╰───────────

	/// ┌──┬──┬──┬──┬──
	/// │  │  │  │  │
	/// │  │  │  │  ╰──
	/// │  │  │  │
	/// │  │  │  ╰─────
	/// │  │  │
	/// │  │  ╰──
	/// │  │
	/// │  ╰──┬──
	/// │     │
	/// │     ╰──
	/// │
	/// ╰────────
	fn next(&mut self) -> Option<Self::Item> {
		if self.skeleton.node_index_is_in_bounds(self.node_index) &&
			(!self.continuing_links || self.skeleton.link_index_is_in_bounds(self.node_index)) {
			let space_spacing_string = " ".repeat(self.options.spacing);
			let space_spacing = space_spacing_string.as_str();
			let line_spacing_string = "─".repeat(self.options.spacing);
			let line_spacing = line_spacing_string.as_str();
			let depth = self.skeleton.depth();
			let last_degree = depth - 1;
			let mut line = String::with_capacity(3 * depth /*todo maybe more*/);
			if self.continuing_links {
				for degree in (0..depth).rev() {
					if self.node_index & (1 << degree) == 0 {
						line += "│";
					} else {
						line += " ";
					}
					line += space_spacing;
				}
				self.continuing_links = false;
				if self.sub_iterator.is_none() {
					self.node_index += 1;
				}
			} else {
				if self.node_index == 0 {
					for degree in (0..depth).rev() {
						if degree == last_degree && !self.is_sublist {
							line += "┌";
						} else {
							line += "┬";
						}
						line += line_spacing;
					}
				} else if self.node_index == self.skeleton.size() {
					for degree in (0..depth).rev() {
						if degree == last_degree {
							line += "╰";
						} else {
							line += "─";
						}
						line += line_spacing;
					}
				} else {
					let link_degree = (self.node_index - 1).trailing_ones() as usize;
					for degree in (0..depth).rev() {
						if degree == link_degree {
							line += "╰";
							line += line_spacing;
						} else if degree < link_degree {
							line += "┬";
							line += line_spacing;
						} else if self.node_index & (1 << degree) == 0 {
							line += "│";
							line += space_spacing;
						} else {
							line += " ";
							line += space_spacing;
						}
					}
				}
				self.continuing_links = true
			}
			if self.options.show_sublists && self.skeleton.link_index_is_in_bounds(self.node_index) {
				if let Some(sublist) = self.skeleton.get_sublist_at(self.node_index) {
					if self.sub_iterator.is_none() && self.continuing_links {
						self.sub_iterator = Some(Box::new(Iter::new(
							sublist.skeleton(),
							SkeletonFormatOptions {
								highlighted_links: self.options.highlighted_links.iter()
								                       .map(|it| it >> depth).collect(),
								highlighted_nodes: self.options.highlighted_nodes.iter()
								                       .map(|it| it >> depth).collect(),
								..self.options
							},
							true,
						)));
					}
				}
				if let Some(iter) = &mut self.sub_iterator {
					if let Some(ref next) = (**iter).next() {
						line += next;
						self.continuing_links = true
					} else {
						self.sub_iterator = None;
						self.node_index += 1;
					}
				}
			}
			Some(line)
		} else {
			None
		}
	}
}

impl<'a, S: Spacing + Display, Sub: SpacedList<S>> Debug for CustomFormat<'a, S, Sub> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.skeleton.size == 0 {
			write!(f, "[empty skeleton]")?;
			return Ok(());
		}

		let iter = self.skeleton.format_lines(self.options.clone());
		for line in iter {
			f.write_str(line.as_str())?;
			f.write_char('\n')?;
		}

		Ok(())
	}
}