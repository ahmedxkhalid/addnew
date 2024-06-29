#!/bin/bash

# Add.cargo/bin to PATH
export PATH=$PATH:~/.cargo/bin

# Make the change permanent by adding it to the ~/.profile file
echo "export PATH=$PATH:~/.cargo/bin" >> ~/.profile

# Reload the ~/.profile file to apply the changes
source ~/.profile