#[macro_export]
/// Macro for logging the result
/// and error of a future.
macro_rules! log_all {
    ($future_obj:expr) => {
        $future_obj.map(|data| {
            println!("{:?}", data);

            ()
        })
        .map_err(|err| {
            println!("{:?}", err);
            
            ()
        })
    };
}

#[macro_export]
/// Macro for logging the error of a future
/// and ignoring the result.
macro_rules! log_error {
    ($future_obj:expr) => {
        $future_obj.map_err(|err| {
            println!("{:?}", err);
            
            ()
        })
    };
}

#[macro_export]
/// Macro for logging the result of a future
/// and ignoring the error.
macro_rules! log_data {
    ($future_obj:expr) => {
        $future_obj.map(|data| {
            println!("{:?}", data);
            
            ()
        })
    };
}