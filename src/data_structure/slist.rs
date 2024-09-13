

pub trait SList {
    type Item;
    fn push_head(&mut self, x: Self::Item);
    fn push_tail(&mut self, x: Self::Item);
    fn pop_head(&mut self) -> Option<Self::Item>;
    fn pop_tail(&mut self) -> Option<Self::Item>;
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
}

type Link<T> = Option<Box<SlistItem<T>>>;
struct SlistItem<T> {
    next : Link<T>,
    val: T,
}

impl<T> SlistItem<T> {
    fn new(val : T, next : Link<T>) -> Self {
        SlistItem{
            next : next,
            val : val,
        }
    }
}
pub struct Slist<T> {
    head : Link<T>,
    size: usize,
}

struct Iter<'a, T> {
    op : Option<&'a Box<SlistItem<T>>>,
}

struct IterMut<'a, T> {
    op : Option<&'a mut Box<SlistItem<T>>>,
}

impl<T> Slist<T> {
    pub fn new() -> Slist<T> {
        Slist {
            head : None,
            size : 0,
        }
    }

    pub fn reset(&mut self) {
        self.head.take();
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &'_ T> {
        Iter{op: self.head.as_ref()}
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut T> {
        IterMut{op : self.head.as_mut()}
    }

    // only for test rust
    pub fn first(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|x| &mut x.val)
    }

    pub fn last(&mut self) -> Option<&mut T> {
        let mut l = &mut self.head;
        while l.is_some() {
            if l.as_ref().unwrap().next.is_some() {
                l = &mut l.as_mut().unwrap().next;
            } else {
                return Some(&mut l.as_mut().unwrap().val);
            }
        }
        None
    }
}

impl<T> SList for Slist<T> {
    type Item = T;

    fn size(&self) -> usize {
        self.size
    }

    fn empty(&self) -> bool {
        assert_eq!(self.size > 0, self.head.is_some());
        self.head.is_some()
    }

    fn push_head(&mut self, val: T) {
        self.head = Some(Box::new(SlistItem::new(val, self.head.take())));
    }

    fn push_tail(&mut self, val: T) {
        let mut op = &mut self.head;

        while let Some(ref mut x) = op {
            op = &mut x.next;
        }
        *op = Some(Box::new(SlistItem::new(val, None)));
    }

    fn pop_head(&mut self) -> Option<T> {
        match self.head.take() {
            Some(x) => {
                self.head = x.next;
                Some(x.val)
            },
            None => None,
        }
    }

    fn pop_tail(&mut self) -> Option<T> {
        let mut op = &mut self.head;
        
        while op.is_some() {
            if op.as_ref().unwrap().next.is_some() {
                op = &mut op.as_deref_mut().unwrap().next;
            } else {
                return Some(op.take().unwrap().val);
            }
        }
        None
    }

}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.op {
            Some(x) => {
                self.op = x.next.as_ref();
                Some(&x.val)
            },
            None => None,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.op.take() {
            Some(x) => {
                self.op = x.next.as_mut();
                Some(&mut x.val)
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;
    use std::ops::AddAssign;

    use super::Slist;
    use super::SList;

#[derive(PartialEq, Debug)]
struct MyInt(i32);
impl Display for MyInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyInt({})", self.0)
    }
}
impl AddAssign for MyInt {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl AddAssign<i32> for MyInt {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs;
    }
}
impl Drop for MyInt {
    fn drop(&mut self) {
        println!("Drop MyInt({})", self.0);
    }
}

fn build_slist(n : usize) -> Slist<MyInt> {
    let mut s = Slist::new();
    for i in 0..n {
        s.push_tail(MyInt(i as i32));
    }
    s
}

#[test]
fn test_slist() {
    let mut slist = Slist::new();
    for i in 4..7 {
        slist.push_head(i);
    }
    

    slist.pop_tail();
    println!("after pop tail");
    for val in slist.iter() {
        println!("val = {val}");
    }

    slist.reset();
    for i in 4..7 {
        slist.push_tail(i);
    }
    {
        for i in slist.iter() {
            println!("iter {i}");
        }
        assert_eq!(slist.pop_head(), Some(4));
        assert_eq!(slist.pop_tail(), Some(6));
        assert_eq!(slist.pop_tail(), Some(5));
        assert_eq!(slist.pop_tail(), None);
        assert_eq!(slist.pop_head(), None);
    }
}

#[test]
fn test_iter() {
    let mut slist = Slist::new();
    for i in 0..10 {
        slist.push_tail(MyInt(i));
    }
    for it in slist.iter() {
        println!("iter = {}", it);
    }

    {
        let mut it = slist.iter();
        for i in 0..10 {
            assert_eq!(it.next(), Some(&MyInt(i)));
        }
        assert_eq!(it.next(), None);
    }
}

#[test]
fn test_iter_mut() {
    let mut slist = Slist::new();
    for i in 0..10 {
        slist.push_tail(MyInt(i));
    }
    for it in slist.iter() {
        println!("iter = {}", it);
    }
    println!("+ 1");
    for it in slist.iter_mut() {
        *it += 1;
    }
    for it in slist.iter() {
        println!("iter = {}", it);
    }
}

#[test]
fn test_first() {
    let mut s = build_slist(3);
    assert_eq!(s.first(), Some(&mut MyInt(0)));
    assert_eq!(s.last(), Some(&mut MyInt(2)));

    *s.first().unwrap() = MyInt(11);
    *s.last().unwrap() = MyInt(17);
    assert_eq!(s.first(), Some(&mut MyInt(11)));
    assert_eq!(s.last(), Some(&mut MyInt(17)));

    {
        let mut it = s.iter();
        assert_eq!(it.next(), Some(&MyInt(11)));
        assert_eq!(it.next(), Some(&MyInt(1)));
        assert_eq!(it.next(), Some(&MyInt(17)));
        assert_eq!(it.next(), None);
    }
}
}