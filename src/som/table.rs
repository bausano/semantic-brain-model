use super::neuron::Neuron;
use super::point::Point;
use std::rc::Rc;

pub struct Table {
    /// Midpoint that splits the table into 4 quadrants.
    split: Point,

    /// Each table is split into four quadrants.
    quadrants: [Option<Rc<Table>>; 4],

    /// Whether the table is limited by a bottom left point.
    lower_bound: Option<Point>,

    /// Whether the table is limited by a top right point.
    upper_bound: Option<Point>,

    /// Whether there is a neuron connected to the table. Table can have either
    /// quadrants or neuron, not both.
    neuron: Option<usize>,
}

impl Table {
    /// Builds new empty table with point that splits it into 4 quadrants.
    pub fn new(split: Point) -> Table {
        Table {
            split,
            neuron: None,
            lower_bound: None,
            upper_bound: None,
            quadrants: [None, None, None, None],
        }
    }

    pub fn insert(&mut self, neuron: usize, at: Point) {}

    /// Retrieves neuron at given point if exists.
    pub fn at(&self, point: Point) -> Option<usize> {
        // If this table has a point, it is a leaf node and split is a point which the neuron
        // lies on.
        if let Some(neuron) = self.neuron {
            return if point == self.split {
                Some(neuron)
            } else {
                None
            };
        }

        // Decides which quadrant should the point belong to.
        //
        //   point.y >= table.y   | point.y >= table.y
        //   point.x < table.x    | point.x >= table.x
        //  ----------------------+--------------------
        //   point.y < table.y    | point.y < table.y
        //   point.x <= table.x   | point.x > table.x
        //
        let quadrant = if point.y >= self.split.y {
            if point.x >= self.split.x {
                0
            } else {
                1
            }
        } else {
            if point.x <= self.split.x {
                2
            } else {
                3
            }
        };

        let boxed: &Option<Rc<Table>> = &self.quadrants[quadrant];

        match boxed {
            Some(cell) => cell.at(point),
            _ => None,
        }
    }

    /// Finds all neurons in given circle.
    pub fn around(&self, center: Point, radius: f64) -> Option<usize> {
        None
    }
}
