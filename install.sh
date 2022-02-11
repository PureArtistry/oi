#!/bin/bash

case "$OSTYPE" in
    "linux-gnu" | "linux-musl") linux=true ;;
    *) linux=false ;;
esac
if ! $linux; then
    echo "This script is designed for linux only, sorry!"
    exit 1
fi

set -e

RED='\x1b[1;31m'
GREEN='\x1b[1;32m'
YELLOW='\x1b[1;33m'
BOLD='\x1b[1m'
RESET='\x1b[0m'

printf '\n%b' '\x1b[36m' && cat << 'EOF'
      ▪  ▄▄
▪     ██ ██▌
 ▄█▀▄ ▐█·▐█·
▐█▌.▐▌▐█▌.▀
 ▀█▄▀▪▀▀▀ ▀

EOF
printf '%b' $RESET

if [ $EUID = 0 ]; then
    printf "%bwarning:%b please don't run random scripts you find on the internet as root!\n" $YELLOW $RESET
    printf '%bsudo or doas will be used when elevated privileges are required%b\n' $BOLD $RESET
    exit 1
fi

if !command -v cargo >/dev/null 2>&1; then
    printf '%berror:%b can not find %bcargo%b in your $PATH, please ensure it is correctly installed\n' $RED $RESET $BOLD $RESET
    exit 1
fi

if command -v sudo >/dev/null 2>&1; then
    PRIV_ESC='sudo'
elif command -v doas >/dev/null 2>&1; then
    PRIV_ESC='doas'
else
    printf '%berror:%b can not find %bsudo%b or %bdoas%b in your $PATH, one of these is required\n' $RED $RESET $BOLD $RESET $BOLD $RESET
    exit 1
fi

cd "$(dirname "$0")"
printf '%bSTEP 1:%b %bbuilding the binary%b (this may take a few minutes)\n\n' $GREEN $RESET $BOLD $RESET
cargo build --release
command -v strip >/dev/null 2>&1 && strip -s ./target/release/oi

printf '\n%bSTEP 2:%b %bcopying files%b (elevated privileges are required)\n\n' $GREEN $RESET $BOLD $RESET
$PRIV_ESC install -Dvm755 ./target/release/oi /usr/local/bin/oi
$PRIV_ESC install -Dvm644 ./etc/completions/_oi /usr/share/zsh/site-functions/_oi
$PRIV_ESC install -Dvm644 ./etc/completions/oi.bash /usr/share/bash-completion/completions/oi
$PRIV_ESC install -Dvm644 ./etc/completions/oi.fish /usr/share/fish/vendor_completions.d/oi.fish

printf '\n%bDONE:%b %bthanks for testing! %b<3%b (this repo is no longer needed and can be deleted)\n' $GREEN $RESET $BOLD $RED $RESET
