use super::node::Node;
use crate::cli;
use std::{cmp::Ordering, convert::From};

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SortType {
    Name,
    Size,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Order {
    sort: SortType,
    dir_first: bool,
}

impl Order {
    /// Yields function pointer to the appropriate `Node` comparator.
    pub fn comparator(&self) -> Option<Box<dyn Fn(&Node, &Node) -> Ordering + '_>> {
        if self.dir_first {
            Some(Box::new(|a, b| {
                Self::dir_comparator(a, b, self.sort.comparator())
            }))
        } else {
            self.sort.comparator()
        }
    }

    fn dir_comparator(
        a: &Node,
        b: &Node,
        fallback: Option<impl Fn(&Node, &Node) -> Ordering>,
    ) -> Ordering {
        match (a.is_dir(), b.is_dir()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => {
                if let Some(sort) = fallback {
                    sort(a, b)
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

impl SortType {
    /// Yields function pointer to the appropriate `Node` comparator.
    pub fn comparator(&self) -> Option<Box<dyn Fn(&Node, &Node) -> Ordering>> {
        match self {
            Self::Name => Some(Box::new(Self::name_comparator)),
            Self::Size => Some(Box::new(Self::size_comparator)),
            _ => None,
        }
    }

    /// Comparator based on `Node` file names.
    fn name_comparator(a: &Node, b: &Node) -> Ordering {
        a.file_name().cmp(b.file_name())
    }

    /// Comparator based on `Node` file sizes
    fn size_comparator(a: &Node, b: &Node) -> Ordering {
        let a_size = a.file_size.unwrap_or(0);
        let b_size = b.file_size.unwrap_or(0);

        b_size.cmp(&a_size)
    }
}

impl From<(cli::Order, bool)> for Order {
    fn from((order, dir_first): (cli::Order, bool)) -> Self {
        Order {
            sort: order.into(),
            dir_first,
        }
    }
}

impl From<cli::Order> for SortType {
    fn from(ord: cli::Order) -> Self {
        match ord {
            cli::Order::Name => SortType::Name,
            cli::Order::Size => SortType::Size,
            cli::Order::None => SortType::None,
        }
    }
}
