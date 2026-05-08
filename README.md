# optimizations

## Изменения 

### lib.rs
были выполнены оптимизации
sum_even - убрал unsafe блок, упростил выражение
leak_buffer - убрал unsafe блок, упростил выражение
normalize - учел пограничные кейсы(перенос строки)
average_positive - поправил логику, выборка и расчеты только для положительных
use_after_free - обернул в unsafe, убрал использование после drop

### algo.rs
улучшил оба алгоритма, см отчеты

### concurrency.rs
Изменил тип COUNTER на AtomicU64
Избавился от unsafe в race_increment

### tests
добавил интеграционный тест на normalize, тк предыдущий не покрывал пограничные кейсы

## Артефакты

### ASAN(asan-report_after и asan-report_before)
В оригинальной версии получали
thread 'sums_even_numbers' (22957932) panicked at src/lib.rs:11:29:
unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice

После того как переписали функцию(убрали get_unchecked) ошибка исчезла

### BENCH(bench_after и bench_before)
В оригинальной версии sum_even вис на измерении, после оптимизации замеры начали ваполняться, но измерить нельзя

по slow_fib_broken - улучшение перформанса на 100%
по slow_dedup_broken - улучшение перформанса на 99%

### CHECK(check_after и check_before)
В оригинальной версии получали 4 ошибки/предупреждения в lib.rs
61 |     let val = *raw;
   |               ^^^^ dereference of raw pointer

58 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

62 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function

63 |     val + *raw
   |           ^^^^ dereference of raw pointer

После исправления ошибки ушли

###  FLAMEGRAPH(flamegraph_after и flamegraph_before)
До оптимизации видно что slow_fib занимает все пространство, после оптимизации функции даже не успевают попасть в замеры

###  MIRI(miri_after и miri_before)
Получали набор ошибок/предупреждений из lib.rs
После проведенных оптимизаций и исправлений ошибки ушли

###  TEST(test_after и test_before)
ПОлучали сломанный тест, после иправлений функции sum_even все заработало + добавили еще один тест для normalize
