# Leaper
---
A simple CLI tool to quickly leap to a directory.

#### Installation
*You have to have cargo installed*
```bash
cargo install leaper
```

#### Usage
You are *home*, please cd instantly the *debug* folder, oh, you can't? :)

    .
    └── home/
        ├── dev/
        │   └── projects/
        │       └── fun
        └── etc/
            ├── misc/
            │   ├── files
            │   ├── more_files
            │   └── even_more_files/
            │       └── cathat.png
            └── more/
                └── test/
                    ├── debug
                    └── release

Try this!
```bash
home $ leap debug
Leaping to home/etc/more/test/debug

home/etc/more/test/debug $
```
Voilà!

What about a file? Sure, let's leap to the PNG.
```bash
home $ leap cathat.png
Leaping to home/etc/misc/even_more_files/cathat.png

home/etc/misc/even_more_files/ $
```
As you can see, it will leap to the parent directory of the file.


#### Notes
- Warp terminal uses a specific profile behaviour, while the tool works, it will change your prompt to your default shell profile
