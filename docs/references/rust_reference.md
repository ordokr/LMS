
TITLE: Complete Async Web Scraper Implementation in Rust
DESCRIPTION: Full implementation of an async web scraper that races two URLs to compare their load times. Uses trpl::race to handle concurrent requests and Either type for results.

LANGUAGE: rust
CODE:
use std::env;

async fn page_title(url: &str) -> (String, Option<String>) {
    let title = trpl::get(url)
        .await
        .text()
        .await
        .pipe(|text| Html::parse(&text))
        .select_first("title")
        .map(|title| title.inner_html());
    (url.to_string(), title)
}

fn main() {
    let urls: Vec<String> = env::args().skip(1).take(2).collect();
    if urls.len() != 2 {
        eprintln!("Please provide exactly two URLs as arguments");
        std::process::exit(1);
    }

    trpl::run(async {
        let title_fut_1 = page_title(&urls[0]);
        let title_fut_2 = page_title(&urls[1]);

        match trpl::race(title_fut_1, title_fut_2).await {
            trpl::Either::Left((url, maybe_title)) => {
                println!("{} finished first!", url);
                match maybe_title {
                    Some(title) => println!("Its title is: {}", title),
                    None => println!("It had no title"),
                }
            }
            trpl::Either::Right((url, maybe_title)) => {
                println!("{} finished first!", url);
                match maybe_title {
                    Some(title) => println!("Its title is: {}", title),
                    None => println!("It had no title"),
                }
            }
        }
    });
}

----------------------------------------

TITLE: Comparing User Guess with Secret Number
DESCRIPTION: Implements comparison logic to check if the user's guess matches the secret number.

LANGUAGE: rust
CODE:
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

----------------------------------------

TITLE: Random Number Generation
DESCRIPTION: Code that generates a random secret number for the game

LANGUAGE: rust
CODE:
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..=100);
println!("The secret number is: {secret_number}");

----------------------------------------

TITLE: Cargo Check Command with ThreadPool Execute Error
DESCRIPTION: Terminal output showing a compilation error when running cargo check. The error indicates an attempt to use an undefined execute method on a ThreadPool struct at line 17 of main.rs.

LANGUAGE: shell
CODE:
$ cargo check
    Checking hello v0.1.0 (file:///projects/hello)
error[E0599]: no method named `execute` found for struct `ThreadPool` in the current scope
  --> src/main.rs:17:14
   |
17 |         pool.execute(|| {
   |         -----^^^^^^^ method not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello` (bin "hello") due to 1 previous error

----------------------------------------

TITLE: Basic URL Fetching with Async/Await in Rust
DESCRIPTION: Defines an async function that fetches a webpage's title element using async/await syntax. The function makes an HTTP request, awaits the response, and extracts the title using HTML parsing.

LANGUAGE: rust
CODE:
async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

----------------------------------------

TITLE: Cargo.toml Dependencies
DESCRIPTION: Project configuration file showing required external dependencies

LANGUAGE: toml
CODE:
[dependencies]
rand = "0.8.5"

----------------------------------------

TITLE: Main Binary Implementation
DESCRIPTION: Main program using the add_one library function from within the workspace.

LANGUAGE: rust
CODE:
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}

----------------------------------------

TITLE: Initial Hello World Program in Rust
DESCRIPTION: The default main.rs file created by Cargo, containing a simple 'Hello, world!' program.

LANGUAGE: rust
CODE:
fn main() {
    println!("Hello, world!");
}

----------------------------------------

TITLE: Converting String Input to Number in Rust
DESCRIPTION: Converts the user's string input to a number for comparison with the secret number.

LANGUAGE: rust
CODE:
let guess: u32 = guess.trim().parse().expect("Please type a number!");

----------------------------------------

TITLE: Compiling Rust Project with Pattern Matching Error
DESCRIPTION: This snippet shows the output of running `cargo run` on a Rust project named 'patterns'. The compilation fails due to an error in the pattern matching syntax used in the main.rs file.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error: `..` can only be used once per tuple pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |          --          ^^ can only be used once per tuple pattern
  |          |
  |          previously used here

error: could not compile `patterns` (bin "patterns") due to 1 previous error

----------------------------------------

TITLE: Using if let to produce a value or return early in Rust
DESCRIPTION: Shows how to use if let to either produce a value or return early from a function.

LANGUAGE: rust
CODE:
fn describe(coin: Coin) {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        println!("That's not a quarter!");
        return;
    };

    if state.existed_in_1900() {
        println!("That's an old quarter from {:?}!", state);
    } else {
        println!("That's a quarter from {:?}!", state);
    }
}

----------------------------------------

TITLE: Library with Test Implementation
DESCRIPTION: Library implementation including a unit test for the add_one function.

LANGUAGE: rust
CODE:
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}

----------------------------------------

TITLE: Adding String Matching in Rust Search Function
DESCRIPTION: Implementation of string matching using the contains() method to check if lines contain the query string.

LANGUAGE: rust
CODE:
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
        }
    }
    vec![]
}

----------------------------------------

TITLE: Using Raw Identifier for Keyword Function Name in Rust
DESCRIPTION: This code snippet shows how to use a raw identifier (r#match) to define a function with a name that would otherwise be a reserved keyword. It also demonstrates how to call this function using the raw identifier syntax.

LANGUAGE: rust
CODE:
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}

----------------------------------------

TITLE: Compiling Rust Function with Incorrect Return Type
DESCRIPTION: This snippet shows a Rust function 'plus_one' that attempts to add 1 to an integer parameter. However, the function has a compilation error due to a mismatched return type. The semicolon at the end of the expression causes it to return '()' (unit type) instead of the expected 'i32'.

LANGUAGE: rust
CODE:
fn plus_one(x: i32) -> i32 {
    x + 1;
}

----------------------------------------

TITLE: Executing Cargo Tests with Failed Test Output
DESCRIPTION: Terminal output showing the execution of cargo test command. The test failed due to a BorrowMutError in the 'it_sends_an_over_75_percent_warning_message' test case at line 60 of src/lib.rs.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling limit-tracker v0.1.0 (file:///projects/limit-tracker)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running unittests src/lib.rs (target/debug/deps/limit_tracker-e599811fa246dbde)

running 1 test
test tests::it_sends_an_over_75_percent_warning_message ... FAILED

failures:

---- tests::it_sends_an_over_75_percent_warning_message stdout ----

thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at src/lib.rs:60:53:
already borrowed: BorrowMutError
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_sends_an_over_75_percent_warning_message

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Rust Type Mismatch Error in String Comparison
DESCRIPTION: Cargo build output showing a compilation error where a String type is incorrectly compared with an integer type. The error occurs in main.rs line 22 where guess.cmp() expects a &String but receives &integer.

LANGUAGE: text
CODE:
$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
   --> src/main.rs:22:21
    |
22  |     match guess.cmp(&secret_number) {
    |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
    |                 |
    |                 arguments to this method are incorrect
    |
    = note: expected reference `&String`
               found reference `&{integer}`
note: method defined here
   --> file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/cmp.rs:964:8
    |
964 |     fn cmp(&self, other: &Self) -> Ordering;
    |        ^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` (bin "guessing_game") due to 1 previous error

----------------------------------------

TITLE: Installing mdBook for Rust Book Building
DESCRIPTION: Command to install mdBook, a tool required for building the Rust book. The version should match the one used in the rust-lang/rust repository.

LANGUAGE: bash
CODE:
$ cargo install mdbook --locked --version <version_num>

----------------------------------------

TITLE: Running cargo check on Rust project
DESCRIPTION: Executes cargo check command to verify if a Rust project compiles correctly without producing an executable. Shows successful validation of project 'hello v0.1.0' with debug information.

LANGUAGE: shell
CODE:
$ cargo check
    Checking hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s

----------------------------------------

TITLE: Implementing Error Handling for Config Creation in Rust
DESCRIPTION: Updates Config::new to return a Result, allowing for proper error handling when creating a Config instance.

LANGUAGE: rust
CODE:
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

----------------------------------------

TITLE: Installing mdBook Plugins for Rust Book
DESCRIPTION: Command to install custom mdBook plugins used in the Rust book build process. These plugins are part of the book repository.

LANGUAGE: bash
CODE:
$ cargo install --locked --path packages/mdbook-trpl

----------------------------------------

TITLE: Implementing the Deref Trait for a Custom Box Type
DESCRIPTION: Demonstrates how to implement the Deref trait for a custom smart pointer type, allowing it to be used like a reference.

LANGUAGE: rust
CODE:
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

----------------------------------------

TITLE: Executing Cargo Tests
DESCRIPTION: Shell command to run tests in a Rust project using cargo test, which compiles and executes all tests including unit tests and doc-tests.

LANGUAGE: shell
CODE:
$ cargo test

----------------------------------------

TITLE: Initial Guessing Game Code
DESCRIPTION: Basic code structure that gets user input and prints it back

LANGUAGE: rust
CODE:
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

----------------------------------------

TITLE: Thread Join Error Example
DESCRIPTION: Code snippet from lib.rs showing an ownership error when attempting to join a thread. The error occurs because JoinHandle::join takes ownership of self, but worker.thread is behind a mutable reference.

LANGUAGE: rust
CODE:
worker.thread.join().unwrap();

----------------------------------------

TITLE: Cargo Check Error Output - ThreadPool Resolution Failure
DESCRIPTION: Command line output showing a compilation error when attempting to use an undeclared ThreadPool type. The error occurs in main.rs line 11 where ThreadPool::new(4) is called without the type being properly imported or defined.

LANGUAGE: shell
CODE:
$ cargo check
    Checking hello v0.1.0 (file:///projects/hello)
error[E0433]: failed to resolve: use of undeclared type `ThreadPool`
  --> src/main.rs:11:16
   |
11 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^ use of undeclared type `ThreadPool`

For more information about this error, try `rustc --explain E0433`.
error: could not compile `hello` (bin "hello") due to 1 previous error

----------------------------------------

TITLE: Using Type Aliases in Rust Functions
DESCRIPTION: Shows how type aliases can be used in function signatures and variable declarations to reduce repetition and improve code readability.

LANGUAGE: rust
CODE:
let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);

----------------------------------------

TITLE: Using a Keyword as Function Name (Invalid Rust)
DESCRIPTION: This code snippet demonstrates an invalid attempt to use the 'match' keyword as a function name, which results in a compilation error.

LANGUAGE: rust
CODE:
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

----------------------------------------

TITLE: Configuring Rust Edition in Cargo.toml
DESCRIPTION: Configuration setting in Cargo.toml to specify the use of Rust 2024 edition features.

LANGUAGE: toml
CODE:
edition = "2024"

----------------------------------------

TITLE: Suggested Fix for Trait Method Call in Rust
DESCRIPTION: This code snippet shows the compiler's suggested fix for the error. It uses a fully-qualified path to specify the 'Dog' implementation of the 'Animal' trait when calling the 'baby_name' function.

LANGUAGE: rust
CODE:
println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

----------------------------------------

TITLE: Extracting Config Parser in Rust
DESCRIPTION: Shows how to extract argument parsing logic into a separate parse_config function to improve code organization

LANGUAGE: rust
CODE:
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
}

----------------------------------------

TITLE: Running Minigrep via Cargo
DESCRIPTION: Command line execution of the minigrep program using cargo run to search for 'the' in poem.txt. Shows compilation output and search results including the full text of the poem being searched.

LANGUAGE: shell
CODE:
$ cargo run -- the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

----------------------------------------

TITLE: Improved Config::build Using Iterator Methods
DESCRIPTION: Shows the refactored Config::build implementation that uses iterator methods instead of indexing, eliminating the need for clone().

LANGUAGE: rust
CODE:
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config { query, file_path })
    }
}

----------------------------------------

TITLE: Struct Constructor Function
DESCRIPTION: Implements a constructor function that creates and returns a User struct instance.

LANGUAGE: rust
CODE:
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

----------------------------------------

TITLE: Reading HTTP Requests in Rust
DESCRIPTION: Implements a handle_connection function that reads data from a TCP stream and prints the HTTP request details.

LANGUAGE: rust
CODE:
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}

----------------------------------------

TITLE: HTTP Request Reader Implementation
DESCRIPTION: Reads and parses incoming HTTP requests from TCP stream using BufReader.

LANGUAGE: rust
CODE:
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}

----------------------------------------

TITLE: Calling Trait Method Without Implementation Specification in Rust
DESCRIPTION: This code snippet demonstrates an error that occurs when calling an associated function 'baby_name' on the 'Animal' trait without specifying which implementation to use. The compiler suggests using a fully-qualified path to resolve the ambiguity.

LANGUAGE: rust
CODE:
println!("A baby dog is called a {}", Animal::baby_name());

----------------------------------------

TITLE: Running Rust Program with Command Line Args
DESCRIPTION: Terminal command showing compilation and execution of a Rust program called minigrep. The program takes 'body' and 'poem.txt' as command line arguments and outputs matching lines from the text file.

LANGUAGE: shell
CODE:
$ cargo run -- body poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep body poem.txt`
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!

----------------------------------------

TITLE: Generic Largest Function Implementation
DESCRIPTION: Generic function implementation to find the largest value in a slice of any type that implements PartialOrd

LANGUAGE: rust
CODE:
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

----------------------------------------

TITLE: Compiling Rust Project with Comparison Error
DESCRIPTION: This snippet shows the output of running 'cargo run' on a Rust project named 'deref-example'. The compilation fails due to a type mismatch in an assert_eq! macro, where an integer is being compared with a reference to an integer.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider dereferencing here
 --> file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/macros/mod.rs:46:35
  |
46|                 if !(*left_val == **right_val) {
  |                                   +

For more information about this error, try `rustc --explain E0277`.
error: could not compile `deref-example` (bin "deref-example") due to 1 previous error

----------------------------------------

TITLE: Creating Add One Library Function
DESCRIPTION: Implementation of a simple add_one function in a library crate within the workspace.

LANGUAGE: rust
CODE:
pub fn add_one(x: i32) -> i32 {
    x + 1
}

----------------------------------------

TITLE: Cargo Build Error Output - Type Mismatch in Rust
DESCRIPTION: Console output from a failed cargo build showing a type mismatch error where a number is used in place of a boolean condition. The error occurs in src/main.rs on line 4.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` (bin "branches") due to 1 previous error

----------------------------------------

TITLE: Sending Multiple Messages Through a Channel in Rust
DESCRIPTION: Shows how to send multiple messages through a channel with pauses between each send operation.

LANGUAGE: rust
CODE:
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

----------------------------------------

TITLE: Creating Basic Threads with spawn
DESCRIPTION: Demonstrates how to create a new thread using thread::spawn and how the main thread and spawned thread run concurrently.

LANGUAGE: rust
CODE:
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

----------------------------------------

TITLE: Collecting command line arguments in Rust
DESCRIPTION: Implements functionality to read and collect command line arguments into a vector using std::env::args.

LANGUAGE: rust
CODE:
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

----------------------------------------

TITLE: Executing Cargo Tests in Shell
DESCRIPTION: Running the cargo test command to execute all tests in a Rust project, including unit tests, integration tests, and documentation tests.

LANGUAGE: shell
CODE:
$ cargo test

----------------------------------------

TITLE: Implementing the search Function in Rust
DESCRIPTION: Implements a search function that finds lines containing a query string in given contents.

LANGUAGE: rust
CODE:
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

----------------------------------------

TITLE: Validating Requests and Sending Different Responses in Rust
DESCRIPTION: Modifies the handle_connection function to check the request path and respond with different content based on the request.

LANGUAGE: rust
CODE:
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
    }
}

----------------------------------------

TITLE: Rust Debug Trait Implementation Error
DESCRIPTION: Demonstrates a compilation error when trying to print a Rectangle struct using the debug format specifier {:?} without implementing the Debug trait. The compiler suggests adding #[derive(Debug)] annotation to fix the issue.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
error[E0277]: `Rectangle` doesn't implement `Debug`
  --> src/main.rs:12:31
   |
12 |     println!("rect1 is {:?}", rect1);
   |                               ^^^^^ `Rectangle` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Rectangle` with `#[derive(Debug)]`
   |
1  + #[derive(Debug)]
2  | struct Rectangle {
   |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rectangles` (bin "rectangles") due to 1 previous error

----------------------------------------

TITLE: Listening for TCP Connections in Rust
DESCRIPTION: Creates a TcpListener to listen for incoming TCP streams on localhost port 7878. It prints a message for each established connection.

LANGUAGE: rust
CODE:
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

----------------------------------------

TITLE: Compiling and Running Rust Program with String Indexing Error
DESCRIPTION: This snippet demonstrates the compilation and execution of a Rust program that encounters a runtime error due to incorrect indexing of a non-ASCII string. The error occurs when trying to access a byte index that is not a valid character boundary in the UTF-8 encoded string 'Здравствуйте'.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/collections`

thread 'main' panicked at src/main.rs:4:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

----------------------------------------

TITLE: Sending a Message Through a Channel in Rust
DESCRIPTION: Shows how to move the transmitter to a spawned thread and send a message through the channel.

LANGUAGE: rust
CODE:
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}

----------------------------------------

TITLE: Failed Thread Closure in Rust
DESCRIPTION: Example showing an E0373 error where a closure attempts to use a borrowed value across thread boundaries. The code fails because the closure needs to take ownership of the variable 'v' using the move keyword.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling threads v0.1.0 (file:///projects/threads)
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {v:?}");
  |                                     - `v` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:6:18
  |
6 |       let handle = thread::spawn(|| {
  |  __________________^
7 | |         println!("Here's a vector: {v:?}");
8 | |     });
  | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++

For more information about this error, try `rustc --explain E0373`.
error: could not compile `threads` (bin "threads") due to 1 previous error

----------------------------------------

TITLE: Generating a Random Number in Rust
DESCRIPTION: Uses the rand crate to generate a random number between 1 and 100.

LANGUAGE: rust
CODE:
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

----------------------------------------

TITLE: Using Raw Identifier for Keyword Function Name (Valid Rust)
DESCRIPTION: This code snippet shows how to use the raw identifier syntax (r#) to use the 'match' keyword as a valid function name, along with its usage in the main function.

LANGUAGE: rust
CODE:
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}

----------------------------------------

TITLE: Cargo Build Error Output for Message Passing
DESCRIPTION: Shows a compilation error that occurs when trying to use a String value after sending it through a channel. The error indicates violation of Rust's ownership rules since the value was moved during send operation.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling message-passing v0.1.0 (file:///projects/message-passing)
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:10:26
   |
8  |         let val = String::from("hi");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {val}");
   |                          ^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `message-passing` (bin "message-passing") due to 1 previous error

----------------------------------------

TITLE: Using Debug Trait with Format Strings in Rust
DESCRIPTION: The Debug trait enables debug formatting in format strings using :? within {} placeholders. It's commonly used with assert_eq! macro for debugging and comparing values.

LANGUAGE: rust
CODE:
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let point = Point { x: 0, y: 0 };
    println!("{:?}", point);
}

----------------------------------------

TITLE: Complete Guessing Game Implementation
DESCRIPTION: Final version of the guessing game with full gameplay logic including loops and error handling

LANGUAGE: rust
CODE:
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

----------------------------------------

TITLE: Executing Cargo Test Command
DESCRIPTION: Running unit tests using the Cargo test command, showing compilation and test execution output including a failed assertion in the larger_can_hold_smaller test.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)

running 2 tests
test tests::larger_can_hold_smaller ... FAILED
test tests::smaller_cannot_hold_larger ... ok

failures:

---- tests::larger_can_hold_smaller stdout ----

thread 'tests::larger_can_hold_smaller' panicked at src/lib.rs:28:9:
assertion failed: larger.can_hold(&smaller)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Executing Rust Program with Cargo
DESCRIPTION: Command line output showing the compilation and execution of a Rust program named minigrep. The program takes two arguments: 'test' as the search string and 'sample.txt' as the input file.

LANGUAGE: shell
CODE:
$ cargo run -- test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt

----------------------------------------

TITLE: Compiling Rust Program with Thread-Safety Error
DESCRIPTION: This snippet shows the compilation error when trying to use Rc<Mutex<i32>> in a spawned thread. The error occurs because Rc is not thread-safe and cannot be sent between threads.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling shared-state v0.1.0 (file:///projects/shared-state)
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   --> src/main.rs:11:36
    |
11  |           let handle = thread::spawn(move || {
    |                        ------------- ^------
    |                        |             |
    |  ______________________|_____________within this `{closure@src/main.rs:11:36: 11:43}`
    | |                      |
    | |                      required by a bound introduced by this call
12  | |             let mut num = counter.lock().unwrap();
13  | |
14  | |             *num += 1;
15  | |         });
    | |_________^ `Rc<Mutex<i32>>` cannot be sent between threads safely
    |
    = help: within `{closure@src/main.rs:11:36: 11:43}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
note: required because it's used within this closure
   --> src/main.rs:11:36
    |
11  |         let handle = thread::spawn(move || {
    |                                    ^^^^^^^
note: required by a bound in `spawn`
   --> file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/std/src/thread/mod.rs:731:8
    |
728 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    |        ----- required by a bound in this function
...
731 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `shared-state` (bin "shared-state") due to 1 previous error

----------------------------------------

TITLE: Executing Rust Program with Cargo
DESCRIPTION: Shows the command line output when compiling and running a Rust program named 'functions' using Cargo. The output includes compilation status, build profile information, and program execution result displaying a value of x.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.21s
     Running `target/debug/functions`
The value of x is: 5

----------------------------------------

TITLE: Listing Installed Rust Toolchains
DESCRIPTION: Command to display all installed Rust toolchains including stable, beta, and nightly versions.

LANGUAGE: powershell
CODE:
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc

----------------------------------------

TITLE: Executing Cargo Test Command
DESCRIPTION: Running unit tests for a Rust guessing game project using the cargo test command. The output shows a failed test case 'greater_than_100' that was expected to panic but did not.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.62s
     Running unittests src/lib.rs (target/debug/deps/guessing_game-57d70c3acb738f4d)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Basic Rust Program
DESCRIPTION: Default Hello World program generated by Cargo

LANGUAGE: rust
CODE:
fn main() {
    println!("Hello, world!");
}

----------------------------------------

TITLE: Cargo Build Error Output for Private Function Access
DESCRIPTION: Shows compilation errors when attempting to access the private function 'add_to_waitlist' both through fully qualified and shortened path syntax. The error indicates the function is private and cannot be accessed from the current scope.

LANGUAGE: shell
CODE:
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:10:37
   |
10 |     crate::front_of_house::hosting::add_to_waitlist();
   |                                     ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:13:30
   |
13 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` (lib) due to 2 previous errors

----------------------------------------

TITLE: Writing HTTP Responses in Rust
DESCRIPTION: Modifies the handle_connection function to write a simple HTTP response back to the client.

LANGUAGE: rust
CODE:
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}

----------------------------------------

TITLE: Rust Compilation Error - Invalid Reference Return
DESCRIPTION: A compilation error showing how Rust prevents returning references to local variables due to lifetime rules. The error occurs because the code attempts to return a reference to 'result' which will be deallocated when the function ends.

LANGUAGE: console
CODE:
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here

For more information about this error, try `rustc --explain E0515`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error

----------------------------------------

TITLE: Non-exhaustive Option Pattern Matching in Rust
DESCRIPTION: Shows incomplete pattern matching on an Option<i32> that fails to handle the None case. The compiler error indicates that the match expression needs to cover all possible variants of the Option enum.

LANGUAGE: shell
CODE:
$ cargo run

LANGUAGE: rust
CODE:
match x {
    Some(i) => Some(i + 1),
}

----------------------------------------

TITLE: Rust Error Handling with eprintln
DESCRIPTION: Implementation showing how to properly write error messages to stderr using the eprintln! macro instead of println! for error handling.

LANGUAGE: rust
CODE:
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}

----------------------------------------

TITLE: Running Rust Program with CLI Arguments
DESCRIPTION: Demonstrates executing a Rust program named minigrep using Cargo, passing 'needle' and 'haystack' as command line arguments. Shows both the compilation output and the debug-printed arguments array.

LANGUAGE: shell
CODE:
$ cargo run -- needle haystack
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/minigrep needle haystack`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
]

----------------------------------------

TITLE: Running Tests for the Rust Book
DESCRIPTION: Commands to run the tests for the Rust book using mdBook test command.

LANGUAGE: bash
CODE:
$ cd packages/trpl
$ mdbook test --library-path packages/trpl/target/debug/deps

----------------------------------------

TITLE: Creating Initial Search Test in Rust
DESCRIPTION: Test setup for a search function that finds matching lines in text content based on a query string.

LANGUAGE: rust
CODE:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

----------------------------------------

TITLE: Running Rust Project with Cargo
DESCRIPTION: This snippet shows the process of compiling and running a Rust project named 'minigrep' using Cargo. It demonstrates the output of the 'cargo run' command, including compilation status, execution, and an error message due to insufficient arguments provided to the program.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments

----------------------------------------

TITLE: Building Rust Project with Missing Type Annotation
DESCRIPTION: Terminal output showing a cargo build command failing due to missing type annotations in the code. The error occurs when trying to parse a string without specifying the target type.

LANGUAGE: shell
CODE:
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `guess` an explicit type
  |
2 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
  |              ++++++++++++

For more information about this error, try `rustc --explain E0284`.
error: could not compile `no_type_annotations` (bin "no_type_annotations") due to 1 previous error

----------------------------------------

TITLE: Executing Cargo Run Command with Error in Rust Project
DESCRIPTION: This snippet demonstrates an attempt to run a Rust project using the 'cargo run' command, resulting in an error due to the absence of a binary target. The error message suggests that the project structure may be incomplete or incorrectly configured.

LANGUAGE: Shell
CODE:
$ cargo run
error: a bin target must be available for `cargo run`

----------------------------------------

TITLE: Defining Recursive Enum in Rust
DESCRIPTION: This code defines a recursive enum 'List' with a 'Cons' variant that contains an i32 and another List. The compilation fails due to the recursive type having infinite size and cycle detection when computing drop behavior.

LANGUAGE: rust
CODE:
enum List {
    Cons(i32, List),
}

----------------------------------------

TITLE: Executing Rust Program with Cargo
DESCRIPTION: Demonstrates running a Rust program named 'functions' using Cargo. Shows the compilation process, build completion with debug information, and program execution displaying a value of x.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/functions`
The value of x is: 5

----------------------------------------

TITLE: Complete Search Function Implementation in Rust
DESCRIPTION: Final implementation of the search function that stores and returns matching lines.

LANGUAGE: rust
CODE:
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

----------------------------------------

TITLE: Executing Cargo Test Command
DESCRIPTION: Shell command to run tests in a Rust project using Cargo's test runner. Displays compilation and test execution results for unit tests and doc-tests.

LANGUAGE: shell
CODE:
$ cargo test

----------------------------------------

TITLE: Demonstrating Refutable Pattern Error in Rust
DESCRIPTION: This code snippet attempts to use a Some(x) pattern in a let binding, which is not allowed as it's a refutable pattern. The compiler suggests using 'let else' to handle the None case.

LANGUAGE: rust
CODE:
let Some(x) = some_option_value;

----------------------------------------

TITLE: Using Fully Qualified Syntax for Method Disambiguation in Rust
DESCRIPTION: This code demonstrates how to use fully qualified syntax in Rust to disambiguate between methods with the same name from different traits or implementations.

LANGUAGE: rust
CODE:
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

----------------------------------------

TITLE: Error Handling with Result in Rust
DESCRIPTION: Shows implementation of proper error handling using Result instead of panic

LANGUAGE: rust
CODE:
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

----------------------------------------

TITLE: Opening Rust Documentation via Rustup
DESCRIPTION: Command to open the Rust book documentation using rustup CLI tool.

LANGUAGE: shell
CODE:
rustup doc --book

----------------------------------------

TITLE: Using Debug Trait for Formatting in Rust
DESCRIPTION: The Debug trait enables debug formatting in format strings, indicated by adding ':?' within '{}' placeholders. It's used for debugging and is required for macros like assert_eq!.

LANGUAGE: rust
CODE:
println!("{:?}", some_value);

----------------------------------------

TITLE: Compiling and Running Rust minigrep Project
DESCRIPTION: This snippet shows the process of compiling and running a Rust project called minigrep. It includes a compilation warning about an unused Result and the execution of the compiled binary with command-line arguments.

LANGUAGE: plaintext
CODE:
$ cargo run -- the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
warning: unused `Result` that must be used
  --> src/main.rs:19:5
   |
19 |     run(config);
   |     ^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
19 |     let _ = run(config);
   |     +++++++

warning: `minigrep` (bin "minigrep") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/minigrep the poem.txt`

----------------------------------------

TITLE: Building the Rust Book with mdBook
DESCRIPTION: Command to build the Rust book using mdBook. The output will be generated in the 'book' subdirectory.

LANGUAGE: bash
CODE:
$ mdbook build

----------------------------------------

TITLE: Defining Trait for Common Behavior
DESCRIPTION: Implementation of a Draw trait to demonstrate polymorphism through trait objects in Rust.

LANGUAGE: rust
CODE:
pub trait Draw {
    fn draw(&self);
}

----------------------------------------

TITLE: Implementing Default Trait in Rust
DESCRIPTION: The Default trait allows creation of default values for a type. It's commonly used with struct update syntax and in methods like unwrap_or_default for Option<T> instances.

LANGUAGE: rust
CODE:
#[derive(Default)]
struct SomeStruct {
    // fields
}

----------------------------------------

TITLE: Default Trait Implementation in Rust
DESCRIPTION: Shows how to provide a default implementation for a trait method that can be used by implementing types.

LANGUAGE: rust
CODE:
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

----------------------------------------

TITLE: Incorrect String Indexing in Rust
DESCRIPTION: This code snippet demonstrates an incorrect attempt to index into a Rust String using integer indexing. It results in a compilation error because Rust strings don't support direct integer indexing due to their UTF-8 encoding.

LANGUAGE: rust
CODE:
let h = s1[0];

----------------------------------------

TITLE: Compiling Rust Program with Type Mismatch Error
DESCRIPTION: This snippet shows the output of attempting to compile a Rust program with a type mismatch in an if-else statement. The compiler reports an E0308 error, indicating incompatible types between the if and else branches.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` (bin "branches") due to 1 previous error

----------------------------------------

TITLE: Executing Rust Program with Command Line Arguments
DESCRIPTION: Terminal command and output demonstrating the compilation and execution of a Rust program called minigrep. The program is run with 'to' and 'poem.txt' as command line arguments and outputs matching lines from the poem.

LANGUAGE: shell
CODE:
$ cargo run -- to poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!

----------------------------------------

TITLE: Updated Main Function Using Iterator
DESCRIPTION: Shows how to modify the main function to pass an iterator directly to Config::build instead of collecting arguments into a vector first.

LANGUAGE: rust
CODE:
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}

----------------------------------------

TITLE: Shell Command - Cargo Build Attempt
DESCRIPTION: Executing cargo run command to compile and run a Rust project, resulting in a compilation error

LANGUAGE: shell
CODE:
$ cargo run

----------------------------------------

TITLE: Compiling Rust Project with Unsafe Function Call Error
DESCRIPTION: This snippet shows the output of running 'cargo run' on a Rust project that incorrectly calls an unsafe function. The compiler generates an error message explaining that the 'dangerous' function call requires an unsafe block.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
error[E0133]: call to unsafe function `dangerous` is unsafe and requires unsafe block
 --> src/main.rs:4:5
  |
4 |     dangerous();
  |     ^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `unsafe-example` (bin "unsafe-example") due to 1 previous error

----------------------------------------

TITLE: Creating ThreadPool Instance in Rust
DESCRIPTION: This code snippet attempts to create a new ThreadPool instance with a size of 4. However, it results in a compilation error due to a missing 'new' function.

LANGUAGE: rust
CODE:
let pool = ThreadPool::new(4);

----------------------------------------

TITLE: Adding the rand Crate as a Dependency
DESCRIPTION: Adds the rand crate to the Cargo.toml file to enable random number generation.

LANGUAGE: toml
CODE:
[dependencies]
rand = "0.8.4"

----------------------------------------

TITLE: Setting Project-Specific Rust Toolchain
DESCRIPTION: Command to override the default Rust toolchain with nightly for a specific project directory.

LANGUAGE: console
CODE:
$ cd ~/projects/needs-nightly
$ rustup override set nightly

----------------------------------------

TITLE: Quitting Game After Correct Guess in Rust
DESCRIPTION: Adds a break statement to exit the game loop when the correct number is guessed.

LANGUAGE: rust
CODE:
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}

----------------------------------------

TITLE: Poll Enum Definition in Rust
DESCRIPTION: Definition of the Poll enum used by futures to indicate completion status

LANGUAGE: rust
CODE:
enum Poll<T> {
    Ready(T),
    Pending,
}

----------------------------------------

TITLE: Config Struct Implementation in Rust
DESCRIPTION: Demonstrates refactoring to use a Config struct for better organization of configuration values

LANGUAGE: rust
CODE:
struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

----------------------------------------

TITLE: Implementing OutlinePrint Trait for Point Struct in Rust
DESCRIPTION: This code attempts to implement the OutlinePrint trait for a Point struct. The compilation fails because Point doesn't implement the std::fmt::Display trait, which is a requirement for OutlinePrint.

LANGUAGE: rust
CODE:
impl OutlinePrint for Point {}

----------------------------------------

TITLE: Implementing Line Iteration in Rust Search Function
DESCRIPTION: Adding line iteration functionality to the search implementation using the lines() method.

LANGUAGE: rust
CODE:
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
    }
    vec![]
}

----------------------------------------

TITLE: Rust Function with Missing Lifetime Specifier
DESCRIPTION: Example of a function signature that fails to compile due to missing lifetime parameters. The function attempts to return a borrowed string slice without specifying its lifetime relationship to the input parameters.

LANGUAGE: rust
CODE:
fn longest(x: &str, y: &str) -> &str {

----------------------------------------

TITLE: Compiling Rust Project with Trait Bound Error
DESCRIPTION: This snippet shows the output of a failed Rust compilation due to a trait bound error. The `String` type does not implement the `Draw` trait, which is required for the `components` vector.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling gui v0.1.0 (file:///projects/gui)
error[E0277]: the trait bound `String: Draw` is not satisfied
 --> src/main.rs:5:26
  |
5 |         components: vec![Box::new(String::from("Hi"))],
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
  |
  = help: the trait `Draw` is implemented for `Button`
  = note: required for the cast from `Box<String>` to `Box<dyn Draw>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `gui` (bin "gui") due to 1 previous error

----------------------------------------

TITLE: Executing Rust Integration Tests with Cargo
DESCRIPTION: This command runs a specific integration test file for a Rust project using Cargo. It compiles the project, executes the test, and displays the results.

LANGUAGE: shell
CODE:
$ cargo test --test integration_test

----------------------------------------

TITLE: Using let...else for clearer control flow in Rust
DESCRIPTION: Demonstrates the let...else syntax for improved readability and control flow in pattern matching scenarios.

LANGUAGE: rust
CODE:
fn describe(coin: Coin) {
    let Coin::Quarter(state) = coin else {
        println!("That's not a quarter!");
        return;
    };

    if state.existed_in_1900() {
        println!("That's an old quarter from {:?}!", state);
    } else {
        println!("That's a quarter from {:?}!", state);
    }
}

----------------------------------------

TITLE: Executing Rust Program with Cargo
DESCRIPTION: Shows the command line execution of a Rust program using Cargo build system, including compilation output and program execution results. The program appears to be a simple example that prints a message about growing Asparagus.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling backyard v0.1.0 (file:///projects/backyard)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/backyard`
I'm growing Asparagus!

----------------------------------------

TITLE: Creating a Type Alias in Rust
DESCRIPTION: Demonstrates how to create a type alias 'Kilometers' for the i32 type using the 'type' keyword. This allows for creating synonyms of existing types.

LANGUAGE: rust
CODE:
type Kilometers = i32;

----------------------------------------

TITLE: Using the Newtype Pattern in Rust
DESCRIPTION: This code demonstrates the newtype pattern in Rust, which allows implementing external traits on external types by creating a wrapper type.

LANGUAGE: rust
CODE:
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("{}", w);
}

----------------------------------------

TITLE: Running Cargo with Ownership Error
DESCRIPTION: Shell command showing compilation error when trying to use a moved String value. The error occurs because String does not implement the Copy trait, and ownership is moved from s1 to s2.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:15
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1}, world!");
  |               ^^^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

----------------------------------------

TITLE: Updating Rust
DESCRIPTION: This command updates Rust to the latest version using rustup. It should be run periodically to keep Rust up-to-date.

LANGUAGE: console
CODE:
$ rustup update

----------------------------------------

TITLE: Handling Invalid Input in Rust Guessing Game
DESCRIPTION: Uses a match expression to handle invalid input, allowing the game to continue instead of crashing.

LANGUAGE: rust
CODE:
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

----------------------------------------

TITLE: Basic Cargo.toml File for a Rust Project
DESCRIPTION: Shows the contents of the Cargo.toml file, which defines project metadata and dependencies.

LANGUAGE: toml
CODE:
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]

----------------------------------------

TITLE: Corrected Function Definition With Lifetime Parameters
DESCRIPTION: The corrected version of the search function that properly specifies lifetime parameters for borrowed references. The 'a lifetime parameter indicates that all references share the same lifetime.

LANGUAGE: rust
CODE:
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {

----------------------------------------

TITLE: Using Function Pointers as Arguments in Rust
DESCRIPTION: Demonstrates how to use the fn type to accept function pointers as arguments. Shows implementation of add_one and do_twice functions where do_twice takes a function pointer and executes it twice on a given argument.

LANGUAGE: rust
CODE:
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

----------------------------------------

TITLE: Incorrect Explicit Destructor Call in Rust
DESCRIPTION: This snippet shows an incorrect attempt to explicitly call the `drop()` method on an object `c`. In Rust, explicit destructor calls are not allowed. The compiler suggests using the `drop` function instead.

LANGUAGE: rust
CODE:
c.drop();

----------------------------------------

TITLE: Nested conditionals with if let in Rust
DESCRIPTION: Demonstrates using if let with nested conditionals to check coin type and state age.

LANGUAGE: rust
CODE:
fn describe(coin: Coin) {
    if let Coin::Quarter(state) = coin {
        if state.existed_in_1900() {
            println!("That's an old quarter from {:?}!", state);
        } else {
            println!("That's a quarter from {:?}!", state);
        }
    } else {
        println!("That's not a quarter!");
    }
}

----------------------------------------

TITLE: Using AsRef Trait for String Reference Conversion in Rust
DESCRIPTION: Demonstrates how to use the AsRef trait to create a generic function that can accept any type that can be converted to a string reference. The function compares the input against the literal "hello" after converting it to a string reference.

LANGUAGE: rust
CODE:
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}

----------------------------------------

TITLE: Implementing PartialEq and Eq for Equality Comparisons in Rust
DESCRIPTION: The PartialEq trait enables equality comparisons with == and != operators. Eq trait indicates that a value is equal to itself. Both are commonly used with HashMap keys and assert_eq! macro.

LANGUAGE: rust
CODE:
#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 0, y: 0 };
    assert_eq!(p1, p2);
}

----------------------------------------

TITLE: Executing Cargo Test Command in Rust
DESCRIPTION: This snippet shows the execution of the cargo test command and its output. It compiles the 'adder' project, runs unit tests from src/lib.rs, and performs doc-tests. The output includes compilation information, test results, and execution time.

LANGUAGE: bash
CODE:
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running unittests src/lib.rs (file:///projects/adder/target/debug/deps/adder-40313d497ef8f64e)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

----------------------------------------

TITLE: Compiling Cons List in Rust with Ownership Error
DESCRIPTION: This snippet shows the output of attempting to compile a Rust program with a cons list implementation. The compilation fails due to an ownership error where a moved value is used again.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
error[E0382]: use of moved value: `a`
  --> src/main.rs:11:30
   |
9  |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
   |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
10 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
11 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `cons-list` (bin "cons-list") due to 1 previous error

----------------------------------------

TITLE: Implementing add_text Method for Post in Rust
DESCRIPTION: Adds the add_text method to Post to allow adding content to a blog post.

LANGUAGE: rust
CODE:
impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

----------------------------------------

TITLE: Implementing Deref Trait for Custom Box Type in Rust
DESCRIPTION: Implementation of the Deref trait for a custom MyBox<T> type that allows dereferencing behavior similar to regular references. The implementation defines the associated Target type and provides the deref method that returns a reference to the inner value.

LANGUAGE: rust
CODE:
use std::ops::Deref;

# struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

----------------------------------------

TITLE: Basic TCP Listener Implementation in Rust
DESCRIPTION: Creates a TCP listener that binds to localhost port 7878 and prints a message when connections are established.

LANGUAGE: rust
CODE:
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

----------------------------------------

TITLE: Complete Web Server Implementation
DESCRIPTION: Final implementation handling different routes and serving HTML responses with proper status codes.

LANGUAGE: rust
CODE:
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

----------------------------------------

TITLE: Rust Compilation Error: Missing Display Trait
DESCRIPTION: This snippet shows a Rust compilation error message. It indicates that the Rectangle struct does not implement the std::fmt::Display trait, which is needed for default formatting in println!.

LANGUAGE: rust
CODE:
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
  --> src/main.rs:12:29
   |
12 |     println!("rect1 is {}", rect1);
   |                             ^^^^^ `Rectangle` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

----------------------------------------

TITLE: Creating a Config Constructor in Rust
DESCRIPTION: Converts parse_config into an associated function named new for the Config struct.

LANGUAGE: rust
CODE:
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

----------------------------------------

TITLE: Rectangle Can Hold Method Implementation in Rust
DESCRIPTION: Implements a method to check if one rectangle can fit inside another by comparing their dimensions. Takes another Rectangle as a parameter and returns a boolean.

LANGUAGE: rust
CODE:
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

----------------------------------------

TITLE: Creating a Stream of Messages with Timeouts in Rust
DESCRIPTION: Demonstrates creating a stream of messages with a timeout applied to each item, and adding variable delays to simulate real-world conditions.

LANGUAGE: Rust
CODE:
use std::time::Duration;
use trpl::{StreamExt, ReceiverStream};

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel(10);
    trpl::spawn_task(async move {
        for (i, msg) in ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"].iter().enumerate() {
            if i % 2 == 0 {
                trpl::sleep(Duration::from_millis(100)).await;
            } else {
                trpl::sleep(Duration::from_millis(300)).await;
            }
            tx.send(msg.to_string()).await.unwrap();
        }
    });
    ReceiverStream::new(rx)
}

async fn main() {
    let mut messages = get_messages().timeout(Duration::from_millis(200)).pin_mut();

    while let Some(result) = messages.next().await {
        match result {
            Ok(msg) => println!("Message: '{}'", msg),
            Err(_) => println!("Problem: Elapsed(())"),
        }
    }
}

----------------------------------------

TITLE: Modifying String with Immutable Reference in Rust
DESCRIPTION: This code snippet demonstrates an error that occurs when trying to modify a String through an immutable reference. The compiler suggests changing the parameter to a mutable reference to fix the issue.

LANGUAGE: rust
CODE:
fn change(some_string: &String) {
    some_string.push_str(", world");
}

----------------------------------------

TITLE: Demonstrating Closure Ownership Error in Rust
DESCRIPTION: This code snippet illustrates a Rust compilation error that occurs when trying to move a captured variable (value) out of an FnMut closure used in sort_by_key. The error suggests using clone() as a potential solution.

LANGUAGE: rust
CODE:
let value = String::from("closure called");

list.sort_by_key(|r| {
    sort_operations.push(value);
    // Error occurs on the line above
});

----------------------------------------

TITLE: Invalid Let Statement Usage in Rust
DESCRIPTION: Demonstrates an incorrect attempt to use a let statement as an expression value. The compiler generates an error because let statements are only supported directly in conditions of if and while expressions. Also shows a warning about unnecessary parentheses.

LANGUAGE: rust
CODE:
let x = (let y = 6);

----------------------------------------

TITLE: Demonstrating Reference Syntax in Rust
DESCRIPTION: Shows the syntax for creating a reference in Rust using the & symbol. References are the most common type of pointer in Rust and borrow the value they point to.

LANGUAGE: rust
CODE:
&

----------------------------------------

TITLE: Opening Built Rust Book in Web Browsers
DESCRIPTION: Commands to open the built Rust book in Firefox or Chrome on different operating systems.

LANGUAGE: bash
CODE:
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)

$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)

----------------------------------------

TITLE: Printing 'Hello, world!' in Rust
DESCRIPTION: A basic Rust program that prints the string 'Hello, world!' to the console. This is typically the first program written when learning a new programming language.

LANGUAGE: Rust
CODE:
fn main() {
    println!("Hello, world!");
}

----------------------------------------

TITLE: Using the Entry API to Insert Values in a Hash Map in Rust
DESCRIPTION: This snippet demonstrates the use of the entry API to insert values into a hash map only if the key doesn't already exist. It uses the or_insert method to provide a default value.

LANGUAGE: rust
CODE:
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);

----------------------------------------

TITLE: Using if let for Pattern Matching in Rust
DESCRIPTION: This snippet demonstrates the use of 'if let' to match a specific pattern (Some(3)) in an Option<u8> value. If the pattern matches, it executes the associated code block.

LANGUAGE: rust
CODE:
let some_u8_value = Some(3u8);
if let Some(3) = some_u8_value {
    println!("three");
}

----------------------------------------

TITLE: Invalid Worker Thread Creation with Moved Receiver
DESCRIPTION: Code demonstrates an error where a channel receiver is incorrectly moved multiple times in a loop while creating worker threads. The receiver is moved in the first iteration, making it unavailable for subsequent iterations.

LANGUAGE: rust
CODE:
let (sender, receiver) = mpsc::channel();
for id in 0..size {
    workers.push(Worker::new(id, receiver));
}

LANGUAGE: rust
CODE:
fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {

----------------------------------------

TITLE: Creating a New Thread with spawn in Rust
DESCRIPTION: Demonstrates how to create a new thread using thread::spawn and print messages from both the main thread and the spawned thread.

LANGUAGE: rust
CODE:
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

----------------------------------------

TITLE: Processing User Input in Rust
DESCRIPTION: Code snippet demonstrating how to get user input from the command line and print it back.

LANGUAGE: rust
CODE:
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

----------------------------------------

TITLE: Executing Cargo Test Command for Rust Project
DESCRIPTION: This snippet shows the command to run tests in a Rust project using Cargo, the Rust package manager and build system. It executes all unit tests and documentation tests for the project.

LANGUAGE: shell
CODE:
$ cargo test

----------------------------------------

TITLE: Implementing ThreadPool::new in Rust
DESCRIPTION: This code implements the ThreadPool::new function in Rust. It creates a new ThreadPool instance with a specified number of threads, asserting that the size is greater than zero.

LANGUAGE: rust
CODE:
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

----------------------------------------

TITLE: Overwriting Values in a Hash Map in Rust
DESCRIPTION: This code shows how to overwrite an existing value in a hash map by inserting a new value for the same key. The original value is replaced with the new one.

LANGUAGE: rust
CODE:
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);

----------------------------------------

TITLE: Using Message Passing with Channels
DESCRIPTION: Shows how to use channels for thread communication by creating a transmitter-receiver pair and sending data between threads.

LANGUAGE: rust
CODE:
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

----------------------------------------

TITLE: Initial Search Function Implementation in Rust
DESCRIPTION: Basic implementation of the search function that compiles but returns an empty vector, demonstrating lifetime parameters.

LANGUAGE: rust
CODE:
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

----------------------------------------

TITLE: Serving HTML Content in Rust Web Server
DESCRIPTION: Updates the handle_connection function to read HTML content from a file and send it as the response body.

LANGUAGE: rust
CODE:
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

----------------------------------------

TITLE: Finding Largest Number in Vector
DESCRIPTION: Basic implementation of finding the largest number in a vector of integers using iteration and comparison

LANGUAGE: rust
CODE:
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

----------------------------------------

TITLE: Running Program with Output Redirection in Console
DESCRIPTION: Demonstrates how to redirect program output to a file using console commands, showing how error messages are currently being written to stdout.

LANGUAGE: console
CODE:
$ cargo run > output.txt

----------------------------------------

TITLE: Multiple Mutable Borrows Error Example in Rust
DESCRIPTION: This code demonstrates a common Rust compiler error (E0499) that occurs when trying to create multiple mutable references to the same variable simultaneously. The code fails because Rust's ownership rules prevent having more than one mutable reference to a value at the same time.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

----------------------------------------

TITLE: Creating New Cargo Project
DESCRIPTION: Commands to create and navigate to a new Cargo project

LANGUAGE: console
CODE:
$ cargo new hello_cargo
$ cd hello_cargo

----------------------------------------

TITLE: Search Results from minigrep Execution
DESCRIPTION: This snippet displays the output of the minigrep program after execution. It shows the search parameters and the contents of the file being searched.

LANGUAGE: plaintext
CODE:
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

----------------------------------------

TITLE: Successful Output in Text File
DESCRIPTION: Shows the contents of the output file containing only the successful program output after implementing proper error handling.

LANGUAGE: text
CODE:
Are you nobody, too?\nHow dreary to be somebody!

----------------------------------------

TITLE: Cargo Project Management Commands
DESCRIPTION: Example of using Git and Cargo to manage an existing project

LANGUAGE: console
CODE:
$ git clone example.org/someproject
$ cd someproject
$ cargo build

----------------------------------------

TITLE: Compiling Rust Project with Module Resolution Error
DESCRIPTION: This snippet shows the output of a failed Cargo build due to a module resolution error. It highlights an issue with an undeclared crate or module 'hosting' and suggests a potential fix.

LANGUAGE: Shell
CODE:
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
   |
help: consider importing this module through its public re-export
   |
10 +     use crate::hosting;
   |

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` (lib) due to 1 previous error; 1 warning emitted

----------------------------------------

TITLE: Cargo Project Configuration
DESCRIPTION: Basic Cargo.toml configuration file structure for a new project

LANGUAGE: toml
CODE:
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]

----------------------------------------

TITLE: Refactoring Config Parsing into a Function in Rust
DESCRIPTION: Extracts argument parsing logic into a separate parse_config function that returns a tuple.

LANGUAGE: rust
CODE:
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

----------------------------------------

TITLE: Improved Search Implementation with Iterator Adapters
DESCRIPTION: Shows the refactored search function using iterator adapters like filter() and collect(), eliminating mutable state.

LANGUAGE: rust
CODE:
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

----------------------------------------

TITLE: Defining a Type Alias for a Complex Type in Rust
DESCRIPTION: Illustrates how to use type aliases to simplify complex types, reducing repetition and improving code maintainability.

LANGUAGE: rust
CODE:
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}

----------------------------------------

TITLE: Unix-like Project Setup Commands
DESCRIPTION: Terminal commands for setting up a new Rust project directory structure on Linux, macOS, and Windows PowerShell.

LANGUAGE: console
CODE:
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world

----------------------------------------

TITLE: Calling C Function Using FFI in Rust
DESCRIPTION: This snippet demonstrates how to declare an external C function 'abs' using the 'extern' keyword and call it from Rust code. The function is marked as unsafe and is called within an unsafe block.

LANGUAGE: rust
CODE:
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

----------------------------------------

TITLE: Sharing State with Mutex and Arc
DESCRIPTION: Demonstrates thread-safe shared state using Mutex for exclusive access and Arc for reference counting across threads.

LANGUAGE: rust
CODE:
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

----------------------------------------

TITLE: Updating Values Based on Old Values in a Hash Map in Rust
DESCRIPTION: This code demonstrates how to update a value in a hash map based on its old value. It counts word occurrences in a text by incrementing the count for each word.

LANGUAGE: rust
CODE:
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);

----------------------------------------

TITLE: Using the Never Type in Rust Functions
DESCRIPTION: Demonstrates the use of the never type (!) in Rust, which represents functions that never return or expressions that can be coerced into any other type.

LANGUAGE: rust
CODE:
fn bar() -> ! {
    // --snip--
}

----------------------------------------

TITLE: Using cargo fmt for Code Formatting
DESCRIPTION: Demonstrates how to use cargo fmt to automatically format Rust code according to community standards. This command reformats all Rust code in the current crate while preserving code semantics.

LANGUAGE: sh
CODE:
$ cargo fmt

----------------------------------------

TITLE: Mutable Struct Field Update
DESCRIPTION: Shows how to modify a field in a mutable struct instance using dot notation.

LANGUAGE: rust
CODE:
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");

----------------------------------------

TITLE: Shadowing with Type Change in Rust
DESCRIPTION: This code snippet shows how shadowing in Rust allows changing the type of a variable while reusing the same name, which is not possible with mutable variables.

LANGUAGE: rust
CODE:
let spaces = "   ";
let spaces = spaces.len();

----------------------------------------

TITLE: Executing Rust Rectangle Program via Cargo
DESCRIPTION: Terminal command and output showing cargo compilation and execution of a Rust program that prints an array of Rectangle structures. The output shows three Rectangle instances with different width and height values.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rectangles`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]

----------------------------------------

TITLE: Executing Cargo Test Command
DESCRIPTION: Running unit tests for a Rust project using the cargo test command. The output shows a failed test case where a greeting function didn't include the expected name in its output.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling greeter v0.1.0 (file:///projects/greeter)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running unittests src/lib.rs (target/debug/deps/greeter-170b942eb5bf5e3a)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----

thread 'tests::greeting_contains_name' panicked at src/lib.rs:12:9:
Greeting did not contain name, value was `Hello!`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Configuring Release Profiles in Cargo.toml
DESCRIPTION: Example of setting optimization levels for development and release profiles in a Rust project's Cargo.toml file

LANGUAGE: toml
CODE:
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

----------------------------------------

TITLE: Running Program with Arguments and Output Redirection
DESCRIPTION: Demonstrates the corrected behavior where successful output goes to a file while errors are displayed on screen.

LANGUAGE: console
CODE:
$ cargo run -- to poem.txt > output.txt

----------------------------------------

TITLE: Building Rust Project with Private Module Access Error
DESCRIPTION: Terminal output showing cargo build command and resulting compilation errors due to attempting to access a private module 'hosting' and its function 'add_to_waitlist' from both absolute and relative paths.

LANGUAGE: shell
CODE:
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
  |                            |
  |                            private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` (lib) due to 2 previous errors

----------------------------------------

TITLE: Initializing a New Rust Project with Cargo
DESCRIPTION: Creates a new Rust project called 'guessing_game' using Cargo, the Rust package manager and build tool.

LANGUAGE: shell
CODE:
$ cargo new guessing_game
$ cd guessing_game

----------------------------------------

TITLE: Complete Guessing Game Implementation in Rust
DESCRIPTION: The final, complete implementation of the number guessing game in Rust.

LANGUAGE: rust
CODE:
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

----------------------------------------

TITLE: Creating a Channel in Rust
DESCRIPTION: Demonstrates how to create a channel using mpsc::channel() and assign the transmitter and receiver halves.

LANGUAGE: rust
CODE:
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}

----------------------------------------

TITLE: Implementing a Timeout Function in Rust
DESCRIPTION: Demonstrates how to build a custom timeout function using async/await, race, and sleep. This function allows running a future with a time limit, returning an error if the timeout is reached.

LANGUAGE: rust
CODE:
async fn timeout<F, T>(future_to_try: F, max_time: Duration) -> Result<T, Duration>
where
    F: Future<Output = T>,
{
    let timer = trpl::sleep(max_time);
    match trpl::race(future_to_try, timer).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

----------------------------------------

TITLE: Function with Parameter and Return Value in Rust
DESCRIPTION: This example demonstrates a function that takes a parameter and returns a value. It shows how to use the parameter in a calculation and return the result.

LANGUAGE: rust
CODE:
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

----------------------------------------

TITLE: Running Rust Program with Type Error
DESCRIPTION: Terminal output showing a compilation error when trying to compile a Rust program where a Point struct is initialized with mismatched types - an integer for x and a float for y where both should be integers.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error

----------------------------------------

TITLE: Running Cargo Check Command
DESCRIPTION: Shell command to check Rust project compilation without producing an executable.

LANGUAGE: shell
CODE:
$ cargo check

----------------------------------------

TITLE: Field Init Shorthand Syntax
DESCRIPTION: Uses Rust's field init shorthand syntax when variable names match struct field names.

LANGUAGE: rust
CODE:
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

----------------------------------------

TITLE: Stream Trait Definition in Rust
DESCRIPTION: Definition of the Stream trait showing its Item type and poll_next method for async iteration

LANGUAGE: rust
CODE:
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}

----------------------------------------

TITLE: Storing an i32 Value on the Heap Using Box<T>
DESCRIPTION: Demonstrates how to use Box<T> to store a value on the heap instead of the stack.

LANGUAGE: rust
CODE:
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}

----------------------------------------

TITLE: Multiple Pattern Matching with OR Operator
DESCRIPTION: Using the | operator to match multiple patterns in a single match arm

LANGUAGE: rust
CODE:
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}

----------------------------------------

TITLE: Running Cargo Tests for Rust Project
DESCRIPTION: This snippet shows the command to run tests for a Rust project using Cargo, and the resulting compilation error output. The error occurs due to an attempt to mutably borrow `self.sent_messages` through an immutable reference.

LANGUAGE: shell
CODE:
$ cargo test

LANGUAGE: text
CODE:
   Compiling limit-tracker v0.1.0 (file:///projects/limit-tracker)
error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> src/lib.rs:58:13
   |
58 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference in the `impl` method and the `trait` definition
   |
2  ~     fn send(&mut self, msg: &str);
3  | }
...
56 |     impl Messenger for MockMessenger {
57 ~         fn send(&mut self, message: &str) {
   |

For more information about this error, try `rustc --explain E0596`.
error: could not compile `limit-tracker` (lib test) due to 1 previous error
warning: build failed, waiting for other jobs to finish...

----------------------------------------

TITLE: Creating Arbitrary Memory Raw Pointer
DESCRIPTION: Shows how to create a raw pointer to an arbitrary memory address using type casting.

LANGUAGE: rust
CODE:
let address = 0x012345usize;
let r = address as *const i32;

----------------------------------------

TITLE: Using Default Trait for Default Values in Rust
DESCRIPTION: The Default trait provides default values for types. It's commonly used with struct update syntax and the unwrap_or_default method on Option<T>.

LANGUAGE: rust
CODE:
#[derive(Default)]
struct Config {
    timeout: u32,
    retry_count: u32
}

fn main() {
    let config = Config {
        timeout: 30,
        ..Default::default()
    };
}

----------------------------------------

TITLE: Implementing Clone and Copy Traits in Rust
DESCRIPTION: Clone allows explicit creation of deep copies, while Copy enables simple bitwise copying of stack-only data. These traits are used when duplication of values is required, such as when calling to_vec on a slice.

LANGUAGE: rust
CODE:
#[derive(Clone, Copy)]
struct SomeStruct {
    // fields
}

----------------------------------------

TITLE: Uninstalling Rust and rustup
DESCRIPTION: This command uninstalls Rust and the rustup tool. It removes all Rust-related files and tools from the system.

LANGUAGE: console
CODE:
$ rustup self uninstall

----------------------------------------

TITLE: Receiving a Message from a Channel in Rust
DESCRIPTION: Demonstrates how to receive a message from a channel in the main thread using recv().

LANGUAGE: rust
CODE:
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

----------------------------------------

TITLE: Using Different Generic Types in Method Signatures
DESCRIPTION: Shows how to implement a method that uses different generic types than those defined in the struct, creating a new Point instance with mixed types.

LANGUAGE: rust
CODE:
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

----------------------------------------

TITLE: Compiling and Running Rust Rectangle Program
DESCRIPTION: Shell command and output showing the compilation and execution of a Rust program using Cargo. The program creates a Rectangle struct with width 30 and height 50.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle { width: 30, height: 50 }

----------------------------------------

TITLE: Implementing Game Loop in Rust
DESCRIPTION: Adds a loop to allow multiple guesses until the correct number is guessed.

LANGUAGE: rust
CODE:
loop {
    println!("Please input your guess.");

    // --snip--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

----------------------------------------

TITLE: Declaring Generic Smart Pointer Types in Rust
DESCRIPTION: Demonstrates the syntax for declaring generic smart pointer types in Rust, including Box<T>, Rc<T>, Ref<T>, and RefMut<T>. These are common smart pointers provided by the Rust standard library.

LANGUAGE: rust
CODE:
Box<T>

LANGUAGE: rust
CODE:
Rc<T>

LANGUAGE: rust
CODE:
Ref<T>

LANGUAGE: rust
CODE:
RefMut<T>

LANGUAGE: rust
CODE:
RefCell<T>

----------------------------------------

TITLE: Implementing Case-Insensitive Search in Rust
DESCRIPTION: Adds a case-insensitive search function that converts both query and content to lowercase before comparing.

LANGUAGE: rust
CODE:
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

----------------------------------------

TITLE: Refactoring Config into a Struct in Rust
DESCRIPTION: Creates a Config struct to group related configuration values and updates parse_config to return an instance.

LANGUAGE: rust
CODE:
struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

----------------------------------------

TITLE: Compiling and Running Rust Minigrep Project
DESCRIPTION: This snippet demonstrates the process of compiling and running a Rust project called 'minigrep' using Cargo. It shows the compilation output, execution, and a runtime error that occurs during the program's execution.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`

thread 'main' panicked at src/main.rs:27:21:
index out of bounds: the len is 1 but the index is 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

----------------------------------------

TITLE: Saving command line arguments in variables
DESCRIPTION: Demonstrates how to save specific command line arguments into variables for further use in the program.

LANGUAGE: rust
CODE:
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

----------------------------------------

TITLE: Demonstrating Tuple Destructuring Error in Rust
DESCRIPTION: This code attempts to destructure a 3-element tuple (1, 2, 3) into a 2-element pattern (x, y), resulting in a compilation error due to mismatched types.

LANGUAGE: rust
CODE:
let (x, y) = (1, 2, 3);

----------------------------------------

TITLE: Windows CMD Project Setup Commands
DESCRIPTION: Commands for setting up a new Rust project directory structure using Windows Command Prompt.

LANGUAGE: cmd
CODE:
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world

----------------------------------------

TITLE: Installing Rustfmt Component
DESCRIPTION: Command to install the rustfmt component using rustup for Rust code formatting.

LANGUAGE: sh
CODE:
rustup component add rustfmt

----------------------------------------

TITLE: Declaring a Generic Function with ?Sized Trait Bound in Rust
DESCRIPTION: This snippet demonstrates how to declare a generic function in Rust that can accept both sized and unsized types. The ?Sized trait bound allows the type parameter T to be potentially unsized.

LANGUAGE: rust
CODE:
fn generic<T: ?Sized>(t: &T) {
    // ...snip...
}

----------------------------------------

TITLE: Installing ripgrep Binary Crate using Cargo Install in Rust
DESCRIPTION: This snippet demonstrates how to use 'cargo install' to download and install the 'ripgrep' binary crate from crates.io. It shows the command and its output, including the installation progress and the final location of the installed binary.

LANGUAGE: shell
CODE:
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)

----------------------------------------

TITLE: Cargo Project Configuration File
DESCRIPTION: The standard Cargo.toml configuration file generated for a new Rust project. Defines project metadata and dependencies.

LANGUAGE: toml
CODE:
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]

----------------------------------------

TITLE: Using Hash Trait for Mapping in Rust
DESCRIPTION: The Hash trait allows mapping instances to fixed-size values using a hash function. It's required for efficient data storage in collections like HashMap.

LANGUAGE: rust
CODE:
#[derive(Hash)]
struct SomeStruct {
    // fields
}

----------------------------------------

TITLE: Defining Web Crawler Access Rules in robots.txt
DESCRIPTION: This snippet configures the robots.txt file to allow all user agents but disallow access to the root directory. It's a common setup to control web crawler behavior on a website.

LANGUAGE: plaintext
CODE:
User-agent: *
Disallow: /

----------------------------------------

TITLE: Returning Reference to Local Variable in Rust
DESCRIPTION: This function attempts to return a reference to a locally created String, which is not allowed in Rust due to ownership rules. The compiler throws errors related to missing lifetime specifiers and returning references to local variables.

LANGUAGE: rust
CODE:
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

----------------------------------------

TITLE: Updated ThreadPool Drop Implementation
DESCRIPTION: Improved Drop implementation using Vec::drain to properly handle thread cleanup.

LANGUAGE: rust
CODE:
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }

----------------------------------------

TITLE: Error Output in Text File
DESCRIPTION: Shows the contents of the output file containing the error message that was incorrectly written to stdout instead of stderr.

LANGUAGE: text
CODE:
Problem parsing arguments: not enough arguments

----------------------------------------

TITLE: Using Rc<T> to Share Data
DESCRIPTION: Shows how to use Rc<T> to create multiple references to the same data, allowing shared ownership.

LANGUAGE: rust
CODE:
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

----------------------------------------

TITLE: Executing Rust Project with Cargo
DESCRIPTION: This snippet shows the process of compiling and running a Rust project named 'functions' using Cargo. It demonstrates the build process, including compilation time and the execution of the resulting binary.

LANGUAGE: Shell
CODE:
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/functions`
The measurement is: 5h

----------------------------------------

TITLE: Sample HTML Response Page
DESCRIPTION: Basic HTML template used as the homepage response.

LANGUAGE: html
CODE:
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>

----------------------------------------

TITLE: Implementing Dynamically Sized Types in Rust
DESCRIPTION: Shows how to work with dynamically sized types in Rust, which are types whose size is known only at runtime, such as str.

LANGUAGE: rust
CODE:
let s1: str = "Hello there!";
let s2: str = "How's it going?";

----------------------------------------

TITLE: Match Expression with Coin Enum
DESCRIPTION: Demonstrates pattern matching using match expression with a coin enum

LANGUAGE: rust
CODE:
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

----------------------------------------

TITLE: Conditional Compilation for MacOS in Rust
DESCRIPTION: Demonstrates how to use the #[cfg] attribute to conditionally compile code for macOS targets. The function will only be included in the build when compiling specifically for macOS operating systems.

LANGUAGE: rust
CODE:
// The function is only included in the build when compiling for macOS
#[cfg(target_os = "macos")]
fn macos_only() {
  // ...
}

----------------------------------------

TITLE: Configuring Package Metadata in Cargo.toml
DESCRIPTION: Shows required metadata fields for publishing a crate on crates.io

LANGUAGE: toml
CODE:
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]

----------------------------------------

TITLE: Executing Rust Program with Mutex Threading Error
DESCRIPTION: Terminal output showing a Rust compilation error when attempting to share a Mutex across multiple threads. The error occurs because the Mutex is moved into thread closures multiple times, violating ownership rules.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling shared-state v0.1.0 (file:///projects/shared-state)
error[E0382]: borrow of moved value: `counter`
  --> src/main.rs:21:29
   |
5  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
8  |     for _ in 0..10 {
   |     -------------- inside of this loop
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved into closure here, in previous iteration of loop
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value borrowed here after move
   |
help: consider moving the expression out of the loop so it is only moved once
   |
8  ~     let mut value = counter.lock();
9  ~     for _ in 0..10 {
10 |         let handle = thread::spawn(move || {
11 ~             let mut num = value.unwrap();
   |

For more information about this error, try `rustc --explain E0382`.
error: could not compile `shared-state` (bin "shared-state") due to 1 previous error

----------------------------------------

TITLE: Fixing Code with rustfix
DESCRIPTION: Shows how to use rustfix to automatically apply compiler suggestions to Rust code.

LANGUAGE: Rust
CODE:
fn main() {
    let mut x = 42;
    println!("{x}");
}

LANGUAGE: bash
CODE:
$ cargo fix

LANGUAGE: Rust
CODE:
fn main() {
    let x = 42;
    println!("{x}");
}

----------------------------------------

TITLE: Custom Smart Pointer Implementation in Rust
DESCRIPTION: Defines a custom MyBox<T> smart pointer type with a basic constructor.

LANGUAGE: rust
CODE:
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

----------------------------------------

TITLE: Panic Configuration in Cargo.toml
DESCRIPTION: Configuration to switch from unwinding to aborting on panic in release mode.

LANGUAGE: toml
CODE:
[profile.release]
panic = 'abort'

----------------------------------------

TITLE: Reading File Contents in Rust
DESCRIPTION: Uses std::fs::read_to_string to read the contents of a file specified by a command line argument.

LANGUAGE: rust
CODE:
use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

----------------------------------------

TITLE: Executing Rust Program with Cargo
DESCRIPTION: Terminal command and output showing the compilation and execution of a Rust program named 'loops'. The output displays multiple loop iterations tracking count and remaining values.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2

----------------------------------------

TITLE: Box Smart Pointer Dereferencing in Rust
DESCRIPTION: Shows how to use the dereference operator with Box<T>, Rust's built-in smart pointer type.

LANGUAGE: rust
CODE:
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

----------------------------------------

TITLE: Accessing Values in a Hash Map in Rust
DESCRIPTION: This code shows how to retrieve a value from a hash map using the get method. It handles the Option return type and provides a default value if the key is not found.

LANGUAGE: rust
CODE:
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

----------------------------------------

TITLE: Original Config::build Implementation with Clone
DESCRIPTION: Shows the initial implementation of Config::build that uses indexing and clone() calls to create a Config instance from command line arguments.

LANGUAGE: rust
CODE:
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

----------------------------------------

TITLE: Executing Rust Program via Cargo
DESCRIPTION: Terminal command and output showing the process of compiling and running a Rust program named 'rectangles' using Cargo. The program calculates and displays a rectangle's area.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/rectangles`
The area of the rectangle is 1500 square pixels.

----------------------------------------

TITLE: Parsing String to Integer with Type Annotation in Rust
DESCRIPTION: Demonstrates how to parse a string to an unsigned 32-bit integer using a type annotation. This is necessary when the compiler cannot infer the desired numeric type.

LANGUAGE: rust
CODE:
let guess: u32 = "42".parse().expect("Not a number!");

----------------------------------------

TITLE: Executing Rust Program with Cargo and Command Line Arguments
DESCRIPTION: This snippet shows how to run a Rust program named 'minigrep' using Cargo, passing 'frog' as a search term and 'poem.txt' as the file to search in. It includes the compilation output, execution time, and a sample result.

LANGUAGE: Shell
CODE:
$ cargo run -- frog poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/minigrep frog poem.txt`
How public, like a frog

----------------------------------------

TITLE: Ferris Table Markdown Structure
DESCRIPTION: Markdown table showing different states of the Ferris mascot and their meanings in code examples throughout the book.

LANGUAGE: markdown
CODE:
|Ferris|Meaning|
|------|-------|
|<img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>|This code does not compile!|
|<img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>|This code panics!|
|<img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/>|This code does not produce the desired behavior.|

----------------------------------------

TITLE: Attempting Mutable Borrow on Immutable Variable in Rust
DESCRIPTION: This code snippet demonstrates an attempt to create a mutable reference to an immutable variable in Rust. It results in a compilation error, highlighting Rust's strict borrowing rules. The error message suggests declaring the variable as mutable to resolve the issue.

LANGUAGE: rust
CODE:
let x = 5;
let y = &mut x;

----------------------------------------

TITLE: Rust Function with Correct Lifetime Annotation
DESCRIPTION: Corrected version of the function signature showing proper lifetime parameter usage, as suggested by the compiler. The 'a lifetime parameter indicates that all references share the same lifetime.

LANGUAGE: rust
CODE:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

----------------------------------------

TITLE: Workspace Configuration in Cargo.toml
DESCRIPTION: Example of configuring a Cargo workspace with multiple packages

LANGUAGE: toml
CODE:
[workspace]
resolver = "2"
members = ["adder", "add_one"]

----------------------------------------

TITLE: Building and Running Rust Application with Cargo
DESCRIPTION: Example of building and running a Rust application using Cargo package manager. The output shows compilation success followed by a runtime panic due to missing command line arguments.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`

thread 'main' panicked at src/main.rs:26:13:
not enough arguments
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

----------------------------------------

TITLE: Unused Map Operation in Rust
DESCRIPTION: This snippet demonstrates an unused map operation on an iterator in Rust. The compiler warns that the operation does nothing unless consumed and suggests using 'let _ = ...' to ignore the result if that's the intended behavior.

LANGUAGE: rust
CODE:
v1.iter().map(|x| x + 1);

----------------------------------------

TITLE: Using JoinHandle to Wait for Thread Completion in Rust
DESCRIPTION: Shows how to use a JoinHandle to ensure a spawned thread completes before the main thread exits.

LANGUAGE: rust
CODE:
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

----------------------------------------

TITLE: Using Generics with Sized Trait in Rust
DESCRIPTION: Demonstrates how to use the Sized trait in generic functions to work with types that may or may not have a known size at compile time.

LANGUAGE: rust
CODE:
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

----------------------------------------

TITLE: Adding Methods to Encapsulated Structure
DESCRIPTION: Implementation of methods for AveragedCollection to manage data and maintain encapsulation through public interfaces.

LANGUAGE: rust
CODE:
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

----------------------------------------

TITLE: Cargo Build Command and Error Output
DESCRIPTION: Shell command showing cargo build execution and resulting compiler error output for the minigrep project.

LANGUAGE: shell
CODE:
$ cargo build
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
error[E0106]: missing lifetime specifier
  --> src/lib.rs:28:51
   |
28 | pub fn search(query: &str, contents: &str) -> Vec<&str> {
   |                      ----            ----         ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `contents`
help: consider introducing a named lifetime parameter
   |
28 | pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
   |              ++++         ++                 ++              ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `minigrep` (lib) due to 1 previous error

----------------------------------------

TITLE: Running Rust Minigrep Project with Cargo
DESCRIPTION: This snippet shows the command to run the 'minigrep' Rust project using Cargo, along with the output generated during compilation and execution. It demonstrates the use of 'cargo run' for building and running a Rust project.

LANGUAGE: Shell
CODE:
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/minigrep`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
]

----------------------------------------

TITLE: Running an Async Function with trpl::run
DESCRIPTION: Using the trpl::run function to execute an async block that calls the page_title function.

LANGUAGE: rust
CODE:
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}

----------------------------------------

TITLE: Compiling Rust Project with Cargo
DESCRIPTION: This snippet shows the output of compiling a Rust project named 'guessing_game' using the 'cargo build' command. It includes a warning about an unused Result that should be handled and suggests a fix.

LANGUAGE: shell
CODE:
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
10 |     let _ = io::stdin().read_line(&mut guess);
   |     +++++++

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s

----------------------------------------

TITLE: Implementing Add Trait for Point Struct in Rust
DESCRIPTION: Shows how to implement the Add trait from std::ops to enable the + operator for a custom Point struct. The implementation allows adding two Point instances by summing their x and y coordinates respectively.

LANGUAGE: rust
CODE:
use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}

----------------------------------------

TITLE: Implementing Summary Trait on Types in Rust
DESCRIPTION: Demonstrates implementing the Summary trait on NewsArticle and SocialPost types with specific summarize method implementations.

LANGUAGE: rust
CODE:
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

----------------------------------------

TITLE: Executing Cargo Tests for Rust Project
DESCRIPTION: This snippet shows the command to run Cargo tests and the resulting output. It includes compilation details, test execution information, and test results for a Rust project named 'adder'.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.62s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

----------------------------------------

TITLE: Rust Compilation Error Output
DESCRIPTION: Terminal output from running 'cargo run' showing a compilation error E0614 indicating that MyBox<integer> cannot be dereferenced. This suggests the Deref trait needs to be implemented for the custom box type.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^

For more information about this error, try `rustc --explain E0614`.
error: could not compile `deref-example` (bin "deref-example") due to 1 previous error

----------------------------------------

TITLE: Using trpl::join to Await Two Anonymous Futures in Rust
DESCRIPTION: This example demonstrates how to use trpl::join to await two anonymous futures concurrently in an async block.

LANGUAGE: rust
CODE:
trpl::run(async {
    let fut1 = async {
        for i in 1..10 {
            println!("hi number {} from the first task!", i);
            trpl::sleep(500).await;
        }
    };
    let fut2 = async {
        for i in 1..5 {
            println!("hi number {} from the second task!", i);
            trpl::sleep(500).await;
        }
    };
    let _ = trpl::join(fut1, fut2).await;
});

----------------------------------------

TITLE: Defining Iterator Trait with Associated Type in Rust
DESCRIPTION: This code snippet demonstrates the definition of the Iterator trait in Rust, which uses an associated type 'Item' to represent the type of values being iterated over.

LANGUAGE: rust
CODE:
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

----------------------------------------

TITLE: Using Fully Qualified Syntax for Associated Functions in Rust
DESCRIPTION: Shows how to use fully qualified syntax to call an associated function from a trait implementation in Rust.

LANGUAGE: Rust
CODE:
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

----------------------------------------

TITLE: Installing Dprint Formatter
DESCRIPTION: Command to install the dprint formatting tool using cargo for markdown and non-Rust code formatting.

LANGUAGE: sh
CODE:
cargo install dprint

----------------------------------------

TITLE: Defining Unit Test Function with Rust Attribute
DESCRIPTION: Demonstrates how to mark a function as a unit test using the #[test] attribute in Rust. The test attribute indicates that this function should be executed when running tests.

LANGUAGE: rust
CODE:
// A function marked as a unit test
#[test]
fn test_foo() {
    /* ... */
}

----------------------------------------

TITLE: Collecting Command Line Arguments in Rust
DESCRIPTION: Uses std::env::args() to collect command line arguments into a vector and print them for debugging.

LANGUAGE: rust
CODE:
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

----------------------------------------

TITLE: Defining a Basic Summary Trait in Rust
DESCRIPTION: Shows how to define a basic trait called Summary that specifies a summarize method signature which must return a String.

LANGUAGE: rust
CODE:
pub trait Summary {
    fn summarize(&self) -> String;
}

----------------------------------------

TITLE: Writing Error Messages to Standard Error in Rust
DESCRIPTION: Uses eprintln! macro to print error messages to standard error instead of standard output.

LANGUAGE: rust
CODE:
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

----------------------------------------

TITLE: Using Automatic Formatting with rustfmt
DESCRIPTION: Demonstrates how to install and use rustfmt to automatically format Rust code.

LANGUAGE: bash
CODE:
$ rustup component add rustfmt
$ cargo fmt

----------------------------------------

TITLE: Implementing content Method for Post in Rust
DESCRIPTION: Adds a placeholder content method to Post that always returns an empty string slice.

LANGUAGE: rust
CODE:
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}

----------------------------------------

TITLE: Creating Basic Workspace Structure
DESCRIPTION: Initial setup of a Cargo workspace configuration file defining workspace members and resolver version.

LANGUAGE: toml
CODE:
[workspace]
members = ["adder"]
resolver = "2"

----------------------------------------

TITLE: Demonstrating Rust Error Type Syntax
DESCRIPTION: Shows the basic syntax for Rust's two main error handling types: Result<T, E> for recoverable errors and panic! for unrecoverable errors.

LANGUAGE: rust
CODE:
Result<T, E>

LANGUAGE: rust
CODE:
panic!

----------------------------------------

TITLE: Error Handling with Result in Rust
DESCRIPTION: Example of handling file operations with Result using match expressions.

LANGUAGE: rust
CODE:
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

----------------------------------------

TITLE: Iterating Over Key-Value Pairs in a Hash Map in Rust
DESCRIPTION: This snippet demonstrates how to iterate over all key-value pairs in a hash map using a for loop. It prints each pair, though the order is arbitrary.

LANGUAGE: rust
CODE:
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}

----------------------------------------

TITLE: Attempting Type Change with Mutable Variables in Rust
DESCRIPTION: This code snippet demonstrates that changing the type of a mutable variable is not allowed in Rust, resulting in a compilation error.

LANGUAGE: rust
CODE:
let mut spaces = "   ";
spaces = spaces.len();

----------------------------------------

TITLE: Implementing a Supertrait in Rust
DESCRIPTION: This example illustrates how to use a supertrait in Rust, where one trait depends on another trait's functionality.

LANGUAGE: rust
CODE:
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

----------------------------------------

TITLE: For Loop Array Iteration in Rust
DESCRIPTION: Shows how to iterate over an array using a for loop, which is safer and more concise than while loops.

LANGUAGE: rust
CODE:
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

----------------------------------------

TITLE: Suggested Fix for Unused Map Operation in Rust
DESCRIPTION: This snippet shows the compiler's suggested fix for the unused map operation. It uses 'let _ = ...' to explicitly ignore the result of the iterator operation, avoiding the unused value warning.

LANGUAGE: rust
CODE:
let _ = v1.iter().map(|x| x + 1);

----------------------------------------

TITLE: Running Rust Program with Miri
DESCRIPTION: Command line output showing compilation and execution of a Rust program using the Miri interpreter. The program appears to be an unsafe example that manipulates a counter value, resulting in output of 'COUNTER: 3'.

LANGUAGE: shell
CODE:
$ cargo +nightly miri run
   Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `file:///home/.rustup/toolchains/nightly/bin/cargo-miri runner target/miri/debug/unsafe-example`
COUNTER: 3

----------------------------------------

TITLE: Implementing request_review Methods in Rust
DESCRIPTION: Implements request_review methods on Post and the State trait to change post state.

LANGUAGE: rust
CODE:
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

----------------------------------------

TITLE: Matching Value Ranges
DESCRIPTION: Using ..= syntax to match an inclusive range of values

LANGUAGE: rust
CODE:
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}

----------------------------------------

TITLE: Function with Parameters in Rust
DESCRIPTION: Shows function definition with parameters and type annotations.

LANGUAGE: rust
CODE:
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

----------------------------------------

TITLE: Installing rustup on Linux or macOS
DESCRIPTION: This command downloads and executes a script to install rustup, which then installs the latest stable version of Rust. It requires an internet connection and may prompt for a password.

LANGUAGE: console
CODE:
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

----------------------------------------

TITLE: Worker Thread Loop with Graceful Exit
DESCRIPTION: Implementation of worker thread loop that gracefully exits when the channel is closed.

LANGUAGE: rust
CODE:
while let Ok(job) = receiver.recv() {
    println!("Worker {id} got a job; executing.");

    job();
}

----------------------------------------

TITLE: Executing Rust Project with Cargo
DESCRIPTION: Demonstrates using cargo run to compile and execute a Rust project named traits-example. Shows the compilation process, build completion message, and program output including text messages.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/traits-example`
This is your captain speaking.
Up!
*waving arms furiously*

----------------------------------------

TITLE: Installing C compiler on macOS
DESCRIPTION: This command installs the Xcode Command Line Tools on macOS, which includes a C compiler. This is necessary as some Rust packages depend on C code.

LANGUAGE: console
CODE:
$ xcode-select --install

----------------------------------------

TITLE: Implementing Methods on Generic Structs in Rust
DESCRIPTION: Demonstrates how to implement a method on a generic Point<T> struct that returns a reference to the x field.

LANGUAGE: rust
CODE:
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

----------------------------------------

TITLE: Creating Multiple Producers for a Channel in Rust
DESCRIPTION: Demonstrates how to create multiple producer threads that send messages to a single receiver using channel cloning.

LANGUAGE: rust
CODE:
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

----------------------------------------

TITLE: Implementing Case-Insensitive Search Function in Rust
DESCRIPTION: This snippet defines the 'search_case_insensitive' function in Rust. It converts both the query and each line to lowercase before comparing them.

LANGUAGE: rust
CODE:
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

----------------------------------------

TITLE: Using if let for concise Option<u8> handling in Rust
DESCRIPTION: Shows how to use if let as a more concise alternative to match when only one pattern needs to be handled.

LANGUAGE: rust
CODE:
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}

----------------------------------------

TITLE: Implementing Add Trait for Point Struct in Rust
DESCRIPTION: This example shows how to implement the Add trait to overload the + operator for a Point struct in Rust, demonstrating operator overloading.

LANGUAGE: rust
CODE:
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}

----------------------------------------

TITLE: Executing Cargo Test Command
DESCRIPTION: Shows the command and output from running specific tests containing 'add' in their name using Cargo's test runner. The output demonstrates successful compilation and execution of two unit tests.

LANGUAGE: shell
CODE:
$ cargo test add
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

----------------------------------------

TITLE: Storing multilingual greetings in Rust strings
DESCRIPTION: This snippet demonstrates that Rust strings can store various UTF-8 encoded texts, including non-Latin characters.

LANGUAGE: rust
CODE:
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");

----------------------------------------

TITLE: Verifying Rust Installation
DESCRIPTION: This command checks the installed version of Rust. It should display the version number, commit hash, and commit date of the latest stable version.

LANGUAGE: console
CODE:
$ rustc --version

----------------------------------------

TITLE: Using panic! macro in Rust
DESCRIPTION: This snippet demonstrates the usage of the panic! macro in Rust. The panic! macro is used for unrecoverable errors and program termination.

LANGUAGE: rust
CODE:
panic!()

----------------------------------------

TITLE: Compiling and Running Rust Rectangle Program
DESCRIPTION: This snippet shows the command to compile and run a Rust program named 'rectangles'. The compilation fails due to a missing trait implementation.

LANGUAGE: shell
CODE:
$ cargo run

----------------------------------------

TITLE: Concatenating Strings in Rust
DESCRIPTION: This snippet demonstrates string concatenation using the + operator and the format! macro.

LANGUAGE: rust
CODE:
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");

----------------------------------------

TITLE: Sample Code with Unused Mutable Variable
DESCRIPTION: Example Rust code demonstrating a common warning case where a variable is marked as mutable but never modified. This code triggers a compiler warning that can be automatically fixed with rustfix.

LANGUAGE: rust
CODE:
fn main() {
    let mut x = 42;
    println!("{x}");
}

----------------------------------------

TITLE: Storing Command Line Arguments in Variables in Rust
DESCRIPTION: Extracts query and file path arguments from the args vector into separate variables.

LANGUAGE: rust
CODE:
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}

----------------------------------------

TITLE: For Loop Pattern Destructuring
DESCRIPTION: Shows how to use pattern matching in for loops to destructure tuples using enumerate().

LANGUAGE: rust
CODE:
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{value} is at index {index}");
}

----------------------------------------

TITLE: Running Rust Rectangle Program with Cargo
DESCRIPTION: This snippet shows the process of compiling and running a Rust program named 'rectangles' using Cargo, the Rust package manager and build system.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`

----------------------------------------

TITLE: Running Rust Program with File Error
DESCRIPTION: Terminal output showing cargo compilation and execution of a Rust program that encounters a file error, resulting in a panic. The error indicates an attempt to open a non-existent file.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`

thread 'main' panicked at src/main.rs:8:23:
Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

----------------------------------------

TITLE: Edition Configuration in Cargo.toml
DESCRIPTION: The edition key in Cargo.toml specifies which Rust edition the compiler should use for the project. If not specified, it defaults to 2015 for backward compatibility.

LANGUAGE: toml
CODE:
edition = "2021"

----------------------------------------

TITLE: Using unwrap and expect for Error Handling in Rust
DESCRIPTION: This code shows the usage of unwrap and expect methods as shortcuts for handling Result types.

LANGUAGE: rust
CODE:
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}

// Using expect
fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

----------------------------------------

TITLE: Running Cargo Check Command
DESCRIPTION: Console output from running 'cargo check' command on a Rust project named 'hello'. Shows successful compilation check completion in development profile.

LANGUAGE: shell
CODE:
$ cargo check
    Checking hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s

----------------------------------------

TITLE: Deref Trait Implementation for Custom Smart Pointer
DESCRIPTION: Implements the Deref trait for MyBox<T> to enable dereferencing with the * operator.

LANGUAGE: rust
CODE:
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

----------------------------------------

TITLE: Compiling and Running Rust Program with Cargo
DESCRIPTION: Terminal commands and output showing the process of compiling and running a Rust program using Cargo. The output includes compilation status, build profile information, and program execution result indicating a false condition.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was false

----------------------------------------

TITLE: Using a Custom Derive Macro in Rust
DESCRIPTION: Shows how a user would use a custom derive macro to implement a trait automatically.

LANGUAGE: rust
CODE:
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

----------------------------------------

TITLE: Adding content Method to State Trait in Rust
DESCRIPTION: Adds the content method to the State trait with implementations for each state.

LANGUAGE: rust
CODE:
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

----------------------------------------

TITLE: Accessing Array Elements in Rust
DESCRIPTION: Demonstrates how to access elements of an array using index notation. Array indexing in Rust starts at 0.

LANGUAGE: rust
CODE:
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

----------------------------------------

TITLE: Creating a New Task with spawn_task in Rust
DESCRIPTION: This snippet demonstrates how to create a new task using trpl::spawn_task and trpl::sleep to implement a counting example with async/await.

LANGUAGE: rust
CODE:
trpl::run(async {
    trpl::spawn_task(async {
        for i in 1..5 {
            println!("hi number {} from the second task!", i);
            trpl::sleep(500).await;
        }
    });

    for i in 1..10 {
        println!("hi number {} from the first task!", i);
        trpl::sleep(500).await;
    }
});

----------------------------------------

TITLE: Implementing Channel-based Job Distribution in Rust ThreadPool
DESCRIPTION: This code implements a channel-based job distribution system in a Rust ThreadPool. It creates a channel for sending jobs to worker threads and uses Arc and Mutex for thread-safe sharing of the receiver.

LANGUAGE: rust
CODE:
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}

----------------------------------------

TITLE: Implementing Screen Struct with Trait Objects in Rust
DESCRIPTION: This code defines a Screen struct that holds a vector of trait objects implementing the Draw trait. It demonstrates how to use Box<dyn Draw> to store different types that implement the same trait.

LANGUAGE: rust
CODE:
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

----------------------------------------

TITLE: Using map() and collect() Methods on an Iterator in Rust
DESCRIPTION: This example demonstrates using the map() method to create a new iterator with modified items, and then using collect() to consume the iterator and create a vector with the results.

LANGUAGE: rust
CODE:
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);

----------------------------------------

TITLE: Converting Markdown Blockquotes to Semantic HTML Notes
DESCRIPTION: Demonstrates the transformation of Markdown blockquotes used for notes and callouts into semantic HTML sections with appropriate class and ARIA attributes. The preprocessor converts both simple notes and titled note sections.

LANGUAGE: markdown
CODE:
> Note: This is some material we want to provide more emphasis for, because it
> is important in some way!

Some text.

> ## Some subject
>
> Here is all the important things to know about that particular subject.

LANGUAGE: html
CODE:
<section class="note" aria-role="note">

This is some material we want to provide more emphasis for, because it is
important in some way!

</section>

Some text.

<section class="note" aria-role="note">

## Some subject

Here is all the important things to know about that particular subject.

</section>

----------------------------------------

TITLE: Original Search Implementation with Loop
DESCRIPTION: Shows the initial implementation of the search function using a for loop and mutable vector.

LANGUAGE: rust
CODE:
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

----------------------------------------

TITLE: Destructuring Structs
DESCRIPTION: Example of destructuring struct fields into separate variables

LANGUAGE: rust
CODE:
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p;
assert_eq!(0, a);
assert_eq!(7, b);

----------------------------------------

TITLE: Defining Associated Types in Rust Traits
DESCRIPTION: This code snippet demonstrates how to define associated types within a Rust trait. It shows the Iterator trait with an associated type 'Item' and a method 'next' that uses this type.

LANGUAGE: rust
CODE:
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

----------------------------------------

TITLE: Function with Return Value in Rust
DESCRIPTION: This snippet shows how to define a function that returns a value. It demonstrates the syntax for specifying the return type and how the last expression in the function body is implicitly returned.

LANGUAGE: rust
CODE:
fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

----------------------------------------

TITLE: Spawning a Thread for Each Stream in Rust Web Server
DESCRIPTION: This code demonstrates how to spawn a new thread for each incoming stream in a Rust web server. It uses thread::spawn to create a new thread for handling each connection.

LANGUAGE: rust
CODE:
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

----------------------------------------

TITLE: Demonstrating Multiple Ownership with Rc<T> in Rust
DESCRIPTION: This code snippet shows how to use Rc<T> to create multiple references to shared data in a cons list implementation. It demonstrates creating and cloning Rc<T> instances.

LANGUAGE: rust
CODE:
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

----------------------------------------

TITLE: Implementing Run Method for Screen in Rust
DESCRIPTION: This snippet shows the implementation of the run method for the Screen struct. It iterates through the components and calls the draw method on each, demonstrating how trait objects allow for polymorphic behavior.

LANGUAGE: rust
CODE:
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

----------------------------------------

TITLE: Implementing a Custom Guess Type with Validation in Rust
DESCRIPTION: This snippet defines a custom Guess type that ensures values are between 1 and 100. It includes a constructor that panics for invalid values and a getter method for the private value field.

LANGUAGE: rust
CODE:
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

----------------------------------------

TITLE: Executing Cargo Test Command
DESCRIPTION: Demonstrates running tests using the cargo test command, which compiles and executes unit tests in a Rust project. The output shows compilation, test execution, and results including test counts and timing information.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

----------------------------------------

TITLE: Creating a String from a string literal in Rust
DESCRIPTION: This example shows two ways to create a String from a string literal: using the to_string() method and the String::from() function.

LANGUAGE: rust
CODE:
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();

let s = String::from("initial contents");

----------------------------------------

TITLE: Configuring Panic Behavior in Cargo.toml
DESCRIPTION: Configuration to switch from unwinding to aborting on panic in release mode by modifying Cargo.toml settings.

LANGUAGE: toml
CODE:
[profile.release]
panic = 'abort'

----------------------------------------

TITLE: Merging and Managing Multiple Streams in Rust
DESCRIPTION: Shows how to create, merge, and manage multiple streams, including throttling and limiting the number of items processed.

LANGUAGE: Rust
CODE:
use std::time::Duration;
use trpl::{StreamExt, ReceiverStream};

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel(10);
    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if tx.send(count).await.is_err() {
                println!("Error sending interval");
                break;
            }
        }
    });
    ReceiverStream::new(rx)
}

async fn main() {
    let messages = get_messages().timeout(Duration::from_millis(200));
    let intervals = get_intervals()
        .map(|n| format!("Interval: {}", n))
        .timeout(Duration::from_secs(10))
        .throttle(Duration::from_millis(100));

    let mut stream = messages.merge(intervals).take(20).pin_mut();

    while let Some(result) = stream.next().await {
        match result {
            Ok(msg) => println!("{}", msg),
            Err(_) => println!("Problem: Elapsed(())"),
        }
    }
}

----------------------------------------

TITLE: Installing Rust Nightly Toolchain
DESCRIPTION: Command to install the nightly version of Rust using rustup package manager.

LANGUAGE: console
CODE:
$ rustup toolchain install nightly

----------------------------------------

TITLE: Matching Option<u8> with match in Rust
DESCRIPTION: Demonstrates using a match expression to handle an Option<u8> value, executing code only for the Some variant.

LANGUAGE: rust
CODE:
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

----------------------------------------

TITLE: Generating Markdown Table of Contents for Rust Documentation
DESCRIPTION: This markdown snippet creates a structured table of contents for Rust programming language documentation. It includes links to various sections and subsections, covering topics from basic syntax to advanced concepts.

LANGUAGE: markdown
CODE:
# Summary

[Introduction](README.md)

* [Getting Started](getting-started.md)
* [Tutorial: Guessing Game](guessing-game.md)
* [Syntax and Semantics](syntax-and-semantics.md)
    * [Variable Bindings](variable-bindings.md)
    * [Functions](functions.md)
    * [Primitive Types](primitive-types.md)
    * [Comments](comments.md)
    * [if](if.md)
    * [Loops](loops.md)
    * [Vectors](vectors.md)
    * [Ownership](ownership.md)
    * [References and Borrowing](references-and-borrowing.md)
    * [Lifetimes](lifetimes.md)
    * [Mutability](mutability.md)
    * [Structs](structs.md)
    * [Enums](enums.md)
    * [Match](match.md)
    * [Patterns](patterns.md)
    * [Method Syntax](method-syntax.md)
    * [Strings](strings.md)
    * [Generics](generics.md)
    * [Traits](traits.md)
    * [Drop](drop.md)
    * [if let](if-let.md)
    * [Trait Objects](trait-objects.md)
    * [Closures](closures.md)
    * [Universal Function Call Syntax](ufcs.md)
    * [Crates and Modules](crates-and-modules.md)
    * [`const` and `static`](const-and-static.md)
    * [Attributes](attributes.md)
    * [`type` aliases](type-aliases.md)
    * [Casting between types](casting-between-types.md)
    * [Associated Types](associated-types.md)
    * [Unsized Types](unsized-types.md)
    * [Operators and Overloading](operators-and-overloading.md)
    * [Deref coercions](deref-coercions.md)
    * [Macros](macros.md)
    * [Raw Pointers](raw-pointers.md)
    * [`unsafe`](unsafe.md)
* [Effective Rust](effective-rust.md)
    * [The Stack and the Heap](the-stack-and-the-heap.md)
    * [Testing](testing.md)
    * [Conditional Compilation](conditional-compilation.md)
    * [Documentation](documentation.md)
    * [Iterators](iterators.md)
    * [Concurrency](concurrency.md)
    * [Error Handling](error-handling.md)
    * [Choosing your Guarantees](choosing-your-guarantees.md)
    * [FFI](ffi.md)
    * [Borrow and AsRef](borrow-and-asref.md)
    * [Release Channels](release-channels.md)
    * [Using Rust without the standard library](using-rust-without-the-standard-library.md)
    * [Procedural Macros (and custom derive)](procedural-macros.md)
* [Glossary](glossary.md)
* [Syntax Index](syntax-index.md)
* [Bibliography](bibliography.md)

----------------------------------------

TITLE: Module Tree Structure Visualization
DESCRIPTION: Text representation of the module hierarchy showing the relationship between different modules in the restaurant example

LANGUAGE: text
CODE:
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

----------------------------------------

TITLE: Sending and Receiving Multiple Messages Over an Async Channel in Rust
DESCRIPTION: This example shows how to send and receive multiple messages over an async channel, using sleep to introduce delays between messages.

LANGUAGE: rust
CODE:
trpl::run(async {
    let (tx, mut rx) = trpl::channel();

    let vals = vec!["hi", "from", "the", "thread"];

    for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(500).await;
    }

    while let Some(message) = rx.recv().await {
        println!("got: {}", message);
    }
});

----------------------------------------

TITLE: Conditional if let Expression in Rust
DESCRIPTION: Shows how to use if let expressions for pattern matching with multiple conditions using favorite_color example.

LANGUAGE: rust
CODE:
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

----------------------------------------

TITLE: Creating and Printing a Box Smart Pointer in Rust
DESCRIPTION: This snippet demonstrates how to create a Box smart pointer containing an integer value and then print its contents. It showcases the basic usage of the Box type in Rust.

LANGUAGE: rust
CODE:
let b = Box::new(5);
println!("b = {b}");

----------------------------------------

TITLE: Creating a Raw Pointer to an Arbitrary Memory Address in Rust
DESCRIPTION: Shows how to create a raw pointer to an arbitrary memory address using type casting.

LANGUAGE: Rust
CODE:
let address = 0x012345usize;
let r = address as *const i32;

----------------------------------------

TITLE: Combining Threads and Async
DESCRIPTION: An example showing how to use threads for sending messages and async for receiving them.

LANGUAGE: rust
CODE:
use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}

----------------------------------------

TITLE: Using Mutable Variables in Rust
DESCRIPTION: This code snippet shows how to declare a mutable variable in Rust using the 'mut' keyword, allowing the value to be changed after initial assignment.

LANGUAGE: rust
CODE:
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

----------------------------------------

TITLE: Implementing Drop Trait for Custom Smart Pointer in Rust
DESCRIPTION: Example showing how to implement the Drop trait for a custom smart pointer structure. The implementation defines custom cleanup behavior that prints a message when the value goes out of scope. The example creates two instances of CustomSmartPointer with different data values.

LANGUAGE: rust
CODE:
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}

----------------------------------------

TITLE: Running Rust Program with Lifetime Error
DESCRIPTION: Terminal output showing a Rust compilation error where string2 doesn't live long enough for its borrowed reference to remain valid. The error occurs because the reference is used after string2 is dropped.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
5 |         let string2 = String::from("xyz");
  |             ------- binding `string2` declared here
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {result}");
  |                                     -------- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error

----------------------------------------

TITLE: Executing Rust Program with Cargo
DESCRIPTION: Terminal commands and output showing the compilation and execution of a Rust program using Cargo. The program outputs incremental values from 10 to 50 in steps of 10.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50

----------------------------------------

TITLE: Creating a New Hash Map in Rust
DESCRIPTION: This snippet demonstrates how to create a new hash map and insert key-value pairs. It uses the HashMap type from the standard library to store team names as keys and their scores as values.

LANGUAGE: rust
CODE:
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

----------------------------------------

TITLE: Demonstrating Immutability in Rust
DESCRIPTION: This code snippet demonstrates Rust's default immutability by attempting to reassign a value to an immutable variable, which results in a compilation error.

LANGUAGE: rust
CODE:
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

----------------------------------------

TITLE: Final ThreadPool Shutdown Implementation
DESCRIPTION: Complete implementation of graceful shutdown including sender dropping and worker cleanup.

LANGUAGE: rust
CODE:
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }

----------------------------------------

TITLE: Demonstrating Loop Structures in Rust
DESCRIPTION: This code snippet showcases the three types of loops in Rust: an infinite loop using 'loop', a conditional loop using 'while', and an iterative loop using 'for'. It demonstrates basic syntax and usage patterns for each loop type.

LANGUAGE: rust
CODE:
loop {
    println!("again!");
}

let mut number = 3;
while number != 0 {
    println!("{number}!");
    number = number - 1;
}

let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("the value is: {element}");
}

----------------------------------------

TITLE: Updating a Vector in Rust
DESCRIPTION: Shows how to add elements to a vector using the push method

LANGUAGE: rust
CODE:
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

----------------------------------------

TITLE: Running Cargo Tests for Rust Project
DESCRIPTION: This snippet shows the output of running 'cargo test' command for a Rust project. It includes compilation details, test execution, and a failed test result for the 'it_adds_two' test.

LANGUAGE: plaintext
CODE:
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----

thread 'tests::it_adds_two' panicked at src/lib.rs:12:9:
assertion `left == right` failed
  left: 5
 right: 4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Initializing Vectors in Rust
DESCRIPTION: Demonstrates two ways to create vectors in Rust: using Vec::new() for an empty vector and the vec! macro for a pre-populated vector. The first example shows type annotation for an integer vector, while the second creates a vector of integers directly.

LANGUAGE: rust
CODE:
let v: Vec<i32> = Vec::new();
let numbers = vec![1, 2, 3];

----------------------------------------

TITLE: Declaring Character Values in Rust
DESCRIPTION: Demonstrates how to declare and initialize character values in Rust. Characters in Rust are represented by the 'char' type and are specified using single quotes.

LANGUAGE: rust
CODE:
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

----------------------------------------

TITLE: Rust Path and Module Syntax
DESCRIPTION: Examples of path-related syntax in Rust, showing module hierarchy navigation and item access patterns.

LANGUAGE: rust
CODE:
ident::ident         // Namespace path
::path              // Path relative to crate root
self::path          // Path relative to current module
super::path         // Path relative to parent module
type::ident         // Associated items
<type as trait>::ident  // Associated items with trait specification

----------------------------------------

TITLE: Implementing Drop Trait for ThreadPool
DESCRIPTION: First attempt at implementing Drop trait to join threads when thread pool is dropped. This version has compilation errors due to ownership issues.

LANGUAGE: rust
CODE:
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }

----------------------------------------

TITLE: Splitting Mutable Slice in Rust (Incorrect Implementation)
DESCRIPTION: This code snippet demonstrates an incorrect attempt to split a mutable slice into two mutable sub-slices. It results in a compilation error due to multiple mutable borrows of the same data.

LANGUAGE: rust
CODE:
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    (&mut values[..mid], &mut values[mid..])
}

----------------------------------------

TITLE: Using Result<T, E> for Recoverable Errors in Rust
DESCRIPTION: Rust uses the Result<T, E> type to handle recoverable errors. This allows for explicit error handling and encourages developers to consider and handle potential error cases.

LANGUAGE: rust
CODE:
Result<T, E>

----------------------------------------

TITLE: Compiling and Running Rust Drop Example
DESCRIPTION: This snippet shows the process of compiling and running a Rust program that demonstrates the Drop trait. It includes the cargo command used and the resulting output, which shows the creation and dropping of CustomSmartPointer instances.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/drop-example`
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.

----------------------------------------

TITLE: Defining a Cons List with Box<T>
DESCRIPTION: Shows how to define a cons list using Box<T> to create a recursive type with a known size.

LANGUAGE: rust
CODE:
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

----------------------------------------

TITLE: Implementing a Procedural Macro for Custom Derive in Rust
DESCRIPTION: Shows the implementation of a procedural macro for custom derive in Rust, generating code to implement a trait.

LANGUAGE: Rust
CODE:
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

----------------------------------------

TITLE: Demonstrating Pattern Matching in Rust
DESCRIPTION: This code snippet showcases pattern matching in Rust using the 'match' expression. It matches against an Option<i32> and demonstrates different pattern cases including a specific value, a variable binding, and a default case.

LANGUAGE: rust
CODE:
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}

----------------------------------------

TITLE: Rust Path and Module Syntax
DESCRIPTION: Examples of path-related syntax for accessing modules and items

LANGUAGE: rust
CODE:
::path           // Absolute path
self::path       // Path relative to current module
super::path      // Path relative to parent module
type::ident      // Associated items

----------------------------------------

TITLE: Common Test Setup Module in Rust
DESCRIPTION: Example of creating a common test setup module for sharing code between integration tests.

LANGUAGE: rust
CODE:
pub fn setup() {
    // setup code specific to your tests
}

----------------------------------------

TITLE: Creating an IpAddr Instance with Hardcoded String in Rust
DESCRIPTION: This snippet demonstrates creating an IpAddr instance by parsing a hardcoded IP address string. It uses expect() to handle the Result, assuming the hardcoded string is always valid.

LANGUAGE: rust
CODE:
let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");

----------------------------------------

TITLE: Filtering a Stream in Rust
DESCRIPTION: Shows how to use the StreamExt::filter method to filter out elements from a stream that are not multiples of three or five.

LANGUAGE: Rust
CODE:
use trpl::StreamExt;

async fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut stream = trpl::stream_from_iter(numbers.iter().cloned())
        .filter(|n| n % 3 == 0 || n % 5 == 0);

    while let Some(value) = stream.next().await {
        println!("Value: {}", value);
    }
}

----------------------------------------

TITLE: Counting non-quarter coins with if let and else in Rust
DESCRIPTION: Shows how to use if let with an else clause to achieve the same functionality as the previous match expression.

LANGUAGE: rust
CODE:
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}

----------------------------------------

TITLE: Cargo Project Creation Commands
DESCRIPTION: Commands to create and navigate to a new Cargo project directory.

LANGUAGE: shell
CODE:
cargo new hello_cargo
cd hello_cargo

----------------------------------------

TITLE: Integration Test with Common Module
DESCRIPTION: Shows how to use a common test module in integration tests for shared functionality.

LANGUAGE: rust
CODE:
use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

----------------------------------------

TITLE: Using join! Macro for Multiple Futures in Rust
DESCRIPTION: Demonstrates how to use the join! macro to await multiple futures concurrently. This approach allows waiting on an arbitrary number of futures without changing the function call.

LANGUAGE: rust
CODE:
let (tx1, rx1) = mpsc::channel();
let (tx2, rx2) = mpsc::channel();
let (tx3, rx3) = mpsc::channel();

let tx1_fut = async move {
    tx1.send("hi").unwrap();
    tx1.send("from").unwrap();
    tx1.send("the").unwrap();
};

let tx2_fut = async move {
    tx2.send("more").unwrap();
    tx2.send("messages").unwrap();
};

let tx3_fut = async move {
    tx3.send("for").unwrap();
    tx3.send("you").unwrap();
};

let rx_fut = async {
    while let Some(msg) = try_join!(rx1.recv(), rx2.recv(), rx3.recv()).await {
        println!("received '{}'\n", msg);
    }
};

join!(tx1_fut, tx2_fut, tx3_fut, rx_fut);

----------------------------------------

TITLE: Casting Integer to Float in Rust Average Function
DESCRIPTION: This code snippet demonstrates type casting in Rust within a function that calculates the average of a slice of f64 values. It uses the 'as' operator to cast the length of the slice from i32 to f64 for accurate division.

LANGUAGE: rust
CODE:
fn average(values: &[f64]) -> f64 {
    let sum: f64 = sum(values);
    let size: f64 = len(values) as f64;
    sum / size
}

----------------------------------------

TITLE: Implementing Encapsulation with Struct and Methods in Rust
DESCRIPTION: Demonstrates encapsulation in Rust using a struct 'AveragedCollection' with private fields and public methods. The struct maintains a list of integers and their average, updating the average whenever the list changes.

LANGUAGE: rust
CODE:
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

LANGUAGE: rust
CODE:
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

----------------------------------------

TITLE: Improving I/O Project with Iterators in Rust
DESCRIPTION: Refactoring the Config::build function to use iterators instead of indexing.

LANGUAGE: Rust
CODE:
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

----------------------------------------

TITLE: Combining Threads and Async in Rust
DESCRIPTION: Shows a practical example of using threads and async together, where blocking operations run in a thread while async code handles message processing.

LANGUAGE: rust
CODE:
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = trpl::channel();
    thread::spawn(move || {
        for i in 1..=10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    trpl::run(async {
        while let Ok(n) = rx.recv().await {
            println!("received: {}", n);
        }
    });
}

----------------------------------------

TITLE: Running Cargo Test for Rust Greeter Project
DESCRIPTION: This snippet shows the command to run tests and the resulting output for a Rust project named 'greeter'. The test fails because the greeting function does not include the expected name 'Carol' in its output.

LANGUAGE: plaintext
CODE:
$ cargo test
   Compiling greeter v0.1.0 (file:///projects/greeter)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running unittests src/lib.rs (target/debug/deps/greeter-170b942eb5bf5e3a)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----

thread 'tests::greeting_contains_name' panicked at src/lib.rs:12:9:
assertion failed: result.contains("Carol")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Compiling and Running a Rust Project with Cargo
DESCRIPTION: This snippet shows the terminal output when compiling and running a Rust project named 'cons-list' using Cargo. It demonstrates the build process and the program's output, which tracks the count of some objects as they are created and destroyed.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/cons-list`
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2

----------------------------------------

TITLE: Declaring Boolean Values in Rust
DESCRIPTION: Shows how to declare and initialize Boolean values in Rust. Boolean types in Rust are represented by the 'bool' keyword and can be either true or false.

LANGUAGE: rust
CODE:
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

----------------------------------------

TITLE: Using move Closures with Threads in Rust
DESCRIPTION: Demonstrates how to use the move keyword with closures in threads to transfer ownership of values from one thread to another.

LANGUAGE: rust
CODE:
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

----------------------------------------

TITLE: Implementing approve Method in Rust
DESCRIPTION: Adds the approve method to Post and State to change post state to Published.

LANGUAGE: rust
CODE:
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

----------------------------------------

TITLE: Running the Rust program with command line arguments
DESCRIPTION: Demonstrates how to run the 'minigrep' program with command line arguments using Cargo.

LANGUAGE: console
CODE:
$ cargo run -- searchstring example-filename.txt

----------------------------------------

TITLE: Implementing PartialEq and Eq Traits in Rust
DESCRIPTION: PartialEq allows comparison for equality, enabling the use of == and != operators. Eq signals that a value is equal to itself. These traits are used in situations requiring equality comparisons, such as with HashMap keys.

LANGUAGE: rust
CODE:
#[derive(PartialEq, Eq)]
struct SomeStruct {
    // fields
}

----------------------------------------

TITLE: Generating SVG from Graphviz DOT File in Bash
DESCRIPTION: This command uses Graphviz to generate an SVG file from a DOT file for diagrams in the book. It outputs the result to the src/img directory.

LANGUAGE: bash
CODE:
$ dot dot/trpl04-01.dot -Tsvg > src/img/trpl04-01.svg

----------------------------------------

TITLE: Using Box<dyn Fn> for Closure Collections in Rust
DESCRIPTION: Shows how to create a collection of different closure implementations using trait objects with Box<dyn Fn>.

LANGUAGE: rust
CODE:
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure() -> Box<dyn Fn(i32) -> i32> {
    let init = 1;
    Box::new(move |x| x + init)
}

let mut closures = vec![];
closures.push(returns_closure());
closures.push(returns_initialized_closure());

----------------------------------------

TITLE: Cargo Build and Run Output
DESCRIPTION: Terminal output showing compilation and execution of a Rust program named 'rectangles'. The output includes compilation statistics and a debug print of a Rectangle struct instance.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle {
    width: 30,
    height: 50,
}

----------------------------------------

TITLE: Correct Usage of drop Function in Rust
DESCRIPTION: This snippet demonstrates the correct way to manually drop an object in Rust using the `drop` function. This is the recommended approach when you need to explicitly release resources.

LANGUAGE: rust
CODE:
drop(c);

----------------------------------------

TITLE: Reading File Contents in Rust
DESCRIPTION: This snippet demonstrates how to read the contents of a file specified by a command-line argument. It uses fs::read_to_string from the standard library to read the file into a String. The code also includes basic error handling with the ? operator.

LANGUAGE: rust
CODE:
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

----------------------------------------

TITLE: Defining an Async Function to Get a Page Title
DESCRIPTION: An async function that fetches a web page, extracts its title, and returns it as an Option<String>.

LANGUAGE: rust
CODE:
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

----------------------------------------

TITLE: Extracting Logic to a Library Crate in Rust
DESCRIPTION: Moves core functionality into a separate library crate, keeping only the main function in the binary crate.

LANGUAGE: rust
CODE:
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}

----------------------------------------

TITLE: Creating Raw Pointers in Rust
DESCRIPTION: Demonstrates how to create immutable and mutable raw pointers using raw borrow operators in Rust.

LANGUAGE: rust
CODE:
let mut num = 5;
let r1 = &raw const num;
let r2 = &raw mut num;

----------------------------------------

TITLE: Implementing Generic Point Struct in Rust
DESCRIPTION: Defines a Point struct with two generic type parameters T and U, allowing for flexible coordinate types. The example shows initialization with different numeric types including integers and floating-point numbers.

LANGUAGE: rust
CODE:
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

----------------------------------------

TITLE: Implementing ThreadPool Interface in Rust Web Server
DESCRIPTION: This snippet shows the ideal interface for a ThreadPool struct in a Rust web server. It creates a new thread pool with a configurable number of threads and uses the execute method to run closures in the pool.

LANGUAGE: rust
CODE:
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

----------------------------------------

TITLE: Documenting Initial Release in Markdown
DESCRIPTION: This snippet documents the initial release (version 0.1.0) of a Rust project. It notes the addition of support code for the first draft of a new async chapter in an associated book.

LANGUAGE: Markdown
CODE:
# CHANGELOG

## 0.1.0

Initial release! Adds support code for the first draft of the new async chapter of the book.

----------------------------------------

TITLE: Demonstrating Raw Identifiers in Rust
DESCRIPTION: Shows how to use raw identifiers to use keywords as function names in Rust.

LANGUAGE: Rust
CODE:
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}

----------------------------------------

TITLE: Implementing a Generic Function in Rust
DESCRIPTION: Demonstrates how to create a generic 'largest' function that works with different types using a type parameter T.

LANGUAGE: rust
CODE:
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

----------------------------------------

TITLE: Cargo.toml Package Configuration
DESCRIPTION: Example of required metadata configuration in Cargo.toml for publishing a crate.

LANGUAGE: toml
CODE:
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]

----------------------------------------

TITLE: Basic Variable Binding in Rust
DESCRIPTION: Demonstrates the basic syntax for creating a variable binding in Rust using the 'let' keyword. The example shows binding the integer value 5 to the variable name 'foo'.

LANGUAGE: rust
CODE:
let foo = 5;

----------------------------------------

TITLE: Using super Keyword for Relative Paths in Rust
DESCRIPTION: Shows how to use the super keyword to reference parent modules in relative paths, demonstrated through a restaurant order management system.

LANGUAGE: rust
CODE:
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

----------------------------------------

TITLE: Basic Panic Example in Rust
DESCRIPTION: Demonstrates a simple use of the panic! macro to crash the program with a custom error message.

LANGUAGE: rust
CODE:
fn main() {
    panic!("crash and burn");
}

----------------------------------------

TITLE: Formatting Rust Code with cargo fmt
DESCRIPTION: Demonstrates how to use cargo fmt to automatically format Rust code according to community style guidelines.

LANGUAGE: bash
CODE:
$ cargo fmt

----------------------------------------

TITLE: Running Cargo Tests for Rust Project
DESCRIPTION: This snippet shows the output of running 'cargo test' command for a Rust project. It includes compilation information, test execution details, and test results. The output indicates that one test passed and one test failed due to an assertion error.

LANGUAGE: plaintext
CODE:
$ cargo test
   Compiling silly-function v0.1.0 (file:///projects/silly-function)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests src/lib.rs (target/debug/deps/silly_function-160869f38cff9166)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8

thread 'tests::this_test_will_fail' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 10
 right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Implementing Encapsulated Data Structure in Rust
DESCRIPTION: Example of encapsulation in Rust using a struct AveragedCollection that maintains a list of integers and their average, demonstrating private fields and public methods.

LANGUAGE: rust
CODE:
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

----------------------------------------

TITLE: Creating Raw Pointers in Rust
DESCRIPTION: Demonstrates how to create immutable and mutable raw pointers in Rust using the raw borrow operators.

LANGUAGE: Rust
CODE:
let mut num = 5;

let r1 = &raw const num;
let r2 = &raw mut num;

----------------------------------------

TITLE: Running Rust Program with Cargo
DESCRIPTION: This snippet shows the command to run a Rust program using Cargo, the Rust package manager and build system. It compiles and executes a program named 'variables'.

LANGUAGE: shell
CODE:
$ cargo run

----------------------------------------

TITLE: Using Environment Variables for Configuration in Rust
DESCRIPTION: Checks for an IGNORE_CASE environment variable to determine whether to perform case-sensitive or case-insensitive search.

LANGUAGE: rust
CODE:
use std::env;

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // --snip--

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

----------------------------------------

TITLE: Creating Basic Hello World Program in Rust
DESCRIPTION: A simple Rust program that prints 'Hello, world!' to the console. This demonstrates the basic structure of a Rust program including the main function declaration and use of the println! macro.

LANGUAGE: rust
CODE:
fn main() {
    println!("Hello, world!");
}

----------------------------------------

TITLE: Creating an Async Channel in Rust
DESCRIPTION: This snippet demonstrates how to create an async channel using trpl::channel and send/receive messages asynchronously.

LANGUAGE: rust
CODE:
trpl::run(async {
    let (tx, mut rx) = trpl::channel();

    tx.send("hello").unwrap();
    let message = rx.recv().await.unwrap();
    println!("got {}", message);
});

----------------------------------------

TITLE: Running Specific Rust Test with Cargo
DESCRIPTION: This snippet shows the command to run a specific test named 'one_hundred' in a Rust project using Cargo, along with the resulting output. It includes the compilation process, test execution details, and the final test result.

LANGUAGE: shell
CODE:
$ cargo test one_hundred
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

----------------------------------------

TITLE: Basic Enum Definition in Rust
DESCRIPTION: Defines a basic enum IpAddrKind with two variants for IP address types.

LANGUAGE: rust
CODE:
enum IpAddrKind {
    V4,
    V6,
}

----------------------------------------

TITLE: Using Arc<T> for Thread-Safe Reference Counting
DESCRIPTION: This code snippet demonstrates the correct way to share a Mutex<T> between multiple threads using Arc<T> for atomic reference counting. It allows multiple threads to safely increment a shared counter.

LANGUAGE: rust
CODE:
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

----------------------------------------

TITLE: Defining Front of House Module Content in Rust
DESCRIPTION: This code defines the content of the front_of_house module in a separate file, including the hosting submodule declaration.

LANGUAGE: rust
CODE:
pub mod hosting {
    pub fn add_to_waitlist() {}
}

----------------------------------------

TITLE: Tracking Reference Count Changes with Rc<T> in Rust
DESCRIPTION: This code snippet demonstrates how to track and print reference count changes when using Rc<T>. It shows the effect of creating and dropping references on the reference count.

LANGUAGE: rust
CODE:
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

----------------------------------------

TITLE: State Pattern Implementation in Rust
DESCRIPTION: Example of implementing the State pattern using trait objects to manage blog post states.

LANGUAGE: rust
CODE:
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

----------------------------------------

TITLE: Creating a new empty String in Rust
DESCRIPTION: This snippet demonstrates how to create a new, empty String using the String::new() method.

LANGUAGE: rust
CODE:
let s = String::new();

----------------------------------------

TITLE: Struct with Enum and String Fields
DESCRIPTION: Defines a struct that combines an enum variant with string data for IP addresses.

LANGUAGE: rust
CODE:
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

----------------------------------------

TITLE: Implementing Draw Trait for GUI Components in Rust
DESCRIPTION: Example showing how to implement the Draw trait for different GUI components (InputBox and Button) and create a Screen struct that can handle multiple drawable components. Demonstrates trait objects usage with generic constraints and vector storage of components.

LANGUAGE: rust
CODE:
pub struct InputBox {
    pub label: String,
}

impl Draw for InputBox {
    fn draw(&self) {
        // Code to actually draw an input box
    }
}

pub struct Button {
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to actually draw a button
    }
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(InputBox {
                label: String::from("OK"),
            }),
            Box::new(Button {
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

----------------------------------------

TITLE: Defining the Draw Trait in Rust
DESCRIPTION: This snippet defines a trait named Draw with a single method draw. This trait will be used to create trait objects for different types that can be drawn.

LANGUAGE: rust
CODE:
pub trait Draw {
    fn draw(&self);
}

----------------------------------------

TITLE: Multiple Named Tests in Rust
DESCRIPTION: Example showing how to create multiple test functions with different names for selective test execution.

LANGUAGE: rust
CODE:
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

----------------------------------------

TITLE: Creating User Struct Instance
DESCRIPTION: Demonstrates how to create an instance of the User struct by providing values for all fields.

LANGUAGE: rust
CODE:
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

----------------------------------------

TITLE: Basic Panic Example in Rust
DESCRIPTION: Simple demonstration of triggering a panic in Rust using the panic! macro directly.

LANGUAGE: rust
CODE:
fn main() {
    panic!("crash and burn");
}

----------------------------------------

TITLE: Calling next() Method on an Iterator in Rust
DESCRIPTION: This example demonstrates calling the next() method directly on an iterator to retrieve items one by one. It shows how the iterator state changes with each call.

LANGUAGE: rust
CODE:
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

----------------------------------------

TITLE: Simulating a Slow Request in Rust Web Server
DESCRIPTION: This snippet shows how to implement a slow request handler in a Rust web server. It uses a match statement to handle different request paths and simulates a slow response by sleeping for 5 seconds when the /sleep path is requested.

LANGUAGE: rust
CODE:
match request_line {
    "GET / HTTP/1.1" => {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
    "GET /sleep HTTP/1.1" => {
        thread::sleep(Duration::from_secs(5));
        let status_line = "HTTP/1.1 200 OK";
        let contents = "Hello from sleep!";
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
    _ => {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}

----------------------------------------

TITLE: Rust Installation Command for Unix Systems
DESCRIPTION: Command to download and install Rust using rustup on Linux or macOS systems.

LANGUAGE: shell
CODE:
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

----------------------------------------

TITLE: Basic Cargo Version Check Command
DESCRIPTION: Command to verify Cargo installation by checking its version

LANGUAGE: console
CODE:
$ cargo --version

----------------------------------------

TITLE: Multiple Lines in Match Arm in Rust
DESCRIPTION: Shows how to use multiple lines of code in a match arm, including printing a message before returning a value.

LANGUAGE: rust
CODE:
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

----------------------------------------

TITLE: Restaurant Module Structure Implementation
DESCRIPTION: Example demonstrating how to organize restaurant-related code using nested modules in Rust, showing front of house operations with hosting and serving submodules

LANGUAGE: rust
CODE:
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

----------------------------------------

TITLE: Creating an Instance of the User Struct in Rust
DESCRIPTION: This code snippet demonstrates how to create an instance of the User struct and assign values to its fields.

LANGUAGE: rust
CODE:
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

----------------------------------------

TITLE: Implementing First Word Function with Byte Index in Rust
DESCRIPTION: This function takes a String reference and returns the index of the first space character or the length of the string if no space is found. It demonstrates iterating over string bytes and using enumerate.

LANGUAGE: rust
CODE:
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

----------------------------------------

TITLE: Running Cargo Test Command
DESCRIPTION: Shows the execution of cargo test command and its output including test compilation, test execution results and documentation tests summary.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

----------------------------------------

TITLE: Checking PATH in Windows CMD
DESCRIPTION: This command displays the PATH environment variable in Windows Command Prompt, which is useful for troubleshooting Rust installation issues.

LANGUAGE: console
CODE:
> echo %PATH%

----------------------------------------

TITLE: Appending to a String in Rust
DESCRIPTION: This example shows how to append a string slice to a String using the push_str() method and how to append a single character using push().

LANGUAGE: rust
CODE:
let mut s = String::from("foo");
s.push_str("bar");

let mut s = String::from("lo");
s.push('l');

----------------------------------------

TITLE: Using Let-Else with Refutable Pattern in Rust
DESCRIPTION: Shows the correct way to handle refutable patterns using let...else, allowing the code to handle cases where the pattern doesn't match.

LANGUAGE: rust
CODE:
let Some(x) = some_option_value else {
    panic!("Expected a value");
};

----------------------------------------

TITLE: Implementing age check for UsState in Rust
DESCRIPTION: Defines a method on UsState to check if the state existed in 1900.

LANGUAGE: rust
CODE:
impl UsState {
    fn existed_in_1900(&self) -> bool {
        // implementation details omitted
    }
}

----------------------------------------

TITLE: Basic Hello World Program in Rust
DESCRIPTION: A simple Rust program that prints 'Hello, world!' to the console. This demonstrates the basic structure of a Rust program with a main function.

LANGUAGE: rust
CODE:
fn main() {
    println!("Hello, world!");
}

----------------------------------------

TITLE: Deref Coercion Example in Rust
DESCRIPTION: Demonstrates deref coercion with a custom smart pointer and string types.

LANGUAGE: rust
CODE:
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

----------------------------------------

TITLE: Basic If-Else Condition Example in Rust
DESCRIPTION: Demonstrates a basic if-else expression in Rust that checks if a number is less than 5 and prints different messages based on the condition. The example shows the standard syntax for conditional branching in Rust.

LANGUAGE: rust
CODE:
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

----------------------------------------

TITLE: Implementing Width Method with Field Name in Rust
DESCRIPTION: Shows how to implement a method with the same name as a struct field. The method returns a boolean indicating if the width is greater than 0.

LANGUAGE: rust
CODE:
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

----------------------------------------

TITLE: Integrating Case-Insensitive Search in Rust Program
DESCRIPTION: This snippet shows how to modify the 'run' function to use either case-sensitive or case-insensitive search based on a configuration option.

LANGUAGE: rust
CODE:
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

----------------------------------------

TITLE: Using sum() Method on an Iterator in Rust
DESCRIPTION: This snippet shows how to use the sum() method, a consuming adapter, to calculate the sum of all items in an iterator. It demonstrates that sum() takes ownership of the iterator.

LANGUAGE: rust
CODE:
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

----------------------------------------

TITLE: Loop with Counter in Rust
DESCRIPTION: Demonstrates using a loop with a counter to accumulate and return a value.

LANGUAGE: rust
CODE:
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

----------------------------------------

TITLE: Lifetime Annotation in Struct
DESCRIPTION: Example of using lifetime annotations in a struct that holds a reference

LANGUAGE: rust
CODE:
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

----------------------------------------

TITLE: Checking Environment Variable for Case-Insensitive Search in Rust
DESCRIPTION: This snippet demonstrates how to check for an environment variable 'IGNORE_CASE' to determine whether to perform a case-insensitive search.

LANGUAGE: rust
CODE:
use std::env;

// --snip--

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

----------------------------------------

TITLE: Using ? Operator in Rust Main Function
DESCRIPTION: This snippet demonstrates an incorrect usage of the ? operator in the main function, which leads to a compilation error. The ? operator can only be used in functions that return Result or Option.

LANGUAGE: rust
CODE:
fn main() {
    let greeting_file = File::open("hello.txt")?;
}

----------------------------------------

TITLE: Opening a File with Error Handling in Rust
DESCRIPTION: This code demonstrates opening a file and using a match expression to handle the Result returned by File::open.

LANGUAGE: rust
CODE:
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

----------------------------------------

TITLE: Creating and Using a Stream from an Iterator in Rust
DESCRIPTION: Demonstrates how to create a stream from an iterator and print its values asynchronously using the StreamExt trait.

LANGUAGE: Rust
CODE:
use trpl::StreamExt;

async fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let mut stream = trpl::stream_from_iter(numbers.iter().map(|n| n * 2));

    while let Some(value) = stream.next().await {
        println!("Value: {}", value);
    }
}

----------------------------------------

TITLE: Rust Generic Type Parameters
DESCRIPTION: Demonstration of generic type parameter syntax in Rust, including constraints and trait bounds.

LANGUAGE: rust
CODE:
Vec<u8>              // Generic type with parameter
fn ident<T> ...      // Generic function
struct ident<T> ...  // Generic structure
enum ident<T> ...    // Generic enumeration
impl<T> ...          // Generic implementation
T: U                 // Generic constraint
T: 'a               // Lifetime constraint

----------------------------------------

TITLE: While Let Loop in Rust
DESCRIPTION: Example of using while let for conditional looping based on pattern matching with channel communication.

LANGUAGE: rust
CODE:
let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for val in [1, 2, 3] {
        tx.send(val).unwrap();
    }
});

while let Ok(value) = rx.recv() {
    println!("{value}");
}

----------------------------------------

TITLE: Implementing the Add Trait for Point Struct in Rust
DESCRIPTION: Demonstrates how to implement the Add trait to overload the + operator for Point instances in Rust.

LANGUAGE: Rust
CODE:
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

----------------------------------------

TITLE: Executing a Rust Project with Cargo
DESCRIPTION: This snippet shows the command to run a Rust project and the resulting output. It demonstrates the compilation process, build completion, and program execution using Cargo.

LANGUAGE: Shell
CODE:
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `file:///projects/guessing_game/target/debug/guessing_game`
Hello, world!

----------------------------------------

TITLE: Defining a User Struct in Rust
DESCRIPTION: This code snippet defines a User struct with four fields: active, username, email, and sign_in_count.

LANGUAGE: rust
CODE:
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

----------------------------------------

TITLE: Improved Code Using Standard PI Constant
DESCRIPTION: Improved version of the circle area calculation using the standard library's PI constant, as suggested by Clippy.

LANGUAGE: rust
CODE:
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}

----------------------------------------

TITLE: Attempting Manual Drop Method Call in Rust (Compiler Error)
DESCRIPTION: This code snippet shows an attempt to manually call the drop method from the Drop trait, which results in a compiler error. It illustrates that Rust doesn't allow explicit calls to the drop method.

LANGUAGE: rust
CODE:
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}

----------------------------------------

TITLE: Creating a New Cargo Project
DESCRIPTION: Example showing how to create a new binary project with Cargo and the default directory structure it creates.

LANGUAGE: bash
CODE:
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs

----------------------------------------

TITLE: Testing Private Functions in Rust
DESCRIPTION: Demonstrates how to test private functions in Rust using the test module. Shows internal_adder function implementation and its test.

LANGUAGE: rust
CODE:
pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}

----------------------------------------

TITLE: Running Rust Drop Example with Cargo
DESCRIPTION: This snippet shows the process of compiling and running a Rust program named 'drop-example' using Cargo. It demonstrates the output of creating and dropping CustomSmartPointer objects.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/drop-example`
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!

----------------------------------------

TITLE: Basic Numeric Operations in Rust
DESCRIPTION: Demonstrates basic mathematical operations in Rust, including addition, subtraction, multiplication, division, and remainder. The example uses integer types for these operations.

LANGUAGE: rust
CODE:
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

----------------------------------------

TITLE: Illustrating Code Block Formatting in Rust Documentation
DESCRIPTION: Demonstrates the proper way to format code blocks in Rust documentation, including file name indication and syntax highlighting.

LANGUAGE: markdown
CODE:
```rust
// filename: main.rs
use std::io;

fn main() {
    println!("Hello, world!");
}
```

LANGUAGE: markdown
CODE:
```bash
$ cargo run
   Compiling hello_world v0.1.0 (file:///projects/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/hello_world`
Hello, world!
```

----------------------------------------

TITLE: Using Enum Initializers as Function Pointers in Rust
DESCRIPTION: Shows how to use enum variant initializers as function pointers that implement closure traits with the map method.

LANGUAGE: rust
CODE:
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> =
    (0u32..20).map(Status::Value).collect();

----------------------------------------

TITLE: Implementing a Thread Pool in Rust
DESCRIPTION: Defines a ThreadPool struct and its associated methods for creating and managing a pool of worker threads.

LANGUAGE: rust
CODE:
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

----------------------------------------

TITLE: Basic Match Expression in Rust
DESCRIPTION: Demonstrates a basic match expression pattern matching against a value x with multiple arms and a catch-all pattern.

LANGUAGE: rust
CODE:
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

----------------------------------------

TITLE: Checking PATH in Linux and macOS
DESCRIPTION: This command displays the PATH environment variable in Linux and macOS terminals, which is useful for troubleshooting Rust installation issues.

LANGUAGE: console
CODE:
$ echo $PATH

----------------------------------------

TITLE: Declaring Type Alias in Rust
DESCRIPTION: Demonstrates how to create a type alias in Rust using the 'type' keyword. In this example, 'Kilometers' is defined as an alias for the 'i32' type.

LANGUAGE: rust
CODE:
type Kilometers = i32;

----------------------------------------

TITLE: Main Function for Multithreaded Web Server in Rust
DESCRIPTION: Implements the main function that sets up the TCP listener, creates a thread pool, and handles incoming connections using the pool.

LANGUAGE: rust
CODE:
use hello::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

----------------------------------------

TITLE: Rust Generic Syntax
DESCRIPTION: Examples of generic type parameters and constraints

LANGUAGE: rust
CODE:
fn name<T>(param: T) {}        // Generic function
struct Name<T> {}            // Generic struct
impl<T> Name<T> {}          // Generic implementation
path::<T>::method()         // Turbofish syntax

----------------------------------------

TITLE: Iterating over characters in a Rust string
DESCRIPTION: This example shows how to iterate over the characters in a string using the chars() method.

LANGUAGE: rust
CODE:
for c in "Зд".chars() {
    println!("{c}");
}

----------------------------------------

TITLE: Using RefCell<T> for Interior Mutability
DESCRIPTION: Demonstrates how to use RefCell<T> to achieve interior mutability, allowing mutable borrows of immutable values.

LANGUAGE: rust
CODE:
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

----------------------------------------

TITLE: Using filter() Method with a Closure in Rust
DESCRIPTION: This snippet shows how to use the filter() method with a closure that captures its environment. It filters a collection of shoes based on a specified size.

LANGUAGE: rust
CODE:
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

----------------------------------------

TITLE: Implementing Drop Trait for CustomSmartPointer in Rust
DESCRIPTION: This code snippet demonstrates how to implement the Drop trait for a custom struct named CustomSmartPointer. The drop method is implemented to print a message when the instance goes out of scope.

LANGUAGE: rust
CODE:
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

----------------------------------------

TITLE: Using an Attribute-like Macro in Rust
DESCRIPTION: Shows an example of using an attribute-like macro for a web application framework.

LANGUAGE: rust
CODE:
#[route(GET, "/")]
fn index() {


----------------------------------------

TITLE: Demonstrating Async/Await Syntax in Rust
DESCRIPTION: This code snippet demonstrates the basic syntax of using async/await in Rust to fetch data asynchronously. It shows how to use the .await keyword to wait for the completion of an asynchronous operation without blocking the entire thread.

LANGUAGE: rust
CODE:
let data = fetch_data_from(url).await;
println!("{data}");

----------------------------------------

TITLE: Defining Hosting Submodule Content in Rust
DESCRIPTION: This code defines the content of the hosting submodule in its own file within the front_of_house directory.

LANGUAGE: rust
CODE:
pub fn add_to_waitlist() {}

----------------------------------------

TITLE: Defining a Generic Struct in Rust
DESCRIPTION: Shows how to create a generic Point struct that can hold x and y coordinates of any type T.

LANGUAGE: rust
CODE:
struct Point<T> {
    x: T,
    y: T,
}

----------------------------------------

TITLE: Rust Closure Type Mismatch Example
DESCRIPTION: Example showing a compilation error when attempting to use the same closure with both String and integer types. The closure's parameter type is inferred to be String from its first use, making the second call with an integer invalid.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |             --------------- ^- help: try using a conversion method: `.to_string()`
  |             |               |
  |             |               expected `String`, found integer
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `String`
 --> src/main.rs:4:29
  |
4 |     let s = example_closure(String::from("hello"));
  |             --------------- ^^^^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
  |             |
  |             in this closure call
note: closure parameter defined here
 --> src/main.rs:2:28
  |
2 |     let example_closure = |x| x;
  |                            ^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` (bin "closure-example") due to 1 previous error

----------------------------------------

TITLE: Running Rust Program with Lifetime Error
DESCRIPTION: Terminal output showing a Rust compilation error (E0597) where a reference to variable 'x' is used after the variable has been dropped from scope.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
5 |         let x = 5;
  |             - binding `x` declared here
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {r}");
  |                  --- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error

----------------------------------------

TITLE: Defining and Calling a Basic Function in Rust
DESCRIPTION: This snippet demonstrates how to define a simple function 'another_function' and call it from the main function. It shows the basic syntax for function declaration and invocation in Rust.

LANGUAGE: rust
CODE:
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}

----------------------------------------

TITLE: Conditional Trait Implementation in Rust
DESCRIPTION: Demonstrates conditionally implementing methods on a generic type based on trait bounds.

LANGUAGE: rust
CODE:
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

----------------------------------------

TITLE: Multiple Conditions with else if in Rust
DESCRIPTION: Shows how to handle multiple conditions using else if expressions to check number divisibility.

LANGUAGE: rust
CODE:
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

----------------------------------------

TITLE: Declaring Constants and Static Variables in Rust
DESCRIPTION: This snippet demonstrates how to declare constants and static variables in Rust. Constants are always immutable and set to constant expressions, while static variables are used for global variables in Rust.

LANGUAGE: rust
CODE:
const MAX_POINTS: u32 = 100_000;
static HELLO_WORLD: &str = "Hello, world!";

----------------------------------------

TITLE: Compiling and Running a Rust Project with Cargo
DESCRIPTION: This snippet shows the command to compile and run a Rust project using Cargo, along with the output it produces. It demonstrates the compilation process, execution time, and the program's output.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/functions`
Hello, world!
Another function.

----------------------------------------

TITLE: Implementing Blog Post Workflow with Types in Rust
DESCRIPTION: Reimplements the blog post workflow using distinct types for each state.

LANGUAGE: rust
CODE:
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

----------------------------------------

TITLE: Underscore Placeholder in Rust Match Expression
DESCRIPTION: Shows how to use the underscore (_) as a catch-all pattern when you don't need to use the matched value.

LANGUAGE: rust
CODE:
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

----------------------------------------

TITLE: Testing Function with println Output
DESCRIPTION: Example demonstrating how to test a function that includes println statements, showing both passing and failing test cases.

LANGUAGE: rust
CODE:
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

----------------------------------------

TITLE: Validating User Input for Number Guessing Game in Rust
DESCRIPTION: This code snippet shows how to validate user input in a number guessing game. It parses the input as an i32 and checks if it's within the valid range of 1 to 100.

LANGUAGE: rust
CODE:
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
        // --snip--
    }
}

----------------------------------------

TITLE: Creating a Vector with the vec! Macro in Rust
DESCRIPTION: Demonstrates how to use the vec! macro to create a new vector containing three integers.

LANGUAGE: rust
CODE:
let v: Vec<u32> = vec![1, 2, 3];

----------------------------------------

TITLE: Using await with a Join Handle in Rust
DESCRIPTION: This snippet shows how to use await with a join handle to run a task to completion in an async context.

LANGUAGE: rust
CODE:
trpl::run(async {
    let handle = trpl::spawn_task(async {
        for i in 1..5 {
            println!("hi number {} from the second task!", i);
            trpl::sleep(500).await;
        }
    });

    for i in 1..10 {
        println!("hi number {} from the first task!", i);
        trpl::sleep(500).await;
    }

    handle.await.unwrap();
});

----------------------------------------

TITLE: Returning Closures Using impl Trait in Rust
DESCRIPTION: Demonstrates how to return closures from functions using the impl Trait syntax, showing proper way to handle closure return types.

LANGUAGE: rust
CODE:
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

----------------------------------------

TITLE: Enum with String Data
DESCRIPTION: Shows how to attach String data directly to enum variants.

LANGUAGE: rust
CODE:
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

----------------------------------------

TITLE: Testing Private Functions in Rust
DESCRIPTION: Demonstrates how to test private functions in Rust using the tests module, showing that private functions can be accessed within tests.

LANGUAGE: rust
CODE:
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

----------------------------------------

TITLE: Creating a Tree Data Structure with Weak References
DESCRIPTION: Shows how to create a tree structure using Rc<T> and Weak<T> to avoid reference cycles.

LANGUAGE: rust
CODE:
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}

----------------------------------------

TITLE: Creating and Slicing Strings in Rust
DESCRIPTION: This code snippet demonstrates how to create a String from a string literal using the from function, and how to create string slices from parts of the String. It shows the use of range syntax for slicing.

LANGUAGE: rust
CODE:
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

----------------------------------------

TITLE: Running a Rust Program with Panic
DESCRIPTION: This snippet shows the command to run a Rust program using Cargo, the Rust package manager and build system. The program intentionally panics, demonstrating Rust's error handling mechanism.

LANGUAGE: shell
CODE:
$ cargo run

----------------------------------------

TITLE: Declaring Front of House Module in Rust
DESCRIPTION: This snippet shows how to declare the front_of_house module in the main lib.rs file, with the module's content moved to a separate file.

LANGUAGE: rust
CODE:
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

----------------------------------------

TITLE: Cargo Build and Run Output
DESCRIPTION: This snippet displays the output of compiling and running a Rust program. It shows the compilation process, build completion, and the program's output which prints the value of a variable 'x' twice.

LANGUAGE: plaintext
CODE:
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6

----------------------------------------

TITLE: Compiling and Running Rust Program with Cargo
DESCRIPTION: This snippet shows the process of compiling and running a Rust program named 'branches' using Cargo. It demonstrates the standard output of a successful compilation and execution, including compilation time and the program's output.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was true

----------------------------------------

TITLE: Basic Test Function Structure in Rust
DESCRIPTION: Demonstrates the basic structure of a test function in Rust, including the #[test] attribute and use of assert_eq! macro.

LANGUAGE: rust
CODE:
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

----------------------------------------

TITLE: Basic User Struct Definition in Rust
DESCRIPTION: Defines a User struct with fields for active status, username, email, and sign-in count. Shows the basic syntax for struct definition in Rust.

LANGUAGE: rust
CODE:
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

----------------------------------------

TITLE: Matching Literals in Rust
DESCRIPTION: Basic example of matching literal values in a pattern match expression

LANGUAGE: rust
CODE:
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}

----------------------------------------

TITLE: Executing Cargo Tests for Rust Project
DESCRIPTION: This snippet shows the command to run tests using Cargo and the initial compilation output. It demonstrates how to execute tests for a Rust project named 'minigrep'.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.97s
     Running unittests src/lib.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

----------------------------------------

TITLE: Using Closures with Iterator Map in Rust
DESCRIPTION: Shows how to use closures with the map method to transform a vector of numbers into strings using an inline closure.

LANGUAGE: rust
CODE:
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(|i| i.to_string()).collect();

----------------------------------------

TITLE: Building Cargo Project
DESCRIPTION: Command output when building a Cargo project for the first time

LANGUAGE: console
CODE:
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs

----------------------------------------

TITLE: Using a Non-Compiling Recursive List in Rust
DESCRIPTION: This snippet demonstrates how the non-compiling recursive List enum would be used to create a list containing the values 1, 2, and 3.

LANGUAGE: rust
CODE:
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

----------------------------------------

TITLE: Customizing Development Profile Optimization
DESCRIPTION: Example of overriding the default development profile optimization level in Cargo.toml, setting it to level 1 for a balance between compilation speed and runtime performance.

LANGUAGE: toml
CODE:
[profile.dev]
opt-level = 1

----------------------------------------

TITLE: Clippy Example: Approximate Constant Usage
DESCRIPTION: Shows a Rust code example that triggers a Clippy lint for using an approximate value of PI instead of the built-in constant.

LANGUAGE: rust
CODE:
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}

----------------------------------------

TITLE: Defining Multiple Traits with the Same Method Name in Rust
DESCRIPTION: Demonstrates how to define multiple traits with the same method name and implement them on a struct in Rust.

LANGUAGE: Rust
CODE:
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

----------------------------------------

TITLE: Idiomatic `use` Path for Functions in Rust
DESCRIPTION: This snippet demonstrates the idiomatic way to bring a function into scope using `use`, by importing the parent module rather than the function directly.

LANGUAGE: rust
CODE:
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

----------------------------------------

TITLE: Implementing a Shirt Giveaway with Closures in Rust
DESCRIPTION: This code snippet demonstrates the use of closures in a shirt giveaway scenario. It defines an Inventory struct with a giveaway method that uses a closure to determine the shirt color based on user preference.

LANGUAGE: rust
CODE:
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
}

----------------------------------------

TITLE: Declaring and Initializing Arrays in Rust
DESCRIPTION: Shows different ways to declare and initialize arrays in Rust. Arrays in Rust have a fixed length and all elements must be of the same type.

LANGUAGE: rust
CODE:
fn main() {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
}

----------------------------------------

TITLE: Creating Raw Pointers in Rust
DESCRIPTION: This snippet demonstrates how to create both constant and mutable raw pointers in Rust. It shows the conversion of references to raw pointers using the 'as' keyword.

LANGUAGE: rust
CODE:
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

----------------------------------------

TITLE: Defining Floating-Point Numbers in Rust
DESCRIPTION: Shows how to declare and initialize floating-point numbers in Rust. The example uses both f32 and f64 types, with f64 being the default for floating-point numbers.

LANGUAGE: rust
CODE:
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

----------------------------------------

TITLE: Documentation Comment Example in Rust
DESCRIPTION: Demonstrates how to write documentation comments with examples for a Rust function

LANGUAGE: rust
CODE:
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

----------------------------------------

TITLE: Visualizing Nightly Release Pattern
DESCRIPTION: Shows the pattern of nightly releases using asterisk notation to represent daily builds.

LANGUAGE: text
CODE:
nightly: * - - * - - *

----------------------------------------

TITLE: Defining User Struct with Reference Fields in Rust
DESCRIPTION: This code snippet defines a User struct with boolean and string reference fields. It demonstrates the need for lifetime specifiers when using references in struct definitions.

LANGUAGE: rust
CODE:
struct User {
    active: bool,
    username: &str,
    email: &str,
}

----------------------------------------

TITLE: Updating content Method to Use State in Rust
DESCRIPTION: Updates the content method on Post to delegate to a content method on State.

LANGUAGE: rust
CODE:
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
}

----------------------------------------

TITLE: Compiling and Running a Rust Project with Cargo
DESCRIPTION: This snippet shows the command to run a Rust project and the resulting output. It demonstrates the compilation process, including timing and target information, followed by the program's execution and output.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/traits-example`
A baby dog is called a puppy

----------------------------------------

TITLE: Using Function References with Iterator Map in Rust
DESCRIPTION: Demonstrates using a function reference instead of a closure with the map method to convert numbers to strings.

LANGUAGE: rust
CODE:
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(ToString::to_string).collect();

----------------------------------------

TITLE: Creating and Updating Strings in Rust
DESCRIPTION: Shows different ways to create and modify String values

LANGUAGE: rust
CODE:
let mut s = String::new();

let data = "initial contents";
let s = data.to_string();

let s = String::from("initial contents");

----------------------------------------

TITLE: Running a Rust Program with Panic
DESCRIPTION: This snippet shows the process of compiling and running a Rust program that results in a panic. The program attempts to access an out-of-bounds index in an array or vector, causing the panic.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`

thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

----------------------------------------

TITLE: Specifying Rust Edition in Cargo.toml
DESCRIPTION: Demonstrates how to specify the Rust edition in a project's Cargo.toml file. If not specified, Rust defaults to the 2015 edition for backward compatibility.

LANGUAGE: toml
CODE:
edition = "2024"

----------------------------------------

TITLE: Finding the Largest Number in a List (Rust)
DESCRIPTION: This code snippet demonstrates how to find the largest number in a list of integers. It initializes a list, sets the first number as the largest, then iterates through the list to find the maximum value.

LANGUAGE: rust
CODE:
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

----------------------------------------

TITLE: Creating a New Vector in Rust
DESCRIPTION: Demonstrates two ways to create a new vector: using Vec::new() and the vec! macro

LANGUAGE: rust
CODE:
let v: Vec<i32> = Vec::new();

LANGUAGE: rust
CODE:
let v = vec![1, 2, 3];

----------------------------------------

TITLE: Running Rust Program with Cargo
DESCRIPTION: Terminal output showing the compilation and execution of a Rust program using Cargo build system. The output demonstrates variable scoping with different values of x being displayed from inner and outer scopes.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6

----------------------------------------

TITLE: Compiling and Running a Rust Project with Cargo
DESCRIPTION: This snippet shows the command to run a Rust project and its output. It demonstrates the compilation process, build completion, and program execution using Cargo.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
The value of number is: 5

----------------------------------------

TITLE: Basic Unit Test Module Structure in Rust
DESCRIPTION: Example of a basic test module structure automatically generated by Cargo, showing the cfg(test) annotation and a simple test function.

LANGUAGE: rust
CODE:
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

----------------------------------------

TITLE: Scoped `use` Statement in Rust
DESCRIPTION: This example shows that `use` statements only apply to the scope they're in, causing a compilation error when the function is moved to a different module.

LANGUAGE: rust
CODE:
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

----------------------------------------

TITLE: Defining the Result Enum in Rust
DESCRIPTION: This snippet shows the definition of the Result enum with generic type parameters T and E for success and error types respectively.

LANGUAGE: rust
CODE:
enum Result<T, E> {
    Ok(T),
    Err(E),
}

----------------------------------------

TITLE: Initializing Variables in Rust
DESCRIPTION: Demonstrates basic variable initialization and mutability in Rust.

LANGUAGE: rust
CODE:
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

----------------------------------------

TITLE: Basic Dereference Operator Usage in Rust
DESCRIPTION: Demonstrates using the dereference operator (*) with a regular reference to an i32 value.

LANGUAGE: rust
CODE:
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

----------------------------------------

TITLE: Creating a New Vector in Rust
DESCRIPTION: This snippet demonstrates how to create a new, empty vector to hold values of type i32 using the Vec::new function.

LANGUAGE: rust
CODE:
let v: Vec<i32> = Vec::new();

----------------------------------------

TITLE: Defining the Iterator Trait in Rust
DESCRIPTION: This snippet shows the definition of the Iterator trait in Rust. It includes an associated type Item and requires implementation of the next() method.

LANGUAGE: rust
CODE:
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

----------------------------------------

TITLE: Using assert_eq! and assert_ne! Macros in Rust Tests
DESCRIPTION: Demonstrates the usage of assert_eq! and assert_ne! macros for equality checks in Rust tests.

LANGUAGE: rust
CODE:
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

----------------------------------------

TITLE: Defining a Trait with an Associated Function in Rust
DESCRIPTION: Demonstrates how to define a trait with an associated function and implement it on a struct in Rust.

LANGUAGE: Rust
CODE:
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}

----------------------------------------

TITLE: Moving Values into Closures with the move Keyword in Rust
DESCRIPTION: This snippet shows how to use the move keyword to force a closure to take ownership of values from its environment, which is useful when passing closures to new threads.

LANGUAGE: rust
CODE:
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

----------------------------------------

TITLE: Implementing Value Calculation with Match in Rust
DESCRIPTION: This snippet demonstrates how to use the match operator in Rust to calculate the value of different coin types. It defines an enum for coin types and a function that uses match to return the cent value of each coin.

LANGUAGE: rust
CODE:
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

----------------------------------------

TITLE: Running Cargo Tests for Rust Project
DESCRIPTION: This snippet shows the command to run Cargo tests and the resulting output. It includes compilation information, unit test results, integration test results, and doc-test results for the 'adder' project.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.31s
     Running unittests src/lib.rs (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

----------------------------------------

TITLE: Variable Shadowing in Rust
DESCRIPTION: This code snippet illustrates variable shadowing in Rust, where a new variable with the same name can be declared, effectively creating a new variable that shadows the previous one.

LANGUAGE: rust
CODE:
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

----------------------------------------

TITLE: Executing Development and Release Builds with Cargo
DESCRIPTION: Example showing the output difference between development and release builds using Cargo commands. Dev builds are unoptimized with debug info, while release builds are optimized.

LANGUAGE: console
CODE:
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s

----------------------------------------

TITLE: Example Code with Mathematical Constant
DESCRIPTION: Sample code that uses an approximate value of pi, demonstrating a case where Clippy can suggest improvements by using built-in constants.

LANGUAGE: rust
CODE:
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}

----------------------------------------

TITLE: Implementing Draw Trait for Button Struct in Rust
DESCRIPTION: This code shows how to implement the Draw trait for a concrete type Button. It demonstrates how different types can provide their own implementation of the draw method.

LANGUAGE: rust
CODE:
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

----------------------------------------

TITLE: Running Clippy Linter on Rust Project
DESCRIPTION: Demonstrates how to use Clippy to perform additional linting on a Rust project.

LANGUAGE: bash
CODE:
$ cargo clippy

----------------------------------------

TITLE: Checking PATH in PowerShell
DESCRIPTION: This command displays the PATH environment variable in PowerShell, which is useful for troubleshooting Rust installation issues.

LANGUAGE: powershell
CODE:
> echo $env:Path

----------------------------------------

TITLE: Matching Enum with Associated Values in Rust
DESCRIPTION: Demonstrates pattern matching on an enum variant that contains an associated value, binding the value to a variable in the match arm.

LANGUAGE: rust
CODE:
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

----------------------------------------

TITLE: Compiling and Running Rust Cons List Project
DESCRIPTION: This snippet shows the process of compiling and running a Rust project named 'cons-list' using Cargo. It includes the compilation output and the time taken to build the project.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/cons-list`

----------------------------------------

TITLE: Implementing first_word Function with String Slices
DESCRIPTION: Demonstrates how to implement a function that returns a string slice of the first word in a String.

LANGUAGE: Rust
CODE:
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

----------------------------------------

TITLE: Creating a new Rust project with Cargo
DESCRIPTION: Shows how to create a new Rust project named 'minigrep' using the Cargo package manager.

LANGUAGE: console
CODE:
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep

----------------------------------------

TITLE: Exploring Mutex<T> API in Single-Threaded Context
DESCRIPTION: This code snippet demonstrates the basic usage of Mutex<T> in a single-threaded context. It shows how to create a mutex, acquire a lock, modify the data, and automatically release the lock.

LANGUAGE: rust
CODE:
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

----------------------------------------

TITLE: Mutable Variable Example in Rust
DESCRIPTION: Shows how to create and modify mutable variables in Rust.

LANGUAGE: rust
CODE:
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

----------------------------------------

TITLE: Managing Node References with Strong and Weak Counts
DESCRIPTION: Example showing how to manage reference counting with strong and weak references in a tree structure, demonstrating proper memory management.

LANGUAGE: rust
CODE:
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch));

        println!("leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf));
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf));
}

----------------------------------------

TITLE: Bringing a Module into Scope with `use` in Rust
DESCRIPTION: This snippet demonstrates how to bring the `hosting` module into scope using the `use` keyword, allowing for shorter function calls within the `eat_at_restaurant` function.

LANGUAGE: rust
CODE:
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

----------------------------------------

TITLE: Implementing a Custom Derive Macro in Rust
DESCRIPTION: Demonstrates how to implement a custom derive macro using the syn and quote crates.

LANGUAGE: rust
CODE:
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

----------------------------------------

TITLE: Specifying Rust 2021 Edition in Cargo.toml
DESCRIPTION: Configuration in Cargo.toml to use Rust 2021 Edition idioms. This setting is crucial for utilizing the latest Rust features and improvements discussed in the book.

LANGUAGE: toml
CODE:
edition="2021"

----------------------------------------

TITLE: Defining a Recursive Cons List Enum in Rust (Non-Compiling)
DESCRIPTION: This snippet shows an attempt to define a recursive cons list as an enum in Rust. However, this code doesn't compile due to the recursive type having an unknown size.

LANGUAGE: rust
CODE:
enum List {
    Cons(i32, List),
    Nil,
}

----------------------------------------

TITLE: Function Using Enum Parameter
DESCRIPTION: Shows how to define a function that accepts an enum as a parameter.

LANGUAGE: rust
CODE:
fn route(ip_kind: IpAddrKind) {}

----------------------------------------

TITLE: Crate-Level Documentation in Rust
DESCRIPTION: Example of using //! comments to document an entire crate in the lib.rs file.

LANGUAGE: rust
CODE:
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

----------------------------------------

TITLE: Iterating over bytes in a Rust string
DESCRIPTION: This snippet demonstrates how to iterate over the raw bytes of a string using the bytes() method.

LANGUAGE: rust
CODE:
for b in "Зд".bytes() {
    println!("{b}");
}

----------------------------------------

TITLE: Running Rust Cons List Program
DESCRIPTION: This snippet shows the command to compile and run a Rust program named 'cons-list'. It uses Cargo, Rust's package manager and build system.

LANGUAGE: Shell
CODE:
$ cargo run

----------------------------------------

TITLE: Public Structs with Private Fields in Rust
DESCRIPTION: Demonstrates how to create public structs with mixed public and private fields, showing field accessibility rules.

LANGUAGE: rust
CODE:
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

----------------------------------------

TITLE: Executing Shirt Company Program with Cargo
DESCRIPTION: Terminal output showing cargo compilation and program execution that demonstrates handling of optional color preferences, where one user specifies Red and another has no preference defaulting to Blue

LANGUAGE: shell
CODE:
$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue

----------------------------------------

TITLE: Function with Multiple Parameters in Rust
DESCRIPTION: This snippet illustrates how to define a function with multiple parameters. It shows parameter declaration with different types and how to use these parameters within the function.

LANGUAGE: rust
CODE:
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

----------------------------------------

TITLE: Using should_panic Attribute in Rust Tests
DESCRIPTION: Shows how to use the #[should_panic] attribute to test for expected panics in Rust code.

LANGUAGE: rust
CODE:
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

----------------------------------------

TITLE: Creating a Stream from an Iterator
DESCRIPTION: Demonstrates how to create a stream from an iterator and process its values asynchronously.

LANGUAGE: rust
CODE:
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}

----------------------------------------

TITLE: Future Trait Definition in Rust
DESCRIPTION: Core definition of the Future trait showing its associated Output type and poll method with Pin and Context parameters

LANGUAGE: rust
CODE:
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

----------------------------------------

TITLE: Creating a Tree Node Structure in Rust
DESCRIPTION: Implementation of a tree data structure with parent-child relationships using Rc<T> for shared ownership and RefCell<T> for interior mutability.

LANGUAGE: rust
CODE:
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

----------------------------------------

TITLE: Implementing Intervals Using Standard Thread APIs in Rust
DESCRIPTION: Demonstrates how to create intervals using std::thread APIs instead of async APIs, showing the similarity in API design between threaded and async approaches.

LANGUAGE: rust
CODE:
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn get_intervals() -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    rx
}

----------------------------------------

TITLE: Using Trait Objects for Dynamic Futures in Rust
DESCRIPTION: Shows how to use trait objects to store futures of different types in a Vec. This approach allows for working with a dynamic collection of futures that have the same output type.

LANGUAGE: rust
CODE:
let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
    Box::pin(tx1_fut),
    Box::pin(rx_fut),
    Box::pin(tx_fut),
];

trpl::join_all(futures).await;

----------------------------------------

TITLE: Ignoring Tests in Rust
DESCRIPTION: Example showing how to mark tests with the ignore attribute to exclude them from regular test runs.

LANGUAGE: rust
CODE:
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}

----------------------------------------

TITLE: Using External Packages
DESCRIPTION: Example showing how to use external dependencies in a Rust project.

LANGUAGE: toml
CODE:
rand = "0.8.5"

LANGUAGE: rust
CODE:
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

----------------------------------------

TITLE: Defining an add_two Function in Rust
DESCRIPTION: This snippet introduces a hypothetical function 'add_two' that adds 2 to a given number. It's used as an example to demonstrate the need for testing beyond Rust's type and borrow checking.

LANGUAGE: rust
CODE:
fn add_two(x: i32) -> i32 {
    x + 2
}

----------------------------------------

TITLE: Defining and Using a Simple Rust Macro
DESCRIPTION: This snippet demonstrates how to define a simple macro named 'five_times' that multiplies its input by 5, and then shows its usage in the main function. The macro abstracts at a syntactic level, allowing for code generation based on the input expression.

LANGUAGE: rust
CODE:
macro_rules! five_times {
    ($x:expr) => (5 * $x);
}

fn main() {
    assert_eq!(25, five_times!(2 + 3));
}

----------------------------------------

TITLE: Rust Test Failure Output
DESCRIPTION: This snippet shows the detailed output of a failing test in Rust. It includes the test name, the assertion that failed, and the values that caused the failure. This information is crucial for debugging and understanding why the test failed.

LANGUAGE: plaintext
CODE:
running 1 test
test tests::one_result ... FAILED

failures:

---- tests::one_result stdout ----

thread 'tests::one_result' panicked at src/lib.rs:44:9:
assertion `left == right` failed
  left: ["safe, fast, productive."]
 right: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Defining a Function Signature with String Parameter in Rust
DESCRIPTION: This snippet shows the initial function signature for a 'first_word' function that takes a String reference as a parameter. The return type is not yet defined, indicated by the question mark.

LANGUAGE: rust
CODE:
fn first_word(s: &String) -> ?

----------------------------------------

TITLE: While let Loop for Channel Receiver in Rust
DESCRIPTION: This snippet demonstrates a while let loop used to receive messages from a channel, continuing until the sender disconnects.

LANGUAGE: rust
CODE:
while let Ok(message) = rx.recv() {
    println!("Received: {}", message);
}

----------------------------------------

TITLE: Vector Index Panic Example in Rust
DESCRIPTION: Example showing how accessing a vector beyond its bounds triggers a panic, demonstrating Rust's memory safety features.

LANGUAGE: rust
CODE:
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}

----------------------------------------

TITLE: Running Cargo Tests for Rust Project
DESCRIPTION: This snippet demonstrates the output of running 'cargo test' command for a Rust project. It shows the compilation process, test execution, and test results for both unit tests and doc-tests.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)

running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rectangle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

----------------------------------------

TITLE: Demonstrating Type Inference and Annotation in Rust
DESCRIPTION: Shows two examples of declaring floating-point numbers in Rust - one using type inference for f64 and another with explicit type annotation for f32.

LANGUAGE: rust
CODE:
let x = 2.0; // f64

let y: f32 = 3.0; // f32

----------------------------------------

TITLE: Using Iterator Adapters in Rust
DESCRIPTION: Demonstrates the use of the map() iterator adapter to create a new iterator.

LANGUAGE: Rust
CODE:
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);

----------------------------------------

TITLE: Declaring Unsafe Function
DESCRIPTION: Shows how to declare and use an unsafe function that requires specific safety guarantees.

LANGUAGE: rust
CODE:
unsafe fn dangerous() {}

fn main() {
    unsafe {
        dangerous();
    }
}

----------------------------------------

TITLE: Rust Print Statement
DESCRIPTION: Shows the println! macro usage in Rust for printing text to the console. This demonstrates macro syntax and string output.

LANGUAGE: rust
CODE:
println!("Hello, world!");

----------------------------------------

TITLE: Rust Test Output Analysis
DESCRIPTION: This snippet displays the output of running Rust tests. It shows compilation information, test results, stdout content for passing and failing tests, and error messages for failed assertions.

LANGUAGE: text
CODE:
   Compiling silly-function v0.1.0 (file:///projects/silly-function)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running unittests src/lib.rs (target/debug/deps/silly_function-160869f38cff9166)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

successes:

---- tests::this_test_will_pass stdout ----
I got the value 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8

thread 'tests::this_test_will_fail' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 10
 right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Unit Value with Underscore in Rust Match Expression
DESCRIPTION: Demonstrates using the unit value () with the underscore (_) to indicate no code should run for unmatched patterns.

LANGUAGE: rust
CODE:
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

----------------------------------------

TITLE: Defining a Rectangle Struct and Area Function in Rust
DESCRIPTION: This code snippet defines a Rectangle struct and an area function that calculates the area of the rectangle.

LANGUAGE: rust
CODE:
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

----------------------------------------

TITLE: Default Profile Optimization Settings in Cargo.toml
DESCRIPTION: Default optimization level settings for development and release profiles in Cargo.toml. Dev profile uses opt-level 0 for faster compilation, while release profile uses opt-level 3 for maximum runtime performance.

LANGUAGE: toml
CODE:
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

----------------------------------------

TITLE: Dereferencing Raw Pointers
DESCRIPTION: Demonstrates how to safely dereference raw pointers within an unsafe block.

LANGUAGE: rust
CODE:
let mut num = 5;
let r1 = &raw const num;
let r2 = &raw mut num;
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

----------------------------------------

TITLE: Creating a New Rust Project with Cargo
DESCRIPTION: Demonstrates the process of creating a new Rust project using the Cargo package manager. It shows the command to create a new project and the resulting directory structure.

LANGUAGE: console
CODE:
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs

----------------------------------------

TITLE: Compiling and Running Rust Traits Example
DESCRIPTION: This snippet shows the process of compiling and running a Rust project using Cargo. It demonstrates the compilation steps, build time, and the program's output.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/traits-example`
A baby dog is called a Spot

----------------------------------------

TITLE: Adding Elements to a Vector in Rust
DESCRIPTION: This snippet demonstrates how to create a mutable vector and add elements to it using the push method.

LANGUAGE: rust
CODE:
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

----------------------------------------

TITLE: Using an Iterator in a for Loop in Rust
DESCRIPTION: This example shows how to use an iterator in a for loop to print each value in a vector. The for loop consumes the iterator implicitly.

LANGUAGE: rust
CODE:
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
for val in v1_iter {
    println!("Got: {}", val);
}

----------------------------------------

TITLE: Creating and Destructuring Tuples in Rust
DESCRIPTION: Shows how to create a tuple with mixed types and how to destructure it to access individual values. Tuples in Rust have a fixed length and can contain values of different types.

LANGUAGE: rust
CODE:
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}

----------------------------------------

TITLE: Complex Message Enum Definition
DESCRIPTION: Defines an enum with variants containing different types and amounts of data.

LANGUAGE: rust
CODE:
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

----------------------------------------

TITLE: Creating and Using Hash Maps in Rust
DESCRIPTION: Demonstrates creation and basic operations with HashMap

LANGUAGE: rust
CODE:
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

----------------------------------------

TITLE: Defining Directory Structure in Rust Project
DESCRIPTION: Example showing the typical directory structure of a Rust binary crate named 'backyard'

LANGUAGE: text
CODE:
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs

----------------------------------------

TITLE: Safe FFI Integration with C
DESCRIPTION: Demonstrates how to safely integrate with C code using extern functions and the FFI interface.

LANGUAGE: rust
CODE:
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

----------------------------------------

TITLE: Declaring Constants in Rust
DESCRIPTION: This code snippet demonstrates how to declare a constant in Rust, which requires type annotation and can only be set to a constant expression.

LANGUAGE: rust
CODE:
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

----------------------------------------

TITLE: Making Structs and Fields Public
DESCRIPTION: Example showing how to make structs and their fields public using the pub keyword.

LANGUAGE: rust
CODE:
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

----------------------------------------

TITLE: Storing an i32 Value on the Heap Using Box<T> in Rust
DESCRIPTION: This snippet demonstrates how to use Box<T> to allocate an i32 value on the heap in Rust. It shows the basic syntax and usage of Box<T>.

LANGUAGE: rust
CODE:
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}

----------------------------------------

TITLE: Declaring a Module in Rust
DESCRIPTION: This snippet demonstrates how to declare a simple module in Rust. It creates a module named 'network' with an empty 'connect' function, showcasing the basic structure of module declaration.

LANGUAGE: rust
CODE:
mod network {
    fn connect() {
    }
}

----------------------------------------

TITLE: Idiomatic `use` Path for Structs in Rust
DESCRIPTION: This example shows the idiomatic way to bring a struct into scope using `use`, by specifying the full path to the struct.

LANGUAGE: rust
CODE:
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

----------------------------------------

TITLE: Accessing Vector Elements in Rust
DESCRIPTION: This snippet shows two ways to access elements in a vector: using indexing syntax and the get method, which returns an Option<&T>.

LANGUAGE: rust
CODE:
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

----------------------------------------

TITLE: Implementing State Transitions with Types in Rust
DESCRIPTION: Implements state transitions as transformations between different types representing post states.

LANGUAGE: rust
CODE:
impl DraftPost {
    // --snip--

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

----------------------------------------

TITLE: Closure Type Annotations in Rust
DESCRIPTION: This snippet shows how to add optional type annotations to closures in Rust, comparing the syntax to regular function definitions.

LANGUAGE: rust
CODE:
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

----------------------------------------

TITLE: Running a Rust Program with Cargo
DESCRIPTION: This snippet shows the process of compiling and running a Rust program named 'branches' using Cargo. It demonstrates the output of the compilation process and the execution of the program.

LANGUAGE: Shell
CODE:
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
number is divisible by 3

----------------------------------------

TITLE: Control Flow with if Expressions
DESCRIPTION: Demonstrates conditional logic using if expressions in Rust.

LANGUAGE: rust
CODE:
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

----------------------------------------

TITLE: Propagating Errors with the ? Operator in Rust
DESCRIPTION: This snippet demonstrates using the ? operator to propagate errors in a function that reads a username from a file.

LANGUAGE: rust
CODE:
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

----------------------------------------

TITLE: Clippy Example: Correct Usage of PI Constant
DESCRIPTION: Demonstrates the correct way to use the PI constant in Rust, which doesn't trigger Clippy warnings.

LANGUAGE: rust
CODE:
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}

----------------------------------------

TITLE: Dereferencing Raw Pointers in an Unsafe Block in Rust
DESCRIPTION: Demonstrates how to dereference raw pointers within an unsafe block in Rust.

LANGUAGE: Rust
CODE:
let mut num = 5;

let r1 = &raw const num;
let r2 = &raw mut num;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

----------------------------------------

TITLE: Defining and Calling Functions in Rust
DESCRIPTION: This snippet demonstrates how to define the main function and a custom function in Rust, as well as how to call the custom function from within main. It showcases the basic structure of Rust functions and the use of the println! macro for output.

LANGUAGE: rust
CODE:
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

----------------------------------------

TITLE: Using Nested Paths in Rust
DESCRIPTION: This snippet demonstrates how to use nested paths to bring multiple items from the same module into scope with a single `use` statement.

LANGUAGE: rust
CODE:
use std::{cmp::Ordering, io};

----------------------------------------

TITLE: Cargo Test Output for Failed Rust Unit Test
DESCRIPTION: This snippet displays the output of a failed unit test in a Rust project. It shows the compilation process, test execution, and detailed information about the failed test, including the panic message and expected output.

LANGUAGE: text
CODE:
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/lib.rs (target/debug/deps/guessing_game-57d70c3acb738f4d)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----

thread 'tests::greater_than_100' panicked at src/lib.rs:12:13:
Guess value must be greater than or equal to 1, got 200.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 200."`,
 expected substring: `"less than or equal to 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Creating and Using Basic Iterator in Rust
DESCRIPTION: Demonstrates how to create an iterator from a vector and use it in a for loop to process each element. The example shows the creation of a vector, obtaining its iterator, and printing each value.

LANGUAGE: rust
CODE:
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val}");
}

----------------------------------------

TITLE: Multi-line Comments in Rust
DESCRIPTION: Demonstrates how to write multi-line comments in Rust using double slashes (//) at the beginning of each line. Shows proper formatting for comments that span multiple lines to explain complex code.

LANGUAGE: rust
CODE:
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.

----------------------------------------

TITLE: Documenting Rust Function with Example Tests
DESCRIPTION: Demonstrates how to write documentation comments for a Rust function including example usage. Shows proper formatting with triple slashes, markdown sections, and integrated test examples.

LANGUAGE: rust
CODE:
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

----------------------------------------

TITLE: Running Cargo Tests in Rust
DESCRIPTION: This snippet shows the command to run tests in a Rust project using Cargo, along with the resulting output. It demonstrates the compilation process, test execution, and failure reporting.

LANGUAGE: shell
CODE:
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::another ... FAILED
test tests::exploration ... ok

failures:

---- tests::another stdout ----

thread 'tests::another' panicked at src/lib.rs:17:9:
Make this test fail
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

----------------------------------------

TITLE: Defining a Basic Trait in Rust
DESCRIPTION: Example of defining a public trait named Summarizable that requires implementing types to provide a summary method returning a String. This demonstrates the basic syntax for trait definitions in Rust.

LANGUAGE: rust
CODE:
pub trait Summarizable {
    fn summary(&self) -> String;
}

----------------------------------------

TITLE: Defining a Function with Parameters in Rust
DESCRIPTION: This example shows how to define a function with a single parameter. It demonstrates parameter declaration, type annotation, and how to use the parameter within the function body.

LANGUAGE: rust
CODE:
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

----------------------------------------

TITLE: Implementing Module Access with Public and Private Paths in Rust
DESCRIPTION: Demonstrates how to use absolute and relative paths to access module functions, showing the implementation of a restaurant service system with public and private module components.

LANGUAGE: rust
CODE:
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

----------------------------------------

TITLE: Racing Futures in Rust
DESCRIPTION: Illustrates how to use the race function to run multiple futures concurrently and get the result of whichever future finishes first. This approach is useful when you only need one future from a set to complete.

LANGUAGE: rust
CODE:
let slow = async {
    println!("slow started");
    trpl::sleep(Duration::from_millis(100)).await;
    println!("slow finished");
};

let fast = async {
    println!("fast started");
    trpl::sleep(Duration::from_millis(50)).await;
    println!("fast finished");
};

trpl::race(slow, fast).await;

----------------------------------------

TITLE: Returning Result from main Function in Rust
DESCRIPTION: This snippet demonstrates how to modify the main function to return a Result type, allowing the use of the ? operator.

LANGUAGE: rust
CODE:
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

----------------------------------------

TITLE: Basic Test Function Example in Rust
DESCRIPTION: Shows how to write a basic test function using the test attribute and assert_eq macro. The code includes a simple add function and its corresponding test.

LANGUAGE: rust
CODE:
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

----------------------------------------

TITLE: Executing Rust Closure Example with Cargo
DESCRIPTION: Demonstrates running a Rust program using Cargo, showing compilation output and program execution results. The program tracks an array's state before and after closure operations.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]

----------------------------------------

TITLE: Using assert! Macro for Boolean Checks in Rust Tests
DESCRIPTION: Shows how to use the assert! macro to check boolean conditions in Rust tests, using a Rectangle struct example.

LANGUAGE: rust
CODE:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}

----------------------------------------

TITLE: Implementing an Associated Function on the Rectangle Struct in Rust
DESCRIPTION: This code snippet demonstrates how to implement an associated function (square) on the Rectangle struct that creates a square rectangle.

LANGUAGE: rust
CODE:
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

----------------------------------------

TITLE: Mutable String Operations in Rust
DESCRIPTION: Demonstrates how to create and modify a mutable String type in Rust, showing string manipulation operations.

LANGUAGE: rust
CODE:
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`

----------------------------------------

TITLE: Debugging Output for Rust Rectangle Calculations
DESCRIPTION: This snippet displays debug output from the Rust program, showing a calculation result and the structure of a Rectangle object. It demonstrates the use of debug print statements in Rust.

LANGUAGE: rust
CODE:
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}

----------------------------------------

TITLE: Integration Test Example in Rust
DESCRIPTION: Shows how to write integration tests in a separate tests directory, demonstrating the use of external testing of public APIs.

LANGUAGE: rust
CODE:
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

----------------------------------------

TITLE: Demonstrating Desired Blog Post Behavior in Rust
DESCRIPTION: Shows example usage of the blog post API to be implemented, including creating a draft post, adding text, requesting review, and publishing.

LANGUAGE: rust
CODE:
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

----------------------------------------

TITLE: Vector Index Panic Example in Rust
DESCRIPTION: Shows how accessing a vector index beyond its bounds triggers a panic.

LANGUAGE: rust
CODE:
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}

----------------------------------------

TITLE: Renaming Types with `as` Keyword in Rust
DESCRIPTION: This snippet demonstrates how to use the `as` keyword to rename imported types, avoiding naming conflicts when importing items with the same name.

LANGUAGE: rust
CODE:
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

----------------------------------------

TITLE: Defining a Basic Enum in Rust
DESCRIPTION: This code snippet demonstrates how to define a simple enum in Rust called IpAddrKind with two variants: V4 and V6. Enums in Rust allow you to define a type by enumerating its possible values.

LANGUAGE: rust
CODE:
enum IpAddrKind {
    V4,
    V6,
}

----------------------------------------

TITLE: Implementing Area Method on Rectangle Struct in Rust
DESCRIPTION: Demonstrates defining an area method on a Rectangle struct using an impl block. The method takes an immutable reference to self and returns the area calculation.

LANGUAGE: rust
CODE:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

----------------------------------------

TITLE: Creating a String and Taking Ownership
DESCRIPTION: Demonstrates creating a String and moving ownership to a function.

LANGUAGE: Rust
CODE:
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

----------------------------------------

TITLE: Using PartialOrd and Ord Traits for Ordering in Rust
DESCRIPTION: PartialOrd enables comparison operators (<, >, <=, >=) for sorting. Ord ensures a valid ordering will always exist. These traits are used in sorting operations and data structures like BTreeSet.

LANGUAGE: rust
CODE:
#[derive(PartialOrd, Ord)]
enum SomeEnum {
    // variants
}

----------------------------------------

TITLE: Using an Iterator in a for Loop in Rust
DESCRIPTION: Shows how to use an iterator within a for loop to iterate over vector elements.

LANGUAGE: Rust
CODE:
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val}");
}

----------------------------------------

TITLE: Demonstrating Borrowing Rules in Rust
DESCRIPTION: This code snippet attempts to create both immutable and mutable references to the same variable, resulting in a compilation error. It illustrates Rust's strict borrowing rules that prevent data races.

LANGUAGE: rust
CODE:
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

----------------------------------------

TITLE: Creating an Iterator in Rust
DESCRIPTION: This snippet demonstrates how to create an iterator from a vector using the iter() method. The iterator is lazy and doesn't do anything until consumed.

LANGUAGE: rust
CODE:
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

----------------------------------------

TITLE: Using Multiple Producers with Async Blocks in Rust
DESCRIPTION: This snippet demonstrates how to use multiple producers with async blocks, cloning the sender and moving it into separate async blocks.

LANGUAGE: rust
CODE:
trpl::run(async {
    let (tx, mut rx) = trpl::channel();

    let tx1 = tx.clone();
    let fut1 = async move {
        let vals = vec!["hi", "from", "the"];
        for val in vals {
            tx1.send(val).unwrap();
            trpl::sleep(500).await;
        }
    };

    let fut2 = async {
        while let Some(message) = rx.recv().await {
            println!("received '{message}'");
        }
    };

    let fut3 = async move {
        let vals = vec!["more", "messages", "for", "you"];
        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(750).await;
        }
    };

    trpl::join3(fut1, fut2, fut3).await;
});

----------------------------------------

TITLE: Using ? Operator with Option in Rust
DESCRIPTION: This code shows how to use the ? operator with Option types to find the last character of the first line in a text.

LANGUAGE: rust
CODE:
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

----------------------------------------

TITLE: Enum with Data Types
DESCRIPTION: Shows how to define an enum with different data types for each variant

LANGUAGE: rust
CODE:
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

----------------------------------------

TITLE: Adding Test for Case-Insensitive Search in Rust
DESCRIPTION: This snippet shows how to add a new test for a case-insensitive search function in Rust. It includes both case-sensitive and case-insensitive test cases.

LANGUAGE: rust
CODE:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

----------------------------------------

TITLE: Using Pin and pin! Macro for Futures in Rust
DESCRIPTION: Demonstrates how to use Pin and the pin! macro to avoid unnecessary heap allocations when working with multiple futures. This approach provides a more efficient way to handle pinned futures.

LANGUAGE: rust
CODE:
use std::pin::pin;

let tx1_fut = pin!(async move {
    tx1.send("hi").unwrap();
    tx1.send("from").unwrap();
    tx1.send("the").unwrap();
});

let rx_fut = pin!(async {
    while let Some(msg) = rx1.recv().await {
        println!("received '{}'\n", msg);
    }
});

let tx_fut = pin!(async move {
    tx2.send("more").unwrap();
    tx2.send("messages").unwrap();
    tx3.send("for").unwrap();
    tx3.send("you").unwrap();
});

let futures: Vec<&mut (dyn Future<Output = ()> + Unpin)> = vec![&mut tx1_fut, &mut rx_fut, &mut tx_fut];

trpl::join_all(futures).await;

----------------------------------------

TITLE: Using Rc<RefCell<T>> for Mutable Shared Data in Rust
DESCRIPTION: This code demonstrates how to use Rc<RefCell<T>> to create a list that can be mutated and shared among multiple owners. It combines the sharing capabilities of Rc with the interior mutability of RefCell.

LANGUAGE: rust
CODE:
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

----------------------------------------

TITLE: Iterating Over Mutable Vector References in Rust
DESCRIPTION: This snippet shows how to iterate over mutable references to vector elements and modify them using a for loop.

LANGUAGE: rust
CODE:
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

----------------------------------------

TITLE: Attempting Refutable Pattern with Let Statement in Rust
DESCRIPTION: Demonstrates incorrect usage of a refutable pattern Some(x) with a let statement, which will fail to compile since let statements require irrefutable patterns.

LANGUAGE: rust
CODE:
let Some(x) = some_option_value;

----------------------------------------

TITLE: Re-exporting Names with `pub use` in Rust
DESCRIPTION: This example shows how to use `pub use` to re-export names, making them available for external code to use as if they were defined in the root module.

LANGUAGE: rust
CODE:
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

----------------------------------------

TITLE: Creating a Generic Enum in Rust
DESCRIPTION: Illustrates the definition of the Option<T> enum, which is generic over type T and has two variants: Some and None.

LANGUAGE: rust
CODE:
enum Option<T> {
    Some(T),
    None,
}

----------------------------------------

TITLE: Running Rust Closure Example Program
DESCRIPTION: This snippet shows the process of compiling and running a Rust program that demonstrates the use of closures. The output indicates the program's execution, including the state of a collection before and after applying a closure.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Before defining closure: [1, 2, 3]
After calling closure: [1, 2, 3, 7]

----------------------------------------

TITLE: Matching Coin Enum Values in Rust
DESCRIPTION: Demonstrates using a match expression to determine the value of different coins represented by an enum.

LANGUAGE: rust
CODE:
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

----------------------------------------

TITLE: Implementing First Word Function with String Slice in Rust
DESCRIPTION: This improved version of the first_word function returns a string slice instead of an index. It demonstrates how to use string slices to create a more robust and safer API.

LANGUAGE: rust
CODE:
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

----------------------------------------

TITLE: Referencing panic! Macro in Rust
DESCRIPTION: Demonstrates the syntax for referencing the panic! macro in Rust code. This macro is used for unrecoverable errors that halt program execution.

LANGUAGE: Rust
CODE:
panic!

----------------------------------------

TITLE: Catch-All Pattern in Rust Match Expression
DESCRIPTION: Demonstrates using a catch-all pattern in a match expression to handle all cases not explicitly matched.

LANGUAGE: rust
CODE:
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

----------------------------------------

TITLE: Reading Vector Elements in Rust
DESCRIPTION: Demonstrates two ways to access vector elements: indexing and get() method

LANGUAGE: rust
CODE:
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element.")
}

----------------------------------------

TITLE: Using String Slices
DESCRIPTION: Shows how to create and use string slices in Rust.

LANGUAGE: Rust
CODE:
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);
}

----------------------------------------

TITLE: Tuple Destructuring in Let Statement in Rust
DESCRIPTION: This code snippet demonstrates how to use a pattern in a let statement to destructure a tuple into individual variables.

LANGUAGE: rust
CODE:
let (x, y, z) = (1, 2, 3);

----------------------------------------

TITLE: Defining a Procedural Macro in Rust
DESCRIPTION: Illustrates the basic structure of defining a procedural macro using the proc_macro crate.

LANGUAGE: rust
CODE:
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}

----------------------------------------

TITLE: Defining a Basic Test Module in Rust
DESCRIPTION: This snippet demonstrates how to create a simple test module in Rust. It uses the #[cfg(test)] attribute to indicate a test configuration and the #[test] attribute to mark a function as a test.

LANGUAGE: rust
CODE:
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

----------------------------------------

TITLE: Iterating Over Vector Elements in Rust
DESCRIPTION: This snippet demonstrates how to use a for loop to iterate over immutable references to each element in a vector and print them.

LANGUAGE: rust
CODE:
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

----------------------------------------

TITLE: Matching Option<T> in Rust
DESCRIPTION: Shows how to use match to handle Option<T> values, demonstrating pattern matching on Some and None variants.

LANGUAGE: rust
CODE:
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

----------------------------------------

TITLE: Implementing a Method on the Rectangle Struct in Rust
DESCRIPTION: This code snippet demonstrates how to implement an area method on the Rectangle struct.

LANGUAGE: rust
CODE:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

----------------------------------------

TITLE: Defining Post Struct and State Trait in Rust
DESCRIPTION: Defines the Post struct to hold content and state, along with a State trait to define shared behavior for state objects.

LANGUAGE: rust
CODE:
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

----------------------------------------

TITLE: Basic Rust Function Structure
DESCRIPTION: Demonstrates the basic structure of a Rust function, showing the main function declaration with empty body. The main function is the entry point of every Rust program.

LANGUAGE: rust
CODE:
fn main() {

}

----------------------------------------

TITLE: Defining Basic IP Address Enum
DESCRIPTION: Demonstrates defining a basic enum for IP address versions

LANGUAGE: rust
CODE:
enum IpAddrKind {
    V4,
    V6,
}

----------------------------------------

TITLE: Capturing References with Closures in Rust
DESCRIPTION: This example demonstrates how closures can capture immutable references from their environment.

LANGUAGE: rust
CODE:
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

----------------------------------------

TITLE: Implementing a LimitTracker with Messenger Trait in Rust
DESCRIPTION: This code defines a LimitTracker struct and a Messenger trait. It demonstrates how to track a value against a maximum and send messages based on the current value's proximity to the maximum.

LANGUAGE: rust
CODE:
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

----------------------------------------

TITLE: Calculating Rectangle Area with Separate Variables in Rust
DESCRIPTION: This snippet shows the initial implementation of rectangle area calculation using separate width and height variables. It demonstrates basic function definition and usage in Rust.

LANGUAGE: rust
CODE:
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

----------------------------------------

TITLE: Rust Operator Examples
DESCRIPTION: Examples of basic Rust operators including arithmetic, logical, and bitwise operations

LANGUAGE: rust
CODE:
var += expr;     // Addition assignment
var -= expr;     // Subtraction assignment
var *= expr;     // Multiplication assignment
var /= expr;     // Division assignment
expr || expr;    // Logical OR
expr && expr;    // Logical AND
!expr;           // Logical NOT

----------------------------------------

TITLE: Compiling and Running Rust Pattern Matching Program with Cargo
DESCRIPTION: This snippet shows the process of compiling and running a Rust program named 'patterns' using Cargo. The program demonstrates pattern matching by indexing elements a, b, and c.

LANGUAGE: shell
CODE:
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/patterns`
a is at index 0
b is at index 1
c is at index 2

----------------------------------------

TITLE: Creating Mutable References in Rust
DESCRIPTION: This example shows how to create and use a mutable reference, allowing a function to modify a borrowed value.

LANGUAGE: rust
CODE:
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

----------------------------------------

TITLE: Demonstrating Iterator Functionality in Rust
DESCRIPTION: A test function showing how to use the next() method of an iterator.

LANGUAGE: Rust
CODE:
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

----------------------------------------

TITLE: Declaring Hosting Submodule in Rust
DESCRIPTION: This snippet shows the declaration of the hosting submodule within the front_of_house module file after extracting its content.

LANGUAGE: rust
CODE:
pub mod hosting;

----------------------------------------

TITLE: Finding the Largest Number in Two Lists (Rust)
DESCRIPTION: This snippet shows how to find the largest number in two different lists of integers. It demonstrates code duplication, which will be addressed in the next example using function extraction.

LANGUAGE: rust
CODE:
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

----------------------------------------

TITLE: Constant Declaration in Rust
DESCRIPTION: Demonstrates how to declare and use constants in Rust.

LANGUAGE: rust
CODE:
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

----------------------------------------

TITLE: Implementing Custom Iterator in Rust
DESCRIPTION: Shows how to implement the Iterator trait for a custom struct.

LANGUAGE: Rust
CODE:
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

----------------------------------------

TITLE: Counting non-quarter coins with match in Rust
DESCRIPTION: Demonstrates using a match expression to count non-quarter coins while announcing the state of quarters.

LANGUAGE: rust
CODE:
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

----------------------------------------

TITLE: Implementing a Recursive Cons List Using Box<T> in Rust
DESCRIPTION: This snippet shows how to correctly implement a recursive cons list in Rust using Box<T> to provide indirection and solve the issue of unknown size.

LANGUAGE: rust
CODE:
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}

----------------------------------------

TITLE: Running Single-Threaded Tests in Rust
DESCRIPTION: Example of running tests sequentially using a single thread to prevent test interference when tests share state.

LANGUAGE: console
CODE:
$ cargo test -- --test-threads=1

----------------------------------------

TITLE: Implementing the Add Trait for Millimeters and Meters in Rust
DESCRIPTION: Shows how to implement the Add trait to add Millimeters and Meters in Rust, demonstrating the use of associated types.

LANGUAGE: Rust
CODE:
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

----------------------------------------

TITLE: Refactoring Rectangle Area Calculation with Tuples in Rust
DESCRIPTION: This snippet refactors the rectangle area calculation to use tuples instead of separate variables. It shows how to group related data using tuples in Rust.

LANGUAGE: rust
CODE:
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

----------------------------------------

TITLE: Comments Above Code in Rust
DESCRIPTION: Shows the preferred style of placing comments on separate lines above the code they describe.

LANGUAGE: rust
CODE:
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}

----------------------------------------

TITLE: Implementing Methods with Lifetime Annotations in Rust
DESCRIPTION: Demonstrates how to implement methods on a struct with lifetime annotations.

LANGUAGE: rust
CODE:
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

----------------------------------------

TITLE: Multi-line Comments in Rust
DESCRIPTION: Shows how to write comments that span multiple lines by using two forward slashes at the start of each line.

LANGUAGE: rust
CODE:
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.

----------------------------------------

TITLE: Demonstrating Prose Style in Rust Documentation
DESCRIPTION: Examples of preferred prose style for Rust documentation, including title case usage, term emphasis, and method references.

LANGUAGE: markdown
CODE:
## Generating a Secret Number

This is an *associated function* of the `read_line` method.

----------------------------------------

TITLE: Art Library Module Structure
DESCRIPTION: Example of organizing a Rust library with modules for colors and utilities.

LANGUAGE: rust
CODE:
//! A library for modeling artistic concepts.

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}

----------------------------------------

TITLE: Creating an Iterator in Rust
DESCRIPTION: Demonstrates how to create an iterator from a vector using the iter() method.

LANGUAGE: Rust
CODE:
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

----------------------------------------

TITLE: Defining an Unsafe Function in Rust
DESCRIPTION: Shows how to define an unsafe function named 'dangerous' in Rust.

LANGUAGE: Rust
CODE:
unsafe fn dangerous() {}

unsafe {
    dangerous();
}

----------------------------------------

TITLE: Option Enum Definition
DESCRIPTION: Shows the definition of Rust's Option enum used to handle null values

LANGUAGE: rust
CODE:
enum Option<T> {
    None,
    Some(T),
}

----------------------------------------

TITLE: Creating Enum Instances
DESCRIPTION: Demonstrates how to create instances of enum variants.

LANGUAGE: rust
CODE:
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

----------------------------------------

TITLE: Integration Test Example in Rust
DESCRIPTION: Shows how to write an integration test in a separate tests directory. The test demonstrates testing the public API of the library.

LANGUAGE: rust
CODE:
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}

----------------------------------------

TITLE: Defining Restaurant Module Structure
DESCRIPTION: Example demonstrating module organization in a restaurant library with front-of-house and serving modules.

LANGUAGE: rust
CODE:
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

----------------------------------------

TITLE: Using Irrefutable Pattern with If Let in Rust
DESCRIPTION: Demonstrates improper usage of an irrefutable pattern with if let, which triggers a compiler warning since if let is designed for refutable patterns.

LANGUAGE: rust
CODE:
if let x = 5 {
    println!("{}\n", x);
}

----------------------------------------

TITLE: Publishing Preview to GitHub Pages using Bash
DESCRIPTION: This snippet outlines the steps to publish a preview of the book to GitHub Pages. It involves installing ghp-import and running a custom script.

LANGUAGE: bash
CODE:
pip install ghp-import

LANGUAGE: bash
CODE:
tools/generate-preview.sh

----------------------------------------

TITLE: Using Clippy for Additional Linting
DESCRIPTION: Demonstrates how to install and use Clippy to catch common mistakes in Rust code.

LANGUAGE: bash
CODE:
$ rustup component add clippy
$ cargo clippy

LANGUAGE: Rust
CODE:
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}

LANGUAGE: Rust
CODE:
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}

----------------------------------------

TITLE: Demonstrating Pattern Examples in Rust
DESCRIPTION: This snippet shows examples of different pattern types in Rust, including simple variables, tuples, and enum variants. These patterns are used to match against data structures and control program flow.

LANGUAGE: rust
CODE:
x
(a, 3)
Some(Color::Red)

----------------------------------------

TITLE: Running Cargo Project
DESCRIPTION: Various ways to run a Cargo project including direct execution and cargo run

LANGUAGE: console
CODE:
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!

----------------------------------------

TITLE: Basic String Declaration in Rust
DESCRIPTION: Demonstrates the basic declaration of a string literal in Rust, showing how to declare a variable that references a string literal value.

LANGUAGE: rust
CODE:
let s = "hello";

----------------------------------------

TITLE: Expressions in Rust Function Bodies
DESCRIPTION: This example demonstrates the use of expressions in Rust function bodies. It shows how a block can be an expression and how the last expression in a function can be used as an implicit return value.

LANGUAGE: rust
CODE:
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

----------------------------------------

TITLE: Annotating Lifetimes in Rust Function Signatures
DESCRIPTION: Demonstrates how to add lifetime annotations to a function signature in Rust.

LANGUAGE: rust
CODE:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

----------------------------------------

TITLE: Implementing List with RefCell<T> in Rust
DESCRIPTION: Definition of a cons list using RefCell<T> to enable modification of list references. The code demonstrates a basic list structure that can be mutably referenced.

LANGUAGE: rust
CODE:
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

----------------------------------------

TITLE: Attempting to Borrow Immutable Value as Mutable in Rust
DESCRIPTION: This code snippet demonstrates an attempt to borrow an immutable value as mutable, which results in a compilation error. It illustrates the borrowing rules in Rust.

LANGUAGE: rust
CODE:
let x = 5;
let y = &mut x;

----------------------------------------

TITLE: Variable Scope Example in Rust
DESCRIPTION: Shows variable scope behavior with string literals in Rust code, illustrating when variables come into and go out of scope.

LANGUAGE: rust
CODE:
let s = String::from("hello"); // s is valid from this point forward
// do stuff with s


----------------------------------------

TITLE: Conditional if let Expression in Rust
DESCRIPTION: This code example shows a complex conditional structure using if let, else if, and else if let to determine a background color based on various conditions.

LANGUAGE: rust
CODE:
let favorite_color: Option<&str> = None;
let is_tuesday = false;
let age: Result<u8, _> = "34".parse();

if let Some(color) = favorite_color {
    println!("Using your favorite color, {color}, as the background");
} else if is_tuesday {
    println!("Tuesday is green day!");
} else if let Ok(age) = age {
    if age > 30 {
        println!("Using purple as the background color");
    } else {
        println!("Using orange as the background color");
    }
} else {
    println!("Using blue as the background color");
}

----------------------------------------

TITLE: Using the Glob Operator in Rust
DESCRIPTION: This example shows how to use the glob operator (*) to bring all public items from a module into scope, with a caution about its potential drawbacks.

LANGUAGE: rust
CODE:
use std::collections::*;

----------------------------------------

TITLE: Handling Different Error Types in Rust
DESCRIPTION: This snippet shows how to handle different types of errors when opening a file, creating it if it doesn't exist.

LANGUAGE: rust
CODE:
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

----------------------------------------

TITLE: Defining Structs with Lifetime Annotations in Rust
DESCRIPTION: Shows how to define a struct that holds a reference, requiring a lifetime annotation.

LANGUAGE: rust
CODE:
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

----------------------------------------

TITLE: End-of-Line Comments in Rust
DESCRIPTION: Demonstrates how to place comments at the end of code lines, though this is less common than placing comments on their own lines.

LANGUAGE: rust
CODE:
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}

----------------------------------------

TITLE: Extracting a Function to Find the Largest Number (Rust)
DESCRIPTION: This code demonstrates how to extract the logic for finding the largest number into a separate function. This abstraction reduces code duplication and improves maintainability.

LANGUAGE: rust
CODE:
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

----------------------------------------

TITLE: Using Result<T, E> in Rust Tests
DESCRIPTION: Demonstrates how to write tests that return Result<T, E> instead of panicking in Rust.

LANGUAGE: rust
CODE:
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

----------------------------------------

TITLE: Documenting Rust Functions with Doc Comments
DESCRIPTION: Example of using documentation comments (///) to create HTML documentation for a Rust function, including description and example usage.

LANGUAGE: rust
CODE:
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

----------------------------------------

TITLE: Implementing a Method for a Rectangle Struct in Rust
DESCRIPTION: This code snippet demonstrates how to implement a method named 'area' for a Rectangle struct. The method calculates and returns the area of the rectangle using its width and height properties.

LANGUAGE: rust
CODE:
# struct Rectangle {
#     width: u32,
#     height: u32,
# }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

----------------------------------------

TITLE: Defining and Using a Rectangle Struct in Rust
DESCRIPTION: This snippet introduces a Rectangle struct to represent the rectangle's dimensions. It demonstrates struct definition, instance creation, and usage in a function.

LANGUAGE: rust
CODE:
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

----------------------------------------

TITLE: Preventing Dangling References in Rust
DESCRIPTION: This code snippet demonstrates how Rust prevents dangling references at compile-time by ensuring that references do not outlive the data they point to.

LANGUAGE: rust
CODE:
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

----------------------------------------

TITLE: Creating a String Slice in Rust
DESCRIPTION: This snippet demonstrates how to create a string slice from a String. It shows the syntax for specifying a range of indices to create a slice.

LANGUAGE: rust
CODE:
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

----------------------------------------

TITLE: Defining a Procedural Macro in Rust
DESCRIPTION: Demonstrates how to define a procedural macro for custom derive in Rust.

LANGUAGE: Rust
CODE:
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

----------------------------------------

TITLE: Displaying Rust Cons List State
DESCRIPTION: This snippet shows the output of the Rust program, displaying the state of three cons list variables (a, b, and c) after some operations. Each cons list is represented using RefCell and Nil.

LANGUAGE: rust
CODE:
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))

----------------------------------------

TITLE: Using Enum to Store Multiple Types in a Vector in Rust
DESCRIPTION: This snippet demonstrates how to use an enum to create a vector that can store elements of different types.

LANGUAGE: rust
CODE:
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

----------------------------------------

TITLE: Memory Scope and Allocation Example
DESCRIPTION: Illustrates memory allocation and deallocation behavior with String type in Rust, showing automatic cleanup when variables go out of scope.

LANGUAGE: rust
CODE:
{
    let s = String::from("hello"); // s is valid from this point
    // do stuff with s
} // this scope is now over, and s is no longer valid

----------------------------------------

TITLE: Defining a Function with Generic String Slice Parameter in Rust
DESCRIPTION: This snippet shows an improved function signature that accepts both &String and &str types as parameters. It demonstrates how to make a function more flexible by using string slices.

LANGUAGE: rust
CODE:
fn first_word(s: &str) -> &str {

----------------------------------------

TITLE: Markdown Links for Rust Documentation
DESCRIPTION: Markdown formatted links directing users to current and archived versions of the Rust documentation about RefCell and interior mutability.

LANGUAGE: markdown
CODE:
[the current version of the book](../ch15-05-interior-mutability.html)
[find a copy distributed with Rust 1.30](https://doc.rust-lang.org/1.30.0/book/second-edition/ch15-05-interior-mutability.html)

----------------------------------------

TITLE: Demonstrating Dangling References in Rust
DESCRIPTION: An example showing how Rust prevents dangling references through compile-time errors.

LANGUAGE: rust
CODE:
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

----------------------------------------

TITLE: Combining Generic Types, Trait Bounds, and Lifetimes in Rust
DESCRIPTION: Shows how to use generic type parameters, trait bounds, and lifetimes together in a single function signature.

LANGUAGE: rust
CODE:
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

----------------------------------------

TITLE: For Loop Pattern Destructuring in Rust
DESCRIPTION: This example shows how to use a pattern in a for loop to destructure a tuple, iterating over characters with their indices.

LANGUAGE: rust
CODE:
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}

----------------------------------------

TITLE: Creating an Array Slice in Rust
DESCRIPTION: This example demonstrates how to create a slice from an array. It shows that slices can be used with various types of collections, not just strings.

LANGUAGE: rust
CODE:
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);

----------------------------------------

TITLE: Defining and Using References in Rust
DESCRIPTION: This snippet demonstrates how to define and use a function that takes a reference to a String as a parameter, allowing it to access the value without taking ownership.

LANGUAGE: rust
CODE:
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

----------------------------------------

TITLE: Creating and Using Basic Closure in Rust
DESCRIPTION: Shows how to define a closure that simulates an expensive calculation by adding a delay. The closure takes a number parameter and returns it after a 2-second sleep, demonstrating how closures can encapsulate behavior.

LANGUAGE: rust
CODE:
# use std::thread;
# use std::time::Duration;

let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
# expensive_closure(5);

----------------------------------------

TITLE: Matching on Option<i32> in Rust
DESCRIPTION: This snippet demonstrates a match expression used to handle an Option<i32> value, showing how patterns are used in match arms.

LANGUAGE: rust
CODE:
match x {
    None => None,
    Some(i) => Some(i + 1),
}

----------------------------------------

TITLE: Function Parameter Pattern Matching in Rust
DESCRIPTION: This example shows how function parameters can be patterns, demonstrating tuple destructuring in the function signature.

LANGUAGE: rust
CODE:
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

let point = (3, 5);
print_coordinates(&point);

----------------------------------------

TITLE: Demonstrating Reference Lifetimes in Rust
DESCRIPTION: This code example illustrates how reference lifetimes work in Rust by showing a simple case with a variable x and a reference r. The comments indicate the lifetimes 'a and 'b using ASCII art to show their respective scopes.

LANGUAGE: rust
CODE:
{
    let x = 5;            // -----+-- 'b
                          //      |
    let r = &x;           // --+--+-- 'a
                          //   |  |
    println!("r: {r}");   //   |  |
                          // --+  |
}                         // -----+

----------------------------------------

TITLE: Basic Rust Operators
DESCRIPTION: Example usage of fundamental Rust operators including arithmetic, logical, and comparison operators with their corresponding traits for overloading.

LANGUAGE: rust
CODE:
// Arithmetic operators
x + y  // Add trait
x - y  // Sub trait
x * y  // Mul trait
x / y  // Div trait
x % y  // Rem trait

// Comparison operators
x == y  // PartialEq trait
x != y  // PartialEq trait
x < y   // PartialOrd trait
x <= y  // PartialOrd trait
x > y   // PartialOrd trait
x >= y  // PartialOrd trait

----------------------------------------

TITLE: Audio Decoder Linear Prediction Implementation
DESCRIPTION: Real-world example from an audio decoder demonstrating linear prediction calculation using iterator chains. The code processes a buffer of data using coefficients and bit shifting to estimate future values.

LANGUAGE: rust
CODE:
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}