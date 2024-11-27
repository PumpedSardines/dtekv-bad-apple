rm -rf out
mkdir out
ffmpeg -i bad-apple.mp4 -vf fps=12 out/%d.png
