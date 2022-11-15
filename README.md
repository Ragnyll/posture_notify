# Posture notify
A simple program for telling the user when to change their posture from a `sitting` -> `standing` -> `break` position

This works with systemd so it will work on linux. Not windows or mac though. So if you wanna do that... idk, fuck you i guess.

Very generally this program creates a cachefile that is reset on boot that is occasionally updated to send the user a notification when to change the position specified in the progression.

## Installation

### Linux:
```
./install.sh
```
