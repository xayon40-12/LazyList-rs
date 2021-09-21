use crate::List::{self, *};
use crate::*;
use std::cmp::PartialOrd;
use std::ops::{Add, Mul};

pub fn once<'a>(ds: List<'a, f64>, m: usize) -> List<'a, (List<'a, f64>, f64)> {
    match ds {
        List::Nil => List::from_iter((0..=m).map(|_| (list![], 0.0))),
        List::Cons(d, mut r) => match r.val() {
            Nil => List::from_iter((0..=m).map(move |i| (list![i as f64], d * d * (i * i) as f64))),
            dr => {
                compact(List::from_iter((0..=m).map(move |i| {
                    List::from_iter(once(dr.clone(), i).into_iter().map(move |(lj, j)| {
                        (Cons(i as f64, lazy!(lj)), j + d * d * (i * i) as f64)
                    }))
                })))
            }
        },
    }
}

pub fn ionce<'a>(ds: List<'a, f64>) -> List<'a, (List<'a, f64>, f64)> {
    once(ds, usize::max_value())
}

pub fn compact<'a, N: 'a + PartialOrd + Mul<Output = N> + Add<Output = N> + Copy>(
    aaa: List<'a, List<'a, (List<'a, N>, N)>>,
) -> List<'a, (List<'a, N>, N)> {
    match aaa {
        Nil => Nil,
        Cons(aa, mut aaar) => match aa {
            Nil => compact(aaar.val()),
            Cons(a, aar) => Cons(
                a,
                lazy!(fuse(aar.clone().val(), compact(aaar.clone().val()))),
            ),
        },
    }
}

pub fn fuse<'a, N: 'a + PartialOrd + Mul<Output = N> + Add<Output = N> + Copy>(
    aa: List<'a, (List<'a, N>, N)>,
    bb: List<'a, (List<'a, N>, N)>,
) -> List<'a, (List<'a, N>, N)> {
    match aa {
        Nil => bb,
        Cons((av, a), ar) => match bb {
            Nil => Cons((av, a), ar),
            Cons((bv, b), br) => {
                if a <= b {
                    Cons(
                        (av, a),
                        lazy!(fuse(
                            ar.clone().val(),
                            Cons((bv.clone(), b.clone()), br.clone())
                        )),
                    )
                } else {
                    Cons(
                        (bv, b),
                        lazy!(fuse(
                            Cons((av.clone(), a.clone()), ar.clone()),
                            br.clone().val()
                        )),
                    )
                }
            }
        },
    }
}

pub fn unt<'a, N: 'a + PartialOrd + Mul<Output = N> + Add<Output = N> + Copy>(
    aa: List<'a, (List<'a, N>, N)>,
    n: N,
) -> List<'a, (List<'a, N>, N)> {
    match aa {
        Nil => Nil,
        Cons((al, a), ar) => {
            if a > n {
                Nil
            } else {
                Cons((al, a), lazy!(unt(ar.clone().val(), n)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::once::*;
    #[test]
    fn test_once() {
        let f = |(av, a)| (Vec::from(av), a);
        println! {"{:?}", once(list![],10).into_iter().map(f).collect::<Vec<_>>()};
        println! {"{:?}", once(list![1.0],10).into_iter().map(f).collect::<Vec<_>>()};
        println! {"{:?}", once(list![1.0,2.0],10).into_iter().map(f).collect::<Vec<_>>()};
        println! {"{:?}", ionce(list![1.0,2.0,3.0]).into_iter().take(20).map(f).collect::<Vec<_>>()};
        //println! {"{:?}", unt(ionce(list![1.0,2.0,3.0]), 300.0).map(f).collect::<Vec<_>>()};
    }
}
