Ffcutter is used to cut mp4 files into 30 second chunks. 

Right after the chunk creation, it converts it to mp3, so as to be
given as an input audio to audiocraft. Then it deletes the intermediate file.

With every chunk it creates, it increases the counter of the chunked mp3 filename.

usage:

```bash

ffcutter filename start-time

```

start-time counts seconds from the start of the audio. e.g. 70 secs, 150 etc.

dependencies: ffmpeg

executable: ffcutter

build from source: 

```bash

rustc main.rs
mv main ffcutter

```
