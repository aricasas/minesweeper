# TODO:

- Command line minesweeper
- It should be 16x16
- You should be able to move with your arrow keys
- Uncover a mine with the space key
- Flag or unflag a mine with the F key
- Press the spacebar on a number with the right number of flagged mines next to it to uncover all the surrounding ones
- Press R to reset and start a new game
- Press Q or esc to quit
- Maybe customizable number of mines with 30 as a default
- The symbols should have colors
- It should have something on top of the screen which displays number of remaining mines
- It shouldn't say how much time you have taken
- It should have some kind of border around the minefield
- It should make sure that in your first click you uncover a completely empty spot (0 mines around it)

Symbols should be the following:

```
0 mines          - no break space ( )
1 mines          - 1
2 mines          - 2
3 mines          - 3
4 mines          - 4
5 mines          - 5
6 mines          - 6
7 mines          - 7
8 mines          - 8
Covered spot     - ▒
Mines you missed - X
Erroneous flags  - @
```

It should look like this:

```
 Remaining mines: 30
 q | quit   r | restart
╔═════════════════════════════════╗
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
║ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ║
╚═════════════════════════════════╝


 Remaining mines: 06
 q | quit   r | restart
╔═════════════════════════════════╗
║ ▒ ▒ ▒ ▒ ▒ ▒ ■ 2     1 2 3 2 1   ║
║ ▒ ▒ ▒ ▒ ▒ 4 ■ 2     1 ■ ■ ■ 1   ║
║ ▒ ▒ ▒ ▒ 3 3 2 2     1 1 1 1 1   ║
║ ▒ ▒ ▒ ▒ 1 1 ■ 1                 ║
║ ▒ ▒ ▒ ▒ 1 2 1 1             1 1 ║
║ ▒ ▒ ▒ 2 ■ 2 2 2 1 1 1 1     1 ■ ║
║ ▒ ▒ ▒ 3 3 5 ■ ■ 1 1 ■ 1     1 1 ║
║ ■ 3 2 2 ■ ■ ■ 3 1 1 1 1         ║
║ 2 ■ 1 1 3 ■ 3         1 1 1 1 1 ║
║ 1 1 1   1 1 1         2 ■ 1 1 ■ ║
║                       2 ■ 1 1 1 ║
║                       1 1 1     ║
║               1 1 1     1 1 1   ║
║     1 1 1     1 ■ 1     1 ■ 1   ║
║     ■ ■ 1 1 1 2 1 1     1 1 1   ║
║     1 1 1 1 ■ 1                 ║
╚═════════════════════════════════╝

 Remaining mines: 06
 q | quit   r | restart
╔═════════════════════════════════╗
║ ▒ ▒ ▒ ▒ ▒ X ■ 2     1 2 3 2 1   ║
║ ▒ ▒ ▒ X X 4 ■ 2     1 ■ ■ ■ 1   ║
║ ▒ ▒ ▒ X 3 3 2 2     1 1 1 1 1   ║
║ ▒ ▒ ▒ ▒ 1 1 ■ 1                 ║
║ ▒ X ▒ ▒ 1 2 1 1             1 1 ║
║ ▒ ▒ ▒ 2 ■ 2 2 2 1 1 1 1     1 ■ ║
║ ▒ ▒ X 3 3 5 ■ ■ 1 1 ■ 1     1 1 ║
║ ■ 3 2 2 ■ ■ ■ 3 1 1 1 1         ║
║ 2 ■ 1 1 3 ■ 3         1 1 1 1 1 ║
║ 1 1 1   1 1 1         2 ■ 1 1 ■ ║
║                       2 ■ 1 1 1 ║
║                       1 1 1     ║
║               1 1 1     1 1 1   ║
║     1 1 1     1 ■ 1     1 ■ 1   ║
║     @ ■ 1 1 1 2 1 1     1 1 1   ║
║     1 1 1 1 ■ 1                 ║
╚═════════════════════════════════╝

```
