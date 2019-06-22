extern crate wasm_bindgen;
extern crate rand;

use wasm_bindgen::prelude::*;
use rand::Rng;


#[derive(Debug)]
enum UsState {
  Alabama, Alaska, Arizona, Arkansas, California, Colorado, Connecticut, Delaware, Florida, Georgia 
, Hawaii, Idaho, Illinois, Indiana, Iowa, Kansas, Kentucky, Louisiana, Maine, Maryland 
, Massachusetts, Michigan, Minnesota, Mississippi, Missouri, Montana, Nebraska, Nevada, NewHampshire, NewJersey 
, NewMexico, NewYork, NorthCarolina, NorthDakota, Ohio, Oklahoma, Oregon, Pennsylvania, RhodeIsland, SouthCarolina 
, SouthDakota, Tennessee, Texas, Utah, Vermont, Virginia, Washington, WestVirginia, Wisconsin, Wyoming
, NoState
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> (u8, String) {
  match coin {
    Coin::Penny          => (1,  String::from(""))
  , Coin::Nickel         => (5,  String::from(""))
  , Coin::Dime           => (10, String::from(""))
  , Coin::Quarter(state) => (25, format!("State quarter from {:?}!", state))
  }
}

fn coin_gen(idx: u8) -> Coin {
  match idx {
    0 => Coin::Penny
  , 1 => Coin::Nickel
  , 2 => Coin::Dime
  , _ => Coin::Quarter(state_gen(rand::thread_rng().gen_range(0, 50)))
  }
}

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
    , _  => UsState::NoState
  }
}

fn as_currency_amount(amt: u16) -> String {
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
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn coin_counter(coins: u32) -> Result<(), JsValue> {
  // Use `web_sys`'s global `window` function to get a handle on the global window object.
  let window   = web_sys::window().expect("Error: Can't acces the global `window` or it does not exist");
  let document = window.document().expect("Error: The document object is missing from the window object");
  let body     = document.body().expect("Error: The window.document object should have a body object");

  // Define a heading element
  let h1 = document.create_element("h1")?;
  h1.set_inner_html("Coin Counter");

  body.append_child(&h1)?;

  // Define a DIV element to contain the message output
  let div1 = document.create_element("div")?;

  let mut purse = Vec::<Coin>::new();
  let mut total: u16 = 0;

  // Generate a random assortment of coins
  for _idx in 0..coins {
    purse.push(coin_gen(rand::thread_rng().gen_range(0, 4)))
  }

  // Add up the total value of the coins in the purse
  for coin in purse.iter().clone() {
    // The use of the match here is a workaround for the fact that Rust doesn't permit tuple destructuring outside the
    // context of a "let" declaration.  Otherwise, it would be much simpler to write:
    // let mut coin_val: u32 = 0;
    // let mut state_txt: &str;
    // (coin_val, state_txt) = value_in_cents(coin);
    match value_in_cents(coin) {
      (coin_val, state_txt) => {
        total += coin_val as u16;

        if state_txt.len() > 0 {
          let p = document.create_element("p")?;
          p.set_inner_html(&state_txt);
          div1.append_child(&p)?;
        }
      }
    }
  }

  let total_msg = document.create_element("p")?;
  total_msg.set_inner_html(&format!("The {} coins in your purse are worth {}", coins, as_currency_amount(total)));
  div1.append_child(&total_msg)?;

  body.append_child(&div1)?;

  Ok(())
}
