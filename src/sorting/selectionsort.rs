pub fn selectionsort<T: Ord + Copy>(items: &mut Vec<T>) {
    let mut position = 0;

    while position < items.len() - 1 {
        let mut start = position;

        for i in start..items.len() {
            if items[start] > items[i] {
                start = i;
            }
        }

        items.swap(start, position);
        position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selectionsort() {
        let mut items = vec![19, 25, 11, 1, 4, 6, 5];
        selectionsort(&mut items);
        assert_eq!(items, vec![1, 4, 5, 6, 11, 19, 25]);
    }
}
