use geo::{
    algorithm::area::Area, map_coords::MapCoords, CoordFloat, CoordNum, Coordinate, LineString,
    Polygon,
};
use std::{f64::consts::PI};

fn main() {
    let polygon = {
        const NUM_VERTICES: usize = 10;
        const ANGLE_INC: f64 = 2. * PI / NUM_VERTICES as f64;

        Polygon::new(
            (0..NUM_VERTICES)
                .map(|i| {
                    let angle = i as f64 * ANGLE_INC;
                    Coordinate {
                        x: angle.cos(),
                        y: angle.sin(),
                    }
                })
                .collect::<Vec<_>>()
                .into(),
            vec![],
        )
    };

    println!("Effect of shift");
    let pre_shift_area = polygon.signed_area();
    for exp in 0..20 {
        let shift = 1.5 * f64::from(exp*3).exp2();
        let shifted_polygon = polygon.map_coords(|&(x, y)| (x + shift, y + shift));

        let area = shifted_polygon.signed_area();
        let no_shift_area = get_linestring_area(shifted_polygon.exterior());
        println!(
            "{:.4e}, {:.4e}, {:.4e}, {:.4e}, {:.4e}",
            shift,
            pre_shift_area,
            area,
            (pre_shift_area - area).abs()*100./area,
            (area - no_shift_area).abs()*100./area
        );
    }

    println!("Effect of expansion");
    let shift = 1.5e15;
    for exp in 0..20 {
        let expansion = 1.5 * f64::from(exp*3).exp2();
        let expanded_polygon = polygon.map_coords(|&(x, y)| (x*expansion, y*expansion));
        let expanded_area = polygon.signed_area();
        let shifted_polygon = expanded_polygon.map_coords(|&(x, y)| (x + shift, y + shift));

        let area = shifted_polygon.signed_area();
        let no_shift_area = get_linestring_area(shifted_polygon.exterior());
        println!(
            "{:.4e}, {:.4e}, {:.4e}, {:.4e}, {:.4e}",
            expansion,
            expanded_area,
            area,
            (expanded_area - area).abs()*100./area,
            (area - no_shift_area).abs()*100./area
        );
    }

    // TODO: How does the error vary when the size of the polygon is changed?
}

// Calculation of simple (no interior holes) Polygon area
pub(crate) fn get_linestring_area<T>(linestring: &LineString<T>) -> T
where
    T: CoordFloat,
{
    twice_signed_ring_area(linestring) / (T::one() + T::one())
}

fn twice_signed_ring_area<T>(linestring: &LineString<T>) -> T
where
    T: CoordNum,
{
    // LineString with less than 3 points is empty, or a
    // single point, or is not closed.
    if linestring.0.len() < 3 {
        return T::zero();
    }

    // Above test ensures the vector has at least 2 elements.
    // We check if linestring is closed, and return 0 otherwise.
    if linestring.0.first().unwrap() != linestring.0.last().unwrap() {
        return T::zero();
    }

    let mut tmp = T::zero();
    for line in linestring.lines() {
        tmp = tmp + line.determinant();
    }

    tmp
}
