pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|v| *v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().copied().filter(|v| *v != 0).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let positives = values
        .iter()
        .copied()
        .filter(|v| *v >= 0)
        .collect::<Vec<i64>>();
    let sum: i64 = positives.iter().sum();

    if positives.is_empty() {
        return 0.0;
    }
    sum as f64 / positives.len() as f64
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub unsafe fn use_after_free() -> i32 {
    unsafe {
        let b = Box::new(42_i32);
        let raw = Box::into_raw(b);
        let val = *raw;
        drop(Box::from_raw(raw));
        val + val
    }
}
