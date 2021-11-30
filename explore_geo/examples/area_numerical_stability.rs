use geo::{
    algorithm::area::Area, map_coords::MapCoords, CoordFloat, CoordNum, Coordinate, LineString,
    Polygon,
};
use std::{f64::consts::PI};

fn main() {
    let polygon = {
        const NUM_VERTICES: usize = 10;
        const ANGLE_INC: f64 = 2. * PI / NUM_VERTICES as f64;
        const RADIUS: f64 = 10.;

        Polygon::new(
            (0..NUM_VERTICES)
                .map(|i| {
                    let angle = i as f64 * ANGLE_INC;
                    Coordinate {
                        x: RADIUS * angle.cos(),
                        y: RADIUS * angle.sin(),
                    }
                })
                .collect::<Vec<_>>()
                .into(),
            vec![],
        )
    };

    let shifts = (0..20).map(f64::from).map(|i| 1.5 * (i*3.).exp2()).collect::<Vec<f64>>();
    let output = compute_output(&polygon, &shifts);
    print_output(&output);
}

struct OutputRow {
    pub shift: f64,
    pub original_area: f64,
    pub shifted_area: f64,
    pub shift_relative_error: f64,
    pub naive_area_computation_error: f64
}

fn compute_output(polygon: &Polygon<f64>, shifts: &[f64]) -> Vec<OutputRow> {
    let original_area = polygon.signed_area();
    let mut output: Vec<OutputRow> = Vec::with_capacity(shifts.len());
    for shift in shifts {
        let shifted_polygon = polygon.map_coords(|&(x, y)| (x + shift, y + shift));
        let shifted_area = shifted_polygon.signed_area();
        let naive_area = get_linestring_area(shifted_polygon.exterior());
        output.push(OutputRow{
            shift: *shift,
            original_area: original_area,
            shifted_area: shifted_polygon.signed_area(),
            shift_relative_error: (original_area - shifted_area).abs()/original_area,
            naive_area_computation_error: (shifted_area - naive_area).abs()/shifted_area
        });
    }
    output
}

fn print_output(output: &[OutputRow]) {
    for row in output {
        println!(
            "{:.4e}, {:.4e}, {:.4e}, {:.4e}, {:.4e}",
            row.shift,
            row.original_area,
            row.shifted_area,
           row.shift_relative_error,
           row.naive_area_computation_error
        );
    }
}

fn get_linestring_area<T>(linestring: &LineString<T>) -> T
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
