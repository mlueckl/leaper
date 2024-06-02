# Leaper
---
A simple CLI tool to quickly leap to a directory.

#### Usage
You are *home*, switch to *debug* or to *cathat.png*'s directory

    .
    └── home/
        ├── dev/
        │   └── ...
        │       └── ...
        └── etc/
            ├── misc/
            │   └── even_more_files/
            │       └── cathat.png
            └── more/
                └── test/
                    ├── debug
                    └── ...

```bash
$ leaper debug
```

```bash
$ leaper cathat.png
```

```bash
$ leaper --help
```

#### Setup
1. Run `cargo install leaper`
2. Create shell action

##### Zsh example
```zsh
leap() {
    # Call tool and forward all args
    local dir_path=$(leaper "$@")

    # Check if a path was returned
    if ["$dir_path"]; then
        # 'cd' to the directory
        cd "$dir_path"
    else
        echo "'$1' not found"
    fi
}
```
