pub fn bubblesort<T: Ord + Copy>(items: &mut Vec<T>) {
    for i in 0..items.len() {
        let mut swapped = false;

        for j in 0..items.len() - i - 1 {
            if items[j + 1] < items[j] {
                items.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubblesort() {
        let mut items = vec![19, 25, 11, 1, 4, 6, 5];
        bubblesort(&mut items);
        assert_eq!(items, vec![1, 4, 5, 6, 11, 19, 25]);
    }
}
