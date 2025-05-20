# Archiver

Troubled with too many projects? Cannot focus on your current work?

**Archiver helps you now!**

## Install

Install script recommended （supports Linux/macOS）

_Does not support Windows yet_

```bash
curl -fsSL https://github.com/baendlorel/archiver/blob/main/archiver-installer.sh | bash
```

## Where does the files go?

Archived files are stored in `~/.archive` where `~` means home directory. You can use following commands to manage your archived files.

```text
.archive          Store all archived files
  ├logs/          Log files, in form of `jsonl`
  └core/          Includes important files
    ├config.json  Configurations
    ├auto-incr    Auto-increment id for archive id
    └list.jsonl   List of all archived objects
```

## Put

Put a file/directory into the archive.

Just get into a directory, run this command and you can move your target into `.archive`.

- Archived target will be given an unique id. The id is auto incremental.

```bash
arv put <target> # e.g. `arv put my.txt`

# Might show
# Archiving my.txt
# 'my.txt' is successfully archived, id:1
```

## Restore

Move archived file/directory back to where it came from.

- Will fail if there is an object with the same name

- `<id>` can be obtained from the `list` command

- The restored targets will be hidden when executing `list` command, unless you use `list --all`.

```bash
# also `arv rst <id>` or `arv r <id>`
arv restore <id>
```

## List

Show all entries in the archive.

- The list will contain archived time, id, target name and original directory
- If the target is a directory, its name will be shown in blue
- The `<id>` will be shown in magenta.

```bash
arv list
arv ls
arv l

# It should show a table like this
# 2025-05-12 17:00:05 4 temp1.a   ~/projects
# 2025-05-12 19:35:07 6 temp2.b.f ~/projects
```

### option: --all

Show all archived records, including the restored ones.

```bash
arv list --all
arv l -a

# It should show a table like this
# 2025-05-12 17:00:05 4 temp1.a   ~/projects
# 2025-05-12 19:35:07 5 temp3(R)  ~/projects
# 2025-05-12 19:35:07 6 temp2.b.f ~/projects
# 2025-05-12 19:35:07 7 temp4(R)  ~/projects
```

## Log

Show the logs of `put`, `restore`, and `config` operations.

`[range]` is optional. It means showing logs within a specific period of time. It can take the following forms:

1. normal ranges : `YYYYMM-YYYYMM`
2. left side only : `YYYYMM`
3. right side only : `*-YYYYMM`
4. Common Era only
5. Archiver will always consider the last 2 digits of the number as the month, and the rest as the year.

```bash
arv log [range]
arv lg  [range]
```

## config

Configure some properties of Archiver. You can set the following items:

1. `alias` : set an alias for a path
2. `auto-check-update`: enable or disable auto check for updates

### basic usage

```bash
arv config # show all configs
arv config alias # show alias config entries
arv config [other-item-name]
```

### alias

Aliases will shorten the paths shown in command `list` and `log`. However, Archiver will still keep the full version. This is just for display purposes.

```bash
# path `/home/user/temp/xxx` displays as `mytemp/xxx`
arv config alias.add mytemp=/home/user/temp

# remove alias config `mytemp=/home/user/temp`
arv config alias.remove mytemp=/home/user/temp
```

### auto-check-update

Will check for updates automatically when you run `put`, `restore` and `config` command. Default value is `on`. If you want to disable this feature, you can set it to `off`.

```bash
arv config auto-check-update.set off
```

## update

Check whether there is a newer version of Archiver. If there is, it will be downloaded and installed automatically.

_Internet connection required!_

```bash
arv update
```

## help

Show help information for all commands.

```bash
arv help
```
