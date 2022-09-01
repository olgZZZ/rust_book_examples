// Reference Cycles Can Leak Memory
// https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html




// Creating a Reference Cycle

/*
Let’s look at how a reference cycle might happen and how to prevent it, starting with the definition of the List enum and a tail method in Listing 15-25:

Filename: src/main.rs
*/
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {}






/*
In Listing 15-26, we’re adding a main function that uses the definitions in Listing 15-25. This code creates a list in a and a list in b that points to the list in a. Then it modifies the list in a to point to b, creating a reference cycle. There are println! statements along the way to show what the reference counts are at various points in this process.

Filename: src/main.rs
*/
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}








/*
We modify a so it points to b instead of Nil, creating a cycle. We do that by using the tail method to get a reference to the RefCell<Rc<List>> in a, which we put in the variable link. Then we use the borrow_mut method on the RefCell<Rc<List>> to change the value inside from an Rc<List> that holds a Nil value to the Rc<List> in b.

When we run this code, keeping the last println! commented out for the moment, we’ll get this output:

$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/cons-list`
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
*/








// Creating a Tree Data Structure: a Node with Child Nodes

/*
To start, we’ll build a tree with nodes that know about their child nodes. We’ll create a struct named Node that holds its own i32 value as well as references to its children Node values:

Filename: src/main.rs
*/
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}




/*
Next, we’ll use our struct definition and create one Node instance named leaf with the value 3 and no children, and another instance named branch with the value 5 and leaf as one of its children, as shown in Listing 15-27:

Filename: src/main.rs
*/
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}





// Adding a Reference from a Child to Its Parent

/*
Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist. This is a case for weak references!

So instead of Rc<T>, we’ll make the type of parent use Weak<T>, specifically a RefCell<Weak<Node>>. Now our Node struct definition looks like this:

Filename: src/main.rs
*/
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}





/*
A node will be able to refer to its parent node but doesn’t own its parent. In Listing 15-28, we update main to use this new definition so the leaf node will have a way to refer to its parent, branch:

Filename: src/main.rs
*/
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}




/*
At this point, when we try to get a reference to the parent of leaf by using the upgrade method, we get a None value. We see this in the output from the first println! statement:

leaf parent = None
*/



/*
When we print the parent of leaf again, this time we’ll get a Some variant holding branch: now leaf can access its parent! When we print leaf, we also avoid the cycle that eventually ended in a stack overflow like we had in Listing 15-26; the Weak<Node> references are printed as (Weak):

leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
*/






// Visualizing Changes to strong_count and weak_count

/*
Let’s look at how the strong_count and weak_count values of the Rc<Node> instances change by creating a new inner scope and moving the creation of branch into that scope. By doing so, we can see what happens when branch is created and then dropped when it goes out of scope. The modifications are shown in Listing 15-29:

Filename: src/main.rs
*/
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}



