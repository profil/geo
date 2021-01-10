use crate::algorithm::dimensions::Dimensions;
use crate::geomgraph::Location;

#[derive(PartialEq, Eq)]
pub(crate) struct IntersectionMatrix([[Dimensions; 3]; 3]);

impl std::fmt::Debug for IntersectionMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fn char_for_dim(dim: &Dimensions) -> &'static str {
            match dim {
                Dimensions::Empty => "F",
                Dimensions::ZeroDimensional => "0",
                Dimensions::OneDimensional => "1",
                Dimensions::TwoDimensional => "2",
            }
        }
        let text = self
            .0
            .iter()
            .flat_map(|r| r.iter().map(char_for_dim))
            .collect::<Vec<&str>>()
            .join("");

        write!(f, "IntersectionMatrix({})", &text)
    }
}

impl IntersectionMatrix {
    pub fn empty() -> Self {
        IntersectionMatrix([[Dimensions::Empty; 3]; 3])
    }

    // CLEANUP: remove?
    #[allow(dead_code)]
    pub fn new(dimensions: [[Dimensions; 3]; 3]) -> Self {
        IntersectionMatrix(dimensions)
    }

    #[cfg(test)]
    pub fn from_str(str: &str) -> Self {
        let mut im = IntersectionMatrix::empty();
        im.set_at_least_from_string(str);
        im
    }

    pub fn set(&mut self, location_a: Location, location_b: Location, dimensionality: Dimensions) {
        self.0[location_a as usize][location_b as usize] = dimensionality;
    }

    pub fn set_at_least(
        &mut self,
        location_a: Location,
        location_b: Location,
        minimum_dimension_value: Dimensions,
    ) {
        if self.0[location_a as usize][location_b as usize] < minimum_dimension_value {
            self.0[location_a as usize][location_b as usize] = minimum_dimension_value;
        }
    }

    pub fn set_at_least_if_valid(
        &mut self,
        location_a: Option<Location>,
        location_b: Option<Location>,
        minimum_dimension_value: Dimensions,
    ) {
        if let Some(location_a) = location_a {
            if let Some(location_b) = location_b {
                self.set_at_least(location_a, location_b, minimum_dimension_value);
            }
        }
    }

    pub fn set_at_least_from_string(&mut self, dimensions: &str) {
        if dimensions.len() != 9 {
            todo!("return proper error, or better yet make this a compile time macro")
        }

        let mut i = 0;
        for c in dimensions.chars() {
            let a = i / 3;
            let b = i % 3;
            i += 1;
            match c {
                '0' => self.0[a][b] = self.0[a][b].max(Dimensions::ZeroDimensional),
                '1' => self.0[a][b] = self.0[a][b].max(Dimensions::OneDimensional),
                '2' => self.0[a][b] = self.0[a][b].max(Dimensions::TwoDimensional),
                'F' => {}
                _ => todo!("return proper error, or better yet make this a compile time macro"),
            }
        }
    }
}
