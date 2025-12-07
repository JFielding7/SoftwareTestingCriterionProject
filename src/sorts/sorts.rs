fn quicksort_rec(vec: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return;
    }

    let pivot = vec[end];
    let mut i = start;
    let mut j = start;

    while i < end {
        if vec[i] < pivot {
            vec.swap(i, j);
            j += 1;
        }

        i += 1;
    }

    vec.swap(j, end);

    if start + 1 < j {
        quicksort_rec(vec, start, j - 1);
    }
    quicksort_rec(vec, j + 1, end);
}

pub fn quicksort(vec: &mut Vec<i32>) {
    let slice = vec.as_mut_slice();
    quicksort_rec(slice, 0, slice.len() - 1)
}

fn swap_down(vec: &mut Vec<i32>, mut i: usize, len: usize) {
    if len < 2 {
        return
    }

    let min_lt_two_children = len - 1 >> 1;

    while i < min_lt_two_children {
        let mut child = (i << 1) + 1;

        if vec[child] < vec[child + 1] {
            child += 1;
        }

        if vec[i] < vec[child] {
            vec.swap(i, child);
            i = child;
        } else {
            break;
        }
    }

    if (len & 1) == 0 && i == min_lt_two_children && vec[i] < vec[(i << 1) + 1] {
        vec.swap(i, (i << 1) + 1);
    }
}

fn heapify(vec: &mut Vec<i32>) {
    let len = vec.len();

    for i in (0..len >> 1).rev() {
        swap_down(vec, i, len);
    }
}

pub fn heapsort(vec: &mut Vec<i32>) {
    heapify(vec);
    let mut len = vec.len();

    for _ in 0..len {
        len -= 1;
        vec.swap(0, len);
        swap_down(vec, 0, len);
    }
}

fn merge(vec: &mut Vec<i32>, mut start: usize, end: usize) {
    let copy = vec[start..=end].to_vec();
    let len = copy.len();
    let mut left = 0;
    let mid = len + 1 >> 1;
    let mut right = mid;

    while left < mid && right < len {
        if copy[left] < copy[right] {
            vec[start] = copy[left];
            left += 1;
        } else {
            vec[start] = copy[right];
            right += 1;
        }

        start += 1;
    }

    while left < mid {
        vec[start] = copy[left];
        left += 1;
        start += 1;
    }

    while right < len {
        vec[start] = copy[right];
        right += 1;
        start += 1;
    }
}

fn merge_sort_rec(vec: &mut Vec<i32>, start: usize, end: usize) {

    if start == end {
        return;
    }

    let mid = (start + end) >> 1;

    merge_sort_rec(vec, start, mid);
    merge_sort_rec(vec, mid + 1, end);

    merge(vec, start, end);
}

pub fn mergesort(vec: &mut Vec<i32>) {
    merge_sort_rec(vec, 0, vec.len() - 1);
}

fn insertion_sort(vec: &mut [i32]) {

    for i in 1..vec.len() {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j -= 1;
        }
    }
}

const THRESHOLD: usize = 64;

fn timsort_rec(vec: &mut Vec<i32>, start: usize, end: usize) {

    if end - start < THRESHOLD {
        insertion_sort(&mut vec[start..=end]);
        return;
    }

    let mid = (start + end) >> 1;

    timsort_rec(vec, start, mid);
    timsort_rec(vec, mid + 1, end);

    merge(vec, start, end);
}

pub fn timsort(vec: &mut Vec<i32>) {
    timsort_rec(vec, 0, vec.len() - 1)
}
