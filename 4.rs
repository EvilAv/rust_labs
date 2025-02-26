// Вычислите модуль вектора просуммировав квадраты его координат
// и вычислив квадратный корень полученного значения. Используйте метод `sqrt()` для вычисления
// корня, следующим образом: v.sqrt().


fn magnitude(vector: &[f64]) -> f64 {
    let mut result = 0.0;
    for coord in vector {
        result += coord * coord;
    }
    return result.sqrt();
}

// Нормализуйте вектор вычислив его модуль и разделив все его координаты на 
// этот модудль.


fn normalize(vector: &mut [f64]) {
    let vector_module = magnitude(&vector);
    for i in 0..vector.len() {
        vector[i] /= vector_module;
    }
}

// Используйте эту функцию main для проверки своей работы.

fn main() {
    println!("Модуль единичного вектора: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Модуль {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Модуль {v:?} после нормализации: {}", magnitude(&v));
}
