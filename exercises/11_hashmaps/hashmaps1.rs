// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.

use std::collections::HashMap;
use std::string::String as s;

fn fruit_basket() -> HashMap<s, u32> {
    // TODO: Declare the hash map.
    // let mut basket =

    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(s::from("banana"), 2);

    // TODO: Put more fruits in your basket.

    basket.insert(s::from("lychee"), 6);
    basket.insert(s::from("prickly pear"), 3);
    basket.insert(s::from("passion fruit"), 1);
    basket.insert(s::from("guava"), 4);

    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
