use super::{Coordinate, Float};

/// Represents a point on an edge which intersects with another edge.
///
/// The intersection may either be a single point, or a line segment (in which case this point is
/// the start of the line segment) The intersection point must be precise.
#[derive(Debug)]
pub(crate) struct EdgeIntersection<F: Float> {
    coord: Coordinate<F>,
    segment_index: usize,
    dist: F,
}

impl<F: Float> EdgeIntersection<F> {
    pub fn new(coord: Coordinate<F>, segment_index: usize, dist: F) -> EdgeIntersection<F> {
        EdgeIntersection {
            coord,
            segment_index,
            dist,
        }
    }

    pub fn coordinate(&self) -> Coordinate<F> {
        self.coord
    }

    pub fn segment_index(&self) -> usize {
        self.segment_index
    }

    pub fn distance(&self) -> F {
        self.dist
    }
}

impl<F: Float> std::cmp::PartialEq for EdgeIntersection<F> {
    fn eq(&self, other: &EdgeIntersection<F>) -> bool {
        // BTreeMap requires nodes to be fully `Ord`, but we're comparing floats. Can we guarantee
        // that all nodes are non-NaN?
        debug_assert!(!self.dist.is_nan() && !other.dist.is_nan());

        self.segment_index == other.segment_index && self.dist == other.dist
    }
}

impl<F: Float> std::cmp::Eq for EdgeIntersection<F> {}

impl<F: Float> std::cmp::PartialOrd for EdgeIntersection<F> {
    fn partial_cmp(&self, other: &EdgeIntersection<F>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: Float> std::cmp::Ord for EdgeIntersection<F> {
    fn cmp(&self, other: &EdgeIntersection<F>) -> std::cmp::Ordering {
        if self.segment_index < other.segment_index {
            return std::cmp::Ordering::Less;
        }
        if self.segment_index > other.segment_index {
            return std::cmp::Ordering::Greater;
        }
        if self.dist < other.dist {
            return std::cmp::Ordering::Less;
        }
        if self.dist > other.dist {
            return std::cmp::Ordering::Greater;
        }

        // BTreeMap requires nodes to be fully `Ord`, but we're comparing floats. Can we guarantee
        // that all nodes are non-NaN?
        debug_assert!(!self.dist.is_nan() && !other.dist.is_nan());

        return std::cmp::Ordering::Equal;
    }
}

impl<F: Float> EdgeIntersection<F> {}
