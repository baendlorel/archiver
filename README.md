# Archiver

Troubled with too many projects? Cannot focus on your current work?

**Archiver helps you now!**

## Commands

Archived files are stored in `~/.archive` where `~` means home directory. You can use following commands to manage your archived files.

### put

Put a file/directory into the archive.

Just get into a directory, run this command and you can move your target into `.archive`.

- Archived target will be given an unique id. The id is auto incremental.

```bash
arv put <target> # e.g. `arv put my.txt`

# Might show
# Archiving my.txt
# 'my.txt' is successfully archived, id:1
```

### restore

Move archived file/directory back to where it came from.

- Will fail if there is an object with the same name

- `<id>` can be obtained from the `list` command

- The restored targets will be hidden when executing `list` command, unless you use `list --all`.

```bash
# also `arv rst <id>` or `arv r <id>`
arv restore <id>
```

### list

List all entries in the archive.

- The list will contain archived time, id, target name and original directory
- If the target is a directory, its name will be shown in blue
- The `<id>` will be shown in magenta.

```bash
arv list # also `arv ls`

# It should show a table like this
# 2025-05-12 17:00:05 4 temp1.a   ~/projects
# 2025-05-12 19:35:07 6 temp2.b.f ~/projects
```

#### option: all

Show all archived records, including the restored ones.

```bash
arv list --all # also `arv ls -a`

# It should show a table like this
# 2025-05-12 17:00:05 4 temp1.a   ~/projects
# 2025-05-12 19:35:07 5 temp3(R)  ~/projects
# 2025-05-12 19:35:07 6 temp2.b.f ~/projects
# 2025-05-12 19:35:07 7 temp4(R)  ~/projects
```

### log

Show the logs of `put`, `restore`, and `config` operations.

`[range]` means showing logs within a specific period of time. It can take the following forms:

1. normal ranges : `YYYYMM-YYYYMM`
2. left side only : `YYYYMM`
3. right side only : `*-YYYYMM`
4. Common Era only
5. Archiver will always consider the last 2 digits of the number as the month, and the rest as the year.

```bash
arv log [range] # also `arv lg [range]`
```

### config

Configure some properties of Archiver.

You can set the following configurations:

1. `alias` : set an alias for a path
2. `list`: show configurations
3. `auto-check-update`: enable or disable auto check for updates

#### option: list \[item\]

Show configurations.

```bash
arv config --list

# show all aliases
arv config --list alias
```

#### option: alias

Aliases will shorten the paths shown in command `list` and `log`. However, Archiver will still keep the full version. This is just for display purposes.

```bash
# path `/b/c/xxx` displays as `a/xxx`
arv config --alias a=/b/c

# remove alias config `a=/b/c`
arv config --alias-remove a=/b/c
```

#### option: auto-check-update

### update

The

### help

Show help information for all commands.

```bash
arv help
```
