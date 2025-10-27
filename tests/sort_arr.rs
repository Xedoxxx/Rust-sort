use rsort::sort_array;
use rand::Rng;
use colored::Colorize;

fn action(text: &str, mut action: impl FnMut()) {
    println!("=== {} ===", text.blue());
    action();
    println!("=== {} FINISH ===", text.red());
}

#[test]
fn array_sort() {
    let mut rand = rand::rng();
    let mut array = [0; 10];
    
    for i in 0..10 {
        array[i] = rand.random_range(0..=100);
    }
    let array_ref = &mut array;
    action("ARRAY SORT TEST", || {
        println!("Origin array: {:?}", array_ref);
        sort_array(array_ref, |v1, v2| v1 > v2);
        println!("Sorted array: {:?}", array_ref);
    });

}