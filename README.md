# Overview

Argrust is a simple argument parser that can perform Complicated tasks.

## Badges

![Crates.io Total Downloads](https://img.shields.io/crates/d/argrust?style=for-the-badge&label=crates.io%20total%20downloads)
![GitHub License](https://img.shields.io/github/license/d33pster/argrust?style=for-the-badge)
![Crates.io Version](https://img.shields.io/crates/v/argrust?style=for-the-badge&label=crates.io%20version)

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)

  - [Importing](#importing)
  - [Arguments](#defining-arguments-and-related)

- [Example](#example)
- [Testing](#testing)

## Installation

can be installed globally using:

```bash
cargo install argrust
```

or, can be installed just for your project:

```bash
cargo add argrust
```

## Usage

### Importing

```rust
use argrust::{Arguments, ArgumentDescription, FetchTypes};
```

Code Description:

- `Arguments`: This is a struct for storing all kinds of info about arguments.

    ```rust
    pub struct Arguments {
        gotargs: Vec<String>,
        allargs: Vec<String>,
        arguments: Vec<String>,
        errors: Vec<String>,
        data: Vec<ArgumentDescription>,
        used: Vec<String>,
    }
    ```

  - **gotargs**: stores all arguments that are received throught the command line.
  - **allargs**: stores all arguments that you define.
  - **arguments**: stores the arguments that were received in `gotargs` and are defined in `allargs`.
  - **errors**: stores errors if any.
  - **data**: this is a vector of custom structs named `ArgumentDescription`. We will come to that soon. For now, just understand that it will store data about the argument if provided.
  - **used**: stores arguments that were already used or visited by the user.

- `ArgumentDescription`: This is a struct for describing the arguments. While it is optional to provide data about arguments, A shorter counterpart for the argument is manadatory.

    ```rust
    pub struct ArgumentDescription {
        name: String,
        description: String,
        shorter_counterpart: String,
    }
    ```

  - **name**: stores a name of the argument. ->Optional
  - **desciption**: stores a short description of the argument. -> Optional
  - **shorter_counterpart**: stores a shorter counterpart of the main argumnet. Such as `--abc` is the main argument and `-a` is a shorter counterpart. -> Mandatory.

- `FetchTypes`: This is an enum for Fetching values of arguments.

    ```rust
    pub enum FetchTypes {
        Fetch(usize),
        TillLast,
        TillNext,
    }
    ```

  - **Fetch(usize)**: This can be used with the `.fetch()` method where you need to specify how many values u are expecting. Suppose there are two arguments `-a` and `-b`, and it is passed as -

    ```bash
    some_program -a 1 2 -b 34 56
    ```

    Then, to fetch the values `1` and `2` after `-a`, you can specify that you are expecting `Fetch(2)`. Similarly to fetch `34` and `56` you can use `TillLast` to fetch all the arguments till the last of the string. Or if you are not sure, just use `Fetch(2)`.

### Defining Arguments and Related

Arguments can be defined using the following way:

```rust
let mut args = Arguments::new(std::env::args().skip(1).collect);
```

The line `std::env::args().skip(1).collect()` returns a list of arguments passed to the program except the first argument which is the filename.

**To add arguments**:

```rust
let description_argument1 = ArgumentsDescription::new()
    .name("argument1") // this is optional
    .description("this is argument 1") // this is optional
    .short("-a"); // this is mandatory

args.add("--arg", description_argument1);
```

**To parse the arguments**:

```rust
args.analyse(); // this will fill the Arguments.arguments if any 
                // argument is received that is defined by you
```

**To check if an argument is received**:

```rust
// suppose the argument -a was received. you can check it by two ways.

if args.ifarg("-a") { // or args.ifarg("--arg")
    // do something
}

// NOTE: the above function will make the above argument as "used".
// Which means the user is already done with it. More Functionalities
// will be added on this later on.

// To avoid marking the argument as used, use:

if args.ifarg_force("-a") {
    // do something
}
```

**To fetch values given to an argument**:

```rust
// suppose the argument was $ some_program -a 1 2 3

// we are expecting 3 values for this.

let values: Vec<String> = args.fetch("-a", FetchTypes::Fetch(3));

// NOTE: if we are only expecting 2 values, then the last value
// will just be discarded.

// if the number of arguments expected is not known at the current time,
// you have two options -> FetchTypes::TillNext or FetchTypes::TillLast

// NOTE: if it is not known if the argument u are fetching for is in the
// middle or at the last, it is better to use FetchTypes::TillNext.
// It will try to find if this argument is in the middle or not.
// If it is in the middle, then it will fetch all the values between the
// argument and the next argument.
// And if it finds out that the argument is at the last, it will auto-rerun
// with FetchTypes::TillLast. Pretty Handy right!

// use this like:

let values: Vec<String> = args.fetch("--arg", FetchTypes::TillNext);

// NOTE: so if there is no next argument for '--arg', it will run again
// with FetchTypes::TillLast

// Suppose the position of the argument is known to be at the last,
// then simple use FetchTypes::TillLast

// This will fetch all the arguments till Last.

let values: Vec<String> = args.fetch("--arg", FetchTypes::TillLast);
```

**To remove an argument from defined arguments**:

```rust
args.remove("-a"); // or args.remove("--arg")

// while defining let mut args = Arguments::new(...);, make sure
// to add "mut" or else, add() and remove() wont work.
```

**To fetch data about arguments**:

```rust
// get from defined arguments:

// by number
let some_arg = args.get_arg_by_number(1); // this will return "--arg"

// by index
let some_arg = args.get_arg_by_index(0); // this will also return "--arg"

// get arg description
let some_arg_description = args.get_arg_description("--arg").get();
```

## Example

Let's see a real working code to understand the concept better:

```rust
use argrust::{Arguments, ArgumentDescription, FetchTypes};
use std::env;

fn main() {
    let mut args = Arguments::new(env::args().skip(1).collect());

    args.add("--setup", ArgumentDescription::new().short("-s"));
    args.add("--init", ArgumentDescription::new().name("initialize").short("-i"));

    args.analyse();

    let mut setup_value = String::new();

    if args.ifarg("--setup") {
        // fetch setup value
        setup_value = args.fetch("-s", FetchTypes::TillLast);
        // rest of the code.
    } else if args.ifarg("-i") {
        // init code
    }

    // NOTE: here, --init and --setup are mutually exclusive
    // meaning: they cannot be used together.

    // if you dont want that and you want multiple simultaneous args,
    // write them individually:

    // if args.ifarg("--setup") {
    //     // fetch setup value
    //     setup_value = args.fetch("-s", FetchTypes::TillLast);
    //     // rest of the code.
    // }
    
    // if args.ifarg("-i") {
    //     // init code
    // }
}
```

NOTE: More features are yet to come.

## Testing

- Auto

    To test the features, you can either run the following command in a terminal:

    ```bash
    argrust --test
    ```

    NOTE: this will clone the github repo `d33pster/argrust` will make a new folder named `argrust` wherever the terminal was opened. Additionally it will install `gcl` - a rust based cli tool which is an alternative to git clone command. For more info about `gcl` click [[here](https://crates.io/crates/gcl)].

    ADDITIONAL NOTE: If argrust is added using `cargo add argrust`, this feature will not work. Try manual mode or Install using `cargo install argrust`

- Manual

    Run the following in a new terminal:

    ```bash
    git clone https://github.com/d33pster/argrust.git
    cd argrust
    cargo test
    ```
