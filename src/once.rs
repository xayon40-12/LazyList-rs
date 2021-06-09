use crate::List::{self,*};
use crate::*;
use std::ops::{Mul,Add};
use std::cmp::Ord;

pub fn once<'a>(d: usize, m: usize) -> List<'a,(List<'a,usize>,usize)> {
    match d {
        0 => List::from_iter((0..=m).map(|_| (list![],0)) ),
        1 => List::from_iter((0..=m).map(|i| (list![i],i*i))),
        _ => compact(List::from_iter((0..=m).map(move |i|
              List::from_iter(once(d-1,i).into_iter().map(move |(lj,j)| (Cons(i,lazy!(lj)),j+i*i) )))
            )),
    }
}

pub fn ionce<'a>(d: usize) -> List<'a,(List<'a,usize>,usize)> {
    once(d, usize::max_value())
}

pub fn compact<'a, N: 'a + Ord + Mul<Output=N> + Add<Output=N> + Copy>(aaa: List<'a,List<'a,(List<'a,N>,N)>>) -> List<'a,(List<'a,N>,N)> {
    match aaa {
        Nil => Nil,
        Cons(aa,mut aaar) => match aa {
            Nil => compact(aaar.val()),
            Cons(a,aar) => Cons(a,lazy!(fuse(aar.clone().val(),compact(aaar.clone().val())))),
        }
    }
}

pub fn fuse<'a, N: 'a + Ord + Mul<Output=N> + Add<Output=N> + Copy>(aa: List<'a,(List<'a,N>,N)>,bb: List<'a,(List<'a,N>,N)>) -> List<'a,(List<'a,N>,N)> {
    match aa {
        Nil => bb,
        Cons((av,a),ar) => match bb {
            Nil => Cons((av,a),ar),
            Cons((bv,b),br) => if a<=b {
                Cons((av,a),lazy!(fuse(ar.clone().val(),Cons((bv.clone(),b.clone()),br.clone()))))
            } else {
                Cons((bv,b),lazy!(fuse(Cons((av.clone(),a.clone()),ar.clone()),br.clone().val())))
            }
        }
    }
}

pub fn unt<'a, N: 'a + Ord + Mul<Output=N> + Add<Output=N> + Copy>(aa: List<'a,(List<'a,N>,N)>, n: N) -> List<'a,(List<'a,N>,N)> {
    match aa {
        Nil => Nil,
        Cons((al,a),ar) => {
            if a > n {
                Nil
            } else {
                Cons((al,a),lazy!(unt(ar.clone().val(),n)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::once::*;
    #[test]
    fn test_once() {
        let f = |(av,a)| (Vec::from(av),a);
        println!{"{:?}", once(0,10).into_iter().map(f).collect::<Vec<_>>()};
        println!{"{:?}", once(1,10).into_iter().map(f).collect::<Vec<_>>()};
        println!{"{:?}", once(2,10).into_iter().map(f).collect::<Vec<_>>()};
        println!{"{:?}", ionce(3).into_iter().take(20).map(f).collect::<Vec<_>>()};
        println!{"{:?}", unt(ionce(3), 300).map(f).collect::<Vec<_>>()};
    }
}
