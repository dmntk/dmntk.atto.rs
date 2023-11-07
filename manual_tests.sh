#!/usr/bin/env bash

# =================================================================================================
# start editor with decision table
# =================================================================================================

# start new session
tmux new-session -d -s Atto './target/debug/atto ./examples/e1.dtb'

# wait until the editor starts
sleep 0.2

# send unhandled key combinations to make debug message to be displayed
tmux send-keys -t Atto C-M-End

# move to cell end
tmux send-keys -t Atto End

# move to cell start
tmux send-keys -t Atto Home

# move down
tmux send-keys -t Atto Down

# move down
tmux send-keys -t Atto Down

# move up
tmux send-keys -t Atto Up

# move to table end
tmux send-keys -t Atto S-End

# move to table start
tmux send-keys -t Atto S-Home

# move to next cell
tmux send-keys -t Atto Tab

# move to previous cell
tmux send-keys -t Atto BTab

# move right
tmux send-keys -t Atto Right

# move left
tmux send-keys -t Atto Left

# delete character before cursor
tmux send-keys -t Atto BSpace

# insert two characters
tmux send-keys -t Atto "UK"

# delete character under cursor
tmux send-keys -t Atto Delete

# display content
# tmux capture-pane -t Atto -p -S

# Ctrl+q : close editor; session will be automatically closed
tmux send-keys -t Atto C-q

# =================================================================================================
# start editor without any arguments
# =================================================================================================

tmux new-session -d -s AttoNoArgs './target/debug/atto'

sleep 0.2

# =================================================================================================
# start editor with non-existing file
# =================================================================================================

tmux new-session -d -s AttoNonExistingFile './target/debug/atto ./examples/nop.dtb'

sleep 0.2