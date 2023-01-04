# Terminal Game

This code is for a simple terminal game where the player can navigate around a grid and change the color of the grid cells by pressing arrow keys.

## Dependencies

This code uses the following crates:
- `std::io` for input and output functionality
- `termion` for terminal interaction and handling user input

## Structs

The `Terminal` struct contains the width and height of the terminal.

## Macros

The `clear_screen` macro is used to clear the terminal screen.

## Functions

The `prepare_terminal` function takes in a mutable reference to a `RawTerminal` and a color, and prepares the terminal for the game by clearing the screen and setting the color.

The `update_terminal` function takes in a mutable reference to a `RawTerminal`, and the x and y coordinates of the player and the height and width of the terminal, and updates the terminal to reflect the player's new position.

The `new_game` function takes in a mutable reference to a `RawTerminal` and mutable references to the height and width of the terminal, and initializes a new game by preparing the terminal and setting the initial position of the player.

The `update_position` function takes in the user input and the current x and y coordinates of the player and the height and width of the terminal, and updates the position of the player based on the input. It returns a `bool` indicating whether the game should continue.

The `game_loop` function takes in a `Stdin` instance and mutable references to a `RawTerminal`, and the x and y coordinates and the height and width of the terminal, and runs the game loop which reads user input and updates the terminal.
