use std::env;
use nalgebra::Vector3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let in_path = &args[1];
    let out_path = &args[2];
    // optional argument 3 should be desired precision in number of decimal numbers
    let precision = args.get(3).unwrap_or(&String::from("1")).parse::<usize>().unwrap();

    let (contents, n_points, point_coords, n_triangles, mut triangle_specs) = parse_input(&in_path);
    println!("Contents: {}", contents);
    println!("Number of points: {}", n_points);
    println!("Point coordinates: {:?}", point_coords);
    println!("Number of triangles: {}", n_triangles);
    println!("Triangle specs: {:?}", triangle_specs);

    let centroid = compute_centroid(&point_coords, n_points);
    println!("Centroid: {:?}", centroid);

    for triangle in &mut triangle_specs {
        let outwards = compute_triangle_norm_vec_direction(&point_coords, triangle, &centroid);
        println!("Triangle {:?} is outwards: {}", triangle, outwards);
        if !outwards {
            println!("Triangle {:?} is inwards, reordering", triangle);
            triangle.swap(1, 2);
            println!("Triangle {:?} reordered", triangle);
        }
    }
    println!("Triangle specs: {:?}", triangle_specs);

    write_output(out_path, n_points, &point_coords, n_triangles, &triangle_specs, precision);


}


#[test]
fn test_parse_input() {
    use nalgebra::Vector3;

    let in_path = "tests/input.txt";
    let (contents, n_points, point_coords, n_triangles, triangle_specs)  = parse_input(in_path);
    assert_eq!(contents, "4\n0.0 0.0 0.0\n0.0 0.0 1.0\n0.0 1.0 0.0\n1.0 0.0 0.0\n4\n0 1 2\n0 3 2\n0 3 1\n1 2 3");
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


fn parse_input(in_path: &str) -> (String, usize,  Vec<Vector3<f64>>, usize, Vec<Vec<usize>>) { 
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

    return (contents, n_points, point_coords, n_triangles, triangle_specs);
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

fn compute_triangle_norm_vec_direction(points: &Vec<Vector3<f64>>, triangle: &Vec<usize>, centroid: &Vector3<f64>) -> bool {
    let v1 = points[triangle[1]] - points[triangle[0]];
    let v2 = points[triangle[2]] - points[triangle[0]];
    let norm_vec = v1.cross(&v2);
    let centroid_to_triangle = points[triangle[0]] - *centroid;
    let dot_prod = norm_vec.dot(&centroid_to_triangle);
    return dot_prod > 0.0;

}

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