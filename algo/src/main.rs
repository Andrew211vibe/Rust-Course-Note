fn bubble_sort<T: Ord>(nums: &mut [T]) {
    let n = nums.len();
    for i in 0..n {
        for j in 1..(n - i) {
            if nums[j - 1] > nums[j] {
                nums.swap(j, j - 1);
            }
        }
    }
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    let (mut ia, mut ib) = (0, 0);
    while ia < a.len() || ib < b.len() {
        if ia < a.len() && ib < b.len() && a[ia] < b[ib] || ib == b.len() {
            res.push(a[ia]);
            ia += 1;
        } else {
            res.push(b[ib]);
            ib += 1;
        }
    }
    res
}

fn merge_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }
    let mid = nums.len() / 2;

    // merge_sort(&mut nums[..mid]);
    // merge_sort(&mut nums[mid..]);
    // let merged = merge(&nums[..mid], &nums[mid..]);
    // for i in 0..nums.len() {
    //     nums[i] = merged[i];
    // }

    let (p1, p2) = nums.split_at_mut(mid);
    merge_sort(p1);
    merge_sort(p2);
    let (p1, p2) = nums.split_at(mid);
    nums.copy_from_slice(&merge(p1, p2));
}

fn quick_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    // let (mut l, mut r) = (0, nums.len() - 1);
    // while l < r {
    //     while l < r && nums[r] >= nums[(l + r) >> 1] {
    //         r -= 1;
    //     }
    //     while l < r && nums[l] <= nums[(l + r) >> 1] {
    //         l += 1;
    //     }
    //     nums.swap(l, r);
    // }
    // nums.swap(l, (l + r) >> 1);

    let left: Vec<i32> = nums[1..].iter().filter(|x| x < &&nums[0]).map(|x| *x).collect();
    let right: Vec<i32> = nums[1..].iter().filter(|x| x >= &&nums[0]).map(|x| *x).collect();
    let l = left.len();
    nums[l] = nums[0];
    nums[..l].copy_from_slice(&left);
    nums[l + 1..].copy_from_slice(&right);

    quick_sort(&mut nums[..l]);
    quick_sort(&mut nums[l + 1..]);
}

fn main() {
    let mut nums: Vec<i32> = vec![8, 2, 1, 0, 9, 3, 8, 0, 1, 2, 8, 3];
    bubble_sort(&mut nums);
    println!("{:?}", nums);
    let mut nums = [8, 2, 1, 0, 9, 3, 8, 0, 1, 2, 8, 3];
    bubble_sort(&mut nums);
    println!("{:?}", nums);

    let mut nums = [2, 8, 1, 0, 5, 3];
    merge_sort(&mut nums);
    println!("{:?}", nums);

    let mut nums = [0, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    quick_sort(&mut nums);
    println!("{:?}", nums);

    let x = ListNode { val: 3, next: None };
    let x = ListNode { val: 2, next: Some(Rc::new(RefCell::new(x))) };
    println!("{:?}", x);
    (*x.next.clone().unwrap()).borrow_mut().val = 10;
    println!("{:?}", x);
}

use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
struct ListNode<T> {
    val: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

pub fn insert<T>(cur: &Rc<RefCell<ListNode<T>>>, i: T) {
    let next = cur.borrow_mut().next().take();
    cur.borrow_mut().next = Some(Rc::new(RefCell::new(ListNode { val: i, next })));
}

pub fn remove<T>(cur: &Rc<RefCell<ListNode<T>>>) {
    if let Some(node) = cur.borrow_mut().next.take {
        cur.borrow_mut().next = node.borrow_mut().next.take()
    }
}