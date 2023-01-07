fn pretty_print(array: &[[i32; 3]; 3]) {
    for row in array {
        println!("{row:?}")
    }
}

fn transpose(array: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];
    for row in 0..3 {
        for column in 0..3 {
            transposed[row][column] = array[column][row]
        }
    }
    transposed
}

pub fn arrays() {
    let array = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    pretty_print(&array);
    println!();
    pretty_print(&transpose(array))
}