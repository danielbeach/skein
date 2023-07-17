# skein
#### Rust-based attempt at Distributed Processing, learning things.

The idea is to use `Rust` to write a simple distributed data processing system. 
Through this exercise, we will learn the basics of building a distributed system.

If you do `cargo run` , the `Scheduler` will start to listen on your local port (1024).
You can send a message to this port from a `terminal` on your machine and the
scheduler will print it out.

- First part written is using `tcp` to bind and listen for messages.

Something like `echo "Justin I did it" | nc -c localhost 1024`

![alt text](https://github.com/danielbeach/skein/blob/main/imgs/echo.png?raw=true)

![alt text](https://github.com/danielbeach/skein/blob/main/imgs/node.webp?raw=true)



