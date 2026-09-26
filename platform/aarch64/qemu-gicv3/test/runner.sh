#!/bin/bash

set -e

PWD=$(pwd)
THIS=$(basename $0)
CARGO_BUILD_INPUT_ARG0=$1

ARCH=${ARCH}
BOARD=${BOARD}

HVISOR_ELF=$CARGO_BUILD_INPUT_ARG0
HVISOR_BIN=$HVISOR_ELF.bin

OBJCOPY=rust-objcopy

AARCH64_GIC_TEST_VERSION=3

YELLOW='\033[1;33m'
END='\033[0m'

info() {
    echo "[INFO | $THIS] $1"
}

info "Running cargo test with env: ARCH=$ARCH, BOARD=$BOARD"

info "Building hvisor with $CARGO_BUILD_INPUT_ARG0"
info "PWD=$PWD, running cargo test"
$OBJCOPY $HVISOR_ELF --strip-all -O binary $HVISOR_BIN

info "Running QEMU with $HVISOR_BIN"
info "Using GIC version: $AARCH64_GIC_TEST_VERSION"

# hvisor is booted directly as QEMU's -kernel: no U-Boot/TF-A blob, no flash.img.
# With secure=on QEMU puts its EL3 stub at 0x40000000 and loads -kernel at
# 0x40400000 (= linker BASE_ADDRESS), passing the DTB in x0.
qemu-system-aarch64 \
    -machine virt,secure=on,gic-version=${AARCH64_GIC_TEST_VERSION},virtualization=on,iommu=smmuv3 \
    -cpu cortex-a57 -smp 4 -m 3G -nographic \
    -semihosting \
    -kernel $HVISOR_BIN \
    -global arm-smmuv3.stage=2

exit 0
