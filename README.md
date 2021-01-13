# sombra-rs

<p align="center">
  A library to create and manage background services across multiple platforms
</p>

<p align="center">
  <a href="#overview">Overview</a> •
  <a href="#how-to-use">How to Use</a> •
  <a href="#platforms">Platforms</a> •
  <a href="#project-goals">Project Goals</a><br>
  [<a href="README-PTBR.md">PT-BR</a>]
<p>

# Overview

<img src="architecture.png" alt="Sombra Architecture"><br>

# How to Use

Create a background service
```bash
# windows
sombra.exe create tcp_echo executables/tcp_echo.exe
# linux
sombra create tcp_echo executables/tcp_echo
```

Create a background service, with parameters
```bash
# windows
sombra.exe create tcp_echo2 executables/tcp_echo.exe -p 30200
#linux
sombra create tcp_echo2 executables/tcp_echo -p 30200
```

Delete a background service (created with Sombra)
```bash
# windows
sombra.exe delete tcp_echo
#linux
sombra delete tcp_echo
```

Execute a python script as a background service (In this version, the python interpreter and the python file must have the absolute path)
```bash
# windows
sombra.exe create python_service 'C:\Program Files\Python37\python.exe' C:\Users\<username>\Documents\tcp_echo.py
# linux
sombra create /usr/bin/python3 /home/<username>/tcp_echo.py
```

# Platforms
- ✅ Windows 10
- ✅ Linux
- ❌ MacOS

## Windows 10 Settings
A special binary (`sombra-windows-service.exe`) is required to run `sombra.exe` on windows platform. 
The binary `sombra-windows-service.exe` wrap target process in a windows service.
This repository contains the special binary in the directory `executables`.

Before execute `sombra.exe`, set environment variable `SOMBRA_WINDOWS_SERVICE_PATH` to the path of `sombra-windows-service.exe`.
Another requirement is execute `sombra.exe` in an administrator terminal.

## Project Goals
- One Interface, many platforms
- Library (Rust) and Binary
- no-panic
- Minimum interference on native service managers
