/// `Result` with default types.
pub type AnyRes<T = (), E = anyhow::Error> = Result<T, E>;

/// Extension trait for [`Result`] types.
pub trait ResultExt<T, E> {
    /// Convert the error to a string and ignore the specific error type.
    /// 
    /// This is useful when you want to log an error but don't need to handle
    /// the specific error type.
    fn ignore_err(self) -> Result<T, ()>;

    /// Convert both success and error values to the same type using provided closures.
    ///
    /// This is a more ergonomic version of `map().map_err()` when you want to
    /// convert both sides to the same type.
    fn map_both<U, F, G>(self, ok_fn: F, err_fn: G) -> U
    where
        F: FnOnce(T) -> U,
        G: FnOnce(E) -> U;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn ignore_err(self) -> Result<T, ()> {
        self.map_err(|_| ())
    }

    fn map_both<U, F, G>(self, ok_fn: F, err_fn: G) -> U
    where
        F: FnOnce(T) -> U,
        G: FnOnce(E) -> U,
    {
        match self {
            Ok(t) => ok_fn(t),
            Err(e) => err_fn(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore_err() {
        let ok_result: Result<i32, String> = Ok(42);
        let err_result: Result<i32, String> = Err("error".to_string());

        assert_eq!(ok_result.ignore_err(), Ok(42));
        assert_eq!(err_result.ignore_err(), Err(()));
    }

    #[test]
    fn map_both() {
        let ok_result: Result<i32, String> = Ok(42);
        let err_result: Result<i32, String> = Err("error".to_string());

        let ok_mapped = ok_result.map_both(|x| x * 2, |_| 0);
        let err_mapped = err_result.map_both(|x| x * 2, |_| 0);

        assert_eq!(ok_mapped, 84);
        assert_eq!(err_mapped, 0);
    }
}
