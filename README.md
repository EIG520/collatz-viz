This project is a vizualizer for the collatz conjecture.
The vizualization will look as follows:
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
Therefore, if x is even, it will lead to x/2 and otherwise 3x+1. Up and to the right will be <br><br>
The right-arrow key will expand as far right as possible by taking (x-1)/3 if possible and 2x otherwise. <br> 
The left key will always go left by reverting to twice the previous number. <br>
The up key will automatically choose and the down key will go backwards. Also, when going back up after going down, the previous choices will be preserved instead of redoing them.
