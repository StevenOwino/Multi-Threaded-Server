Remember to terminate the server with Control C.

Check that the code compiles without any warnings: 
 $ cargo check

 (base) Steven-Owinos-iMac:hello stevenowino$ cargo check
    Checking hello v0.1.0 (/Users/stevenowino/RUST_Cookbook/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 4.14s
(base) Steven-Owinos-iMac:hello stevenowino$

Change the code on line 26 on the "main.rs" file to: 
 Line 26 : for stream in listener.incoming().take(2) { 

   (base) Steven-Owinos-iMac:hello stevenowino$ cargo check
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s

Start the server with : 
 $ cargo run 'hello' 
   
   (base) Steven-Owinos-iMac:hello stevenowino$ cargo run 'hello'
      Compiling hello v0.1.0 (/Users/stevenowino/RUST_Cookbook/hello)
       Finished dev [unoptimized + debuginfo] target(s) in 2.45s
        Running `target/debug/main hello`

Then, make requests on your web browser,
To demonstrate that the graceful shutdown and cleanup is in working order.
http://127.0.0.1/ and http://127.0.0.1/sleep on multiple tabs 


Shutting down.
Sending terminate message to all workers.
Shutting down all workers.
Shutting down worker 0
Worker 0 got a job; executing.
Worker 1 got a job; executing.
Worker 2 was told to terminate.
Worker 3 was told to terminate.
Worker 0 was told to terminate.
Shutting down worker 1
Worker 1 was told to terminate.
Shutting down worker 2
Shutting down worker 3
(base) Steven-Owinos-iMac:hello stevenowino$





