fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();
    if total as usize % n != 0 {
        return -1;
    }

    let avg = total / n as u32;
    let mut moves = 0;
    let mut balance = 0;

    for &ship in shipments {
        balance += ship as i32 - avg as i32;
        moves += balance.abs() as usize;
    }

    moves as isize
}

fn can_balance(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    total as usize % shipments.len() == 0
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut vec = vec![4; n];
    if n >= 2 {
        vec[0] += 2;
        vec[1] -= 2;
    }
    vec
}

fn main() {
    let example1 = vec![2, 8, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];

    println!("Example 1: {:?}", example1);
    println!("Moves: {}", count_permutation(&example1));

    println!("Example 2: {:?}", example2);
    println!("Moves: {}", count_permutation(&example2));

    let generated = gen_shipments(5);
    println!("Generated: {:?}", generated);
    println!("Can balance? {}", can_balance(&generated));
    println!("Moves: {}", count_permutation(&generated));
}
