pub mod sorting;

#[cfg(test)]
mod tests {
    use crate::sorting::bubblesort::sort;

    #[test]
    fn test_bubblesort() {
        let mut items = vec![19, 25, 11, 1, 4, 6, 5];
        sort(&mut items);
        assert_eq!(items, vec![1, 4, 5, 6, 11, 19, 25]);
    }
}
