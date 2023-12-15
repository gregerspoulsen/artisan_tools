#!/bin/bash

# Script for prepare system to run CI workflows

set -e

# Install Task
sudo snap install task --classic

# Install artisan-tools
pip install git+git://github.com/gregerspoulsen/artisan-tools.git@version_check