@echo off
@cd /d "%~dp0"
SET fxproc=rust-test.exe
SET retrycounter=0
mode con:cols=152 lines=48

:loop
%fxproc% 12 0
timeout /t 1 /nobreak > NUL
GOTO loop
:endscript
mode con:cols=152 lines=48
cls
echo Exiting now (or exiting due to repeated crashes, ehh, you will know why it is closing)
pause