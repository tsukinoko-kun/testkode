hello world

The cursor blinks, a little light,
Against the hush of late at night.
One line appears, then one line more,
And morning finds an open door.

## A small haunting in color

First `ESC` (`\x1b`) slips through the crack,
Then `[` brings a number on its back.
An `m` seals the spell: `\x1b[31m` burns red;
`\x1b[0m` wakes the terminal instead.

Forget that last charm, and the color won't leave.
Your prompt wears the ghost's coat all through the eve.

```sh
printf '\033[35mthe ghost is magenta\033[0m\n'
printf '\033[38;2;13;255;173mthe ghost has learned RGB\033[0m\n'
```

`35` paints magenta, a bruise on the night;
`38;2;13;255;173` gives true-color mint its light.
Three numbers for red, green, blue in the glow.
Trade `38` for `48`, and the background will show.
The words stay the same, though their costumes get strange.
The terminal shivers. The letters don't change.
