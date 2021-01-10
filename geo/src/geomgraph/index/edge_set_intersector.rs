use super::super::{Edge, Float};
use super::SegmentIntersector;

use std::cell::RefCell;
use std::rc::Rc;

pub(crate) trait EdgeSetIntersector<F>
where
    F: Float,
{
    fn compute_intersections(
        &mut self,
        edges: &[Rc<RefCell<Edge<F>>>],
        segment_intersector: &mut SegmentIntersector<F>,
        test_all_segments: bool,
    );

    fn compute_intersections_testing_all_segments(
        &mut self,
        edges0: &[Rc<RefCell<Edge<F>>>],
        edges1: &[Rc<RefCell<Edge<F>>>],
        segment_intersector: &mut SegmentIntersector<F>,
    );
}
