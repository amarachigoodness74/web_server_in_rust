# web_server_in_rust
A simple project to learn how to build a custom web server using Rust. This web server returns a home page when you visit `/` or a hello page when you visit `/hello` or not found page for any other route.

Libraries used are: 
- fs: to read files
- io: to read and write to incoming stream
- net: to listen for request and send back response

## How to use
- Install rust on your machine
- Clone/fork this repo
- Run: `cargo run`
- Open your web browser and open `localhost:8081`
- You can enter a wrong path to view the `not found` page
