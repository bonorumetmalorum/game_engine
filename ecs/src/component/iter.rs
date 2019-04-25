use core::slice;
use component::ComponentEntry;
use component::Component;

///Must be implemented for all iterators
pub trait Iter{
    type Item;
    ///retrives the next element
    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)>;
    ///creates a new `ComponentIteratorJoin` zipping the two iterators together
    fn join<H: Iter>(self, other: H) -> ComponentIteratorJoin<Self, H> where Self: Sized{
        ComponentIteratorJoin(self, other)
    }
    ///returns this iterator in a iterator wrapper implementing all other iterator functionalities
    fn into_iterator_wrapper(self) -> IteratorWrapper<Self> where Self: Sized {
        IteratorWrapper(self)
    }
}
///two joing iterators
pub struct ComponentIteratorJoin<H, T>(H, T);

impl<H: Iter, T: Iter> Iter for ComponentIteratorJoin<H, T> {
    type Item = (H::Item, T::Item);
    ///returns the next element that has both components
    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        match (self.0.next_element(until), self.1.next_element(until)){
            (Some((mut i1, mut ind1)), Some((mut i2, mut ind2))) => loop {
                if ind1 < ind2 {
                    if let Some(res) = self.0.next_element(Some(ind2)) {
                        i1 = res.0;
                        ind1 = res.1;
                    }else{
                        return None;
                    }
                } else if ind1 > ind2 {
                    if let Some(res) = self.1.next_element(Some(ind1)){
                        i2 = res.0;
                        ind2 = res.1;
                    }else{
                        return None;
                    }
                }else{
                    return Some(((i1, i2), ind1))
                }
            },
            _ => None
        }
    }
}
///convenience wrapper for iterators
pub struct IteratorWrapper<H>(H);

impl<H> Iterator for IteratorWrapper<H> where H: Iter{
    type Item = H::Item;
    ///returns the next element of the contained iterator
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.0.next_element(None){
            Some(val.0)
        }else{
            None
        }
    }
}