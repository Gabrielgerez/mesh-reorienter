use std::env;
use nalgebra::Vector3;

/// # Triangle meshgrid vertex reorienter.
/// 
/// Run the script as follows:
/// ```shell
/// cargo run --release input.txt output.txt
/// ```
/// where input.txt is the input file with the following format:
/// ```
/// <number of points>
/// <x0> <y0> <z0>
/// ...
/// <number of triangles>
/// <point_index0> <point_index1> <point_index2>
/// ...
/// ```
/// and output.txt is the output file with the same format.
/// The first line of the input file is the number of points in the meshgrid as an integer.
/// the next n lines are the x, y, z coordinates of the points in the meshgrid.
/// The next line is the number of triangles in the meshgrid as an integer.
/// The next n lines are the indices of the points that form the triangles in the meshgrid. 
/// The indices index the point list above.
/// 
/// the program works as follows:
/// 1. compute the centroid of the surface described by the meshgrid. 
/// 2. For each triangle, 
///     1. compute the normal vector of the triangle as the cross product of the two vectors described by indices 0->1 and 0->2.
///     2. Compare the normal to the centroid of the meshgrid. 
///         - If the dot product of the normal and the vector from the centroid to the first point of the triangle is positive, the normal points outwards from the centroid and the triangle is correctly aligned.
///         - If the dot product is negative, the normal points inwards from the centroid, and the order of the verteces must be reversed.
/// 3. Once all triangles have been checked and rearranged, write the output file with the same format as the input file.
/// 
/// in this program we assume that the centroid of the meshgrid is contained inside the surface described by the triangle meshgrid.
/// 
fn main() {
    
    let args: Vec<String> = env::args().collect();

    let in_path = &args[1];
    let out_path = &args[2];
    // optional argument 3 should be desired precision in number of decimal numbers
    let precision = args.get(3).unwrap_or(&String::from("1")).parse::<usize>().unwrap();

    let (n_points, point_coords, n_triangles, mut triangle_specs) = parse_input(&in_path);

    let centroid = compute_centroid(&point_coords, n_points);

    for triangle in &mut triangle_specs {
        let outwards = compute_triangle_norm_vec_direction(&point_coords, triangle, &centroid);
        if !outwards {
            triangle.swap(1, 2);
        }
    }
    write_output(out_path, n_points, &point_coords, n_triangles, &triangle_specs, precision);
}


#[test]
fn test_parse_input() {
    use nalgebra::Vector3;

    let in_path = "tests/input.txt";
    let ( n_points, point_coords, n_triangles, triangle_specs)  = parse_input(in_path);
    assert_eq!(n_points, 4);
    assert_eq!(point_coords, vec![
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.0, 0.0, 1.0), 
        Vector3::new(0.0, 1.0, 0.0), 
        Vector3::new(1.0, 0.0, 0.0)
    ]);
    assert_eq!(n_triangles, 4);
    assert_eq!(triangle_specs, vec![vec![0, 1, 2], vec![0, 3, 2], vec![0, 3, 1], vec![1, 2, 3]]);
}

/// Parse the input file and return the number of points, the point coordinates, the number of triangles and the triangle specifications.
fn parse_input(in_path: &str) -> (usize,  Vec<Vector3<f64>>, usize, Vec<Vec<usize>>) { 
    // read file and divide it in lines to be parsed later
    let contents = std::fs::read_to_string(in_path)
        .expect("Something went wrong reading the file");

    let mut lines = contents.lines();
    let n_points = lines.next().unwrap().parse::<usize>().unwrap();

    let point_coords: Vec<Vector3<f64>> = lines
        .by_ref()
        .take(n_points)
        .map(|line| {
            let mut coords = line.split_whitespace();
            let x = coords.next().unwrap().parse::<f64>().unwrap();
            let y = coords.next().unwrap().parse::<f64>().unwrap();
            let z = coords.next().unwrap().parse::<f64>().unwrap();
            Vector3::new(x, y, z)
        })
        .collect();

    let n_triangles = lines.next().unwrap().parse::<usize>().unwrap();

    let triangle_specs: Vec<Vec<usize>> = lines
    .map(|line| {
        let indices = line.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        indices
    })
    .collect();

    return (n_points, point_coords, n_triangles, triangle_specs);
    }


#[test]
fn test_compute_centroid() {
    let points = vec![
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.0, 0.0, 1.0), 
        Vector3::new(0.0, 1.0, 0.0), 
        Vector3::new(1.0, 0.0, 0.0)
    ];
    let npoints : usize = 4;
    let centroid = compute_centroid(&points, npoints);
    assert_eq!(centroid, Vector3::new(0.25, 0.25, 0.25));
}
/// Compute the centroid of a set of points.
/// This is done by summing all the points and dividing the result by the number of points.
fn compute_centroid(points: &Vec<Vector3<f64>>, npoints: usize) -> Vector3<f64> {
    let mut centroid = Vector3::new(0.0, 0.0, 0.0);
    for i in 0..npoints {
        centroid += points[i];
    }
    centroid /= npoints as f64;
    return centroid;
}

#[test]
fn test_compute_triangle_norm_vec_direction() {
    let points = vec![
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.0, 0.0, 1.0), 
        Vector3::new(0.0, 1.0, 0.0), 
        Vector3::new(1.0, 0.0, 0.0)
    ];
    let triangle = vec![0, 1, 2];
    let centroid = Vector3::new(0.25, 0.25, 0.25);
    let outwards : bool = compute_triangle_norm_vec_direction(&points, &triangle, &centroid);
    assert_eq!(outwards, true);
}

/// Compute the direction of the normal vector of a triangle with respect to the centroid of the meshgrid.
/// The normal vector is computed as the cross product of the vectors described by the indices 0->1 and 0->2.
/// The direction of the normal vector is determined by the dot product of the normal vector and the vector from the centroid to the first point of the triangle.
fn compute_triangle_norm_vec_direction(points: &Vec<Vector3<f64>>, triangle: &Vec<usize>, centroid: &Vector3<f64>) -> bool {
    let v1 = points[triangle[1]] - points[triangle[0]];
    let v2 = points[triangle[2]] - points[triangle[0]];
    let norm_vec = v1.cross(&v2);
    let centroid_to_triangle = points[triangle[0]] - *centroid;
    let dot_prod = norm_vec.dot(&centroid_to_triangle);
    return dot_prod > 0.0;

}

/// Write the output file with the same format as the input file.
fn write_output(out_path: &str, n_points: usize, point_coords: &Vec<Vector3<f64>>, n_triangles: usize, triangle_specs: &Vec<Vec<usize>>, precision: usize) {
    let mut out_contents = String::new();
    out_contents.push_str(&n_points.to_string());
    out_contents.push_str("\n");
    for i in 0..n_points {
        out_contents.push_str(&format!("{:.*}", precision, point_coords[i].x));
        out_contents.push_str(" ");
        out_contents.push_str(&format!("{:.*}", precision, point_coords[i].x));
        out_contents.push_str(" ");
        out_contents.push_str(&format!("{:.*}", precision, point_coords[i].x));
        out_contents.push_str("\n");
    }
    out_contents.push_str(&n_triangles.to_string());
    out_contents.push_str("\n");
    for i in 0..n_triangles {
        out_contents.push_str(&triangle_specs[i][0].to_string());
        out_contents.push_str(" ");
        out_contents.push_str(&triangle_specs[i][1].to_string());
        out_contents.push_str(" ");
        out_contents.push_str(&triangle_specs[i][2].to_string());
        out_contents.push_str("\n");
    }
    std::fs::write(out_path, out_contents)
        .expect("Something went wrong writing the file");
}