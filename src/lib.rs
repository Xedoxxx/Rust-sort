pub fn sort_array<T: Clone + Copy>(array: &mut [T], compare: fn(item1: T, item2: T) -> bool) {
    for _ in 0..array.len() {
        for i in 0..array.len() {
            if i+1 >= array.len() { continue; }
            let value = array[i].clone();
            let next = array[i+1].clone();
            
            if compare(value.clone(), next.clone()) {
                array[i] = next;
                array[i+1] = value;
            }
        }
    }
}