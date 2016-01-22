
pub struct BinaryHeap<T>{
    array: Vec<T>
}

impl<'a, T: Ord + PartialEq> BinaryHeap<T>{
    pub fn new() -> Self{
        BinaryHeap::<T>{ array:Vec::new() }
    }

    pub fn push(&mut self, newV: T){
        let mut idx = self.array.len();
        self.array.push(newV);
        while idx > 0{
            let idx2 = (idx-1)/2;
            if self.array[idx2]>self.array[idx]{
                self.array.swap(idx2, idx);
                idx = idx2;
            }
            else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T>{
        if self.array.len() == 0{
            return None;
        }
        
        let al = self.array.len()-1;
        self.array.swap(0, al);
        let mut idx = 0;
        
        while idx < self.array.len()-1{
            let mut idx2 = idx * 2 + 1;
            if idx2 >= self.array.len()-1{
                break;
            }
            
            if idx2 + 1 < self.array.len()-1 && self.array[idx2+1] < self.array[idx2]{
                idx2 = idx2 + 1;
            }

            if self.array[idx] > self.array[idx2] {
                self.array.swap(idx2, idx);
            }
            else{
                break;
            }
            
            idx = idx2;
        }
        self.array.pop()
    }

    pub fn peek(&'a self) -> Option<&'a T>{
        self.array.get(0)
    }

    pub fn len(&self) -> usize{
        self.array.len()
    }
}

#[cfg(test)]
mod test{
    use super::BinaryHeap;
    #[test]
    fn push_elemnt(){
        let mut heap = BinaryHeap::<u32>::new();
        heap.push(3);
        heap.push(1);
        heap.push(5);
        heap.push(0);
        let ff = heap.peek().unwrap();
        assert_eq!(ff, &0);
    }

    
    fn is_heap<T:Ord + PartialEq>(heap: &BinaryHeap<T>) -> bool{
        for i in 0..heap.array.len(){
            if i*2+1 >= heap.array.len(){
                break;
            }
            if heap.array[i] > heap.array[i*2+1]{
                return false;
            }
            if i*2 + 2 < heap.array.len() && heap.array[i] > heap.array[i*2+2]{
                return false;
            }
        }
        
        true
    }

    #[test]
    fn push_pop(){
        let mut heap = BinaryHeap::<u32>::new();
        heap.push(3);
        heap.push(1);
        assert_eq!(heap.pop().unwrap(), 1);
        {
            assert!(is_heap(&heap));
        }
        heap.push(2);
        assert_eq!(heap.pop().unwrap(), 2);
        assert_eq!(heap.pop().unwrap(), 3);

        let a:u64 = 214013;
        let c:u64 = 2531011;
        let mut ra: u32 = 2983798125;
        for i in 0..10000{
            ra = (a * u64::from(ra) + c) as u32;
            if (ra % 7) != 0{
                heap.push(ra);
            }
            else{
                heap.pop();
            }
            
            {
                assert!(is_heap(&heap));
            }
        }
    }
}
