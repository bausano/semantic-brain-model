use super::neuron::Neuron;
use super::point::Point;
use std::rc::Rc;

pub enum Table {
    /// Leaf only needs to know where is it in the space and what neuron it represents.
    Leaf(Point, usize),
    Node {
        /// Midpoint that splits the table into 4 quadrants.
        split: Point,

        /// Each table is split into four quadrants.
        quadrants: [Option<Rc<Table>>; 4],

        /// Whether the table is limited by a bottom left point.
        lower_bound: Option<Point>,

        /// Whether the table is limited by a top right point.
        upper_bound: Option<Point>,
    },
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

    pub fn insert(&mut self, neuron: usize, at: Point) {
        let quadrant = which_quadrant(self.split, at);

        if let Some(neuron) = self.neuron {
            self.neuron = None;

            return;
        }

        let boxed: &Option<Rc<Table>> = &self.quadrants[quadrant];

        match boxed {
            Some(cell) => cell.insert(neuron, at),
            None => {
                let mut quadrant_table = Table::new(at);
                quadrant_table.neuron = Some(neuron);
            }
        }

        // if neuron already exists
        //   create two new tables with both new and old neurons
        //   remove neuron from this table
        //   push new tables into this table with appropriate bounds
        // else if quadrant table doesnt exist
        //   create new table with the neuron and assign it as the quadrant
        // else
        //   calt insert on the quadrant table
    }

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

        let quadrant = which_quadrant(self.split, point);

        let boxed: &Option<Rc<Table>> = &self.quadrants[quadrant];

        match boxed {
            Some(cell) => cell.at(point),
            None => None,
        }
    }

    /// Finds all neurons in given circle.
    pub fn around(&self, center: Point, radius: f64) -> Option<usize> {
        None
    }
}

/// Decides to which quadrant does a point belong to with respect to a center point.
///
///   point.y >= table.y   | point.y >= table.y
///   point.x < table.x    | point.x >= table.x
///  ----------------------+--------------------
///   point.y < table.y    | point.y < table.y
///   point.x <= table.x   | point.x > table.x
fn which_quadrant(center: Point, point: Point) -> usize {
    if point.y >= self.split.y {
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
}
