# COIN_SORT_WASM2

Same functionality as <https://github.com/ChrisWhealy/coin_sort_wasm> but this project imports the WASM module into the browser and executes it directly.

This allows the public functions in the WASM module (denoted in the Rust module using the directive `#[wasm_bindgen]`) to be called directly from JavaScript.

HTML output is written to `document.body` instead of the console.

## Compilation

Change into the `coin_sort_wasm2` directory and run 

`wasm-pack build --target web`

## Execution

The easiest way to run this is to start a local Web server using Python:

`python3 -m http.server`

Open your browser and visit <http://0.0.0.0:8000/>   

![Start screen](./img/Screenshot%201.png)

Enter an integer value in the input field and press "Calculate!"  

![Result screen](./img/Screenshot%202.png)

