use super::{Location, Position, TopologyLocation};

use std::fmt;

/// A `Label` indicates the topological relationship of a component of a topology graph to a given
/// [Geometry].
///
/// This class supports labels for relationships to two `Geometry`s, which is sufficient for
/// algorithms for binary operations.
///
/// Topology graphs support the concept of labeling nodes and edges in the graph.  The label of a
/// node or edge specifies its topological relationship to one or more geometries.  (In fact, since
/// JTS operations have only two arguments labels are required for only two geometries).  A label
/// for a node or edge has one or two elements, depending on whether the node or edge occurs in one
/// or both of the input `Geometry`s.  Elements contain attributes which categorize the topological
/// location of the node or edge relative to the parent `Geometry`; that is, whether the node or
/// edge is in the interior, boundary or exterior of the `Geometry`.  Attributes have a value
/// from the set `{Interior, Boundary, Exterior}`.  In a node each element has  a single attribute
/// `(On)`.  For an edge each element has a triplet of attributes `(Left, On, Right)`.
///
/// It is up to the client code to associate the 0 and 1 [TopologyLocation]s with specific
/// geometries.
#[derive(Clone)]
pub(crate) struct Label {
    // REVIEW: better name? what does this stand for - "element location's topology"?
    elt: [TopologyLocation; 2],
}

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Label {{ A: {:?}, B: {:?} }}",
            &self.elt[0], &self.elt[1]
        )
    }
}

impl Label {
    pub fn new_with_on_location(location: Option<Location>) -> Label {
        Label {
            elt: [
                TopologyLocation::new_on(location),
                TopologyLocation::new_on(location),
            ],
        }
    }

    pub fn new_with_geom_on_location(geom_index: usize, location: Option<Location>) -> Label {
        Label {
            elt: [
                if geom_index == 0 {
                    TopologyLocation::new_on(location)
                } else {
                    TopologyLocation::new_on(None)
                },
                if geom_index == 1 {
                    TopologyLocation::new_on(location)
                } else {
                    TopologyLocation::new_on(None)
                },
            ],
        }
    }

    pub fn new_with_locations(
        on_location: Option<Location>,
        left_location: Option<Location>,
        right_location: Option<Location>,
    ) -> Label {
        Label {
            elt: [
                TopologyLocation::new_on_left_right(on_location, left_location, right_location),
                TopologyLocation::new_on_left_right(on_location, left_location, right_location),
            ],
        }
    }

    pub fn new_with_geom_locations(
        geom_index: usize,
        on_location: Location,
        left_location: Location,
        right_location: Location,
    ) -> Label {
        let mut label = Label {
            elt: [
                TopologyLocation::new_on_left_right(None, None, None),
                TopologyLocation::new_on_left_right(None, None, None),
            ],
        };
        label.elt[geom_index].set_locations(on_location, left_location, right_location);
        label
    }

    pub fn flip(&mut self) {
        self.elt[0].flip();
        self.elt[1].flip();
    }

    pub fn location(&self, geom_index: usize, position: Position) -> Option<Location> {
        self.elt[geom_index].get(position)
    }

    pub fn on_location(&self, geom_index: usize) -> Option<Location> {
        self.elt[geom_index].get(Position::On)
    }

    pub fn set_location(&mut self, geom_index: usize, position: Position, location: Location) {
        self.elt[geom_index].set_location(position, Some(location));
    }

    pub fn set_on_location(&mut self, geom_index: usize, location: Option<Location>) {
        self.elt[geom_index].set_location(Position::On, location);
    }

    pub fn set_all_locations(&mut self, geom_index: usize, location: Location) {
        self.elt[geom_index].set_all_locations(location)
    }

    pub fn set_all_locations_if_empty(&mut self, geom_index: usize, location: Location) {
        self.elt[geom_index].set_all_locations_if_empty(location)
    }

    pub fn geometry_count(&self) -> usize {
        self.elt
            .iter()
            .filter(|location| !location.is_empty())
            .count()
    }

    pub fn is_empty(&self, geom_index: usize) -> bool {
        self.elt[geom_index].is_empty()
    }

    pub fn is_any_empty(&self, geom_index: usize) -> bool {
        self.elt[geom_index].is_any_empty()
    }

    pub fn is_area(&self) -> bool {
        self.elt[0].is_area() || self.elt[1].is_area()
    }

    pub fn is_geom_area(&self, geom_index: usize) -> bool {
        self.elt[geom_index].is_area()
    }

    pub fn is_line(&self, geom_index: usize) -> bool {
        self.elt[geom_index].is_line()
    }
}
