use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};
use std::cmp::Ordering;
use rand::Rng;

// So far, all the main functions we’ve used return ().
// The main function is special because it’s the entry and exit point of executable programs, and there are restrictions on what its return type can be for the programs to behave as expected.


// Luckily, main can also return a Result<(), E>. Listing 9-12 has the code from Listing 9-10 but we’ve changed the return type of main to be Result<(), Box<dyn Error>> and added a return value Ok(()) to the end.
// This code will now compile:

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

// The Box<dyn Error> type is a trait object, which we’ll talk about in the “Using Trait Objects that Allow for Values of Different Types” section in Chapter 17.
// For now, you can read Box<dyn Error> to mean “any kind of error.” 
// Using ? on a Result value in a main function with the error type Box<dyn Error> is allowed, because it allows any Err value to be returned early.
// Even though the body of this main function will only ever return errors of type std::io::Error, by specifying Box<dyn Error>, this signature will continue to be correct even if more code that returns other errors is added to the body of main.

// When a main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(()) and will exit with a nonzero value if main returns an Err value. Executables written in C return integers when they exit: programs that exit successfully return the integer 0, and programs that error return some integer other than 0. Rust also returns integers from executables to be compatible with this convention.

// The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode. Consult the standard library documentation for more information on implementing the Termination trait for your own types.




fn main() {
    // panic!("Goodbye, world!");

    // let v = vec![1, 2, 3];
    // v[99];

    // Recoverable errors with Result
    // enum Result<T, E> {
    //      Ok(T),
            //Err(E),
    //  }
    
    // let greeting_file_result = File::open("hello.txt");
    // the return type of file:: open is a Result<T, E>
    //  The generic parameter T has been filled in by the implementation of File::open with the type of the success value,
    // The type of E used in the error value is std::io::Error.

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening file {:?}", error)
    // };

    // we can match different errors
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("problem creating the file {:?}", e)
//             },
//             other_error => {
//                 panic!("Problem opening file {:?}", error);
//             }
//         }
//     };

    // the above can be simplified
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // shortcuts unrwap and expect
    // unwrap will return the value inside the ok; if err variant unwrap will call panic for us
    // let greeting_file = File::open("hello.txt").unwrap();

    // the expect method lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    // let result = read_username_from_file();
    // let mut username = String::new();
    // match result {
    //     Ok(s) => { username = s; }
    //     Err(e) => {println!("{:?}", e)}
    // }
    // println!("{:?}", username);
    // let last_char = last_char_of_first_line(&username);
    // println!("{:?}", last_char);
    guess_a_number();

}


// propogating success/error upward for it to handle properly
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         // instead of panic! we return error
//         Err(e) => return Err(e),
//     };
//     let mut username = String::new();
//     // file.read_to_string and file.open happen to have the same
//     // error type io::Error
//     match username_file.read_to_string(&mut username) {
//         // if succeeds, function succeeds; return username from file wrapped in an Ok
//         Ok(_) => Ok(username),
//         // if fails, return error
//         Err(e) => Err(e),
//     }
// }

// // shortcut:
// fn read_username_from_file() -> Result<String, io::Error> {
//     //  the ? at the end of the File::open call will return the value inside an Ok to the variable username_file. If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code. The same thing applies to the ? at the end of the read_to_string call.
//     let mut username = String::new();
//     // the ? after a result is like a match expression
//     // if the value of the Result is an ok, the value inside ok will get returned
//     // if value is err, err will be returned
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

// shortest 
fn read_username_from_file() -> Result<String, io::Error> {
    // Reading a file into a string is a fairly common operation, so the standard library provides the convenient fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it. Of course, using fs::read_to_string doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.
    fs::read_to_string("hello.txt")
}

// Listing 9-11: Using the ? operator on an Option<T> value
fn last_char_of_first_line(text: &str) -> Option<char> {
    // This function returns Option<char> because it’s possible that there is a character there, but it’s also possible that there isn’t. 
    // This code takes the text string slice argument and calls the lines method on it, which returns an iterator over the lines in the string. Because this function wants to examine the first line, it calls next on the iterator to get the first value from the iterator. 
    // If text is the empty string, this call to next will return None, in which case we use ? to stop and return None 
    //  If text is not the empty string, next will return a Some value containing a string slice of the first line in text.
    text.lines().next()?.chars().last()
    // The ? extracts the string slice, and we can call chars on that string slice to get an iterator of its characters.
    // We’re interested in the last character in this first line, so we call last to return the last item in the iterator.
}

// Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match.
// The ? operator won’t automatically convert a Result to an Option or vice versa;
//  in those cases, you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.


fn guess_a_number() {
    let small_variations = [
        "Well, butter my biscuit! That guess is smaller than a flea on a flea's back! Try again!",
        "Oh dear, that guess is tinier than a teaspoon in a sea of soup! Give it another shot!",
        "Whoa, that guess is smaller than a pixel on a smartphone screen! Back to the drawing board!",
        "Yikes! That guess is smaller than a seed in a sunflower! Let's aim higher, shall we?",
        "Goodness gracious! That guess is smaller than a snowflake in July! Let's try something bigger!",
        "Oh my stars! That guess is smaller than a teaspoon in a galaxy-sized cup of cosmic cocoa! Give it another go!",
        "Golly gee! That guess is tinier than a tater tot in a toddler's lunchbox! Let's aim higher, champ!",
        "Holy guacamole! That guess is smaller than a seed in an avocado! Time to think bigger!",
        "Oh snap! That guess is smaller than a speck of dust on a flea's wing! Let's beef it up!",
        "Well, slap my knee! That guess is smaller than a raindrop in a desert! Try again, partner!",
    ];

    
    let large_variations =  [
        "Whoa there, that guess is bigger than a burger on cheat day! But not quite right!",
        "Hoo boy, that guess is larger than a slice of cake at a birthday party! Try again!",
        "Holy moly, that guess is grander than a triple-decker sandwich! Let's dial it back a bit!",
        "Goodness gracious, that guess is as huge as a whale in a goldfish bowl! Try something smaller!",
        "Gee whiz, that guess is larger than a mountain in a molehill contest! Let's scale it down!",
        "Well, slap my hand! That guess is bigger than Texas on a map! Let's rein it in a tad!",
        "Whoopsie daisy! That guess is bigger than a bus in a bike lane! Try something more modest!",
        "Oh my, that guess is larger than life itself! But alas, not quite right! Try again, champ!",
        "Hold your horses! That guess is bigger than a parade float on a sidewalk! Let's trim it down!",
        "Whoosh! That guess is flying higher than a kite in a thunderstorm! Bring it back to earth, buddy!",
    ];



    let secret_number = rand::thread_rng().gen_range(0..100);
    let mut attempts :i32 = 0;
    println!("Welcome to the guessing game of epic proportions!");
    println!("Alrighty, what number are you tossing into the ring today?");

    loop {
        let random_index_small = rand::thread_rng().gen_range(0..small_variations.len());
        let random_index_large = rand::thread_rng().gen_range(0..large_variations.len());
    
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read lines");

        let guess_num = guess.trim().parse();
        let guess = Guess::new(guess_num.expect("guess must be a number"));
        attempts += 1;

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("{}", small_variations[random_index_small]),
            Ordering::Greater => println!("{}", large_variations[random_index_large]),
            Ordering::Equal => {
                println!("Cue the confetti!!"); 
                println!("The secret number is indeed {}! You guessed it right with only {} tries! ", secret_number, attempts); 
                break;
            }
        }
    }
}

// First, we define a struct named Guess that has a field named value that holds an i32. This is where the number will be stored.
pub struct Guess {
    value: i32
}

impl Guess {
    // Then we implement an associated function named new on Guess that creates instances of Guess values. The new function is defined to have one parameter named value of type i32 and to return a Guess. 
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
        //If value doesn’t pass this test, we make a panic! call, which will alert the programmer who is writing the calling code that they have a bug they need to fix, because creating a Guess with a value outside this range would violate the contract that Guess::new is relying on. 
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        // If value does pass the test, we create a new Guess with its value field set to the value parameter and return the Guess.
        Guess { value }
    }
    //Next, we implement a method named value that borrows self, doesn’t have any other parameters, and returns an i32. 
    //This kind of method is sometimes called a getter, because its purpose is to get some data from its fields and return it.
    // This public method is necessary because the value field of the Guess struct is private. 
    //It’s important that the value field be private so code using the Guess struct is not allowed to set value directly
    // code outside the module must use the Guess::new function to create an instance of Guess, thereby ensuring there’s no way for a Guess to have a value that hasn’t been checked by the conditions in the Guess::new function.
    pub fn value(&self) -> i32 {
        self.value
    }
}