This project is a command line visualizer for the collatz conjecture.
The visualization will look as follows:
```
│   │   ├───52
│   │   ├───26
│   │   ├───13
│   ├───40
│   ├───20
│   │   ├───24
│   │   ├───12
│   │   ├───6
│   │   ├───3
│   ├───10
│   ├───5
├───16
├───8
├───4
├───2
├───1
0
```
Going down from any number is the same as applying one collatz operation.<br>
In other words, if x is even, it will lead to x/2 and otherwise 3x+1. <br><br>
The right-arrow key will expand as far right as possible by taking (x-1)/3 if possible and 2x otherwise. <br> 
The left key will always go left by reverting to twice the previous number. <br>
The up key will automatically choose and the down key will go backwards. Also, when going back up after going down, the previous choices will be preserved instead of redoing them. <br>
The 'q' key can be pressed to exit the program once run.

# Building
The project can be built with cargo. It is as simple as installing cargo/rust, entering the directory of the project, and the running the command
```
$ cargo build
```
Or, to run it,
```
$ cargo run
```
