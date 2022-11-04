# Grabber
A Rust tool to manage multiple repositories in different platforms.

## What is it intended for?
This is intended for people who have to manage multiple repositories from different clients. This tool helps you to organize all the repositories you want to have cloned in your computer.

## What can I do with grabber?
With `grabber` you can do:
- Manage platform ssh keys.
- Add new clients.
- Add as many platforms per client.
- Add repositories per platforms. (WIP)
- List repositories of a client.
- Clone repositories and store a record in a toml file.
- Work with a DynamoDB table to share new repositories (WIP).

## Initialize
First you will start configuring the tool:
```shell 
grabber setup
```
### What this command does:
- Creates a new directory called `.grabber` at yout **HOME**.
- Creates inside this new directory two files:
  - `grabber-config.toml`: SSH config file.
  - `grabber-repositories.toml`: Repositories database file.
- Will ask you to introduce some values to configure the SSH config file:
  - An alias to identify this keys.
  - The private key and public key path.

#### grabber-config
```toml
[alias]
private_key = "/home/grabber/.ssh/azure"
public_key = "/home/grabber/.ssh/azure.pub"

[github]
private_key = "/home/grabber/.ssh/github"
public_key = "/home/grabber/.ssh/github.pub"

```

#### grabber-repositories
```toml
[client.alias]
repositories = ["git@github.com:IT-Noobie/grabber.git"]
[it-noobie.github]
repositories = ["git@github.com:IT-Noobie/grabber.git"]
```

## New client
To add a new client just type:
```shell 
grabber new --client <CLIENT>
grabber new -c <CLIENT>
```

## Add repository
This is still in WIP:
```shell 
grabber add -c <CLIENT>
```

## List (WIP)
### List platforms
```shell
grabber list
╭─────────────────────────╮
│ PLATFORMS SSH KEY ALIAS │
╞═════════════════════════╡
│ azure                   │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ github                  │
╰─────────────────────────╯
```
### List platform configuration
```shell 
grabber list -p azure
╭────────────────────────────────────────────────╮
│ AZURE                                          │
╞════════════════════════════════════════════════╡
│ private_key = "/home/it-noobie/.ssh/azure"     │
│ public_key = "/Users/it-noobie/.ssh/azure.pub" │
│                                                │
╰────────────────────────────────────────────────╯
```
### List client platforms
```shell
grabber list -c <CLIENT>
╭────────────────────╮
│  CLIENT PLATFORMS  │
╞════════════════════╡
│ azure              │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ github             │
╰────────────────────╯
```
### List repositories of a client in a given platform
```shell
grabber list -c it-noobie -p github
╭────────────────────────────────────────╮
│ IT-NOOBIE GITHUB REPOSITORIES          │
╞════════════════════════════════════════╡
│ "git@github.com:IT-Noobie/grabber.git" │
╰────────────────────────────────────────╯
```





