# Calculator
This is a simple command line calculator, like `bc`. It supports integers, hexadecimal integers (with `0x` or `0X`) and floats.
The 4 operations (`+`, `-`, `*`, `/`) are supported as well as parentheses.

You can run it as follow:
```shell
> ./calculator
> 1 + 1
2
>
```

You can also pass the input as a command line argument `./calculator "(1+1)"` or piped in: `echo "(1-1)" | ./calculator`.

There is one option: `-d` to display debugging informations.

# TODO
[ ] Add a proper logging library to avoid passing the debugging info everywhere

[ ] Add support for built-in functions (`cos`, `sin`, `tan`, `sqrt`)

[ ] Add support for exponential

[ ] Add support for custom variables

[ ] Add support for custom functions