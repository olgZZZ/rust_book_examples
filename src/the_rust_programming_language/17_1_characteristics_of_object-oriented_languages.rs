// Characteristics of Object-Oriented Languages
// https://doc.rust-lang.org/stable/book/ch17-01-what-is-oo.html





// Encapsulation that Hides Implementation Details

/*
We discussed how to control encapsulation in Chapter 7: we can use the pub keyword to decide which modules, types, functions, and methods in our code should be public, and by default everything else is private. For example, we can define a struct AveragedCollection that has a field containing a vector of i32 values. The struct can also have a field that contains the average of the values in the vector, meaning the average doesn’t have to be computed on demand whenever anyone needs it. In other words, AveragedCollection will cache the calculated average for us. Listing 17-1 has the definition of the AveragedCollection struct:

Filename: src/lib.rs
*/
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}




/*
The struct is marked pub so that other code can use it, but the fields within the struct remain private. This is important in this case because we want to ensure that whenever a value is added or removed from the list, the average is also updated. We do this by implementing add, remove, and average methods on the struct, as shown in Listing 17-2:

Filename: src/lib.rs
*/
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}







// Inheritance as a Type System and as Code Sharing

/*

    Polymorphism

    To many people, polymorphism is synonymous with inheritance. But it’s actually a more general concept that refers to code that can work with data of multiple types. For inheritance, those types are generally subclasses.

    Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called bounded parametric polymorphism.

*/




