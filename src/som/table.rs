use super::point::Point;

pub struct Table {
    /// Midpoint that splits the table into 4 quadrants.
    split: Point,

    /// Each table is split into four quadrants.
    quadrants: [Option<Box<Table>>; 4],

    /// Whether the table is limited by a bottom left point.
    lower_bound: Option<f64>,

    /// Whether the table is limited by a top right point.
    upper_bound: Option<f64>,
}

impl Table {
    /// Builds new empty table with point that splits it into 4 quadrants.
    pub fn new(split: Point) -> Table {
        Table {
            split,
            lower_bound: None,
            upper_bound: None,
            quadrants: [None, None, None, None],
        }
    }
}
