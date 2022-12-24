pub mod common {
    pub fn get_third() {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
    }
    pub fn hash() {
        use std::collections::HashMap;

        let text = "hello world wonderful world";

        let mut map = HashMap::new();                    // empty hashmap
    
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0); // count is a mutable reference of <value> or 0
            *count += 1;                                                     // to mutate twice dereference "*" count and add 1 
        }
    
        println!("{:?}", map);
    }
}