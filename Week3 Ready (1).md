Common Programming Concepts
Understanding Ownership



// Week #3  STRUCTS, ENUMS AND MATCHING 

// or under umbrella of custom defined data types.


// Well I believe, nothing in this week, will be like wow knowledge, especially if you have some familiriaty with Object-Oriented programming.

// But since we learned a new concept of Ownership, we need to keep it in your mind.


// Section 1 : Structs.

// Well of course since it's a programming language, we could expect that there should be some way to construct our own user defined data type, in order for us to be more efficient as a programmer to solve our specific problem.

// Everything you learned on examples of Dog and Cats (most popular examples in OOP so far), will be applicable here of course. But before we start, let's remind us the main motivation and philosophy for user defined data types.


// Well motivation is pretty simple, you want to have related data and methods (functions which can be used only on objects of this type)
// So it will be easier for us to maintain and develop code.

// Phylosophy is put in one box (variables and methods together) and allow objects to talk to each other in order for program to accomplish some goal.
// Assuming that our objects will mimic real life objects

// In Java and many OOP languages it's a "class".

// When I was learning C++, I came across the idea of struct, and like what ???, C++ is OOP languages, why we need structs. And I was more confused when I discovered that I can even declare methods inside the struct. I was like, why we then need classes at all in CPP.

// Till today I don't really know, why.
// I heard many explanation like, simple data without methods goes to Struct, data with methods goes to Class.
// To support backward compatibility with C language, and so on.
// May be you know the motivation. But as for me, all of them doesn't make much sense. Because, techincally I could define methods in struct!

// But ok, now we will not have such confusions at all, we gonna have only STRUCT


#[allow(dead_code)] 
// this line mute warning regarding unused values

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    }


fn main() {

    // Defining and Instantiating Structs

    let user1 = User {
        email: String::from("dotdot@mail.com"),
        username: String::from("coder"),
        active: true,
        sign_in_count: 1,
    };

    println!("User 1 email : {}",user1.email);
    // Keep in mind, user1 is still immutable, so as soon as struct instantiated, I can't change fields

    // Instantiating like this is a cumbersome, you can create a helper function

    fn build_user(email: String, username: String) -> User {
        User { 
            username: username,
            email: email,
            sign_in_count: 1, 
            active: true, 
        } // it's expression no need for ;, because then it became statement. Last expression is automatically returned from any function
    }

    let user2 = build_user(String::from("google@gmail.com"), String::from("Gimmy"));

    println!("User name {}",user2.username);

    
#[allow(dead_code)] 
// this line mute warning regarding unused values

struct Student {
    name: String,
    sid: String,
    major: String,
    active: bool,
    }
fn main() {
    // Defining and Instantiating Structs
    let s1 = Student {
        name: String::from("dotdot@mail.com"),
        sid: String::from("777-777-777"),
        major: "Computer Engineering".to_string(),
        active: true,
    };
    println!("Student major : {}",s1.major);

    fn decipher(major: String) -> String{
        if major == "CS"{
            return "Computer Science".to_string()
        }

        if major == "CE" {
            return "Computer Enginnering".to_string()
        }
        "Major is unknown".to_string()
    }
    fn build_student(name: String, sid: String, major: String) -> Student {
        Student { 
            name: name, 
            sid: sid, 
            major: decipher(major.to_uppercase()), 
            active:true,
        } 
        
    }
    let s2 = build_student(String::from("Gimmy"), "888".to_string(),"ce".to_string());

    println!("Student 2 Major {}",s2.major);



}


    // Textbook Example: Motivation for using Structs and Implementing Related Methods

    // This example is very interesting from pedagogical point of view. Or explanation goes from bottom to top, nice approach.
    // May be it will convince you to group related data and methods together.

    // Problem statement : Calculate the area of Triangle.

    // Attempt # 1:  Approach Straightforward
    {
    let width1 = 30;
    let height1 = 50;

    println!("Attempt 1: The area of the recntangle is {} square pixels.", area(width1, height1));

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    }
    // So far so good. 
    
    // Attempt # 2: Since width and height are related we can try to pack them together in one tuple
    {
    let rect2 = (30,50);
    println!("Attempt 2: The area of the recntangle is {} square pixels.", area_2(rect2));

    fn area_2(dimensions: (u32,u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    }
    // Attempt # 3: Let's go with struct, what is this 0 and 1
    {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect3 = Rectangle { width: 30, height: 50 };

    println!("Attempt 3: The area of the recntangle is {} square pixels.", area_3(&rect3)); // we pass a reference , not to deal with ownership

    fn area_3(rectange: &Rectangle) -> u32 { // we borrow pointer, just for calculation
        rectange.width * rectange.height
    }
    }

    
    // Attempt # 4: Ok it seems that area method should belong to Rectangle class, well no doubt about it
    // let's literally implement that method
    {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle { // impl -> key word implement
        fn area(&self) -> u32 { // &self we refering to our own reference (no ownership staff)
            self.width * self.height // -> cpp reference and dereference happens automatically
        }
    }

    let rect = Rectangle {width : 30, height : 50};

    println!("Attempt 4: The area of the recntangle is {} square pixels.", rect.area());

    // What I find particularly useful. That if at some point. There is a need for additional functionality, 
    // I can start implementing methods on the "fly"

    impl Rectangle { 
        fn perimeter(&self) -> u32 {
            (self.width + self.height) * 2
        }
    }
    
    println!("Attempt 4: The perimeter of the recntangle is {} pixels.", rect.perimeter());

    // Another example, if you familiar with idea of comparators, you will find this, very useful.
    // But I will keep it simple

    impl Rectangle { 
        fn is_areas_equal(&self, other: &Rectangle) -> bool {
            self.area() == other.area()
        }
    }

    println!("Attempt 4: The area of two recntangles are equal: {} ", rect.is_areas_equal(&Rectangle{width: 10,height:150}));


    // Ok to conclude, let's write so called Associated Functions (in OOP this concept is a static function, you don't need an instance to use it, often used as a factory function (create new objects) -> in OOP there is even a special design pattern called Factory ))

    impl Rectangle {
        fn base_model(width:u32, height:u32) -> Rectangle {
            Rectangle {width,height}
        }
    }

        let rect1 = Rectangle::base_model(50, 200); // notice :: access to static methods -> same as String::from() ))

        println!("Attempt 4: The area of the base recntangle is {} square pixels.", rect1.area());
        
        
    }
    
        
     // Side note. How to print content of Struct
    #[allow(dead_code)] 
    #[derive(Debug)] // activates :? print data in debug mode
    struct RectanglePrintable {
        width: u32,
        height: u32,
    }

    let printable = RectanglePrintable {width: 30, height: 50};
    println!("My rectangle : {:?}", printable);

}

// And of course some practice, and trust me you already know everything to code this problem

// Warm -Up

Exercise 1: Defining and Using a Struct
Create a Person struct that holds a name (String), an age (u32), and a country (String). In your main function, create a person, then print their name, age, and country.

Exercise 2: Using Methods with Structs
Expand on the Person struct from Exercise 1 by implementing a method introduction that prints a formatted string introducing the person. For example, "Hello, my name is Alice, I am 30 years old and I come from Canada."

Exercise 3: Associated Functions and Multiple Impls
Implement a Rectangle struct with two fields, width and height (both u32).

a. Implement a method area that returns the area of the rectangle.

b. Implement an associated function (a static method) square that takes a single u32 argument side and returns a Rectangle where width and height are both side.

c. Create another impl block and add a method is_square that returns a boolean indicating whether the rectangle is a square (i.e., whether width and height are equal).


https://leetcode.com/problems/design-parking-system/
1603. Design Parking System

// Section 2 : ENUMS

// Enumerated data type should be somewhat familiar for you,
// but since there is no definition for abstract classes. All job is done by enums data type (because it's still custom data type, which means programmer can define exactly what he wants)

// But before diving in, how Rust treats enums. Let's remind us classic enum usage, with conjunction of switch statement (just Java example). Most likely you will notice that Rust treats it in same manner


// Simple ENUM

// Main logic is to define a custom data type which can take only specified values, like constans, which basically means, you can be either one of these values, or you will not exist at all. Pretty cruel. But sometimes we need that logic.


// Java
/*


enum Lights {
    RED,
    GREEN,
    YELLOW
}

// And usually enums, heavily used with switch statements

public class Main {
  public static void main(String[] args) {
    Lights status = Lights.RED;

    switch(status) {
      case RED:
        System.out.println("STOP");
        break;
      case GREEN:
         System.out.println("GO");
        break;
      case YELLOW:
        System.out.println("WAIT");
        break;
    }
  }
}

// I believe nothing really fancy in this example, in C++, it will look pretty similar
*/


// Well nothing is new under the Sun, so RUST often relies on ENUM as well, but it treats each value as opportunity to define not just a value, but a basically unique class, which will belong to this enum type. You could think it like this

/*

enum Pets {
    CAT,
    DOG,
    ELEPHANT
}

// plus in addition to value, you can specify what exactly that value will be.
//may be even a new "struct(class)"
*/

// OK, now since you know, which dots you need to connect, let's turn to classic THE BOOK source.

// Problem: Define a data type which resemble IP-address.

// So IP address is a parent, and it can have two childs as V4 and V6

#[allow(dead_code)]
#[allow(unused)]
fn main(){
    
enum IpAddrKind {
    V4,
    V6,
}

    let four = IpAddrKind::V4; // notice access through ::
    let six = IpAddrKind::V6;

    // grouping like that, basically allows us achieve so-called polymorhism
    fn route(ip_type: IpAddrKind) {} // we can pass V4 or V6 around, as both them belong to IpAddrKind



    // OK, but as I said, it's not main benefit

    // like in classes, we can do like that
    
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let office = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // not bad, but it is too much code, and our enum basically become as a component of struct, this
    // in fact divides our attention, we want to just define IpAddr, but then we end up createing additional IpAddrKind enum, well, this is not a bad example of composition, but if you came from Java, you know it' pretty popular idea, it's like classical OOP.

    // But let me tell you a fun opinion: There are famous design patterns, which teaches you to structure your code in so-called efficient manner, well, there is a saying that, "design patterns, exist only to hide flaws of programming language design", very funny opinion, but it has some logic

    // Ok back to RUST

    enum IpAddrPure {
        V4(String),
        V6(String),
    }

    let home = IpAddrPure::V4(String::from("127.0.0.1"));
    let office = IpAddrPure::V6(String::from("::1"));

    // Almost perfect, but it turns out there is even more flexibility, because enum, allows you to define variants of enum, with absolutely different associated values and even implement methods for enum as well. This is really nice idea, from language design perspective (Rust claims to beat C++, they gotta work hard for that)

    enum Message {
        Quit, 
        Move { x: i32, y: i32 }, // you can specify variant with the help of anonoymous struct, we don't need to name it, becase it will be associated with Move only
        Write(String), // tuple struct
        ChangeColor(i32, i32, i32), // tuple struct, remember types can be different
}

    // in essence this kind of syntax allows you to define fields inside of your custom data type enum

    // Ok let's now create our own example of enum ))

    enum SuperHero {
        Tryndamire{name:String, ulta: String, weapon: String},
        Jax{name:String, ulta: String,damage: i32},
        Sona{name:String, heal: i32},
    }

    impl SuperHero { 
        fn who_am_i(&self) {
            
            match &self {
                SuperHero::Tryndamire{name,ulta,weapon} => {println!("My name is {}. My ulta: {} My fav weapon: {}",
                                                                      name, 
                                                                      ulta, 
                                                                      weapon);}
            
                SuperHero::Jax{name,ulta,damage} => {println!("My name is {}. My ulta: {} My damage is pretty good: {}",
                                                                       name, 
                                                                       ulta, 
                                                                       damage )} 
                
                SuperHero::Sona{name,heal} => println!("My name is {}. I can heal you: +{}",
                                           name, heal ) 
            }
        }
    }

    let x = SuperHero::Tryndamire { name: ("Tryn".to_string()), ulta: ("5 sec immortality".to_string()), weapon: ("sword".to_string())};

    let y = SuperHero::Jax { name: ("JAX".to_string()), ulta: ("extra damage".to_string()), damage: (40) };

    let z = SuperHero::Sona { name: ("SONA".to_string()), heal: (50) };

    let heroes = [x,y,z];

    for h in heroes {
        h.who_am_i();
    }
 
}

//Section # 3: Big promise of RUST regarding Null pointer exception ))


// If you have been solving coding problems from problem like LeetCode, you probably noticed
// that some test case will give you some null case, to make sure that you will handle it.
// Especially for linked lists and binary tree.



// And what we usually do, is write 
// if node == null:
//   return

// Proably not a bad idea, but we can forget about this case more often.
// Because it requires us to handle imaginative situation, which may or not happen, so our minds can say, why to bother

// But since it's not a good practice to ignore null situation, creators of Rust decided to enforce check for null value inside the compiler design.

// Well basically it means, your code will not compile, if there is a situation, where null pointer exception could raise, and you did not specifically treat that situation.

// Other than that, nothing is new under the sun.

// Ok let's go to Rust, we already know what enum is.

// Well it happened to be there is Enum in Rust, which automatically defined by standard library 

/*
enum Option<T> { // T is usually accepted convention for generic type or basically any data type which is handled (u32,String,Node, anything)
    Some(T), 
    None,

    // Well variant I believe named strangely, but in essence we could say, any data could be of 2 types, either exist or Some or don't exist None
}

*/

#[allow(dead_code)]
#[allow(unused)]

    
fn main() {
    let num = Some(5);
    let word = Some("a word");

    let not_value: Option<i32> = None; // well any data can be put inside Option enum (so we decided to put i32)

    // inside of not_value we expect to find either i32 or nothing, in any given day, In my mental model, I consider it as some kind of wrapper, if I open wrapper I expect to see that type T value or nothing.

    // Let's talk about next example, Rust Books is proud of this code snippet, but as for me who came from Python world, it's a bit a burden, but at least, it will teach us good habits to write safe code:

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;

    // OK, let's believe it's a smart, but now next question is, how to extract the value of T, from Some variant of Option enum

// Section #4 Pattern Matching with Match and concept of ARMS


// Ok at this point I believe you already see the pattern between switch and new "idea" match

// I keep it simple in my mind

// match "switch" some value {
    // case which evaluates to true => do something
    // case which evaluates to true => do something
    // case which evaluates to true => do something
    // case which evaluates to true => do something

//}

// Well in case of Rust, it's almost the same with just couple of additional rules.
// You must cover all possible scenarious explicitly (you can't forget to explicitly handle some case, which you believ unlikely to occur, you don't need them)

// Well you see, the connection between ENUM and Match => Enum can take only limited values, match should handle all possible values.

// Ok with that in mind let's go


// Problem: Based on name of the coin, provide how many cents it has

#[allow(dead_code)]
#[allow(unused)]

fn main() {
    enum Coin {
        Penny = 1, // just to remind us that enum does have order (which allows it to live in stack)
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin:: Dime => 10,
            Coin:: Quarter => 25,
        }
    }

    fn int_to_coin(idx: i32) -> Coin {
        match idx {
            1 => Coin::Penny,
            2 => Coin::Nickel,
            3 => Coin::Dime,
            4 => Coin::Quarter,
            _ => Coin::Penny, // _ useful to matching all other cases
    
        }
    }

    for i in 1..5 {
        println!("{}",(value_in_cents(int_to_coin(i))));
    }

    // back to previous problem

    fn add_two(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 2), 
        }
        
    }

    fn value_extractor (x: Option<i32>) -> i32 {
        match x {
            None => 0,
            Some(i) => i,
    
}
    }

    let five = Some(5);
    let sev = add_two(five);
    println!("{}", value_extractor(sev));
    

}

Warm -up Enums

Creating and Using Basic Enum:

In this exercise, you should define an enum called Language that can have one of the following values: Python, Rust, Java, JavaScript.

Then, create a function named print_language_preference() which takes a parameter of type Language and print a message in the console based on the value of the Language enum.

Enum with Associated Data:

Define an enum called Shape that can represent two different geometrical shapes: Circle and Rectangle. Circle should have one associated value representing the radius (a floating-point number), and Rectangle should have two associated values representing the width and height (two floating-point numbers).

Then, create a function called calculate_area() that takes a Shape as a parameter and calculates the area based on the type of shape (use π=3.1416 for the circle).

Match Statement with Enum:

Define an enum called Color that can represent three different colors: Red, Blue, Green.

Then, create a function called color_message() that uses a match statement to assign a different message to each color. This function should take a Color as a parameter and print a different message for each color: "Red is warm.", "Blue is cool.", "Green is natural.".


// Problem
13. Roman to Integer
https://leetcode.com/problems/roman-to-integer/




