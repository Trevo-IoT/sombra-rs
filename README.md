# sombra-rs
A library to create and manage background services across multiple platforms

## Goals
- One Interface, many platforms
- Library (Rust) and Binary
- no-panic
- Minimum interference on native service managers

## Platforms
- ✅ Windows 10
- ✅ Linux
- ❌ MacOS

## Windows 10 Settings
A special binary (`sombra-windows-service.exe`) is required to run `sombra.exe` on windows platform. 
The binary `sombra-windows-service.exe` wrap target process in a windows service.
This repository contains the special binary in the directory `executables`.

Before execute `sombra.exe`, set environment variable `SOMBRA_WINDOWS_SERVICE_PATH` to the path of `sombra-windows-service.exe`.
Another requirement is execute `sombra.exe` in an administrator terminal.
