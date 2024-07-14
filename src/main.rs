use learn_computer_graphics_in_rust::math::{utils::root_mean_square, Matrix, Vector};

fn main() {
    // println!("{:?}", quadratic_equation(2.0, 6.0, 4.0));
    // println!("{:?}", quadratic_equation(2.0, -6.0, 4.0));
    // println!("{:?}", quadratic_equation(1.0, 1.0, 1.0));

    // let vector = Vector::new(1.0, -1.0, 0.0);
    let vector = Vector::new(0.00038527316, 0.00038460016, -0.99999988079);
    if let Some((u, v)) = vector.orthonormal_basis_simple() {
        let w = vector / vector.length();
        let mat = Matrix::from_vectors(u, v, w);
        println!("The simple method works well for vector [0.00038527316, 0.00038460016, -0.99999988079]:\n{}", &mat * mat.transpose());
        println!("RMS: {}", root_mean_square(&w, &u, &v));
    }

    println!("\nFrisvad's method has large errors for vector [0.00038527316, 0.00038460016, -0.99999988079]");

    // if let Some((u, v)) = vector.orthonormal_basis_frisvad() {
    //     let w = vector / vector.length();
    //     let mat = Matrix::from_vectors(u, v, w);
    //     println!("Should be identity matrix:\n{}", &mat * mat.transpose());
    //     println!("RMS: {}", root_mean_square(&w, &u, &v));
    // }
    // use unit vector directly
    let w = Vector::new(0.00038527316, 0.00038460016, -0.99999988079);
    let (u, v) = w.orthonormal_basis_frisvad_from_unit();
    let mat = Matrix::from_vectors(u, v, w);
    println!("Should be identity matrix:\n{}", &mat * &mat.transpose());
    println!("RMS: {}", root_mean_square(&w, &u, &v));

    println!("\nFrisvad's method also has large errors for another vector [-0.00019813581, -0.00008946839, -0.99999988079]");

    let w = Vector::new(-0.00019813581, -0.00008946839, -0.99999988079);
    let (u, v) = w.orthonormal_basis_frisvad_from_unit();
    let mat = Matrix::from_vectors(u, v, w);
    println!("Resulting frame:\n{}", mat);
    println!("RMS: {}", root_mean_square(&w, &u, &v));

    println!("\nTest the revised method:");

    let w = Vector::new(0.00038527316, 0.00038460016, -0.99999988079);
    let (u, v) = w.orthonormal_basis_revised_from_unit();
    let mat = Matrix::from_vectors(u, v, w);
    println!("Should be identity matrix:\n{}", &mat * &mat.transpose());
    println!("RMS: {}", root_mean_square(&w, &u, &v));

    println!();

    let w = Vector::new(-0.00019813581, -0.00008946839, -0.99999988079);
    let (u, v) = w.orthonormal_basis_revised_from_unit();
    let mat = Matrix::from_vectors(u, v, w);
    println!("Resulting frame:\n{}", mat);
    println!("RMS: {}", root_mean_square(&w, &u, &v));
}
