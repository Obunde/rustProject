use std::collections::HashSet;

// Function to find the greatest common divisor (GCD)
fn find_gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        find_gcd(b, a % b)
    }
}

// Function to generate the multiplicative group of integers modulo n
fn multiplicative_grp(n: u64) -> Vec<u64> {
    let mut mul_grp: Vec<u64> = Vec::new();

    // Loop through the numbers from 1 to n-1
    for i in 1..n {
        if find_gcd(i as i32, n as i32) == 1 {
            mul_grp.push(i);
        }
    }

    mul_grp
}

// Function to generate a subgroup given a generator
fn generate_subgroup(n: u64, generator: u64) -> Vec<u64> {
    let mut subgroup: Vec<u64> = Vec::new();
    let mut current = generator;

    // Loop until we find a repeating element in the subgroup
    while !subgroup.contains(&current) {
        subgroup.push(current);
        current = (current * generator) % n;
    }

    // Sort the subgroup to maintain order
    subgroup.sort_unstable();
    subgroup
}

// Function to generate all subgroups of a multiplicative group
fn generate_all_subgroups(n: u64) -> Vec<Vec<u64>> {
    // Get the elements of the multiplicative group
    let elements = multiplicative_grp(n);

    // Create an empty vector to store subgroups
    let mut subgroups: Vec<Vec<u64>> = Vec::new();

    // Create a HashSet to keep track of seen subgroups
    let mut seen_subgroups: HashSet<Vec<u64>> = HashSet::new();

    // Add the full group as a potential subgroup
    if !elements.is_empty() {
        subgroups.push(elements.clone());
        seen_subgroups.insert(elements.clone());
    }

    // Iterate through each element to generate subgroups
    for &e in &elements {
        let subgroup = generate_subgroup(n, e);

        // Check if the subgroup has not been seen before
        if !seen_subgroups.contains(&subgroup) {
            seen_subgroups.insert(subgroup.clone());
            subgroups.push(subgroup);
        }
    }

    subgroups
}

// Main function
fn main() {
    let n = 12;
    
    // Generate multiplicative group of integers modulo n
    let multiplicative_group = multiplicative_grp(n);
    println!("Multiplicative group for n = {}: {:?}", n, multiplicative_group);

    // Generate all subgroups of the multiplicative group
    let subgroups = generate_all_subgroups(n);
    println!("All subgroups: {:?}", subgroups);
}