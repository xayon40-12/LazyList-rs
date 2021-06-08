use crate::List::{self,*};
use crate::*;

pub fn once<'a>(d: usize, m: usize) -> List<'a,List<'a,usize>> {
    match d {
        0 => (0..=m).map(|_| list![]).collect::<List<_>>(),
        1 => (0..=m).map(|i| list![i]).collect::<List<_>>(),
        _ => (0..=m).map(|i| list![i]).collect::<List<_>>(), //FIXME do recursive compact fuse
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
        println!{"{:?}", once(3,10).into_iter().map(Vec::from).collect::<Vec<Vec<_>>>()};
    }
}
