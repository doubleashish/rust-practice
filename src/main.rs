//https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-18 

use std::fs;
use rand::{Rng, thread_rng};
use chrono::{Local, Utc};
use std::collections::HashMap;
use std::fmt::Display;
use std::thread; //if use std::thread{self,spawn} then just writing spawn(...) spawns the thread
use std::time::Duration;
use std::sync::mpsc;
/*
cargo init
If program compiles then it will run
You cant segfault(using the memory that you dont have permission to) if you dont have null(null pointer points to nothing causing sefault)
Rust has type safety (variable can has number value but later can be assigned a string) but rust doesnt allow it
Rust has access to lots of resources in system memory (incredibly fast)
eg: mediasoup (has js api to talk to rust worker)
Rust (separate compiling step) => binary => runs
assign Multiple threads/cores (node js has inter process communication: slow)
Rust is used for making browsers, CLI and backend of full stack
cargo init --lib (makes library project) => cargo init (end user app)
cargo.toml is like package.json file
Finished => Runnng(main execution)
./target/debug/Rust runs the exact file(binary) and shows result
debug folder has local binary, release build has optimised binary
i8(2^7-1),i16(2^15),i32 ,i64,i128 number of bytes required
the first bit has the value stored of storage (+31)
-1 is for 0 the first number
i means signed integer, u means unsigned (cannot be negative)
f is for float (f32, f68)
print! ("{} {} ", x,y); print like this
cargo build is used to just compile not run
//run cargo fmt to format your code if indentation error


MEMORY MANAGEMENT
Program needs space in RAM, rust gives direct connection to it
JS: when x in loop is finished then garbage collector deletes it and frees memory
C: Allows you to allocate memory to your program (lead to dangling pointers: variable pointing wherwe no data)
Rust: own ownership model for memory management (forces you to write code so no error in memory management)
Memory leak means pointer is deleted but data is still present

MUTABILITY:
By default all values are immutable in rust
mut makes binding mutability but string can have .push_str() but not literals
Immutable data is inherently thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently.
Knowing that certain data will not change allows the compiler to optimize code better.
______________________________________________________________

STACK VS HEAP
space in RAM is stack for rust program. Some variables are fixed at compile time
Very fast allocation and deallocation eg boolean, numbers
capacity(how much more can be stored), len(how many bytes currently used), ptr(pointer to 1st index address)
this can be stored in stack because all these 3 things have known value
Heap is for randomised data (variables whose data can be changed maybe via loop)
eg vectors, strings 
Heavy structs and arrrays are also stored in heap
stack frame is word used for memory allocations in stack
heap has dynamically allocated memory thats why slow
We push a stack frame not indivisual variables in stack memory.
If one func calls another then whole stack frame of second func is pushed over it, when it is completed the data is removed
//The actual text ("hello") lives on the heap, not inline in the variable.
in stack we have a variable that points to heap and stack has the address of first index address
Memory capacity mostly stays same on concurrect checks unless length is changed (lots of time it stays same)

______________________________________________________________________________________
OWNERSHIP:
rules that govern how rust manages memory
these rules can make compiling slow but not program running
{variable in scope} is not present outside scope
doublefree error means first we deallocated, google put data, then allocated same memeory to our program
//THIS DIDNT WORK BECAUSE THE VALUE OF SENTENCE WENT TO THE FUNCTION SO EMPTY STRING HERE.


____________________________________________________________________
BORROWING AND REFERENCES
if borrower dies heap data doesnt die
&s1 points to stack location, doesnt get the ownership (s1 doesnt have to be mutable)
If one mut refernce then no reference of any kind
can have multiple immutable references but no mutable then
sentence.clone() creates a new heap memory and points via that

fn ...(some_string: &String). IF IT IS BORROWED USE &STRING
this &STRING has the pointer to the pointer of stack pointing to heap

YOU CAN HAVE ALL REFERENCES BUT AS LONG AS THEIR LIFETIME DOESNT COLLIDE, CODE COMPILES
if someone makes a immutable reference they dont expect the value to change suddenly
but if more than 1 mutable then data races can happen



//CARGO IS NODE JS LIBRARY IMPORT FOR RUST
actix.rs helps in making extermely fast http servers
serde.rs for serialisation and deserialisation (solana)
tokio.rs asynch await in rust
sqlx connects sql database
reqwest sents http requests
chrono for date and time


Mutable references: */
fn update(s: &mut String) {
    s.push_str("Works")
}




//STUCT
struct User {
    active: bool,
    username: String,
    email: String,
    age: u32,
}

struct Rect {
    width: u32,
    length: u32,
}
impl Rect { 
    fn area(&self) -> u32 { //first argueemnt has to be reserved for &self if more arguements are added => only 1 argueemnt in callingh
        self.width * self.length //adding semicolor here makes it into a stateemnt not expression
    }
    //i can  call a debug function but that works on Struct rect not on data rect::debug()
    fn peri(&self) -> u32 {
        return 2 * (self.width + self.length);
    }
}

struct NoShape; //unit struct for no value
impl NoShape {
    fn area(&self) -> u32 {
        return 0;
    }
}







//ENUMS:
#[derive(Debug)]  //this helps in printing whole struct {:?} format which is used for structs
enum Direction { //fixed value
    North,
    East,
    West,
    South,
}
fn movear(_direction: Direction) {// this function can only have values of Direction
    println!("{:?}", _direction);
}

enum Shape {
    Circle(f64),         
    Square(f64),         
    Rectangle(f64, f64),
}
fn calculate_area(shape: Shape) -> f64 {

    let created = match shape { //storing final value in created but only match works
        // enum functions would need match which selefcts which kind of enum variant is being used
        Shape::Circle(radius) => 3.14 * radius * radius, //std::f64::const::PI
        Shape::Rectangle(width, length) => {
            print!("Hi fron enum function");
            width * length
        }
        Shape::Square(length) => length * length,
    };
    return created;
}





//Option____________________________________________________________
fn first_a(s: String) -> Option<i32> {  
    //if i write usize instead of i32 then no need for index as i32
    //usize is 32 for 32bit system, 64 for 64bit system
    for (index,character) in s.chars().enumerate(){
        //chars breaks the string to individual characters
        //enumerate acts gives index to character = ((0,'a'), (1,'b') and so on)
        if character == 'a'{
        return Some(index as i32);}
    } return None;
}





//OWNERSHIPS
fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string;
} // return the string ownership back to the original main fn

fn first_word(sentence: String) -> String {
    //2nd String is the return type. Sentence is a variable that holds a pointer, length, and capacity.

    let mut ans = String::from(""); //ans is the owner of ""
    //sentence.char().count() counts number of characters
    for char in sentence.chars() {
        ans.push(char); //no need to str conversion
        if char == ' ' {
            break;
        }
    }
    return ans;
}











fn main() {
    println!("{}", is_even(24));
    println!("{}", fib(10));
    println!("Hello world!");
    let x: i8 = 5;
    let y = 34;
    let z = 23.23;
    let mut xx: u8 = 4;
    print!("{} {} ", x, y);
    print!("z: {}, xx: {}", z, xx);
    xx = 30;
    print!("{}", xx);

    //if we assign a number too big, literal too big for i8
    //wont let you compile if number is big
    //cannot mutate if not specified
    //compiler would exit processing if attempt to add to overflow (a loop that increases the variable above limited value)
    //even after mut cant change the data type
    //CONDITIONALS
    let mut is_male = false;
    let is_above_18 = true;
    is_male = true;
    if is_male {
        println!("you are a male")
    } else {
        println!("you are not a male")
    }
    if is_male && is_above_18 {
        println!("YOu are a legal male")
    }
    //LOOPS
    for i in 1..=5 {
        //without = it only runs till 4
        println!("{}", i);
    }

    //put _a if a is defined but not used

    //changing values in string is tough
    let greeting = String::from("hello aashish ");
    let yy = "aashish"; //str is confusing here

    //______________________________________________________________________________________________
    println!(
        "{},{},{:p}",
        greeting.capacity(),
        greeting.len(),
        greeting.as_ptr()
    );

    print!("{}", greeting);
    println!("{},{}", greeting, yy);

    let char1 = greeting.chars().nth(1); //.chars() breaks string into individual lettrs. nth(1) means which index of it
                                         //this is to check if char1 has a value because rust allows you to pick out of index as well
    print!("{}", char1.unwrap()); //this crashes the program, use match code because it offers alternates
    let char2 = greeting.chars().nth(2999);
    match char2 {
        Some(c) => println!("{}", c),
        None => println!("No character found"),
    }
    //println!("{}", char1) is not allowed

    //js allows to accewss 10000 character of greeting as undefined
    //but rust has option<char>
    //can change space at runtime
    //this string only has 7 letters not 32 or 64 bytes assigned

    //ITERATING OVER MAPS, ARRAYS, STRINGS

    let sentence = String::from("This is a sentence");
    let first_word = first_word(sentence);
    print!("{}", first_word);
    //println!("{}", &sentence[1..4]);
    //let forst_word = first_word(sentence.clone());
    //OR ... first_word(&sentence); borrow the sentence like this










    //_________________________________________________________________________________________________________________________
    //String is  a growable heap entitiy unlike &str which is just as it is
    //String::from is same as .to_string()
    //first argueemnt of println has to be a string literal not dynamic

    let mut s1 = String::from("hellow again AFTER OWNERSHIP"); //it has to be mutable
    let s1 = takes_ownership(s1); //s1 gets the ownership if return in function
    //let s3 = ... if you want value to come back to it or clone
    println!("{}", s1);

    let mut str = String::from("CHECKING IF IT WORKS? ");
    //let s4 = &mut str and then push the value to it (function not required)
    update(&mut str);
    //let s2: &mut s1; => s2.push_str("World")
    println!("{}", str);

    //mutable references
    let mut a1 = String::from("Hello from immutable/mutable");
    let _a2 = &mut a1;
    let a3 = &a1;
    println!("{}", a3);













    //STRUCTS _______________________________________________________________

    let user = User {
        username: String::from("Aashish"),
        age: 30,
        active: true,
        email: String::from("aashish@gmail.com"),
    };
    println!("{},{}", user.username, user.active);

    let rectan = Rect {
        width: 10,
        length: 923,
    };
    println!("{},{}", rectan.area(), rectan.peri()); //() for functions

    let cir = NoShape;
    println!("{}", cir.area());

    
    
    
    
    
    
    
    
    
    
    
    //____________ENUMS________________________________________

    //allow us to define types by enumerating its possible variants
    //if you have limited variants with defined types then use enums
    movear(Direction::North); 

    //ENUMS with shapes

    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    println!("{}", calculate_area(rectangle));

    //ERROR HANDLING _______________________________________________________
    //Compilation error (borrow checker, type checker) while making binary
    //Runtime error when running of binary haves a problem
    //we catch an error so rest of code can run
    //RESULT ENMUM
    //If a function can stop a process by causing an error then it usually results an Err type

    let res = fs::read_to_string("text.txt"); //returns <String/std:io:err>
    //res will have the data type of result
    match res {
        Ok(content) => {
            println!("file content {:?}", content)
        }
        Err(err) => {
            eprintln!("Error found {}", err)
        }
    }
    println!("Current directory: {:?}", std::env::current_dir());
    //this can give error be

    //____OPTION ENUM_______________________________________
//handles null for rust
//use when returned value cannot exist
//Return an option instead of Nulll

pub enum Option<T>{ //pub here makes the enum public to other modules
    None, Some(T)}


    let mastring = String::from("hellow this is a string");
    let mut res2 = first_a(mastring);
    match res2 {
        Some(location) => print!("{}", location),
        None=> println!("Nothing found")
    }
    res2 = first_a(String::from("hellow this is 
    string"));
    match res2 {
        Some(location) => print!("{}", location),
        None=> print!("Nothing found")
    }


    let mut rng = thread_rng();
    let n:u32 = rng.gen();
    println!("Random number: {}", n);
    

    //CHRONO time ________________________________________________________________________________________________________________________

    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);

    // Format the date and time
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}", formatted);

    // Get local time
    let local = Local::now();
    println!("Current date and time in local: {}", local);











//COLLECTIONS:____________________________________________________________________________________-
//COLLECTION can contain multiple values, data is stored in heap
//VECTORS
let mut vec = Vec::new(); //mut to push data
vec.push(1);
vec.push(2);
vec.push(3);
vec.push(4);
vec.push(5);
vec.push(6);
println!("{:?}", vec);

println!("{:?}", even_filter(&vec)); //value is ended after this
//i can send even_filter(&vec) and func input should also have vec: &vec<i32>
//same ownership as strings, mutable, slice 

 let mut num = vec!["true","hello","Third value"];
 println!("{:?}", num);
//vectors automatically infer the type of vector but in functions you have to specify












//_____________________________________HASHMAPS_____________________________________
//helps in keeping in key value pair
//insert, get, remove, clear
let mut hashy = HashMap::new();
hashy.insert(String::from("Aashish"), 22);
hashy.insert(String::from("Avinash"), 69);
let hashyuser = hashy.get("Avinash"); //it returns an option, because it might be wrong key
println!("{:?}", hashy);
match hashyuser{
    Some(age)=> println!("{}", age),
    None => println!("Wrong key")
}

let vctr = vec![(String::from("Aashish"),29),(String::from("Avinash"),69)];
let pvctr = shift(vctr);
println!("{:?}", pvctr);













//____________ITERATOR_________________________________________________________-----
//HELPS us to perform task over vector, string ,hashmaps
//no effect unless we call methods that consume iterator

 let itrnum = vec.iter();   
 
 //no meaning of this, the variable is storing the iterator not any value
 //itrnum has the iterator typer
//in for loops for i in num => num becomes an iterator
// for i in itrnum also works. it is a struct of type iterator(borrowing the value not consuming(not the owner))
//for i in num consumes the value, cant print num after this loops

let mutitrnum = vec.iter_mut();
for val in mutitrnum{
    *val = *val + 1 //multirnum dies in this loop, changes value of vec
}
println!("{:?}", vec);

//while let some(value) in mulitrnum.next()   //next returns an option some,none
//println!("{}", val)                         //next brings the cursor to next index
// let a = multirnum.next() will result and option but  after b and c, third value would be none 
//.iter is borrowing, mulitrnum is a mutable borrower
//INTO ITER: converts collection to iterator by taking ownership (lose initial variable value)
//it is used because .iter has the pointer to value but into iter has exact value

let intoiter = vec.into_iter();
println!("value of vec is gone now, into_iter has it: {:?}", intoiter);
//println!("{:?}", vec); results in error
//BY DEFAULT IT IS SIMILAR TO INTO ITERATOR


//FUNCTION IN ITERATORS
//CONSUMING ITERATor
let sumitr: i32 = intoiter.sum();  //cant put & as an input
println!("{}", sumitr);
//cant use intoitr now because .sum() consumed it


//ITERATOR ADAPTER
let vec3 = vec![1,2,3,4,5,6];
let intovec3 = vec3.iter();
let adaptitr = intovec3.filter(|x| *x % 2 != 0).map(|x| x+1);


println!("{:?}", adaptitr); //ot returns iterator item of filter and map
    let vec4: Vec<i32> = adaptitr.collect(); //thos actually makes the vector not the line above
    //.collect consumes the iterator and builds a new collection
    println!("{:?}", vec4);


//STRING VS SLICES .////////////////////////////////////////////////////////////////////  
//string is mut, growable, UTF8, owned (bytes are converted to utf8 representation)
//&str slices are reference squence of elements in a collection (no ownership as it is reference)
let mut name = String::from("Aashishkumar");
name.replace_range(5..name.len(), "_"); //replaces everything from 8th index to end of string with _
println!("{}", name);


//to return a view of the original string we use slices, so no extra data created 
//name.clear() emties the string
//String, String slice, String literal
//binary has hardcoded string literal
//for i in ans.char() borrows the string but for i in num will own the string











//GENERICS:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::;;;
let bigger = largest(1,2);
let bigger_char = largest('a','b');
println!("{},{}",bigger, bigger_char);

//TRAITS___________________________________________________________________________________________
//similar to interfaces in javascript
//trait defines functionality  of a type and can share with other types/
//trait bounds to specify that a generic type can be any type that has a certain behaviour
//trait is a blueprint that say, whatever struct uses trait should have
//traits can have a default function, so if any struct is using that trait, it can have that default function or choose to make a new one
//Display is a trait that helps in println!("{}",value)

let ser = Usering{
    name: "Harkirat".to_string(), //implements a to_string trait
    age:30
};
println!("{}",ser.summarize());
notify(ser);

//TRAITS AS PARAMETERS
notify("This trait has string".to_string());
let f = fix;
notify(f);

//notify(1) would result in an error because notify only take types that have summary implementation

//If multiple types implement the same trait, then you can treat them all the same using a single function, loop, or container — even if their actual types are different.
//To generalize a function like talk_twice() — which calls another method like speak() — you use a trait so the compiler knows:
//"Whatever type you pass in must have a speak() method.
let ddog = Dog;
let ccat = Cat;
talk_twice(ddog);
talk_twice(ccat);













//LIFETIMES_________________________________________________________________________
//you can define a value forst and then assign value: let a; a = "hi"
//{values created in a scope} dont print outside the scope 
fn main() {
    let ans;
    let str1 = String::from("small");
{let str2 = String::from("longer");}
ans = &str1;
        
    //    ans = longest(&str1,&str2);} 

    //ans is point to a slice that goes out of scope so answer would be dangling pointer
//RUSTS ASKS FOR LIFETIME OF OUTPUT TO THE INPUT
  println!("{}", ans);}
//now if str1 is longer  still compiler wont let the code run because of possibility of error


//STRUCTS WITH LIFETIMES____________________________________________________________________________-
let names = String::from("Checkthisout");
let user = Userstruct{ name: &names};
println!("{:?}", user.name); //the output has ""
//"Lifetimes are used to get error in a good format"


    let s1 = String::from("short");
    let s2 = String::from("definitely longer");

    let result = longest_with_an_announcement(&s1, &s2, "Choosing the longer string:");
    println!("{}", "hello"); // Works — "hello" is a &str so any &str is a display type

    println!("Longest is: {}", result);








//MULTITHREADING______________________________________________________________________
//thread pools in java
//multiple requests can be handled by server if multiple threads are being used

thread::spawn(|| {
    for i in 1..9{
        println!("hi numbr {i} from the spawned thread");
        thread::sleep(Duration::from_millis(1));
    }
});
//this is interweaving the threads
//if let handle = thread::spawn(|| {
//for i in 1..10{
//println!("hi numbr {i} from the spawned thread");
//thread::sleep(Duration::from_millis(1));}
//handle.join().unwrap() will first wait for all threads to run then print the result together and then move to next thread

//unless we spawn threads it runs in single thread only
//we use move keyword with closures passed to threads because it will take ownership of values it takes
//from environment thus transferring ownership of values from one thread to another
//BASICALLY assigning a value and printing it in another thread errors

let v = String::from("this is going in another thread");
thread::spawn(move||{
    println!("{}", v);
    //thread::sleep(Duration::from_millis(1))
}); 

//MESSAGE PASSING ________________________________________________________________________________
 //mpsc: multiple producer single consumer struct is used for this
 let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send(String::from("sending from thread")) //use .unwrap() if sure that it wont return error
});
let rvalue = rx.recv(); //rvalue sends a result
match rvalue{
    Ok(rvalue) => println!("{}",rvalue),
    Err(err) => println!("error found")
}

//for multiple threads use tx1 = tx.clone() and pass it
//single reciever can take all clones
//you can spawn threads using a loop but in that case
//drop(tx) cause reciever will keep on waiting for it to end even if all clones have ended
//rx is a stream of data of a particular datatype
//if threads are sending diff data then match it


//pushing it here so program doesnt end before threads being completed
for i in 1..11{ //after this program gets finished no matter the thread above is completed or not
    println!("Hi number {i} from the main thread");
    thread::sleep(Duration::from_millis(1));}



}









fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T, //can take slice, stirng, i32 etc etc
) -> &'a str
where
    T: Display,  //similar to T: Display
    {println!("Announcement! {ann}");
    if x.len() > y.len() {x} 
        else {y}
}

struct Userstruct<'a>{
    name: &'a str,
}

//GENERIC LIFETIME ANNOTATION
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str { //Similar to <T>
    //this mean return type has lifetime of intersection(make them equal)
    if a.len() > b.len() {
        return a;
    } else {
        return b;}}
//

trait Speak {
    fn speak(&self);
}
// Define two structs
struct Dog;
struct Cat;

// Implement the trait for each struct
impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}
// Generic function that works with any type that implements Speak
fn talk_twice<T: Speak>(animal: T) {
    animal.speak();
    animal.speak();}


trait Summary{
    fn summarize(&self) -> String{
            return String::from("hi there");
            //if i dont have summarize function in struct, calling summarise would use this
    }
}
struct Usering{
name: String,
age: u32,
}
struct fix;
impl Summary for fix{}
impl Summary for Usering{
    fn summarize(&self)->String{
        return format!("The name is {}, and age is {}", self.name, self.age);
    }
}
impl Summary for String{}


fn notify(u: impl Summary){ //u: &impl Summary would not have reference to it => notify(&f)
println!("{}", u.summarize()) //THIS IS SYNTACTICAL SUGAR FOR TRAIT BOUND
} //fn notify<T: Summary + newtrait>(item: T) T generic is bound to suummary
//one generic(struct) can be bounded by multiple traits


fn is_even(num: i32)-> (bool){
    //you can store more data in signed than in unsigned
    if num % 2 == 0 {
        return true;
    } else{
        return false;
    }
}

fn fib(num: i32)-> i64{
    let mut a = 0;
    let mut b= 1;

    if num == 1{
        return a;
    } if num ==2 {
        return b;
    } else {
        for n in 0..num-1 {
            let mut temp = b;
            b = b + a;
            a = temp;
        }  b //implicit return
    } 
}
//you cant define a function inside another in rust

fn largest<T: std::cmp::PartialOrd>(a:T, b:T)-> T{
    //this std::cmp(comparison) is use for comparing queries >,<,=>,=<
    //ord is in built in integers
    //NaN != NaN weird thing but yes (partialeq)
    if a>b{
        a
    } else{b}
}



fn shift(vec: Vec<(String, i32)>) -> HashMap<String, i32>{
    //since vec(arguement) is defined therefore brackets are needed and not in result
    let mut hm = HashMap::new();
    for (key,value) in vec{
        hm.insert(key,value); //the values can be mixed

    } hm
}


fn even_filter(vec: &Vec<i32>) -> Vec<&i32>{ //we have to tell what type of vector it is
let mut vect = Vec::new();
for val in vec{
    if val%2==0{
        vect.push(val);
    }
}vect
}

// fn readfile(contents: String)-> String{
//     let res = fs::read_to_string(text.txt);
//     return res.unwrap();
// } This will either crash or return the string. Use match


//explaining result
enum Result<T, E> {
    Ok(T), //the program can return a string or struct
    Er(E), //or result in error
}

//explaining structs
struct Point<T, Y> {
    x: T,
    y: T,
    z: Y,
}


/*
true
55
Hello world!
5 34 z: 23.23, xx: 430you are a male
YOu are a legal male
1
2
3
4
5
14,14,0x5b4f4ad2db10
hello aashish hello aashish ,aashish
eNo character found
This hellow again AFTER OWNERSHIP
hellow again AFTER OWNERSHIP
CHECKING IF IT WORKS? Works
Hello from immutable/mutable
Aashish,true
9230,1866
0
North
Hi fron enum function18
Error found No such file or directory (os error 2)
Current directory: Ok("/home/ash/Desktop/Coding/rust")
15Nothing foundRandom number: 3191078156
Current date and time in UTC: 2025-06-24 11:14:33.176901083 UTC
Formatted date and time: 2025-06-24 11:14:33
Current date and time in local: 2025-06-24 16:44:33.176935325 +05:30
[1, 2, 3, 4, 5, 6]
[2, 4, 6]
["true", "hello", "Third value"]
{"Avinash": 69, "Aashish": 22}
69
{"Aashish": 29, "Avinash": 69}
[2, 3, 4, 5, 6, 7]
value of vec is gone now, into_iter has it: IntoIter([2, 3, 4, 5, 6, 7])
27
Map { iter: Filter { iter: Iter([1, 2, 3, 4, 5, 6]) } }
[2, 4, 6]
Aashi_
2,b
The name is Harkirat, and age is 30
The name is Harkirat, and age is 30
hi there
hi there
Woof!
Woof!
Meow!
Meow!
"Checkthisout"
Announcement! Choosing the longer string:
hello
Longest is: definitely longer
hi numbr 1 from the spawned thread
this is going in another thread
sending from thread
Hi number 1 from the main thread
hi numbr 2 from the spawned thread
Hi number 2 from the main thread
hi numbr 3 from the spawned thread
Hi number 3 from the main thread
hi numbr 4 from the spawned thread
Hi number 4 from the main thread
hi numbr 5 from the spawned thread
Hi number 5 from the main thread
hi numbr 6 from the spawned thread
Hi number 6 from the main thread
hi numbr 7 from the spawned thread
Hi number 7 from the main thread
hi numbr 8 from the spawned thread
Hi number 8 from the main thread
Hi number 9 from the main thread
Hi number 10 from the main thread
*/


//print! is a MACRO(single line of code that expands to more code ! is macro)
//FMT in rust for debug and display use std::fmt;
//impl fmt::Display for MyType {fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {// write something into `f'}}
//let formatted = now.format("%Y-%m-%d %H:%M:%S"); dont know how this works
//RAII pattern
//Mix of datatypes in vectors
//Shared state concurrency

//tuple struct, unit struct, TRAITS(copy, clone)
//ASYNC AWAIT AND TOKIO (FOR SERVERSS) , FUTURES, MACROS
//printing rectangle with fmt
//can only fs have error that should have match code
//Rand crate for random numbers

// 1. User-defined fallbacks or alternate branches
// You might use Result<T, T> just to represent two different paths—even when both are valid:
// fn maybe_caps(word: &str) -> Result<String, String> {
//     if word.len() > 5 
//     {Ok(word.to_uppercase()) // "Success" path} 
//     else {Err(word.to_lowercase()) // "Alternate" path, not really an error } }
// Here, Err isn’t a failure—it’s just a different outcome.
// 2. Custom control flow: You might use Result to design early exits or decision points in a computation, even inside an app that isn’t “error-prone” by nature.
// 3. Testing alternate business logic: For example, parsing a config file and intentionally using Result to return a default in the Err branch. It’s not wrong—it’s just fallback logic.

//https://chatgpt.com/s/t_685a8c87b6748191a426fa9a9dd8b077