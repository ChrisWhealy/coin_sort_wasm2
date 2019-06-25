extern crate wasm_bindgen;
extern crate web_sys;
extern crate rand;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ HtmlElement };

use rand::Rng;

// Without this derive(Debug) directive the format macro inside function "value_in_cents" won't work
#[derive(Debug)]
enum UsState {
  Alabama, Alaska, Arizona, Arkansas, California, Colorado, Connecticut, Delaware, Florida, Georgia 
, Hawaii, Idaho, Illinois, Indiana, Iowa, Kansas, Kentucky, Louisiana, Maine, Maryland 
, Massachusetts, Michigan, Minnesota, Mississippi, Missouri, Montana, Nebraska, Nevada, NewHampshire, NewJersey 
, NewMexico, NewYork, NorthCarolina, NorthDakota, Ohio, Oklahoma, Oregon, Pennsylvania, RhodeIsland, SouthCarolina 
, SouthDakota, Tennessee, Texas, Utah, Vermont, Virginia, Washington, WestVirginia, Wisconsin, Wyoming

// The NoState value exists only to keep the compiler happy
, NoState
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

// Return the numerical value of a particular coin
fn value_in_cents(coin: Coin) -> (u8, String) {
  match coin {
    Coin::Penny          => (1,  String::from(""))
  , Coin::Nickel         => (5,  String::from(""))
  , Coin::Dime           => (10, String::from(""))
  , Coin::Quarter(state) => (25, format!("State quarter from {:?}!", state))
  }
}

// Generate a coin from an integer in the range 0..3
fn coin_gen(idx: u8) -> Coin {
  match idx {
    0 => Coin::Penny
  , 1 => Coin::Nickel
  , 2 => Coin::Dime
  , _ => Coin::Quarter(state_gen(rand::thread_rng().gen_range(0, 50)))
  }
}

// Generate a US state from an integer expected to be in the range 0..49
fn state_gen(idx: u8) -> UsState {
  match idx % 50 {
       0 => UsState::Alabama,        1 => UsState::Alaska,      2 => UsState::Arizona,        3 => UsState::Arkansas,      4 => UsState::California 
    ,  5 => UsState::Colorado,       6 => UsState::Connecticut, 7 => UsState::Delaware,       8 => UsState::Florida,       9 => UsState::Georgia 
    , 10 => UsState::Hawaii,        11 => UsState::Idaho,      12 => UsState::Illinois,      13 => UsState::Indiana,      14 => UsState::Iowa 
    , 15 => UsState::Kansas,        16 => UsState::Kentucky,   17 => UsState::Louisiana,     18 => UsState::Maine,        19 => UsState::Maryland 
    , 20 => UsState::Massachusetts, 21 => UsState::Michigan,   22 => UsState::Minnesota,     23 => UsState::Mississippi,  24 => UsState::Missouri 
    , 25 => UsState::Montana,       26 => UsState::Nebraska,   27 => UsState::Nevada,        28 => UsState::NewHampshire, 29 => UsState::NewJersey 
    , 30 => UsState::NewMexico,     31 => UsState::NewYork,    32 => UsState::NorthCarolina, 33 => UsState::NorthDakota,  34 => UsState::Ohio 
    , 35 => UsState::Oklahoma,      36 => UsState::Oregon,     37 => UsState::Pennsylvania,  38 => UsState::RhodeIsland,  39 => UsState::SouthCarolina 
    , 40 => UsState::SouthDakota,   41 => UsState::Tennessee,  42 => UsState::Texas,         43 => UsState::Utah,         44 => UsState::Vermont 
    , 45 => UsState::Virginia,      46 => UsState::Washington, 47 => UsState::WestVirginia,  48 => UsState::Wisconsin,    49 => UsState::Wyoming

    // Hmmm, that's weird... How did we end up here?
    , _  => UsState::NoState
  }
}

// Convert a current value to its formatted string equivalent
fn as_currency_string(amt: u16) -> String {
  format!("${:.*}", 2, (amt as f32) / 100.0)
}

// *********************************************************************************************************************
// Start here...
// *********************************************************************************************************************
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  Ok(())
}

// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
#[wasm_bindgen]
pub fn coin_counter(coins: u32, div_id: String) -> Result<(), JsValue> {
  // Use `web_sys`'s global `window` function to get a handle on the global window object.
  let window   = web_sys::window().expect("Error: Can't acces the global `window` or it does not exist");
  let document = window.document().expect("Error: The document object is missing from the window object");

  // Clear output DIV of any existing content
  document
    .get_element_by_id(&div_id)
    .expect(&format!("Can't find element with id '{}'", &div_id))
    .dyn_ref::<HtmlElement>()
    .expect(&format!("'{}' should be an HTML element", &div_id))
    .set_inner_html("");

  let mut total: u16 = 0;

  // Generate the requested number of random coins then find out how much each coin is worth
  for _idx in 0..coins {
    // The use of the "match" statement here is a workaround for the fact that Rust doesn't permit tuple destructuring 
    // outside the context of a "let" declaration.  Otherwise, it would be much simpler to write:
    // let mut coin = coin_gen(rand::thread_rng().gen_range(0, 4));
    // let mut coin_val: u32 = 0;
    // let mut state_txt: &str;
    // (coin_val, state_txt) = value_in_cents(coin);
    match value_in_cents(coin_gen(rand::thread_rng().gen_range(0, 4))) {
      (coin_val, state_txt) => {
        total += coin_val as u16;

        if state_txt.len() > 0 {
          let p = document.create_element("p")?;
          p.set_inner_html(&state_txt);
          document
           .get_element_by_id(&div_id)
           .expect(&format!("Can't find element with id '{}'", &div_id))
           .dyn_ref::<HtmlElement>()
           .expect(&format!("'{}' should be an HTML element", &div_id))
           .append_child(&p)?;
        }
      }
    }
  }

  // Write out the total line
  let total_msg = document.create_element("p")?;
  total_msg.set_inner_html(&format!("The {} coins in your purse are worth {}", coins, as_currency_string(total)));

  document
    .get_element_by_id(&div_id)
    .expect(&format!("Can't find element with id '{}'", &div_id))
    .dyn_ref::<HtmlElement>()
    .expect(&format!("'{}' should be an HTML element", &div_id))
    .append_child(&total_msg)?;

  Ok(())
}
