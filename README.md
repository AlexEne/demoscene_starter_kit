# Rust demoscene starter kit
Demoscene starter kit for rust

## How to build
1. Clone and follow the build instructions of https://github.com/neosmart/msvcrt.lib
2. Create a folder with the following contents: `support\msvcrt.lib`
3. Run `rustup install nightly`
4. Run `cargo +nightly run --release`
5. Observe a nice error:
```
error: process didn't exit successfully: `target\debug\demoscene_starter_kit.exe` (exit code: 10)
```

Inspecting the assembly generated we can see:
![alt text](https://alexene.github.io/images/demoscene_starter_kit/assembly.png)


## Credits
Most of the work here is not mine and it was possible with the amazing help from [cpdt](https://gist.github.com/cpdt). 
99.9% of the work here is copy-pasted from him :)
