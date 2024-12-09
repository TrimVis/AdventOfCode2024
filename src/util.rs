#[macro_export]
macro_rules! time_function {
    ($func:path) => {{
        let start = std::time::Instant::now();
        let res = $func();
        let duration = start.elapsed();
        println!(
            "Function `{}` executed in {:?}",
            stringify!($func),
            duration
        );
        res
    }};
}
