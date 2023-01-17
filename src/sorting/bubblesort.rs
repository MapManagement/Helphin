pub fn sort<T: Ord + Copy>(items: &mut Vec<T>) {
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
