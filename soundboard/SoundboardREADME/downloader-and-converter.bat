@echo off
@cd /d "%~dp0"
set /p url="Enter URL to attempt to download: "
set /p name="Enter name (try to use lower-case and '-' symbols only for names. WILL OVERWRITE FILES WITH SAME NAME): "
echo "Will be downloaded then converted to destination: [sounds\%name%.mp3]"
REM youtube-dl.exe -U
REM youtube-dl.exe --quiet --no-playlist --output - %url% | ffmpeg.exe -y -loglevel panic -hide_banner -i - -ar 48000 -ac 2 -c:a mp3 ..\sounds\downloaded-unsorted\%name%.mp3 || pause
yt-dlp.exe -U
yt-dlp.exe --extractor-args youtube:player-skip=js --quiet --no-playlist --output - %url% | ffmpeg.exe -y -loglevel panic -hide_banner -i - -ar 48000 -ac 2 -c:a mp3 ..\sounds\downloaded-unsorted\%name%.mp3 || timeout 15 && pause