use super::{Location, Position};

use std::fmt;

#[derive(Clone)]
pub(crate) struct TopologyLocation {
    // CLEANUP: location is either 1 or 3, maybe cleaner to just have 3 separate Option<Location>
    // attributes, one for each: [on_location, left_location, right_location]
    // CLEANUP: can we make this non-optional (or some of them, if we split up properties)?
    // Or maybe something like:
    // pub enum TopologyLocation {
    //     On(Location),
    //     OneLeftRight(Location, Location, Location)
    // }
    location: Vec<Option<Location>>,
}

impl fmt::Debug for TopologyLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn location_to_str(location: Option<Location>, f: &mut fmt::Formatter) -> fmt::Result {
            match location {
                Some(Location::Interior) => write!(f, "i"),
                Some(Location::Boundary) => write!(f, "b"),
                Some(Location::Exterior) => write!(f, "e"),
                None => write!(f, "_"),
            }
        }
        match self.location.len() {
            1 => location_to_str(self.location[Position::On as usize], f)?,
            3 => {
                location_to_str(self.location[Position::Left as usize], f)?;
                location_to_str(self.location[Position::On as usize], f)?;
                location_to_str(self.location[Position::Right as usize], f)?;
            }
            _ => {
                debug_assert!(false, "invalid TopologyLocation");
                write!(f, "invalid TopologyLocation")?;
            }
        }
        Ok(())
    }
}

impl TopologyLocation {
    pub fn new_on_left_right(
        on_location: Option<Location>,
        left_location: Option<Location>,
        right_location: Option<Location>,
    ) -> TopologyLocation {
        TopologyLocation {
            location: vec![on_location, left_location, right_location],
        }
    }

    pub fn new_on(on_location: Option<Location>) -> TopologyLocation {
        TopologyLocation {
            location: vec![on_location],
        }
    }

    pub fn get(&self, pos: Position) -> Option<Location> {
        if (pos as usize) < self.location.len() {
            return self.location[pos as usize];
        } else {
            return None;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.location.iter().all(Option::is_none)
    }

    pub fn is_any_empty(&self) -> bool {
        self.location.iter().any(Option::is_none)
    }

    pub fn is_area(&self) -> bool {
        self.location.len() > 1
    }

    pub fn is_line(&self) -> bool {
        self.location.len() == 1
    }

    pub fn flip(&mut self) {
        if self.location.len() <= 1 {
            return;
        }
        self.location
            .swap(Position::Left as usize, Position::Right as usize);
    }

    pub fn set_all_locations(&mut self, location: Location) {
        for i in 0..self.location.len() {
            self.location[i] = Some(location)
        }
    }

    pub fn set_all_locations_if_empty(&mut self, location: Location) {
        for i in 0..self.location.len() {
            if self.location[i].is_none() {
                self.location[i] = Some(location)
            }
        }
    }

    // REVIEW: can we make location non-optional?
    pub fn set_location(&mut self, position: Position, location: Option<Location>) {
        self.location[position as usize] = location;
    }

    pub fn set_on_location(&mut self, location: Location) {
        self.location[Position::On as usize] = Some(location);
    }

    pub fn set_locations(&mut self, on: Location, left: Location, right: Location) {
        self.location[Position::On as usize] = Some(on);
        self.location[Position::Left as usize] = Some(left);
        self.location[Position::Right as usize] = Some(right);
    }
}
