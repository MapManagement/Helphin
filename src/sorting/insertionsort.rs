pub fn insertionsort<T: Ord + Copy>(items: &mut Vec<T>) {
    for i in 0..items.len() {
        let mut position = i;

        while position > 0 && items[position - 1] > items[position] {
            items.swap(position, position - 1);
            position -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertionsort() {
        let mut items = vec![19, 25, 11, 1, 4, 6, 5];
        insertionsort(&mut items);
        assert_eq!(items, vec![1, 4, 5, 6, 11, 19, 25]);
    }
}
