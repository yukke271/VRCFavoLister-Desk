FROM rust:slim-bookworm
ENV DISPLAY=:1
ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_TARGET_DIR=/app/target
ENV WATCHPACK_POLLING=true

# Refresh package list
RUN echo "deb http://ftp.jp.debian.org/debian/ bookworm main contrib non-free non-free-firmwaren" >> /etc/apt/sources.list
RUN rm -rf /var/lib/apt/lists/*
RUN apt-get update

# Tauri dependencies
# See https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux
RUN apt-get install -y \
    libwebkit2gtk-4.0-dev \
    curl \
    wget \
    file \
    build-essential \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \ 
    librsvg2-dev \ 
    pkg-config

# Install Node.js
# See https://github.com/nodesource/distributions
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - &&\
    apt-get install -y nodejs

# Install Yarn
# See https://classic.yarnpkg.com/en/docs/install/#debian-stable
RUN curl -fsSL https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - &&\
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list &&\
    apt-get update && apt-get install -y yarn

# Install VNC server
RUN apt-get install -y --no-install-recommends \
    lxde \
    tigervnc-standalone-server \
    tigervnc-common \
    tigervnc-tools \
    novnc \
    websockify

# Install Git
# RUN apt-get install -y git

# Refresh package cache
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Rust tools
RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit tauri-cli cargo-watch

# Set up VNC server
RUN echo "password" | vncpasswd -f > /.vncpasswd && \
    chmod 0600 /.vncpasswd

WORKDIR /workspace