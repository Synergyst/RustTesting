@echo off
@cd /d "%~dp0"
echo If you see this message and the command prompt didn't close within a reasonable amount of time something probably went wrong with the conversion..
for /R "unconverted" %%f in (*.*) do (
  echo ffmpeg.exe -hide_banner -i %%f -ar 48000 -ac 2 -c:a pcm_s16le ..\sounds\%%~nf.mp3
  ffmpeg.exe -hide_banner -y -i %%f -ar 48000 -ac 2 -c:a mp3 ..\sounds\downloaded-unsorted\%%~nf.mp3 || pause
)