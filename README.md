# Mass move

This is a simple project - analogue of [mmv](https://manpages.ubuntu.com/manpages/focal/man1/mmv.1.html), a utility for mass moving or renaming files according to a template.

I paid the most attention to proper design, clean code, testing and documentation of the entire system

## Interface

The program is supplied with two arguments: a template for selecting files and a template for the target path:

```bash
$  ./mmv 'path/to/some_*_filename.*' 'path2/to/changed_#1_filename.#2'
```

- The template for selecting files consists of a path, a name, and a **`*`** symbol inside the name indicating a substring of any length (including an empty one).

*Note: the **`*`** character can only be in the file name*

- The template for the final path is formed from ordinary characters, and also uses special markers like **`#1`**, **`#2`** and so on. These markers indicate which fragments, marked with asterisks in the original template, should be inserted into the new file name.

The utility displays a list of the original file paths and their paths after moving:

```bash
$ ./mmv 'path/to/some_*_filename.*' 'path2/to/changed_#1_filename.#2'
path/to/some_A_filename.bin -> path2/to/changed_A_filename.bin
path/to/some_A_filename.jpg -> path2/to/changed_A_filename.jpg
path/to/some_B_filename.bin -> path2/to/changed_B_filename.bin
path/to/some_B_filename.jpg -> path2/to/changed_B_filename.jpg
```

and moves the files.

## Flags

`-h` `--help` — shows help info

`-f` `--force` — overwrites existing files if they exist

## Errors

- If no files are found according to the template, an error is displayed and the program terminates with a non-zero code:

```bash
$ ./mmv 'not_exist.*' 'smth.#1'
mmv: Files for pattern 'not_exist.*' not found
```

- If there is an existing file among the target paths and there is no `-f` flag, an error is displayed and the program terminates with a non-zero code:

```bash
$ ./mmv 'smth.*' 'exists.#1'
mmv: Not able to replace existing file: exists.bin 
```

## Tests

Both unit tests are implemented for those functions that can be tested in this way, as well as integration tests to verify the functionality of the entire utility as a whole.

## Documentation

Documentation is embedded in the written code.
