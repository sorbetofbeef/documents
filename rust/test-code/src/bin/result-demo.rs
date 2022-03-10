/*
* -NOTE: IMPORTANT Go and read the github.com/SorbetofBeef/rust-programming.../a18b.rs for a decently
* -NOTE:            detailed example program
*/

//////////////////////////////////////////////////////////////////////////////////////////////////////

/*
* - Result wraps its returned value with Ok(inner_data) and Err(inner_data). 
*    - To access the inner data one must match on the returned Result varient:
*      Ok(...) or Err(...)
*
*    EXAMPLE:
*        match data {
*            Ok(inner_data) => arbiterary_fn(&inner_data),
*            Err(e) => println!("error: {:?}", e),
*        }
*
*    - This passes the data within the Ok(...) wrapper to our function rather than the
*      returned Result<...> type.
*/

//////////////////////////////////////////////////////////////////////////////////////////////////////

/*
* - Using the "question mark operator"
*     - This is a faster method for match inner data 
*         - displayed like "?"
*     - This method looks confusing and not very intuitive
*         - apparently once you get used to it, it feels natural
*             - would argue thats true of anything
*     - Result must be returned. so in the event you dont want or need the Ok(...) portion
*       returned you can use "()" for the return type in Result<>. So result would look like 
*       Result<(), E>. 
*         -NOTE: "()" implicitly returns nothing, in other words, ignore the return value
*
*       EXAMPLE:
*           fn pick_choice(input: &str) -> Result<(), String> {
*             // explicit type annotation optional, to remind us of what we're working with
*             let choice: MenuChoice = get_choice(input)?;
*             print_choice(&choice);
*             Ok(()) // returning nothing
*           }
*
* - What this does behind the scenes
*     - First assigns choice to the returned value of get_choice(input)
*         - which is a Result
*     - Then, the ? gets read and performs a hidden match on itself
*         - uses the data stored in the Ok(...) and Err(...) variant
*     - If Ok(...) matches, then the function continues
*     - If Err(...) matches, then the function early returns with an error
*
* - Allows for chaining function calls
*     - Because of the early return, you can chain a bunch of functions up
*       and if one fails the chain ends and potentially fails the whole process
*     - If nothing throws the Err(...) varient, then the function chain continues until
*       completion.
*/

//////////////////////////////////////////////////////////////////////////////////////////////////////

/*
* - RECAP -
*    In order to access the data within the varient wrapper Result applies, one must match
*    on the Ok(...) and Err(...) varients. With the inner data being a barrowed variable 
*    to be used on the other side of the fat arrow in the match arm. To avoid having to
*    write out tons of match blocks Rust employs something called the "Question Mark"
*    operator.
*
*    Basically the question mark operator checks to see if Result is returned with the
*    Ok(...) varient or the Err(...). If its the Err(...) varient then the function will
*    early return with the error, and if its Ok(...) the function continues. Allowing for
*    easy function chaining.
*    IMPORTANT: Result<> must have the Ok(...) varient returnable. Meaning it cannot be
*    ignored. Like with an underscore, "_". If the Ok(...) varient is unwanted/unneeded, then
*    the unitvalue, "()", should be used instead and then Ok(()) must be returned at the end
*    of the function.
*           
*/

//////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("not a valid choice".to_owned()),
    }
}

fn print_choice(choice: MenuChoice) {
    println!("choice: {:?}", choice);
}

// using the question mark operator rather than "match" block chains
fn pick_choice(input: &str) -> Result<(), String> {
  let choice = get_choice(input)?;
  // on Err() this code is not read because of the early return from the line above
  print_choice(choice);
  Ok(())
}

fn main() {
    // using the qurstion mark operator rather than match block chains
    let choice = pick_choice("start");
    println!("choice value: {:?}", choice)
}



