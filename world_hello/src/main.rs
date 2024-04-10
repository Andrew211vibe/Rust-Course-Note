pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 1 {
        quick_sort_range(arr, 0, arr.len() - 1);
    }
}

fn quick_sort_range<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) {
    // 只有当元素个数大于一时才进行排序
    if lo <= hi {
        let pos = partition(arr, lo, hi);
        if pos != 0 {
            quick_sort_range(arr, lo, pos - 1);
        }
        quick_sort_range(arr, pos + 1, hi);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = lo;
    let (mut l, mut r) = (lo, hi);
    while l < r {
        while l < r && arr[r] >= arr[pivot] { r -= 1; }
        while l < r && arr[l] <= arr[pivot] { l += 1; }
        if l != r {
            arr.swap(l, r);
        }
    }
    arr.swap(pivot, l);
    l
}

fn main() {
    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
    quick_sort(&mut vec);
    assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    println!("{:?}", vec);
}