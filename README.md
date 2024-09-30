# epita_format

**C files formatting project for EPITA (2024 standards)**  
> This is based on [Antoine's idea](https://github.com/antoinedray/coding-style/).

I want to [install](#install-for-epita-students-only) it.  
I want to [collaborate](#collaborate).  

## Install (for EPITA students only)

Download the [GETME.sh](./GETME.sh) file or copy it.  
Run it with `./GETME.sh`.  
You should be good to go after a few minutes.  
See [Use](#use) section bellow.

## Use

Once [installed](#install-for-epita-students-only), you can either use `clang-format -i --style=file <FILES>` and `~/afs/epita_format <FILES>` or setup a shell script that does both for you like this:

```sh
#!/bin/sh

clang-format -i --style=file "$@"
~/afs/epita_format "$@"

```

> Couple this with an alias to only use a few keystrokes to run it: `alias cf='path/script.sh'` (replace `path/script.sh` with the actual script path).

## Collaborate

See [todo list](./TODO.md) for features that need to be added.  
You want to add a new cool feature or report a bug? Report it as an issue on the [github](https://github.com/abelr-githubplace/epita_format).
