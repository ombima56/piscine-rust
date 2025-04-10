pub mod areas_volumes;

pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let container_area = x * y;

    // Calculate the area of a single shape based on its type
    let single_shape_area = match objects {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
    };

    // Calculate total area needed for all shapes
    let total_area_needed = single_shape_area * (times as f64);

    // Compare and return result
    (container_area as f64) >= total_area_needed
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    // Calculate the container volume
    let container_volume = x * y * z;

    // Calculate the volume of a single object based on its type
    let single_volume = match objects {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        GeometricalVolumes::Pyramid => {
            // For Pyramid, 'a' is the base_area and 'b' is the height
            // But we need to ensure base_area is a f64 as expected by the function
            areas_volumes::triangular_pyramid_volume(a as f64, b)
        },
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
    };

    // Calculate total volume needed for all objects
    let total_volume_needed = single_volume * (times as f64);

    (container_volume as f64) >= total_volume_needed
}
