mod neuron;
mod point;
mod table;

use self::neuron::Neuron;
use self::point::Point;
use self::table::Table;

pub struct SelfOrganizingMap {
    /// Root node which represents the whole search space.
    root: Table,

    /// Size of the ideal leaf that will eventually most likely contain only four
    /// elements.
    leaf_size: f64,

    /// How many neurons does the map contain.
    items: usize,

    /// How many leafs have more than 4 items.
    overused_leafs: usize,
    
    /// How many leafs have less than 4 items.
    underused_leafes: usize,
}

impl SelfOrganizingMap {
    /// This implementation runs in O(n^2). The performance to be improved with a
    /// list ordered by x and approximating several division lines parallel to x.
    pub fn new(items: Vec<(Point, Neuron)>) {
        assert!(items.len() > 3);

        let mut average_x = 0_f64;
        let mut average_y = 0_f64;
        let mut average_square = 0_f64;

        for (index, (point, _)) in items.iter().enumerate() {
            average_x += (point.x - average_x) / (index + 1) as f64;
            average_y += (point.x - average_y) / (index + 1) as f64;

            let mut neighbours: Vec<Option<(f64, Point)>> = vec![None, None, None];

            // This iterates through all other points, making this operation extremely expensive.
            for (another_index, (another_point, _)) in items.iter().enumerate() {
                if another_index == index {
                    continue;
                }

                // Calculates the distance between the pair of points.
                let mut distance = point.distance_to(another_point);

                // Updates the neighbours vec storing only the 3 closest points.
                for mut neighbour in neighbours.iter_mut() {
                    if neighbour
                        .get_or_insert_with(|| (distance, *another_point))
                        .0
                        < distance
                    {
                        distance = neighbour.replace((distance, *another_point)).unwrap().0;
                    }
                }
            }

            // Finds max and min x, y among all neighbours and the point.
            let (min_x, min_y, max_x, max_y) = neighbours.iter().fold(
                (point.x, point.y, point.x, point.y),
                |(min_x, min_y, max_x, max_y), o| {
                    // We have asserted that there are at least 4 items in the vector,
                    // therefore we can safely unwrap.
                    let (_, neighbour) = o.unwrap();

                    (
                        min_x.min(neighbour.x),
                        min_y.min(neighbour.y),
                        max_x.max(neighbour.x),
                        max_y.max(neighbour.y),
                    )
                },
            );

            // Size of the square that would include all 4 points.
            let square_size = (max_x - min_x).abs().max((max_y - min_y_.abs());
            average_square += (square_size - average_square) / (index + 1) as f64;
        }

        // TODO: Create the basic tables based on averages.
        
    }
}

