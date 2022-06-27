pub(crate) trait ResultExtension {
    fn into_ok<E>(self) -> Result<Self, E>
    where
        Self: Sized,
    {
        Ok(self)
    }
}

impl<T> ResultExtension for T {}
