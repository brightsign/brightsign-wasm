# brightsign-wasm
A simple webassembly project to run on brightsign devices

# Project Structure
The project consists of two subprojects:

# xtask
xtask-wasm is a code build tool to help generate and package wasm files. It gives us some convenience features
like file monitoring and opening up a dev server for us. It should be noted that at the moment the dev server
doesn't appear to be working quite right ( for this project ). The benefit of using this over something like wasm-pack is that it offers greater control over how your project is built, allows us to package additional files ( static assets ) and doesn't require an external tool. The downside is that setting it up can be quite complicated outside of the most basic configurations ( which this project uses ) and it pulls in loads of additional libraries and can take a very long time to compile the first time you use it.

# brightsign-wasm
brightsign-wasm is a simple rust project that acts as a template for further web assembly projects meant to be
run on brightsign devices. It provides a number of files:

* lib.rs - An example wasm-bindgen project that creates a wasm entry/start point ( run on initialization ) and
  exports an adder function. This demonstrates how to use wasm-bindgen/web_sys to modify the dom and
  output to console. It should be noted that every web-sys feature you want to use needs to be added to the Cargo.toml features list. By default nothing is enabled so if you find you need additional features you should take a look at https://docs.rs/crate/web-sys/latest/source/Cargo.toml and pull in what you need.
* static/index.html - A very barebones html page for wasm-loader to be run from.
* static/wasm-loader.js - The file responsible for importing and loading the wasm file.
* static/autorun.brs - The file responsible for creating a webserver for the wasm file to be served
  from and for creating the html widget.

# Usage
1.) Compile the project: cargo xtask dist
2.) Assuming compilation succeeded the project files should be located in the dist/ directory.
3.) Upload the files in dist/ to a brightsign device.
