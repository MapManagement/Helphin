pub fn mergesort<T: Ord + Copy>(items: &mut Vec<T>) {
    divide(items);
}

fn merge<T: Ord + Copy>(left: &Vec<T>, right: &Vec<T>, items: &mut Vec<T>) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut items_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            items[items_index] = left[left_index];
            left_index += 1;
        } else {
            items[items_index] = right[right_index];
            right_index += 1;
        }

        items_index += 1;
    }
    
    while left_index < left.len() {
        items[items_index] = left[left_index];
        left_index += 1;
        items_index += 1;
    }

    while right_index < right.len() {
        items[items_index] = right[right_index];
        right_index += 1;
        items_index += 1;
    }
}

fn divide<T: Ord + Copy>(items: &mut Vec<T>) {
    if items.len() <= 1 {
        return;
    }

    let mid: usize = items.len() / 2;
    divide(&mut items[0..mid].to_vec());
    divide(&mut items[mid..].to_vec());

    merge(&items[0..mid].to_vec(), &items[mid..].to_vec(), items);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mergesort() {
        let mut items = vec![19, 25, 11, 1, 4, 6, 5];
        mergesort(&mut items);
        assert_eq!(items, vec![1, 4, 5, 6, 11, 19, 25]);
    }
}

