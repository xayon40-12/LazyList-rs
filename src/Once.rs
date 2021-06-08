use crate::List::{self,*};
use crate::*;

pub fn once<'a>(d: usize, m: usize) -> List<'a,List<'a,usize>> {
    match d {
        0 => List::from_iter((0..=m).map(|_| list![]) ),
        1 => List::from_iter((0..=m).map(|i| list![i])),
        _ => compact(0,List::from_iter((0..=m).map(move |i|
              List::from_iter(once(d-1,i).into_iter().map(move |j| Cons(i,lazy!(j)))))
            )),
    }
}

use std::ops::{Mul,Add};
pub fn sqr<'a, N: Mul<Output=N> + Add<Output=N> + Copy>(s: N, l: List<'a,N>) -> N {
    l.into_iter().fold(s, |a,i| a + i*i)
}

pub fn compact<'a, N: 'a + Ord + Mul<Output=N> + Add<Output=N> + Copy>(s: N, aaa: List<'a,List<'a,List<'a,N>>>) -> List<'a,List<'a,N>> {
    match aaa {
        Nil => Nil,
        Cons(aa,mut aaar) => match aa {
            Nil => compact(s, aaar.val()),
            Cons(a,aar) => Cons(a,lazy!(fuse(s.clone(), aar.clone().val(),compact(s.clone(), aaar.clone().val())))),
        }
    }
}

use std::cmp::Ord;
pub fn fuse<'a, N: 'a + Ord + Mul<Output=N> + Add<Output=N> + Copy>(s: N, aa: List<'a,List<'a,N>>,bb: List<'a,List<'a,N>>) -> List<'a,List<'a,N>> {
    match aa {
        Nil => bb,
        Cons(a,ar) => match bb {
            Nil => Cons(a,ar),
            Cons(b,br) => if sqr(s,a.clone())<=sqr(s,b.clone()) {
                Cons(a,lazy!(fuse(s,ar.clone().val(),Cons(b.clone(),br.clone()))))
            } else {
                Cons(b,lazy!(fuse(s,Cons(a.clone(),ar.clone()),br.clone().val())))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::Once::*;
    #[test]
    fn test_once() {
        println!{"{:?}", once(0,10).into_iter().map(Vec::from).collect::<Vec<Vec<_>>>()};
        println!{"{:?}", once(1,10).into_iter().map(Vec::from).collect::<Vec<Vec<_>>>()};
        println!{"{:?}", once(2,10).into_iter().map(Vec::from).collect::<Vec<Vec<_>>>()};
        println!{"{:?}", once(3,10000).into_iter().take(20).map(Vec::from).collect::<Vec<Vec<_>>>()};
        println!("{}", sqr(0, list![1,2,3]));
        let l1 = List::from_iter((0..10).map(|i| list![i]));
        let l2 = List::from_iter((0..10).map(|i| list![i*2]));
        println!("{:?}", fuse(0,l1,l2).map(Vec::from).into_iter().collect::<Vec<_>>());
    }
}
