trait Cache {
    fn get(key: &str) -> Result<>,
    fn set(key: &str) -> Result<>,
    fn clear() -> Result<>
}