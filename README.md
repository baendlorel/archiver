# Archiver

Put your unused files and folders into `/home/user/.archive`. Supported for logging, listing and restoring

## Commands

Archived files are stored in `/home/user/.archive`. You can use the following commands to manage your archived files.
Archived files/directories are meant to be readonly.

### put

Put a file/directory into the archive. The target file will be moved to `~/.archive` where `~` means home directory.

```bash
arv put <target> # e.g. arv put my.txt
```

### restore

Move archived file/directory back to where it came from.

If there is an object with the same name, the restoration will fail.

`<id>` can be obtained from the `list` command.

```bash
arv restore <id> # also `arv rst <id>` or `arv r <id>`
```

### list

List all entries in the archive.

- The list will contain time, id, target name and original directory
- If the target is a directory, the color of its name will be blue
- The color of `<id>` is magenta.

```bash
arv list # also `arv ls`

# It should show a table like this
# 2025-05-12 17:00:05 id:4 - temp1.a   - ~/projects/personal/archiver
# 2025-05-12 19:35:07 id:6 - temp2.b.f - ~/projects/personal/archiver
```

### log

Show the logs of `put`, `restore`, `config`.

`[range]` means show logs with in this period of time. It can be in the forms below.

1. normal ranges : `YYYYMM-YYYYMM`
2. left side only : `YYYYMM`
3. right side only : `*-YYYYMM`

```bash
arv log [range] # also `arv lg [range]`
```

### config

Do some configuration about Archiver.

#### option: alias

Aliases will shorten the path shown in command `list` and `log`. But data files would still keep the full version. It is just for display.

```bash
arv config --alias a=/b/c # path `/b/c/xxx` will now display as `a/xxx`
arv config --alias-list  # show all aliases
arv config --alias-remove a=/b/c  # remove alias entry `a=/b/c`
```

### help

Show help information.

```bash
arv help
```
