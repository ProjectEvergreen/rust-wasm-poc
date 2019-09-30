# rust-wasm-poc
A playground for things related to Rust, WASM and the web
[checkout here!](https://github.com/ProjectEvergreen/rust-wasm-poc/tree/wasm-rust)

----------------------------------------------
## rustc 
| Commands                  | Output                                                            |                   
| ---------------------------------------------------------------------------------------------- | -----------------------------------------------------------------:|                   
| `rustc .\helloWorld.rs`   | This produced a helloWorld binary that can be executed.           |                     
| `.\helloWorld.exe`        | This is to run the .exe file to be seen.                          |                   
| `rustc crateExecutable.rs --extern crateFunction=libcrateFunction.rlib && .\crateFunction.rs` | This is to run lib into .exe file |

![alt text](https://i.imgur.com/5No412X.png "example to exe file helloWorld")
![alt text](https://i.imgur.com/mIaz0qr.png "example to exe file newPerson")
![alt text](https://i.imgur.com/cMOYHmP.png "example to exe file ifElseCounter")


