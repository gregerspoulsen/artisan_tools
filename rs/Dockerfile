# Basic Dockerfile to illustrate how to setup nix and direnv on Ubuntu.

FROM ubuntu

# Install direnv
RUN apt update && apt install -y \
    direnv \
    curl
RUN echo eval "$(direnv hook bash)" >> /root/.bashrc


# Install Nix (the install arguments can be left out when installing native)
RUN curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install linux --no-confirm --init none


RUN mkdir -p /workdir
WORKDIR /workdir

# Copy nix files for testing:
COPY flake.nix flake.lock .envrc ./

# Enable direnv in the folder
RUN direnv allow