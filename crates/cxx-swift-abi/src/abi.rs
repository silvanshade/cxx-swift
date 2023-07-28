type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

pub fn process_artifacts() -> BoxResult<()> {
    Ok(())
}
