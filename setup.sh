#!/bin/bash

echo "===================================="
echo "=========DANS SETUP SCRIPT=========="
echo "===================================="

echo "Enter a number corresponding to the term whose definition you'd like to view"
PS3="My selection is:"
os_name="$(uname -s)"
echo "OS: ($os_name)"

var=-1
if "$(uname -s)"=="Darwin"; then 
    echo "MacOS"
    var=1
else
    echo "Linux"
    var=2
fi

select TERM in Rust btm C-Tools neovim exa exit;
do
    case $TERM in
        rust-install)
            echo "Installing Rust"
            curl https://sh.rustup.rs -sSf | sh
            # Enabling sparse rust indexing to speed compiling, might have to switch to nightly
            echo "CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse" >> ~/.cargo/env
            rustup component add rust-analyzer
            ;;
        btm)
            echo "Downloading bottom"
            cargo install bottom --locked
            ;;
          
        C-Tools)
            echo "INSTALLING: C Tools"
            sudo apt-get install gcc, make
            ;;
        neovim)
            echo "INSTALLING: NeoVim"

            ;;
        exa)
            echo "INSTALLING: exa"
            cargo install exa 
            ;;
        exit)
            echo "You are now exiting this script."
                break
                ;;
          *)
            echo "Please make a selection from the provided options."
    esac
done

