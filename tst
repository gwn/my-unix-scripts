#!/bin/sh

# tabbed terminal

cd "$HOME/$1"
EDITOR=vimshorttitle tabbed -r 2 -f -p -1 st -w x
