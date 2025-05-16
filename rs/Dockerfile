FROM ubuntu

RUN apt update && apt install -y \
    direnv \
    curl

RUN curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install linux --no-confirm --init none

RUN echo eval "$(direnv hook bash)" >> /root/.bashrc

RUN mkdir -p /workdir
WORKDIR /workdir

COPY flake.nix flake.lock .envrc ./

RUN direnv allow