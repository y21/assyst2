#[macro_export]
macro_rules! ok_or_break {
    ($expression:expr) => {
        match $expression {
            Ok(v) => v,
            Err(_) => break,
        }
    };
}

#[macro_export]
macro_rules! ok_or_continue {
    ($expression:expr) => {
        match $expression {
            Ok(v) => v,
            Err(_) => continue,
        }
    };
}
