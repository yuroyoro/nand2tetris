#[macro_export]
macro_rules! debug {
    ($format: literal, $( $args:expr ), * ) => {
        if crate::CONFIG.debug {
            println!($format, $( $args ), *);
        }
    }
}
