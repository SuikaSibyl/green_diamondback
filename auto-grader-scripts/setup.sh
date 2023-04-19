#!/usr/bin/env bash

apt-get update
apt-get install -y build-essential python3 curl nasm
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
