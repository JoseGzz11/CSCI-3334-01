// Week # 4

// Section : 1. How to Organize our Code in Crates and Modules

// Ok, techincally we are a couple of lectures, from starting writing our own projects. So it's the time to learn about meaty part of Rust.

// In order to better understand, why built-in package manage of Rust is useful and blessing. Let's try to organize next code.

// Toy problem Bank System:

// Imagine the sitation, where you have bucnh of functions and one struct. For example:

// fn withdraw_money(){}

// fn deposit_money(){}

// fn apply_for_loan(){}

// fn consult_potential_clients(){}
        
// fn open_account(){}

// struct BankAccount{
//     name: String,
//     money: i32,
// }

// Our src folder is considered as crate - or just folder with all source code, for now we just have main.rs

// Literally, structure of our code is flat
// -> crate (main.rs)
//   -> withdraw_money
//   -> deposit_money
//   -> apply_for_loan
//   -> consult_potential_clients
//   -> open_account
//   -> struct BankAccount

// Well, the problem with this organization of code, is that it's a total mess, even without implementing anything, it's easy to lose track what is happening, just random functions, probably somehow related to BankAccount




// Let's take a First step: Organize into modules
// Keyword mod comes to resque, it's keyword, which allows organize our code in related modules, where
// each module has it's own scope (we will learn about use keyword soon.)


// mod customer_relation {
//     mod provide_bank_service {
//         fn withdraw_money(){}

//         fn deposit_money(){}

//         fn apply_for_loan(){}
//     }

//     mod assistant_service {
//         fn consult_potential_clients(){}
        
//         fn open_account(){}
//     }
// }

// mod bank_entities {
    
//     struct BankAccount{
//     name: String,
//     money: i32,
//     }
    
// }

// Ok, now our code has a next structure

// -> crate
//   -> customer_relation
//      -> provide_bank_service
//           -> withdraw_money
//           -> deposit_money
//           -> apply_for_loan
//       -> assistant_service
//           -> consult_potential_clients
//           -> open_account
//   -> bank_entities
//       -> struct BankAccount


// I hope first idea which cames to your mind, is that it looks like a filesystem, and keyword mod is just a way to create a new folder. 

// Ok, perfect, as with filesytem there is absolute and relative path, so if we want to reach some item (function,struct, etc) from another folder, we first need to provide a pathh to that folder.

// At this point, I believe we should start to write some code

// Second step: Add some functionality

#![allow(dead_code)]

mod customer_relation {
    
    mod provide_bank_service {
        
        fn withdraw_money(){}

        fn deposit_money(){}

        fn apply_for_loan(){}
    }

    pub mod assistant_service {
        
        fn consult_potential_clients(){}

        use crate::bank_entities; // I need to provide a path to the module I plan to use
        pub fn open_account() -> bank_entities::BankAccount {
            bank_entities::BankAccount::create_account("Anon")
        } 
    }
}

mod bank_entities {
    
    pub struct BankAccount{
        name: String,
        money: i32,
    }

    impl BankAccount {
        pub fn create_account(name: &str) -> BankAccount {
            BankAccount{
                name: String::from(name),
                money: 0,
            }
        }
        
        pub fn money_details(&self) -> i32 {
            self.money
        }
    
        pub fn get_name(&self) -> &str {
            &self.name[..]
        }   
    }
}

fn main() { 
    let anon = customer_relation::assistant_service::open_account();
    println!("Name: {}. Money: {}",anon.get_name(),anon.money_details());
    
}

// So far so good, even though we still techincally work in a same main, our code looks much better,
// and at least we can create a some kind of mind map, of what is happening in our program.

// And now is the final third step :) Enjoy the magic
// It's time to to clean up our main.rs
// Logically, we just to need to create a file-structure exactly as in our main

// -> customer_relation (folder)
//     -> assistanct_service.rs
//     -> provide_bank_service.rs
// -> bank entities.rs
// -> customer_relation.rs (declaration for our module)

// and at the end we end up with just
// and everything still gonna work in the same way, guranateed

// #![allow(dead_code)]

// mod customer_relation;
// mod bank_entities;

// fn main() { 
//     let anon = customer_relation::assistant_service::open_account();
//     println!("Name: {}. Money: {}",anon.get_name(),anon.money_details());
    
// }

// Section : 2 
// Common Collections: Vectors,HashMap, BinaryHeap(Priority Queue)


// If you are curious and know what you need, next link will serve you well in future
// https://doc.rust-lang.org/std/collections/index.html

// Ok. now to my favorite topic, after covering collections, we gonna start to solve
// real LeetCode problems ))

// To be honest, we can spend all semester just on this topic, but we will just pick up absolute minimum to move further.


// I am 100 % sure, that nothing is gonna be new today, but ownership concept can now spoil the situation. And we need to learn how to use
// the collection with that idea in mind.

// And always remember collections live on the heap, key idea

// => Vectors
#[allow(unused_variables)]

fn main() {

    // Vector creating
    let v: Vec<i32> = Vec::new(); // if we don't provide value, to allow compiler infer the type, we need to specify the data type

    let v = vec![1,2,3]; // or use vec macro, if we know the type

    // Update vector

    let mut nums: Vec<i32> = Vec::new(); // and of course mut keyword
    nums.push(5); // adds element to the end of the vector, simulates stack data structure, with the famous push command ))
    
    nums.push(6);
    nums.push(7);
    nums.push(8);

    // ok, first point
    // since vector lives in heap, as soon as vector goes out of scope, it will be dropped, as well as all values it keeps, even though they are pointers!!!

    // Accessing elements

    // By index or get method of course (one for cpp one for java programmers :))
    {
    let last: &i32 = &nums[3];
    println!("Last element is {}", last);

    let last = nums.get(3);
    println!("Last element is {:?}", last);
    }
    // Ok what's the difference ?

    // There is a nasty error: index out of bounds
    // If you access not existing index with brackets -> you get crash or panick
    // By get method result gonna be Option enum -> you get None

    // Ok another thing, which in other languages will be normal

    {
        let first = &nums[0]; // we borrowing for reading
        // nums.push(10); // we writing into , good luck with this
        println!("First element: {}", first);
    }

    // Let's make it work

    {
        let first = &nums[0]; // we borrowing for reading
        println!("First element: {}", first);

        // Borrowed reference will be dropped here, then Dobby is free :)
        // Variable scope is important concept keep it in mind
    }

    {
        nums.push(10); 
    }

    // Iterating over vectors of course

    // Getting immutable reference
    
    for num in &nums {
        println!("My num: {}",num);
    }

    // Mutable reference
    for num in &mut nums {
        *num += 2; // dereference with *
    }
    
    for num in &nums {
        // No issues with borrowing since for loop has it's own variable scope
        println!("My num: {}",num);
    }


    // Ok now let's to hack vectors, well not really ))
    // Just if we want to keep in vectors different types of data
    // enums comes to rescue, becaue variant can be anything, but all of them lives under umbrella of enum data type
    {
        #[derive(Debug)]
        enum MyData {
            Int(i32),
            Text(String),
            Vector(Vec<i32>), // who is gonna stop us
        }

        let mut mix: Vec<MyData> = Vec::new();

        mix.push(MyData::Int(100));
        mix.push(MyData::Text("Hello World".to_string()));
        mix.push(MyData::Vector(vec![1,2,5]));

        for elem in &mix {
            println!("My elem: {:?}",elem);
        }
        // And of course pop()
        let last = mix.pop();
        println!("My last: {:?}",last); // notice save pop(), if no elements what to pop()??? only none value      
    }
    
}

// Couple of words regarding a String, it's better to understand it once, than to struggle for nothing.



#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {

    // Main point of failures or confusing Strings

    // Key idea: String is a collection of bytes. Not characters!!!

    // => Create a new String
    let mut s = String::new(); // empty
    // or
    let mut text = "text literal".to_string(); 
    // or
    let mut text2 = String::from("string still literal"); // with some content
    println!("=>{}<=:::=>{}<=:::=>{}<=",text2,s,text);

    // => Updating a String

    // Update
    s.push_str("hello");
    s.push('z');
    
    println!("s: {}",s);

    // Quck example, or don't mess with ownership (push_str()-> accept literal string not a String)
    let mut hello = String::from("hello");
    
    let world = " world";
    hello.push_str(world);
    println!("{}",hello);
        
    
    let world = String::from("world");
    //hello.push_str(world); // but good look with this
    hello.push_str(&world[..]); // borrow a pointer, the take a slice with will be &str or string literal
    // code above will save you from hours of debug 
    println!("{}",hello);
    
    // Update with concatenation
    let s1 = String::from("UT");
    let s2 = String::from("RGV");
    let s3 = s1 + &s2;
    println!("{}",s3);

    // Important comment or what is happenging.
    // Regarding the operators such +, always remember operators, still is implemented as a function, and they are just shorter way to write common operation.

    // https://doc.rust-lang.org/src/alloc/string.rs.html#2278-2286

    // Let me prove it to you
    use std::ops::Add;
    let s1 = String::from("UT");
    let s2 = String::from("RGV");
    let s4 = Add::add(s1,&s2);
    println!("{}",s4);

    // Well, what it techically means, that, since you still call the function you need to provide right parameters.
    // so as you can see, second parameter should get only reference, don't want to mess with ownership again ))
    // what happens is &s2-> converts to &s2[..], 

    // I guess at this point you think what is wrong with that Rust.
    // Don't worry, language policy is universal, and brings another benefits.

    // Ok how to concatenate multiple strings then // format! macros, wil make our lives simpler
    // so instead of printing we can obtain string back !macros

    let s1 = String::from("UT");
    let s2 = String::from("RGV");
    let s3 = String::from ("Edinburg");
    let s4 = format!("{}{} {} {}",s1,s2, "at", s3); // nohing will stop us
    println!("{}", s4);

    // Indexing String.
    // Advice just forget about indeexing string like s4[0], no and no 
    // Explantion: 

    // Rust saves Strings in memory as a vector of bytes, s what it means, depending on the encoding, 
    // UTF-8 1 character 1 byte
    // UTF-16 1 character 2 byte
    // all way up to 4 bytes

    // So there is no correlation between index of single byte with a letter or character at all!

    // Ok add some languages with ieroglyph, then it's a total mess 

    /*

    use std::mem;

    fn main() {
        let hello ="Hello".as_bytes();
        println!("{:?} len of hello = {}", hello,hello.len());
        println!("Size of hello in bytes {}",mem::size_of_val(&hello));
        
        let hello = "Hola".as_bytes();
        println!("{:?} len of hello = {}", hello,hello.len());
        println!("Size of hello in bytes {}",mem::size_of_val(&hello));
        
    
        let hello = String::from("Здрав");
        println!("{:?} len of hello = {}", hello,hello.len());
        println!("Size of hello in bytes {}",mem::size_of_val(&hello));
        
    }

    */

    // Ok don't worry of course there is a way around

    // Method for iteration, explicitly convert to char

    let hello = String::from("Здрав");
    
    for c in hello.chars(){
        println!("{}",c)
    }

    // Method for iteration, explicitly convert to raw byte value, but one character can consit of many bytes
    // This is the main reason why on coding interview, it's always good to ask through how one character is encoded, 1 byte, 2 byte, 4 byte ????? 

    for c in hello.bytes(){
        println!("{}",c)
    }
}

// => HashMap or Dictionary

// General idea:
// Vectors and arrays associate values with specific index
// HashMap associates values with arbitrary key(which can be encoded with some hash function), and basically playes the role of index 




#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {

    // => Creating a New Hash Map

    use std::collections::HashMap;
    let mut populations: HashMap<String,i32> = HashMap::new();

    populations.insert("UTRGV".to_string(), 30000);
    populations.insert("Edinburg".to_string(), 97000);

    //or another from the idea of zip, I don't like this example

    let names = vec!["UTRGV".to_string(),"Edinburg".to_string()];
    let population = vec![30000,97000];

    let populations: HashMap<_,_> = names.iter().zip(population.iter()).collect();

    // => Question of ownership

    // for simple types i32,etc values will be copied, for more complicated data types String, values will be moved, and HashMap will be the owner.

    let name = "McAllen".to_string();
    let p = 142000;
    let mut populations: HashMap<String,i32> = HashMap::new();
    populations.insert(name, p);

    //println!("Name: {}", name); // name is moved, so ownership is moved to hashmap
    println!("Population: {}",p);

    //Note: We can technically move reference to that value, but then we need to make sure that value,  will not go out of scope, before our HashMap

    // => Accessing Values
    
    let mccallen_population = populations.get("McAllen"); // Get is safe beecause it will return Option enum, so if key doesn't exist you just get None
    println!("McAllen population : {:?}", mccallen_population);

    let ny_population = populations.get("NewYork"); 
    println!("NewYork population : {:?}", ny_population);

    

    match populations.get("McAllen") {
        Some(value) => println!("McAllen population : {}", value),
        None => println!("No records!"),
    }

    populations.insert("UTRGV".to_string(), 30000);
    populations.insert("Edinburg".to_string(), 97000);

    // =>  Iterating through HashMap
    for (key,value) in &populations { // both key and value will borrow reference, no ownership responsobility
        println!("{}:{}",key,value);
    }


    // =>  Updating HashMap

    //Overriding old value
    populations.insert("UTRGV".to_string(), 45000);
    println!("{:?}",populations);

    // Only insert if key doesn't exist
    // => Entry Variant
    {

        let info = populations.entry("UTRGV".to_string());// quick note entry(key) will return mutable reference
        println!("{:?}",info);
    }
    {
        let ny = populations.entry("NY".to_string());
        println!("{:?}",ny);
    }
    // Entry is an API call, which return enum type called Entry with Variants {Occupied, Vacant}
    // https://doc.rust-lang.org/stable/std/collections/hash_map/enum.Entry.html


    // To continue on that Entry, you can apply a method .or_insert()

    populations.entry("UTRGV".to_string()).or_insert(10000); // if key exist it will not change it
    populations.entry("NY".to_string()).or_insert(18000000);
    println!("{:?}",populations);


    // Updating a Value Based on the Old Value (or simpler update key-value)

    // let's implement counter of words in a text

    let text = "hello top bot hot top hot bot";

    let mut word_frequency = HashMap::new();

    for word in text.split_whitespace(){
        let count = word_frequency.entry(word).or_insert(0);// if key doesn't exist, value will become zero
        *count += 1; // entry will return as a mutable reference, so to update the value we need to dereference it
    }

    println!("{:?}",word_frequency);

    // For curoius student
    // there are other HashMap implementation, other than Rust default.
    // please feel free to explore
    // https://github.com/rust-lang/hashbrown
    
}




