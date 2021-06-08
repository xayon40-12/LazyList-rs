pub mod Once;

use std::iter::FromIterator;
use std::rc::Rc;

#[derive(Clone)]
pub struct Lazy<'a, T: Clone> {
    res: Option<Rc<T>>,
    lazy: Rc<dyn Fn() -> T + 'a>,
}

#[macro_export]
macro_rules! lazy {
    ($s:expr) => {{
        //let tmp = Rc::new($s);
        Lazy::new(move || $s.clone())
    }};
}

impl<'a, T: Clone> Lazy<'a, T> {
    pub fn new<F: Fn() -> T + 'a>(f: F) -> Lazy<'a, T> {
        Lazy {
            res: None,
            lazy: Rc::new(f),
        }
    }
    pub fn val(&mut self) -> T {
        match &self.res {
            None => {
                let t = Rc::new((*self.lazy)());
                self.res = Some(t.clone());
                t.as_ref().clone()
            }
            Some(t) => t.as_ref().clone(),
        }
    }
}

#[derive(Clone)]
pub enum List<'a, T: Clone> {
    Nil,
    Cons(T, Lazy<'a, List<'a, T>>),
}

#[macro_export]
macro_rules! list {
    ($s:expr, $($r:expr),+) => {
        cons($s, list!($($r),+))
    };
    ($s:expr) => {cons($s,Nil)};
    () => {Nil};
}

#[macro_export]
macro_rules! listp {
    ($s:expr, $($r:expr),+) => {
        cons($s, listp!($($r),+))
    };
    ($s:expr) => {$s};
}

#[macro_export]
macro_rules! listl {
    ($s:expr, $s2:expr, $($r:expr),+) => {
        cons($s, listl!($s2, $($r),+))
    };
    ($s:expr, $s2:expr) => {
        Cons($s, $s2)
    };
    ($s:expr) => {$s};
}

pub fn cons<'a, T: 'a + Clone>(t: T, l: List<'a, T>) -> List<'a, T> {
    List::Cons(t, lazy!(l))
}

impl<'a, T:Clone> List<'a,T> {
}

impl<'a, T: Clone> From<&'a [T]> for List<'a, T> {
    fn from(s: &'a [T]) -> Self {
        use crate::List::*;
        match s.len() {
            0 => Nil,
            _ => Cons(s[0].clone(), lazy!(List::from(&s[1..]))),
        }
    }
}

impl<'a, T: Clone> From<List<'a, T>> for Vec<T> {
    fn from(s: List<'a, T>) -> Vec<T> {
        let mut i = s;
        let mut t = true;
        let mut res = vec![];
        while t {
            match i {
                List::Nil => t = false,
                List::Cons(v, r) => {
                    res.push(v.clone());
                    i = r.clone().val();
                }
            }
        }
        res
    }
}

impl<'a,T:Clone> Iterator for List<'a,T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.clone() {
            List::Nil => None,
            List::Cons(a,mut s) => {
                *self = s.val();
                Some(a.clone())
            }
        }
    }
}

impl<'a,T:'a + Clone> List<'a,T> {
    fn from_iter(iter: impl 'a + Iterator<Item=T> + Clone) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            None => List::Nil,
            Some(i) => List::Cons(i,lazy!(Self::from_iter(iter.clone())))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::List::*;
    use crate::*;
    #[test]
    fn it_works() {
        let e = list![4, 5, 6];
        let l = listp![1, 2, 3, e];
        let tot = cons(1, cons(2, cons(3, cons(4, cons(5, cons(6, Nil))))));
        let la = cons(
            1,
            cons(
                2,
                Cons(
                    3,
                    lazy!({
                        let a = list![5, 6];
                        cons(4, a)
                    }),
                ),
            ),
        );
        let la2 = listl![
            1,
            2,
            3,
            lazy!({
                let a = list![5, 6];
                cons(4, a)
            })
        ];
        let v: Vec<i32> = l.into();
        let vtot: Vec<i32> = tot.into();
        let vla: Vec<i32> = la.clone().into();
        let vla2: Vec<i32> = la2.into();
        let vcol: Vec<i32> = List::from_iter(1..=6).into();
        println! {"{:?}", v};
        println! {"{:?}", la.clone().into_iter().take(3).collect::<Vec<_>>()};
        assert_eq!(vec![1,2,3], la.clone().into_iter().take(3).collect::<Vec<_>>());
        assert_eq!(v, vec![1,2,3,4,5,6]);
        assert_eq!(v, vcol);
        assert_eq!(v, vtot);
        assert_eq!(v, vla);
        assert_eq!(v, vla2);
        assert_eq!(2 + 2, 4);
    }
}
