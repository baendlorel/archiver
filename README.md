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

- Would faill if there is an object with the same name.

- `<id>` can be obtained from the `list` command.

- Restored target will be hidden in the `list`, unless you use `list --all`

```bash
arv restore <id> # also `arv rst <id>` or `arv r <id>`
```

### list

List all entries in the archive.

- The list will contain archived time, id, target name and original directory
- If the target is a directory, the color of its name will be blue
- The color of `<id>` is magenta.

```bash
arv list # also `arv ls`

# It should show a table like this
# 2025-05-12 17:00:05 id:4 - temp1.a   - ~/projects
# 2025-05-12 19:35:07 id:6 - temp2.b.f - ~/projects
```

#### option: all

Show all archived records including restored entries.

```bash
arv list --all # also `arv ls -a`

# It should show a table like this
# 2025-05-12 17:00:05 id:4 - temp1.a   - ~/projects
# 2025-05-12 19:35:07 id:5 - temp3(R)  - ~/projects
# 2025-05-12 19:35:07 id:6 - temp2.b.f - ~/projects
# 2025-05-12 19:35:07 id:7 - temp4(R)  - ~/projects
```

### log

Show the logs of `put`, `restore`, `config`.

`[range]` means show logs with in this period of time. It can be in the forms below.

1. normal ranges : `YYYYMM-YYYYMM`
2. left side only : `YYYYMM`
3. right side only : `*-YYYYMM`
4. CE only
5. Archiver will always consider last 2 digits of the number as month, and the rest as year

```bash
arv log [range] # also `arv lg [range]`
```

### config

Do some configuration about Archiver.

#### option: alias

Aliases will shorten the path shown in command `list` and `log`. But data files would still keep the full version. It is just for display.

```bash
arv config --alias a=/b/c # path `/b/c/xxx` displays as `a/xxx`
arv config --alias-list  # show all aliases
arv config --alias-remove a=/b/c  # remove alias config `a=/b/c`
```

### help

Show help information.

```bash
arv help
```
