fn subdivide_vector<'a, T, F>(original_vector: &'a Vec<T>, condition: F) -> Vec<Vec<&'a T>>
where
    T: 'a,
    F: Fn(&T) -> bool,
{
    let mut result: Vec<Vec<&'a T>> = Vec::new();
    let mut current_subvector: Vec<&'a T> = Vec::new();

    for element in original_vector.iter() {
        if condition(element) {
            // Start a new subvector
            if !current_subvector.is_empty() {
                result.push(current_subvector.clone());
                current_subvector.clear();
            }
        } else {
            // Add to the current subvector
            current_subvector.push(element);
        }
    }

    // Add the last subvector if non-empty
    if !current_subvector.is_empty() {
        result.push(current_subvector);
    }

    result
}

fn main() {
    let original_vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Example: Subdivide based on even numbers
    let even_numbers = subdivide_vector(&original_vector, |&x| x % 2 == 0);
    println!("Even Numbers: {:?}", even_numbers);

    // Example: Subdivide based on odd numbers
    let odd_numbers = subdivide_vector(&original_vector, |&x| x % 2 != 0);
    println!("Odd Numbers: {:?}", odd_numbers);
}