pub fn quick_sort(arr: &mut [u32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let pivot_index = partition(arr);

    let (left, right) = arr.split_at_mut(pivot_index);

    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition(arr: &mut [u32]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;

    arr.swap(pivot_index, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}
