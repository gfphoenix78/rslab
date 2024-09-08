use std::cmp::PartialOrd;

pub struct Heap<T> {
  arr : Vec<T>,
}
 #[inline]
fn heap_left(parent: usize) -> usize {
    parent * 2 + 1
}

#[inline]
fn heap_parent(child: usize) -> usize {
    assert!(child > 0);
    (child - 1) / 2
}

fn down<T: PartialOrd>(arr: &mut [T], index : usize) {
    let n = arr.len();

    assert!(index < n);

    let mut idx = index;
    let mut c = heap_left(index);
    while c < n {
        let r = c + 1;
        if r < n && arr[r] < arr[c] {
            c = r;
        }
        if arr[c] < arr[idx] {
            arr.swap(c, idx);
            idx = c;
            c = heap_left(idx);
        } else {
            break;
        }
    }
}

fn up<T: PartialOrd>(arr : &mut[T], index : usize) {
    let mut index = index;
    while index > 0 {
        let parent = heap_parent(index);
        if arr[index] < arr[parent] {
            arr.swap(index, parent);
            index = parent;
        } else {
            break;
        }
    }
}

pub fn heap_build<T: PartialOrd>(arr: &mut [T]) {
    for index in (0..arr.len()/2).rev() {
        down(arr, index);
    }
}

impl<T: PartialOrd> Heap<T> {
    pub const fn new() -> Self {
        Heap {
            arr : Vec::new(),
        }
    }

    pub fn from_vec(mut vec : Vec<T>) -> Self {
        heap_build(vec.as_mut_slice());
        Heap {
            arr : vec,
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        self.arr
    }

    pub fn push(&mut self, val : T) {
        self.arr.push(val);
        let n = self.arr.len();
        up(self.arr.as_mut_slice(), n - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        let n = self.arr.len();
        match n {
            0 => None,
            1 => {
                self.arr.pop()
            },
            _ => {
                self.arr.swap(0, n-1);
                let x = self.arr.pop();
                down(&mut self.arr, 0);
                x
            },
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.arr.first()
    }

}

pub fn heap_sort<T: PartialOrd>(arr : &mut[T]) {
    // build heap
    heap_build(arr);
    let mut n = arr.len();

    for i in (1..n).rev() {
        arr.swap(0, i);
        n -= 1;
        
        let arr = &mut arr[0..n];
        down(arr, 0);
    }
    arr.reverse();
}

#[cfg(test)]
mod tests {
extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::data_structure::heap::heap_sort;
use crate::data_structure::heap::Heap;

#[test]
fn test_heap_sort() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let origin = vec.clone();

    // 获取一个随机数生成器
    let mut rng = thread_rng();

    // 使用 shuffle 方法对 vec 进行洗牌
    vec.shuffle(&mut rng);

    // 输出洗牌后的 Vec
    println!("{:?}", vec);

    heap_sort(vec.as_mut_slice());

    println!("{:?}", vec);
    // test
    assert_eq!(vec, origin);
}

#[test]
fn test_heap() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let origin = vec.clone();

    // 获取一个随机数生成器
    let mut rng = thread_rng();

    // 使用 shuffle 方法对 vec 进行洗牌
    vec.shuffle(&mut rng);

    // 输出洗牌后的 Vec
    println!("{:?}", vec);

    let mut heap = Heap::from_vec(vec);
    for index in 0..origin.len() {
        if let Some(v) = heap.pop() {
            assert_eq!(v, origin[index]);
        } else {
            panic!("heap ");
        }
    }
    assert!(heap.pop().is_none());
}
}
