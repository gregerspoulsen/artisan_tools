#!/bin/bash

# Script for prepare system to run CI workflows

set -e

# Install Task
sudo snap install task --classic

# Start SSH Agent (only needed because docker compose forwards it)
eval $(ssh-agent -s)