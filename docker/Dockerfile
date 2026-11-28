# hvisor 开发环境（本地构建，仓库不发布预构建镜像）
#
# 国际网络：
#   docker build -t hvisor-dev -f docker/Dockerfile .
# 国内网络（已验证 docker.1ms.run 可用）：
#   docker build -t hvisor-dev -f docker/Dockerfile \
#     --build-arg BASE_IMAGE=docker.1ms.run/library/ubuntu:26.04 \
#     --build-arg RUSTUP_INIT_URL=https://rsproxy.cn/rustup-init.sh \
#     --build-arg RUSTUP_DIST_SERVER=https://rsproxy.cn \
#     --build-arg RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup \
#     .

ARG BASE_IMAGE=ubuntu:26.04
FROM ${BASE_IMAGE}

ENV DEBIAN_FRONTEND=noninteractive

# QEMU 用 26.04 的 qemu-hwe（当前 10.2.x，满足 RISC-V rva23s64 对 >= 10.1 的要求）；
# 三套交叉工具链覆盖 aarch64 / riscv64 / x86_64 的 hvisor-tool 构建。
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates curl wget xz-utils git \
        build-essential flex bison bc libssl-dev libelf-dev \
        python3 python3-venv \
        device-tree-compiler u-boot-tools \
        qemu-system-hwe qemu-utils-hwe \
        gcc-aarch64-linux-gnu gcc-riscv64-linux-gnu gcc-x86-64-linux-gnu \
        e2fsprogs cpio gzip busybox-static tio \
        mmdebstrap debian-archive-keyring \
    && rm -rf /var/lib/apt/lists/*

# Rust 工具链的版本由仓库根目录的 rust-toolchain.toml 决定，这里只准备 rustup。
# 三个 URL 都可以用国内镜像覆盖，见文件头的示例。
ARG RUSTUP_INIT_URL=https://sh.rustup.rs
ARG RUSTUP_DIST_SERVER=https://static.rust-lang.org
ARG RUSTUP_UPDATE_ROOT=https://static.rust-lang.org/rustup
ENV RUSTUP_DIST_SERVER=${RUSTUP_DIST_SERVER}
ENV RUSTUP_UPDATE_ROOT=${RUSTUP_UPDATE_ROOT}

RUN curl --proto '=https' --tlsv1.2 -sSf "${RUSTUP_INIT_URL}" \
        | sh -s -- -y --default-toolchain none
ENV PATH="/root/.cargo/bin:${PATH}"

# llvm-objcopy / llvm-nm 来自 llvm-tools-preview
RUN rustup toolchain install nightly-2026-11-01 --profile minimal \
        --component rust-src,llvm-tools-preview,rustfmt,clippy,rust-analyzer

WORKDIR /ws
CMD ["bash"]