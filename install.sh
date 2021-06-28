#!/bin/sh

if [[ ! -e $XDG_CONFIG_HOME/polybar/coins.json ]]; then
  ln -sf $DOTFILES/polybar/crypto/coins.json $XDG_CONFIG_HOME/polybar/coins.json
fi

cargo build --release
