# corautils

## .ssh/config

Set ssh to not close connection when you don't interact with the terminal for too long.

Put it in the local machine, not the server.

## coreutils

- **mv**: A change in the `mv` command that makes it create the destination directory if it does not exist yet. The flag is `-c`, with no arguments.

## .gdbinit

Append this to your ~/.gdbinit. It makes debuggin in gdb more similar to the real environment.

## git

My .gitconfig file

## scripts

### ssh-log

An ssh wrapper that logs the hour and IP when you connect to another machine via ssh.

Useful to compare with the logs in the remote machine for suspicious connections.

### whatismyip

Return your public ip address.

### gcc-bof

A script to compile buffer overflow test programs easily.
