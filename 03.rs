fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            result[x][y] = matrix[y][x]
        }
    }
    return result
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- комментарий заставляет rustfmt добавить новую строку
        [201, 202, 203],
        [301, 302, 303],
    ];
    

    println!("матрица: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("транспонированная матрица: {:#?}", transposed);
}
