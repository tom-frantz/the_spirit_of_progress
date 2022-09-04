use crate::components::world::tectonics::plate::Plate;

#[derive(Clone, Debug)]
pub enum PlateBoundaryType {
    Convergent(i32),
    Divergent(i32),
    Transform,
}

pub enum ConvergentBoundary {}

pub enum DivergentBoundary {}

impl PlateBoundaryType {
    pub fn new(src_plate: &Plate, dest_plate: &Plate) -> Self {
        let plate_vec_difference = src_plate.plate_drift_speed() - dest_plate.plate_drift_speed();

        let delta_speed = plate_vec_difference.vertical() + plate_vec_difference.horizontal();
        if delta_speed == 0 {
            PlateBoundaryType::Transform
        } else if delta_speed > 0 {
            // Convergent boundary => older = denser = will subduct.
            PlateBoundaryType::Convergent(delta_speed)
        } else {
            PlateBoundaryType::Divergent(delta_speed)
        }
    }

    // pub
}

#[derive(Clone, Debug)]
pub struct PlateBoundary<'a> {
    src_plate: &'a Plate,
    dest_plate: &'a Plate,
}

impl<'a> PlateBoundary<'a> {
    pub fn new(src_plate: &'a Plate, dest_plate: &'a Plate) -> Self {
        PlateBoundary {
            src_plate,
            dest_plate,
        }
    }

    pub fn boundary_type(&self) -> PlateBoundaryType {
        PlateBoundaryType::new(self.src_plate, self.dest_plate)
    }
}
