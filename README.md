hello world

The cursor blinks, a little light,
Against the hush of late at night.
One line appears, then one line more,
And morning finds an open door.

## A small haunting in color

An ANSI escape code starts with `ESC` (`\x1b`), followed by `[` and a command.
For color, the command is `m`: `\x1b[31m` turns the following text red.
The terminal keeps that color until `\x1b[0m` resets it, so forgetting the reset can dye the rest of your prompt like a tiny, very bureaucratic curse.

```sh
printf '\033[35mthe ghost is magenta\033[0m\n'
printf '\033[38;2;13;255;173mthe ghost has learned RGB\033[0m\n'
```

`35` selects magenta from the basic foreground colors. `38;2;13;255;173` selects a foreground color by its red, green, and blue values; a terminal with true-color support renders it as a suspicious mint. For backgrounds, replace `38` with `48`. None of this changes the words. It only changes what the terminal thinks the words are wearing.
