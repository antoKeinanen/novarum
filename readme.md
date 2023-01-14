# Novarum
<center>

  [![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
  [![GitHub release](https://img.shields.io/github/release/antoKeinanen/novarum.svg)](https://github.com/antoKeinanen/novarum)
</center>

Novarum is open source project setup tool. It has easy to use user interface. Novarum is only tested on Arch linux but it should work on all unix based distributions and windows as it is written in rust.

# Features
* Command line interface
* Multiple configuration support
* Custom scripting language
  * Vscode language support

# Installation
## Most Unix based systems
1. Donwload the novarum linux binary from the releases page
2. Rename the binary to `novarum`
3. Move the binary to `/usr/bin`
4. Validate that the app is installed correctly by running `novarum` and completing setup steps.
5. (Optional) Install Novarum language spport extension.

# Updating
## Most Unix based systems
1. Download the latest version of novarum from repeases page.
2. Rename the binary to `novarum`
3. Delete the old binary from `/usr/bin`
3. Move the new binary to `/usr/bin`
4. Validate that the app is installed correctly by running `novarum` and completing setup steps.


# Building from source
1. To build from source make sure you have cargo installed and updated. It should be at least of version 1.66.0.
2. Clone the repository with `git clone https://github.com/antoKeinanen/novarum.git`
3. `cd novarum`
4. Build into binary in release mode with `cargo build --release`
5. The binary will be placed to `target/release/novarum`. Move it to `/usr/bin`
6. Validate that the app is installed correctly by running `novarum` and completing setup steps.
7. (Optional) Install Novarum language spport extension.

# Configuration

Configuration path:

| Windows   | Linux/*BSD | macOS   |
|-----------|------------|---------|
|%APPDATA%\novarum (C:\Users\%USERNAME%\AppData\Roaming\novarum) | $XDG_CONFIG_HOME/novarum (~/.config/novarum) | ~/Library/Application Support/novarum |

After running the novarum for the first time open the configuration folder corresponding to your operating system. All files ending with .novconf will be auto detected by novarum. 

# Config scripting documentation
.novconf files use custom made scripting language. 

## Shell
The shell keyword is used to run shell commands. Commands will be run synchronously and their stdout output will be printed out normally.
```
shell [shell command to run]
```
### Examples
```
shell echo "Hello, world!"
```

## Chdir
Since `cd` command will not work when running it we created chdir keyword. This key word can be used to move to another directory just like cd. Chdir supports absolute and relative paths, just note that to use relative paths remember to use `./` before the path.

```
chdir [path]
```

### Examples
```
chdir /usr/bin
chdir ..
chdir  ./config
```
note that is not correct:
```
chdir directory/second
```

## Print
`print` keyword to be put simply just outputs rest of the line to stdout.
```
print [message]
```
### Examples
```
print example message!
```

## Select
`Select` creates selection interface. Users can use arrow keys to navigate between the options and enter to select one. Select supports only single selection if you want the user to select multiple options refer to `multiselect`. Select keyword is always followed by list of options followed by `end` keyword. Everything should be on their own lines. Before each option there should be `-` followed by space. By default the message of the selection is 'Select' to change this refer to `message` keyword. Note that using 2 select blocks after each other will prevent `if` from reading the answer from the first one as `select` overwrites last answer using.

```
select
  [list of options]
end
```

### Examples
```
select
  - option 1
  - option 2
  - option 3
end
```

## Multiselect
`multiselect` creates selection interface, just like select except they can choose multiple options. Users can use arrow keys to navigate between the options, space to select and deselect options, and enter continue. Multiselect supports multiple option selections if you want to restrict the user's selection options to only 1 refer to `select`. Multiselect keyword is always followed by list of options followed by `end` keyword. Everything should be on their own lines. Before each option there should be `-` followed by space. By default the message of the selection is 'Select' to change this refer to `message` keyword. Note that using 2 `multiselect` blocks after each other will prevent `if` from reading the answer from the first one as `multiselect` overwrites last answer using.

```
multiselect
  [list of options]
end
```

### Examples
```
multiselect
  - option 1
  - option 2
  - option 3
end
```

## Message
`message` keyword is used to set the message of `multiselect` or `select`. Note that there should not be trailing spaces or colon (:).

```
message [message]
```

### Examples
```
message Select one or more
```

## If
With `if` keyword you can check wether or not some option was selected. `if` is always followed by the commands to be executed if condition is true followed by `end` keyword. Note that answers from `select` and `multiselect` are overwritten every time you use them. The `if` block does not support conditions and can just be used to check if in the last `select` or `multiselect` specified option was selected.

```
if [option]
  [commands]
end
```

### Examples
The code below will not work!!!!
```
select
  - option 1
  - option 2
  - option 3
end
select
  - option 4
  - option 5
  - option 6
end
if option 1
  shell echo "will never run"
end
```
This is the correct way to do it:
```
select
  - option 1
  - option 2
  - option 3
end

if option 1
  shell echo "will run"
end

select
  - option 4
  - option 5
  - option 6
end
```

## End
`end` keyword is used to terminate `if`, `select`, and `multiselect` blocks.

### Examples
```
select
  - option 1
  - option 2
  - option 3
end
```
```
multiselect
  - option 1
  - option 2
  - option 3
end
```

```
if option 1
  shell echo "option 1 was selected"
end
```

## Comments
Comments are marked by `#` in the beginning of the line. Comments will be ignored by the interpreter.

```
# [comment]
```

### Examples
```
# This is a comment and it will be ignored!
```
