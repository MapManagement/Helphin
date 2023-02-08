pub fn binary_it<T: Ord + Copy>(items: Vec<T>, item: T) -> isize {
    let mut start_idx: usize = 0;
    let mut end_idx: usize = items.len() - 1;

    while start_idx <= end_idx {
        let mid_idx: usize = (start_idx + end_idx) / 2;
        println!("{}", mid_idx);
        let mid_item = items[mid_idx];

        if mid_item == item {
            return mid_idx as isize;
        } else if mid_item < item {
            start_idx = mid_idx + 1;
        } else {
            end_idx = mid_idx - 1;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_it_even() {
        let items_even = vec![3, 7, 11, 20, 25, 27];
        let item = 25;

        let item_idx = binary_it(items_even, item);
        assert_eq!(item_idx, 4);
    }

    #[test]
    fn test_binary_it_odd() {
        let items_odd = vec![3, 7, 11, 20, 25, 27, 41];
        let item = 3;

        let item_idx = binary_it(items_odd, item);
        assert_eq!(item_idx, 0);
    }

    #[test]
    fn test_binary_it_not_found() {
        let items = vec![3, 7, 11, 20, 25, 27, 41];
        let item = 8;

        let item_idx = binary_it(items, item);
        assert_eq!(item_idx, -1);
    }
}
