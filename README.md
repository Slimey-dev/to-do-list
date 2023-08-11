# to-do-list

## Description

This is a simple to-do list app written in rust that allows the user to add, edit, and delete tasks. The user can also
mark tasks as complete.

## Usage

To run the app, clone the repository and run `cargo run` in the root directory. The app will then prompt you to enter a
command. The commands are as follows:

### not implemented yet, just mockup

- `add` - Add a new task
- `edit` - Edit an existing task
- `delete` - Delete an existing task
- `complete` - Mark a task as complete
- `list` - List all tasks
- `exit` - Exit the app
- `help` - Display the help menu
- `clear` - Clear the screen
- `save` - Save the current task list to a file
- `load` - Load a task list from a file
- `quit` - Exit the app

## Example

```
$ cargo run
Welcome to your to-do list!
Enter a command (help to see all commands): help
add - Add a new task
edit - Edit an existing task
delete - Delete an existing task
complete - Mark a task as complete
list - List all tasks
exit - Exit the app
help - Display the help menu
clear - Clear the screen
save - Save the current task list to a file
load - Load a task list from a file
quit - Exit the app
Enter a command (help to see all commands): add
Enter a task: Do the dishes
Enter a command (help to see all commands): add
Enter a task: Do the laundry
Enter a command (help to see all commands): list
1. Do the dishes
2. Do the laundry
Enter a command (help to see all commands): complete
Enter the number of the task to complete: 1
Enter a command (help to see all commands): list
1. Do the laundry
Enter a command (help to see all commands): exit
```

