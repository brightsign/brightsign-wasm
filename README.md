# brightsign-wasm
A simple webassembly project to run on brightsign devices

# Project Structure
The project consists of two subprojects:

# xtask
xtask-wasm is a code build tool to help generate and package wasm files. It gives us some convenience features
like file monitoring and opening up a dev server for us. It should be noted that at the moment the dev server
doesn't appear to be working quite right ( for this project ).

# brightsign-wasm
brightsign-wasm is a simple rust project that acts as a template for further web assembly projects meant to be
run on brightsign devices. It provides a number of files:

* lib.rs - An example wasm-bindgen project that creates a wasm entry/start point ( run on initialization ) and
  exports an adder function. This demonstrates how to use wasm-bindgen/web_sys to modify the dom and
  output to console.
* static/index.html - A very barebones html page for wasm-loader to be run from.
* static/wasm-loader.js - The file responsible for importing and loading the wasm file.
* static/autorun.brs - The file responsible for creating a webserver for the wasm file to be served
  from and for creating the html widget.

