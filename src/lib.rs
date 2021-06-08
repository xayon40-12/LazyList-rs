use std::rc::Rc;

#[derive(Clone)]
pub struct Lazy<'a, T: Clone> {
    res: Option<T>,
    lazy: Rc<dyn Fn() -> T + 'a>,
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
    Cons(T, Lazy<'a, Box<List<'a, T>>>),
}

macro_rules! list {
    ($s:expr) => {
        Cons($s, Lazy::new(move || Box::new(Nil)))
    };
    ($s:expr, $($r:expr),*) => {
        Cons($s, Lazy::new(move || Box::new(list!($($r),*))))
    };
}

impl<'a, T: Clone> From<&'a [T]> for List<'a, T> {
    fn from(s: &'a [T]) -> Self {
        use crate::List::*;
        match s.len() {
            0 => Nil,
            _ => Cons(
                s[0].clone(),
                Lazy::new(move || Box::new(List::from(&s[1..]))),
            ),
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
                List::Cons(v, mut r) => {
                    res.push(v);
                    i = *r.val();
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
        let l = list![1, 2, 3];
        let v: Vec<i32> = l.into();
        println! {"{:?}", v};
        assert_eq!(2 + 2, 4);
    }
}
