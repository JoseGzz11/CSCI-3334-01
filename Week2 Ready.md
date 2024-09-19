
// Week # 2 

// Concept of Ownership or In What Kind Of Memory Managment Rust Believes.


// -> Intro to discussion

// SO before we start, let's talk about mythical elephant in the room or Reality.

// Every program no matter in which programming language written, will require some memomory. Because data should be stored somewhere and somehow. Nothing in life happens just by itself right?

// Modern computer can execute many program's at the same time. You can play online game, and listen music on Spotify  and you can even read the textbook from webpage at the same time.

// When any programs starts to execute. It should make agreement with Operation System (mother of all programs) and ask for some virtual memory to use while running the program.

// Or to be more specific("Program" asks "Kernel" who manages all hardware resources for Virtual Memory, including a "RAM" (Random Access Memory)(this is a place where all working (data and machine code) is keeped during the execution of application) Kernell is a heart of any Operation System.

// And those agreements usually referred to as system calls, because you commincate directly with the boss aka OS.

// Virtual Memory model Helps to make sure, that if RAM becomes full during the execution of the some program, extra data will be transformed to Hard Drive for a while. Even though it will come with the cost.


// Ok let's keep it simple. Let's say program is granted with access to Virtual Memory, 
// based on logic of OS, Windows nowdays gives up to 2gb. Mac OS like up to 4 gb )) per process. you can search it on google of course
// Memory will be allocated based on your usage and needs.
// So as soon as you program stop needing some memory it must return it back to OS. Because at the end of the day there is a limit.


// "When you write your program, you can just assume that if everything goes well, you will be granted with memory, so you write a program based on that assumption"

// Like planing spending money which you didn't get, yet )))

// Next big question for you as a programmer How your  program(process) gonna use this virtual memory?
// Well in fact then there is a war for memory starts))


//  Programming Languages regarding Memory managment 
//can be splitted in two main groups (who supports manual memory managment vs who believes in Garbage collector)

// Memory management from Stacks vs Heap

// Usually rule of thumb is next. For better abstraction let's think in terms of C,C++ vs Java.
// Because they are leaders of two groups )))

// So every language compiler has it's own memory managment system, but in general it just allocates the space
// for the use of  Stacks and Heaps.

// First, every (variable )data which size was known at a compile time goes to Stack.
// (Array has a fixed size and known data types it goes here)

// Second, if it's unknown it goes to Heap. ("Usually it's a heap who requests additional memory from OS")
// (Vector for example, who knows how many element it will need to keep???)


// At this point starts really interesting thing, because as a software engineer, you need to write effecient program, because if you gonna use too much memory for your program (process). 
// Your program starts to freeze in best scenario. And end-users will not be happy about that,that is for sure.


// With stacks, things are more or less crystal clear.
// Because data is kept contigiously in sequence and it follows First In Last out rule.
// There is nothing really to manage
// Every command gets executed in it's own stack frame(so all data which is necessary is kept here), as soon it gets executed it will be removed from stack 

// Conflict starts in the Heap. Because data is writen there and there and is always changes in size.
// So program needs way to manage allocation and freeing memory during execution of the program.
// To make sure that it runs correctly, so here is the crux of all problems

// Let's start with First => Garbage collector or GC Approach.

// GC basically says next, you as a programmer should not worry about memory. I will manage it for you, you can trust me.
// Well, here you just gonna rely on GC, he has it's own policy to decides which space of memory is free.
// Not much a control, but I honestly don't know if it is for good or bad. It becomes just a Black Box. 
// Usually it works good, but sometimes don't. It slows down your program, by definition.


// Second approach goes like => You are the programmer, you are the Creator you must know what you are doing.
// So compiler will just fully trusts you with all memory managment.
// Sounds like a good stuff, but it comes with responsobility

// Two most common problems

// 1) When exactly to relize memory?
// We can forget (clutter our memory usage), relise early (lost data, like null exception), relize twice (release first time, another process came and write something on space which is assumed free, and then relize second time, so other process just losses the data !!!)


// 2) Data race.
// If I have a two threads, who has access to the same memory space. (Concurrency in practice)
// And one is reading, other is writing? or even worse(both them are writing) What I should do?????

Note: In general, the amount of code and abstractions you are able to keep in mind in one moment is proportional to your coding strength.

Imagine mastering, "manual memory managment" in addition to "program logic", at this moment you become monster of coding.



// Ok with that all being said, now you know what's the all business all about.

//We can now turn to Rust Phylosophy of Memory Managment or the Concept known as Ownership.


// Part # 1 : Rust Unique Feature: Ownership

//General problem we are trying to solve:
// Keeping track of memory space is being used and release it as soon as possible (you don't need it)


// Ownership is 3 rules:

// 1) Each value in Rust has a variable that'called owner
// 2) There can be one owner at a time
// 3) When variables goes out of scope, the value will be dropped.

fn main() {

    // Let's start with simple concept
    // Variable scope 
    // { } variable scope is usually referred to any variable which lives between curly brackets

    {
        let s = "hello"; // this a string literal -> or hardcoded string // Rust distunguishes string literals(lives in stack, immutable) and string data type(lives on heap, we keep only pointer, mutable or changes in size)
        
        println!("message from stack : {}", s);
    }

    // println!("message : {}", s); // error[E0425]: cannot find value `s` in this scope --> src/main.rs:23:30


    // The String Type (data which lives on the heap)

    let s = String::from("Hello"); // implementaion of creating String in our case from()
    // 1) memory is requested from operating system at runtime
    // 2) well we must release it at some point back to OS!
    
    println!("message from heap : {}", s);

    let mut s = 1234.to_string(); // Note: rules regarding mutability still in place 
    println!("message from heap : {}", s);

    // Strings are mutable
    s.push_str("4567");
    println!("My string number  : {}", s);

    // How do we know when to release memory?
    // Rust here is unique it enforce the 3 rule of ownership: the memory released when variables goes out of scope(special drop() function called immediately after closing 
    // curly braces )
    

    // Well you might say that, it's same as Garbage Collector, well almost. 
    // 1) Memory is actually released immediately, no need to wait for GC to free the space
    // 2) There is a second rule which makes it not that straightforward, because as you noticed Rust follows his phylosophy diligently

    

    // => Common operations we perform, but know we will start to think about them!

    // Here since size is known at a compile time.Values will be keeped on stack. No problem. Values just copied.
    
    let x = 5;
    let y = x;
    println!("x is : {}, y is : {}", x, y );

    // But here Second rules takes in charge
    
    // let s1 = String::from("Hello");
    // let s2 = s1;

    // println!("s1 is : {}, s2 is : {}", s1, s2);


    // Since our String "Hello"  is a data value which lives on a heap, it can only have one owner at a time.

    // So this rule enforce us basically from freeing memory twice. 
    // As soon you pass value of your pointer (to the data in heap to some other variable) -> you lost ownership :) 
    // This idea referred as a MOVE
    
    //No two pointers to the same memory space at a time. Preaty neat idea. Though not trivial.

    // So if we do need a copy, we need to make a deep copy or clone, cause pointers are not copied!!! They move ownership

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 is : {}, s2 is : {}", s1, s2);

    // Ok, now things become interesting, because at some point we want to pass our pointer to the function
    // Concept of References and Borrowing 


    // By the previous logic I think you already know what's gonna happen.

    // If we want pointer back, we should return, because as soon function finishes executing
    // all variables will be "eliminated", since {} brackets scope.

    // Short example

    let mut word = String::from("UT");

    println!("Before {}", word);

    fn logo(mut name: String) -> String { // I must return ownership back!!!
        name.push_str("RGV");
        name
        //String::from("UTRGV")
    }

    
    // => Borrowing Ownership with help of References
    word = logo(word);

    //let new_word = logo(word);

    println!("After {}", word);
    //println!("After {}", new_word);


    // At this point, you may start to think that this is absolutely lame : )

    // So here comes next idea

    // Using references, without taking responsobility of being owner )) we just borrow a pointer for 
    // the time of execution of our function, again because of first rule, as soon as function is executed reference will not exist anymore ))

    let mut word = String::from("UT");
    println!("Before :  {}", word);

    fn modern_logo(name: &mut String) { // we expect mutable reference to the value of type String
        name.push_str("RGV THE BEST");
    }

    modern_logo(&mut word);
    println!("After :  {}", word);

    // Note to Keep in mind:
    // References comes in two types:
    // mutable (one at a time)
    // read-only (as many you want)

    // This again ensures no "data race" (no two pointers can write at the same time)

    // And of course you can't have reference to the same data one reading and one writing.

    let s = String::from("Borrow to write once or many read. Not both at the same time");

    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s; // this line will not even compile

    println!("{} \n {}", r1, r2);

    // 
    // Another type of reference : Slicing 
    // Golang uses same idea. Using Slicing as a Slice Type ;)
    // You can think of this as obtaining a "window" into another data type, to see what's there. Pretty strange idea, but I imagine it like that, so I thought it could help you too.


    // To better understand the motivation for a need for such type of reference. Let's look at The Book example

    // Problem statement: "Write a function that takes a string and returns the first word it finds in that string. If the function doesn’t find a space in the string, it means the whole string is one word, so the entire string should be returned"


    // Let's try to solve problem on paper first.

    // Well, words in string are separated by spaces. And since, we believe that our first characer in string
    // starts with the letter, techincally what we need to do is just find first space ->" ". At this point,
    // we already know what to do, we just iterate through a string with a for loop and as soon as we hit empty space we can just return.

    // To better understand example from the book, we need to remind to ourself another idea. String is a collection of characters -> every character asumming ASCII takes a memory of 1 byte (trust me I learned this hard way. Year ago I failed Google interview, because I did not know that UTF-16 takes 2 byte)

    // So I hope at this moment this function is straighforward(just notice we pass a reference, since we don't want to deal with ownership)

    // At first let's just return the index of first word (it's assumed it starts at 0 and first empty space indicates the end of the word)
        
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); // convert to array of bytes
    
        for (i, &item) in bytes.iter().enumerate() { // uses reference to iterate, enumerate helps to keep track of indexes (nice example of tuple unpacking)
            println!("Byte under index {}: = {} or for a human this is a leter {}", i, item, item as char);
            if item == b' ' { // byte of empty space -> b" "
                return i;
            }
        }
        s.len()
    }

    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("First word ends at index: {} ", word);
    println!("First word is : {} ", &s[0..word]); // This is string slicing, which we are about to discuss


    // So far so good. But here's the caveat. If at some point our string will be changed, our result (5) is not VALID.
    // And we will never know about it, so if later we decide to read first word by relying on our function call, we can end up nowhere
    
    
    s.clear(); // This empties the String, making it equal to "".

    println!("Our string : {} ", s);
    println!("First word ends at index: {} ", word);
    // println!("First word is : {} ", &s[0..word]); try it by yourself)))

    // So to address this issue Rust's solution is -> using Slices, well in our case to be specific String Slices.

    let s = String::from("hello world");

    let hello = &s[0..5]; // it's reference not for the whole, but part of the String
    let world = &s[6..11]; // Rust implements internally slices to keep starting position and it's length

    // [start..end] -> last is not included of course. len is calculated as (end - start)

    

    println!("{} == {} {}", s, hello, world);

    // Well with that knowledge we can update function and return a whole first_word, not an index

    fn first_word_2(s: &String) -> &str { // we expect a string slice -> &str that's the type of it
        
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]; // we return string slice
            }
        }
    
        &s[..] // way to refer to the whole string, in case of no empty spaces
    }

    // So let's repeat, what we were not really happy last time

    let s = String::from("hello world");

    let word = first_word_2(&s);

    
    println!("Our string : {} ", s);
    println!("First word : {} ", word);

    

    // s.clear(); // Nice Error! Run it )) Not bad at all. Now Rust works to ensure stability
    
    // Well it's because of support of the idea of ownership. We borrowed string, and returned a "window view" on borrowed string
    // So at a time by the promise of onwership principle (one writer or many readers, but not both at a time)

    // Well let's try to hack it if it's possible, just for fun or try to obtain same result by passing ownership

    
    

    // // Attempt # 1: Let's take an ownership and try to return it back along with the first_word
    
    // fn first_word_3(s: String) -> (String, &'static str) { // we expect a string and &str string slice from our passed String
        
    //     let bytes = s.as_bytes();
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return (s, &s[0..i]); // we return string slice
    //         }
    //     }
    //     (s, &s[..]) // Hehe 
    // }

    // let s = String::from("hello world");
    // let (s, word) = first_word_3(s);


    // // Attempt # 2
    // fn first_word_4(s: String) -> (String, String) {
        
    //     let bytes = s.as_bytes();
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return (s, String::from(s[0..i])); // we return new string from a string slice of passed String
    //         }
    //     }
    //     (s, String::from("")) // Hehe 
    // }

    // let s = String::from("hello world");
    // let (s, word) = first_word_4(s);
    

    // println!("Our string : {} ", s);
    // println!("First word : {} ", word);


    // Attempt # 3 Ok with some cheating, I was able to do it, but now I have two different strings with separate lifetime

    
   fn first_word_5(s: String) -> (String, String) {
    let mut first_word = String::from("");
    
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            break; // we try to return newly string explicitly (it doesn't make sense to return a slice)
            // since lifetime will end as soon as function returns
        }
        first_word.push_str(&s[i..i+1]);
        
    }
    (s, first_word)
    
}

    let s_new = String::from("hello world");
    let (mut s_new, word) = first_word_5(s_new);
    
    
    println!("Our  original string : {} ", s_new);
    println!("First word with different life time : {} ", word);
    s_new.clear();
    println!("Our  original string : {} ", s_new);
    println!("First word with different life time : {} ", word);

} 
