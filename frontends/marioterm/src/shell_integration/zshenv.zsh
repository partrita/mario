# Sourced by zsh because mario pointed ZDOTDIR here. Restores the user's
# ZDOTDIR, chains their real .zshenv, then loads the integration for
# interactive shells, so their own configuration always runs first.
if [[ -n "${RIO_ZSH_ZDOTDIR+x}" ]]; then
    builtin export ZDOTDIR="$RIO_ZSH_ZDOTDIR"
    builtin unset RIO_ZSH_ZDOTDIR
else
    builtin unset ZDOTDIR
fi

builtin typeset _mario_user_zshenv="${ZDOTDIR:-$HOME}/.zshenv"
if [[ -r "$_mario_user_zshenv" && ! -d "$_mario_user_zshenv" ]]; then
    builtin source -- "$_mario_user_zshenv"
fi
builtin unset _mario_user_zshenv

if [[ -o interactive ]]; then
    builtin typeset _mario_integration_file="${${(%):-%x}:A:h}/mario-integration.zsh"
    if [[ -r "$_mario_integration_file" ]]; then
        builtin source -- "$_mario_integration_file"
    fi
    builtin unset _mario_integration_file
fi
