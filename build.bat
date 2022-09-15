@echo off
set LITCRYPT_ENCRYPT_KEY=%random%%random%%random%%random%%random%%random%%random%%random%%random%%random%%random%%random%
cargo build -q --release
move %cd%\target\release\xanax-stub.exe build.exe
