#!/bin/bash

# Script for prepare system to run CI workflows

set -e

# Install just
sudo snap install just --classic --edge

# Install artisan-tools
#pip install git+https://github.com/gregerspoulsen/artisan_tools.git@version_check
