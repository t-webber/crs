#!/bin/sh

##############
### Checks ###
##############

set -eo pipefail

error() {
        printf "\x1b[31m%s\x1b[m\n" "$1"
        exit 6
}

log() {
        printf "\x1b[33m%s...\x1b[m\n" "$1"
}

[ ! -f Makefile ] && error "Script must be run script in the server folder"

for prog in uv tmux make cargo; do
        which "$prog" 2>/dev/null >&2 || error "$prog not found, please install it and try again."
done

###############
### Install ###
###############

log "Installing dependencies"

if [ ! -d .venv ]; then
        uv venv
fi

venv="builtin cd $PWD && . .venv/bin/activate"
. .venv/bin/activate
make install

##############
### Config ###
##############

log "Loading configuration"

[ -f ../.env ] && . ../.env

res=""
load() {
        [ -n "$1" ] && {
                res=$1
                return
        }
        [ -n "$4" ] && x=" (don't use real credentials, only meant for local use)"
        printf "\x1b[33m$2$x (default: $3): \x1b[m" >&2
        read var
        [ -n "$var" ] && {
                res=$var
                return
        }
        res=$3
}

load "$CRS_TMUX_SESSION" "Tmux session name" crs
session="$res"
load "$CRS_SERVER_URL" "Server url" "http://localhost:8008"
url="$res"
load "$CRS_USER" "Matrix user name" user y
user="$res"
load "$CRS_PASSWORD" "Matrix password" password y
password="$res"

echo "CRS_SERVER_URL=$url
CRS_TMUX_SESSION=$session
CRS_USER=$user
CRS_PASSWORD=$password
" >../.env

############
### Tmux ###
############

log "Setting up tmux"

make=$(which make)
cargo=$(which cargo)

tmux has-session -t "$session" 2>/dev/null >&2 && error "Tmux session '$session' already exists"

tmux new-session -d -s "$session" -n server
tmux send-keys -t "$session":server "$venv && $make start" C-m

tmux new-window -t "$session" -n whatsapp
tmux send-keys -t "$session":whatsapp "$venv && $make mautrix-whatsapp-run" C-m

sleep 2

register_new_matrix_user -u $user -p $password -c homeserver.yaml -a || {
        log "Will keep old user, password may have changed"
        sleep 4
}

tmux new-window -t "$session" -n tui
tmux send-keys -t "$session":tui "builtin cd $PWD/.. && $cargo run" C-m

tmux attach-session -t "$session"
