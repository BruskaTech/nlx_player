pub trait AsResult {
    type OkType;
    type ErrorType;

    fn as_result(self) -> Result<Self::OkType, Self::ErrorType>;
}

impl AsResult for () {
    type OkType = ();
    type ErrorType = std::io::Error;

    fn as_result(self) -> Result<Self::OkType, Self::ErrorType> {
        Ok(self)
    }
}

impl<E> AsResult for Result<(), E> {
    type OkType = ();
    type ErrorType = E;

    fn as_result(self) -> Result<Self::OkType, Self::ErrorType> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    #[test]
    fn main() -> Result<(), Box<dyn Error>> {
        let a = ();
        let b: Result<(), String> = Ok(());
        let c: Box<Result<(), String>> = Box::new(Ok(()));
        let d: Result<(), Box<dyn Error>> = Ok(());
        let e: Result<(), Box<dyn Error>> = Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")));

        a.as_result()?;
        b.as_result()?;
        c.as_result()?;
        d.as_result()?;
        assert!(e.as_result().is_err());

        Ok(())
    }
}