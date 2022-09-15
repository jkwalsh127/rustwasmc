# rustwasmc
Learn to use Rust fxns with Node.js applications

Installing rust: installs a tool called “rustup”, allowing you to manage multiple versions of Rust
	- Rustup installs 
        - rustc (Rust compiler)
        - cargo (Rust’s package manager)
        - rust -std (Rust’s standard libraries)
        - rust-docs (documentation

Install wasm=pack with cargo. Helps compile code to WebAssembly, and produce the right packaging for use in the browser.

Create new rust package: $ cargo new –your_name hello-wasm
	- Creates subdirectory with:
		- Cargo.toml (to configure your build, like package.json for npm)	
        - src/your_name.rs (a little Rust code)

*** Libraries in Rust are called “crates” (Cargo ships crates…) ***
*** in Rust, the “use” command imports code from a library ***

Wasm-pack uses wasm-bindgen, another tool, to provide a bridge between the types of JavaScript and Rist. It allows JS to call a Rust API with a string, or a Rust function to catch a JS exception.

*** Rust preludes statements with #[ ], providing “attributes” to modify the following code ***

```
#[wasm_bindgen]
extern {
	Pub fn alert(s: &str);
}
```
This statement is an “extern”, which tells Rust we want to call some externally defined functions. The attribute says “wasm-bindgen knows how to find these fxns.” The function is written in Rust, saying that the alert function takes a string named “s”, but the “alert” function itself is a JS function!
	- You can call any JS function, and wasm-bindgen takes care of everything.
```
#[wasm_bindgen]
pub fn greet(name: &str) {
	alert(&format!(“Hello, {}!”, name));
}
```
Here, wasm_bindgen is modifying a function rather than an extern block, meaning that we want this Rust function to be able to be called by JS. This is the opposite of extern, since these are not functions we need, but are the functions we’re giving out to the world.
	- The greet function calls on the alert function we asked for in the extern block. 
	- The format! macro allows us to concatenate strings. It’s two arguments are a format
 	- string and a variable to put it in. The curly braces are where variables will be interpolated. Calling greet(“Steve”) will pass “Hello, Steve!” to the JS alert() function.

To correctly compile the code, configure the Cargo.toml as follows:
```
[package]
name = "project_name"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A sample project with wasm-pack"
license = "MIT/Apache-2.0"
repository = "https://github.com/yourgithubusername/project_name"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

	- [lib] tells Rust to build a cdylib version of our package (a “dynamic system library”, 
	Allowing a dynamic library to be loaded from another language when compiling.

To build the package using webpack, run “wasm-pack build --target web”
(To build the package using node.js, run “wasm-pack build --target nodejs”)
	- This does several things, and will take longer if you haven’t run wasm-pack before. In short, this:
        - Compiles your rust code to WebAssembly
        - Runs wasm-bindgen on that WebAssembly, generating a JS file that wraps that WebAssembly file into a module the browser can understand
        - Creates a pkg directory and move that JS file and your WebAssembly code into it
        - Reads your Cargo.toml and produces an equivalent package.json
        - Copies your README.md (if present) into the package
	- Ultimately, this means you have a package inside the pkg directory
*** The generated WebAssembly code size is a few hundred kilobytes, but instructing rust to optimize for size cuts this down a lot https://rustwasm.github.io/book/game-of-life/code-size.html#shrinking-wasm-size ***

This package can now be made available to npm:
	- First, a few changes must be made:
        - Recompile Rust with the target bundle option:
        - “wasm-pack build –target bundler”
        - Ensure node.js and npm are installed
        - Use ‘npm link’ to make the package available to other JS packages installed 
        - “cd pkg” “npm link”
        - We now have an npm package, written in Rust, but compiled to WebAssembly. It is ready to use from JS, and doesn’t require the user to have Rust installed (the code included was the WebAssembly code, not the Rust source)