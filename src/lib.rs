use std::rc::Rc;

#[derive(Clone)]
pub struct Lazy<'a, T: Clone> {
    res: Option<Rc<T>>,
    lazy: Rc<dyn Fn() -> Rc<T> + 'a>,
}

macro_rules! lazy {
    ($s:expr) => {{
        let tmp = Rc::new($s);
        Lazy::new(move || tmp.clone())
    }};
}

impl<'a, T: Clone> Lazy<'a, T> {
    pub fn new<F: Fn() -> Rc<T> + 'a>(f: F) -> Lazy<'a, T> {
        Lazy {
            res: None,
            lazy: Rc::new(f),
        }
    }
    pub fn val(&mut self) -> Rc<T> {
        match &self.res {
            None => {
                let t = (*self.lazy)();
                self.res = Some(t.clone());
                t
            }
            Some(t) => t.clone(),
        }
    }
}

#[derive(Clone)]
pub enum List<'a, T: Clone> {
    Nil,
    Cons(T, Lazy<'a, List<'a, T>>),
}

macro_rules! list {
    ($s:expr, $($r:expr),+) => {
        cons($s, list!($($r),+))
    };
    ($s:expr) => {$s};
}

macro_rules! llist {
    ($s:expr, $s2:expr, $($r:expr),+) => {
        cons($s, llist!($s2, $($r),+))
    };
    ($s:expr, $s2:expr) => {
        Cons($s, $s2)
    };
    ($s:expr) => {$s};
}

pub fn cons<'a, T: 'a + Clone>(t: T, l: List<'a, T>) -> List<'a, T> {
    List::Cons(t, lazy!(l))
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
        let mut i = Rc::new(s);
        let mut t = true;
        let mut res = vec![];
        while t {
            match i.as_ref() {
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

#[cfg(test)]
mod tests {
    use crate::List::*;
    use crate::*;
    #[test]
    fn it_works() {
        let e = list![4, 5, 6, Nil];
        let l = list![1, 2, 3, e];
        let tot = cons(1, cons(2, cons(3, cons(4, cons(5, cons(6, Nil))))));
        let la = cons(
            1,
            cons(
                2,
                Cons(
                    3,
                    lazy!({
                        let a = list![5, 6, Nil];
                        cons(4, a)
                    }),
                ),
            ),
        );
        let la2 = llist![
            1,
            2,
            3,
            lazy!({
                let a = list![5, 6, Nil];
                cons(4, a)
            })
        ];
        let v: Vec<i32> = l.into();
        let vtot: Vec<i32> = tot.into();
        let vla: Vec<i32> = la.into();
        let vla2: Vec<i32> = la2.into();
        println! {"{:?}", v};
        assert_eq!(v, vtot);
        assert_eq!(v, vla);
        assert_eq!(v, vla2);
        assert_eq!(2 + 2, 4);
    }
}
