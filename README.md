# Archiver

Put your unused files and folders into `/home/user/.archive`. Supported for logging, listing and restoring

## Commands

Archived files are stored in `/home/user/.archive`. You can use the following commands to manage your archived files.
Archived files/directories are meant to be readonly.

### list

List all entries in the archive. If `[name]` is provided, it will list the file/directory named `[name]`.

```bash
arv list [name] # e.g. arv list 'Program Files'
arv ls [name]

# This should show a table like this
# id | name  | isDir | archivedAt           | dir
# 1  | a.txt | 0     | 2023-10-01 00:00:00  | /home/user/file1
```

### log

Show the operation log.

`[time-interval]` is the time period to show the logs. It can be in the format of below. (both sides are included)

1. normal intervals like `YYYY-MM YYYY-MM`
2. left side only like `YYYY-MM` or `YYYY-MM *`
3. right side only like `* YYYY-MM`

```bash
arv log [time-interval]
arv lg [time-interval]
```

### archive

Put a file/directory into the archive. The archived files will be moved to `/home/user/.archive`.

```bash
arv archive <name> # e.g. arv archive my.txt
arv a <name>
```

### restore

Move file/directory to the original directory.

If same name exists, the operation will fail and inform you this problem.

```bash
arv restore <name|uid>
```

### help

Show help information.

```bash
arv help
```
