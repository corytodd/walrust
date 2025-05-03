pub struct Repository;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository() {
        assert_eq!(std::mem::size_of::<Repository>(), 0);
    }
}
