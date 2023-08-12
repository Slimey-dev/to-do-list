# to-do-list

## Description

This is a simple to-do list app written in rust that allows the user to add, edit, and delete tasks. The user can also
mark tasks as complete.

## Usage

To run the app, clone the repository and run `cargo run` in the root directory. The app will then prompt you to enter a
command. The commands are as follows:

- `help` - show this message
- `list` - show the to-do list
- `add` - add an item to the to-do list
- `check` - check an item on the to-do list
- `uncheck` - uncheck an item on the to-do list
- `remove` - remove an item from the to-do list
- `remove completed` - remove all completed items from the to-do list
- `quit` - quit the program

## Example

```
$ cargo run
Welcome to your to-do-list!
Enter a command (help to see all the commands): help
Commands:
help - show this message
list - show the to-do list
add - add an item to the to-do list
check - check an item on the to-do list
uncheck - uncheck an item on the to-do list
remove - remove an item from the to-do list
remove completed - remove all completed items from the to-do list
quit - quit the program
Enter a command (help to see all commands): add
Enter the item you want to add: Do the dishes
Enter a command (help to see all commands): add
Enter the item you want to add: Do the laundry
Enter a command (help to see all commands): list
1. [ ] Do the dishes
2. [ ] Do the laundry
Enter a command (help to see all commands): check
Enter the index of the item you want to check:
1. [ ] Do the dishes
2. [ ] Do the laundry
1
Enter a command (help to see all commands): list
1. [x] Do the dishes
2. [ ] Do the laundry
Enter a command (help to see all commands): exit
```
