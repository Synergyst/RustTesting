~~~~~~~~~~~~~~~~~~~~~~~
~ QUICK START SECTION ~
~~~~~~~~~~~~~~~~~~~~~~~

1. Install VoiceMeeter Banana (do not reboot when asked). Please see `credits.txt`

2. Install VB-Audio Cable (do not reboot when asked). Please see `credits.txt`

3. Follow `voicemeeter-banana-setup.png` for setup instructions

4. Reboot the computer so VoiceMeeter Banana and VB-Audio are fully setup

5. See `CUSTOM SOUNDS/LIBRARY SECTION` for layout/sound library instructions

6. Once you have some sounds for the soundboard (and have possibly changed some settings in `soundboard.bat`), double-click `soundboard.bat` and enjoy!

~~~~~~~~~~~~~~~~~~~~~~~~~~~~
~ DEFAULT KEY BIND SECTION ~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

NOTE: These keybinds are configurable, see `EXTRA INFO SECTION`

Left-Alt: transmit key

Numpad 4: cycle library backward

Numpad 5: cycle library forward

Numpad 1: cycle sound backward

Numpad 0: cycle sound forward

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
~ CUSTOM SOUNDS/LIBRARY SECTION ~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

By default the soundboard does not have any MP3's included.
The soundboard will need a file/folder structure like so before you can start it/fully utilize it:
`soundboard.exe`
`sounds\`
`sounds\downloaded-unsorted\`
`sounds\librarynamehere\`
`sounds\anotherlibrarynamehere\`
`sounds\librarynamehere\soundfile.mp3`
`sounds\librarynamehere\anothersoundfile.mp3`
`sounds\anotherlibrarynamehere\soundfile.mp3`
`sounds\anotherlibrarynamehere\anothersoundfile.mp3`
`...`

For YouTubeDLP support you can rename `SoundboardREADME\` to `YouTubeDLP` and place it in the same directory with `soundboard.exe` (optionally move this `README.md` file to the same directory with `soundboard.exe` for ease of access)

YouTubeDLP and FFMPEG is included as an aid to allow you to download and convert using a URL automatically to your soundboard directory.
Please see `credits.txt`

1. Just double-click `downloader-and-converter.bat` and right-click to paste your URL.

2. Next type the name you want for the downloaded file which will go into the `sounds\downloaded-unsorted\` directory

3. Modify the downloaded file in Audacity and export it as a 48000Hz stereo MP3 file once finished

4. Move the edited file to the preffered library folder, or create your own folder and place it there! :)

~~~~~~~~~~~~~~~~~~~~~~
~ EXTRA INFO SECTION ~
~~~~~~~~~~~~~~~~~~~~~~

The soundboard will automatically find the VoiceMeeter Banana device and use that, if it is not found then it will use your default output device.. 
Though that would be a sign that you either renamed/disabled the VoiceMeeter Banana device or the installation for VoiceMeeter Banana had failed.

If you want more information please run `soundboard.exe --help` to get more information so you can modify `soundboard.bat` with any changes you wish