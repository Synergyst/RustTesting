@echo off
@cd /d "%~dp0"
SET fxproc=soundboard.exe
SET retrycounter=0
mode con:cols=152 lines=48

:loop
%fxproc% --voice 12 --name "VoiceMeeter Input (VB-Audio VoiceMeeter VAIO)"
timeout /t 1 /nobreak > NUL
GOTO loop
:endscript
mode con:cols=152 lines=48
cls
echo Exiting now (or exiting due to repeated crashes, ehh, you will know why it is closing)
pause