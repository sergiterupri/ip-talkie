# VoIP Walkie-Talkie Application

A simple peer-to-peer voice chat application that works on Linux, macOS, and Windows. This application uses audio devices to capture and play audio over a network connection.

## Table of Contents

- [Installation](#installation)
  - [Linux](#linux)
  - [macOS](#macos)
  - [Windows](#windows)
- [Usage](#usage)
- [Troubleshooting](#troubleshooting)
- [License](#license)

## Installation

### Linux

1. **Install Rust and Cargo**:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    ```

2. **Install ALSA Development Libraries**:
    ```bash
    sudo apt-get update
    sudo apt-get install libasound2-dev
    ```

3. **Clone the Repository and Build**:
    ```bash
    git clone https://github.com/yourusername/voip_walkie_talkie.git
    cd voip_walkie_talkie
    cargo build --release
    ```

### macOS

1. **Install Rust and Cargo**:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    ```

2. **Install PortAudio**:
    ```bash
    brew install portaudio
    ```

3. **Clone the Repository and Build**:
    ```bash
    git clone https://github.com/yourusername/voip_walkie_talkie.git
    cd voip_walkie_talkie
    cargo build --release
    ```

### Windows

1. **Install Rust and Cargo**:
    - Download and install Rust from [rustup.rs](https://rustup.rs/).

2. **Install Windows Build Tools**:
    ```bash
    npm install -g windows-build-tools
    ```

3. **Clone the Repository and Build**:
    ```bash
    git clone https://github.com/yourusername/voip_walkie_talkie.git
    cd voip_walkie_talkie
    cargo build --release
    ```

## Usage

1. **Run the Application**:
    ```bash
    ./target/release/voip_walkie_talkie --host <FRIEND_IP> --port <FRIEND_PORT>
    ```

    Replace `<FRIEND_IP>` and `<FRIEND_PORT>` with the IP address and port number of the friend you want to connect to.

2. **Voice Chat**:
    - Speak into your microphone to send audio.
    - Listen through your speakers or headphones to hear audio from your friend.

3. **Graceful Shutdown**:
    - Press `Ctrl+C` to stop the application and exit gracefully.

## Troubleshooting

### Common Issues

1. **Audio Device Not Found**:
    - Ensure that your audio devices are properly connected and recognized by the system.
    - Check the device settings and permissions.

2. **Connection Issues**:
    - Verify that the IP address and port are correct and accessible.
    - Ensure that firewall or network settings are not blocking the connection.

3. **Missing ALSA on Linux**:
    - Ensure `libasound2-dev` is installed. You may need to set `PKG_CONFIG_PATH` if the ALSA library is not found.
    ```bash
    export PKG_CONFIG_PATH=/usr/lib/pkgconfig:/usr/local/lib/pkgconfig
    ```

4. **PortAudio on macOS**:
    - Ensure `portaudio` is installed via Homebrew.
    ```bash
    brew install portaudio
    ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
