#!/usr/bin/env bash
# 在开发容器里执行命令。
#   ./tools/dev.sh bash
#   ./tools/dev.sh make BID=aarch64/qemu-gicv3 run
#
# 镜像由本地构建：docker build -t hvisor-dev -f docker/Dockerfile .
# 挂载根默认为本仓库的父目录（hvisor-book 里约定的 $WS），可用 HVISOR_WS 覆盖。
set -euo pipefail

IMAGE=${HVISOR_DEV_IMAGE:-hvisor-dev}
TOP=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
REPO=$(basename "$TOP")
WS=${HVISOR_WS:-$(dirname "$TOP")}

args=(-it --rm -v "$WS:/ws" -w "/ws/$REPO"
      -v hvisor-cargo:/root/.cargo
      -v "hvisor-target:/ws/$REPO/target"
      -v "hvisor-kconfig-venv:/ws/$REPO/tools/kconfig/.venv")

# x86 上跑 x86 需要 KVM；其它架构走 TCG 模拟
[ -e /dev/kvm ] && args+=(--device /dev/kvm)

# git worktree 的 .git 是指向主仓的绝对路径，必须一起挂载，否则容器内 git 不可用
if gitdir=$(git rev-parse --git-common-dir 2>/dev/null); then
    gitdir=$(cd "$gitdir" && pwd)
    args+=(-v "$gitdir:$gitdir")
fi

exec docker run "${args[@]}" "$IMAGE" "$@"
