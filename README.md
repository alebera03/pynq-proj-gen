# PYNQ-PROJ-GEN
## Installation
Installation is pretty easy:
- ensure that you are connected with your board via ssh
- run this command
```
bash <(curl -sSL https://raw.githubusercontent.com/AleBera03/pynq-proj-gen/master/build.sh)
```
## Dependencies
Imagine you have a PC and a board connected via ssh, on PC you must have:
- `git` because each project is initialized (behaviour similar to `Cargo new`)
- rust installed. If you have not rust, follow [the guide](https://rust-lang.org/tools/install/), for compile the source code (it is very small)