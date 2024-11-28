rm -rf out
mkdir out
ffmpeg -i bad-apple.mp4 -vf fps=24 out/%d.png
