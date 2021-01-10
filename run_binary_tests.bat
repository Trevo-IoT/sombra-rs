set SOMBRA_WINDOWS_SERVICE_PATH=%CD%\executables\sombra-windows-service.exe
:: echo %SOMBRA_WINDOWS_SERVICE_PATH%
cargo build --release
cargo test --test binary_test -- --test-threads 1
