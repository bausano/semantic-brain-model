## Preparing data

Install `ffmpeg` with `sudo apt-get install ffmpeg`.

### Video

`ffmpeg -i input.avi -s 640x360 -r 4 output_%04d.png`

Where `4` represents the frame rate (how many images per second should be
captured) and `%04d` generates the suffix with length of 4 digits.

### Audio

Convert the audio of the video input into a `.wav` format with following
properties:

- channels: `mono`
- sample rate: `44100 Hz`
- bit depth: `16 bit`
- bit rate: `705 kbps`

From `stereo` to `mono` conversion:

`ffmpeg -i input.wav -c:v copy -ac 1 -q:a 2 output.wav`
