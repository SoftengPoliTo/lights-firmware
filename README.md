# Lights-firmware

A `ESP32-C3` firmware written in Rust for the
[Ascot Project](https://ascot.iit.cnr.it/) to turn lights on and off.

# Setup

Before installing the required tools, you should decide where they should be
placed in your system. For convenience, `~/.cargo/bin` directory could be a good
alternative since it is reachable from any path in your system.

1. Install [Rust](https://www.rust-lang.org/tools/install)

2. Install [RISC-V Target](https://esp-rs.github.io/book/installation/riscv.html).

3. Install [std requirements](https://esp-rs.github.io/book/installation/std-requirements.html) to use the `esp-idf-hal` containing simpler APIs to interact with the `std` library

4. Install [espflash](https://esp-rs.github.io/book/tooling/espflash.html#espflash-1) to flash the
firmware and view some information about `ESP32-C3`.

# Build & flash the firmware

1. Enable `espflash` permissions following the instructions explained [here](https://github.com/esp-rs/espflash/blob/main/espflash/README.md#permissions-on-linux)

2. After connecting your board to your computer, run `cargo run`

# View your board information

After connecting your board to your computer, run `espflash board-info` to
view some information about `ESP32-C3`.

# Simulation

1. Install [wokwi-cli](https://docs.wokwi.com/wokwi-ci/getting-started#cli-installation)
2. Create a `WOKWI_CLI_TOKEN` following these [instructions](https://docs.wokwi.com/wokwi-ci/getting-started#cli-usage)
3. Replace `WOKWI_CLI_TOKEN` in `wokwi.sh.example`
4. Rename `wokwi.sh.example` as `wokwi.sh`
5. Run `wokwi.sh`

## Online simulation

Sign in on [Wokwi](https://wokwi.com/) using your `Google` or `GitHub` account
and create an online simulation of your circuit and the associated code. Then
`Save` the project publicly.
