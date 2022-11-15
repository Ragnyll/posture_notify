#!/bin/bash

# install the actual program
cargo install --path .

# create the systemd units
#
# Unit file to make sure i start from a clean position
printf "
[Unit]
Description=Cleans up a $HOME/.cache/posture file on startup

[Service]
Type=simple
ExecStart=-/bin/bash -c \'/usr/bin/rm -f $HOME/.cache/posture\'
KillMode=process

[Install]
WantedBy=multi-user.target
" > ./posture-clean.service


printf "
[Unit]
Description=runs the posture changer every 23 minutes
Wants=posture-notifier.timer

[Service]
ExecStart=$HOME/.cargo/bin/posture-notifier
KillMode=process

[Install]
WantedBy=multi-user.target
" > ./posture-notifier.service

printf "
[Unit]
Description=runs the posture changer every 23 minutes
Wants=posture-notifier.service

[Timer]
Unit=posture-notifier.service
OnUnitInactiveSec=23m
AccuracySec=1m

[Install]
WantedBy=timers.target
" > ./posture-notifier.timer


# setup the systemd unit for clean startup
cp ./posture_clean.service $HOME/.config/systemd/user/
systemctl --user enable posture_clean.service
systemctl --user start posture_clean.service

# setup the systemd unit for posture-notifier.timer
cp ./posture-notifier.timer $HOME/.config/systemd/user/
systemctl --user enable posture-notifier.timer
systemctl --user start posture-notifier.timer

# setup the systemd unit for posture-notifier.service
cp ./posture-notifier.service $HOME/.config/systemd/user/
systemctl --user enable posture-notifier.service
systemctl --user start posture-notifier.service
