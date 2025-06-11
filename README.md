# Archiver

Troubled with too many projects? Cannot focus on your current work?

**Archiver helps you now!**

Archiver is a command line tool that helps you manage files and directories you may not use recently but cannot be removed. You can use `put` to move items into Archiver, and `restore` to move them back. You can also use `list` to see what you have archived, and `vault` to group your archived items.

## Installation

Using installation script recommended （supports Linux/macOS）

<details>
  <summary>_Does not support Windows yet_ (click to show why)</summary>
    <p>The code itself is Windows compatible, but it is weird to use command line tools in Windows. And I do not intend to make a GUI for it.</p>
    <p>If you are  Windows users, you can use WSL to run Archiver (just like I did).</p>
</details>
<br>

```bash
curl -fsSL https://github.com/baendlorel/archiver/releases/download/scripts/archiver-installer.sh | sh
```

## Where do the files go?

Archived files are stored in `~/.archiver` where `~` means home directory.

`.archiver` structure is as follows:

```mermaid
graph TD
    A[.archiver/] --> B[logs/]
    A  --> C[core/]
    B  --> B2[2025.jsonl]
    C  --> C1[config.jsonc]
    C  --> C2[auto-incr.jsonc]
    C  --> C3[list.jsonl]
    C  --> C4[vaults.jsonl]
    A  --> D[vaults/]
    D  --> D1[1/]
    D  --> D2[2/]
    D1 --> DD1[1]
    D1 --> DD2[2]
    D2 --> DD3[3]
```

# Commands

You can use the following commands to manage your archived items.

## Put

Put a file/directory into Archiver.

Simply navigate to a directory, run this command, and you can move your target into `.archiver`.

- Archived target will be given a unique id. The id is auto incremental.

```bash
# also `arv p`
arv put <items>
arv put temp1.txt temp2.txt temp3
```

### option: --vault/-v

Put the archived items into a specific vault. This would fail if the vault does not exist.

```bash
arv put --vault <vault-name> <items>
arv put -v myvault temp1.txt temp2.txt
```

### option: --message/-m

You can specify your archiving reason by using this option.

```bash
arv put <item> --message xxx
```

## Restore

Restore archived files or directories to their original locations.

- Will fail if there is an object with the same name, or archived file is missing

- `<ids>` can be obtained from the `list` command

- Restored targets will be hidden when running `arv list`, unless you use `arv list --all`.

```bash
# also `arv r` or `arv rst`
arv restore <ids>
arv restore 4 5 6
```

## Move

Move archived items to another vault.

```bash
# also `arv m`, `arv mv` or `arv mov`
arv move <ids> --to <vault-name>
arv move 1 2 3 -t myvault
```

## Vault

Manage vaults, which group archived items.

### use

Change the current vault. The default vault is `@` with an internal vault id `0`.

```bash
# also `arv v` or `arv vlt`
arv vault use <vault-name>
```

### create

```bash
arv vault create <vault-name> --message <message>
arv vlt create myvault -m "This is my vault"
```

#### option: --remark/-r

Add a remark to the vault. This will be shown in the `arv vault list` command.

#### option: --activate/-a

Activate the vault. This means the created vault will be used as the current vault.

### list

```bash
# also `arv vlt ls`
arv vault list # Show only valid vaults
arv vault list --all # Show all vaults
```

### remove

Remove a vault. Archived items in this vault will be moved to the default vault `@`.

```bash
# also `arv vlt rm xxx`
arv vault remove <vault-name>
```

### recover

When you want to use the same name as a removed vault, use this.

```bash
arv vault recover <vault-name>
```

## List

Show the archived items.

- The list table will contain archived time, id, item name, original directory and message
- If the target is a directory, its name will be shown in blue

```bash
# also `arv l` or `arv ls`
arv list
```

### option: --all

Show all archived records, including restored ones.

```bash
# also `arv ls -a`
arv list --all
```

### option: --restored

Show restored records.

```bash
# also `arv ls -r`
arv list --restored
```

### option: --vault

Show records in a specific vault.

```bash
# also `arv ls -v myvault`
arv list --vault <vault-name>
```

## Log

Show the logs for `put`, `restore`, and `config` operations.

`[range]` is optional and specifies a time period for the logs to display.

Supported formats:

1. normal ranges : `YYYYMM-YYYYMM`
2. left side only : `YYYYMM`
3. Common Era only

```bash
# also `arv lg 20250101`
arv log [range]
```

### option: --id

Show logs for a specific id. Related list and vault records will also be displayed.

```bash
# also `arv lg -i 123`
arv lg --id <log-id>
```

## Config

Display or modify configurations.

### list

```bash
arv config list # show all configs
```

### alias

Aliases shorten the paths shown in `list` and `log` commands. Archiver will still stores the full path. Aliases are for display only.

```bash
arv config alias <alias>=<absolute-path>
arv config alias mytemp=/etc/aa
arv config alias --remove mytemp=/etc/aa
```

### update-check

Automatically checks for updates when you run non-display commands.

Default value is `on`. To disable this feature, set it to `off`.

```bash
arv config update-check off
```

### vault-item-sep

Changes the separator between the vault name and item name in the `list` and other commands. Default value is `:`. You can change it to any character you like.

```bash
arv config vault-item-sep <character>
```

## Update

Check for a newer version of Archiver. If available, it will be downloaded and installed automatically.

_Internet connection required!_

```bash
arv update
```

## Check

Check whether the core files of Archiver are logically valid. Errors will be shown with advice for fixing them.

By default, only entries that fail the check will be displayed.

Checks include:

1. Home dir, `.archiver` dir, current working dir, logs dir, core dir, vaults dir is valid
2. Config file is valid
3. Archived item all exist and matches the list file
4. Vaults exist and match the vaults list
5. Auto-increment file is correct
6. No duplicated ids

```bash
arv check
```

### option: --verbose/-v

Show the passed checks.

## Help

Show help information for all commands.

```bash
arv help
```
