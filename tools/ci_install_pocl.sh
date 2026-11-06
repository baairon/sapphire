#!/usr/bin/env bash
# Installs the PoCL OpenCL runtime (CPU device) for the device CI steps.
#
# Each attempt runs apt-get update and apt-get install under one time limit. A slow
# mirror can trickle bytes without going idle, so apt's own timeouts never fire; the
# limit here is what ends a stalled attempt. Before attempt 2 apt is moved from the
# runner's default mirror to archive.ubuntu.com and the package index is fetched again.
#
# Environment (all optional):
#   POCL_APT_ARCHIVES        directory used as apt's package archive (the CI cache directory)
#   POCL_ATTEMPTS            number of attempts (default 3)
#   POCL_ATTEMPT_TIMEOUT     seconds allowed per attempt (default 240)
#   POCL_SUDO                command prefix for privileged calls (default sudo; set empty to run without)
#   POCL_APT                 apt binary (default apt-get)
#   POCL_DPKG                dpkg binary (default dpkg)
#   POCL_APT_MIRRORS         mirror list read by the runner image (default /etc/apt/apt-mirrors.txt)
#   POCL_OLD_MIRROR          host to replace (default azure.archive.ubuntu.com)
#   POCL_NEW_MIRROR          replacement host (default archive.ubuntu.com)
set -u

PACKAGES="pocl-opencl-icd ocl-icd-libopencl1 clinfo"
ATTEMPTS="${POCL_ATTEMPTS:-3}"
LIMIT="${POCL_ATTEMPT_TIMEOUT:-240}"
SUDO="${POCL_SUDO-sudo}"
APT="${POCL_APT:-apt-get}"
DPKG="${POCL_DPKG:-dpkg}"
MIRRORS="${POCL_APT_MIRRORS:-/etc/apt/apt-mirrors.txt}"
OLD="${POCL_OLD_MIRROR:-azure.archive.ubuntu.com}"
NEW="${POCL_NEW_MIRROR:-archive.ubuntu.com}"
ARCHIVES="${POCL_APT_ARCHIVES:-}"

APT_OPTS=(-o Acquire::Retries=3 -o Acquire::http::Timeout=30 -o Acquire::https::Timeout=30)
INSTALL_OPTS=("${APT_OPTS[@]}")
if [ -n "$ARCHIVES" ]; then
  mkdir -p "$ARCHIVES/partial"
  # apt drops privileges to its sandbox user for downloads; root keeps the cached files writable.
  INSTALL_OPTS+=(-o "Dir::Cache::Archives=$ARCHIVES" -o APT::Sandbox::User=root)
fi

one_attempt() {
  # An attempt cut off by the time limit during the dpkg phase leaves dpkg interrupted; finish its
  # configuration first, or the next apt-get call refuses to run.
  $SUDO "$DPKG" --configure -a || true
  # shellcheck disable=SC2086
  $SUDO "$APT" "${APT_OPTS[@]}" update || return 1
  # shellcheck disable=SC2086
  $SUDO "$APT" "${INSTALL_OPTS[@]}" install -y --no-install-recommends $PACKAGES
}

if [ "${1:-}" = "--one-attempt" ]; then
  one_attempt
  exit $?
fi

switch_mirror() {
  if [ -f "$MIRRORS" ]; then
    echo "ci_install_pocl: switching mirror $OLD -> $NEW in $MIRRORS"
    # shellcheck disable=SC2086
    $SUDO sed -i "s/$OLD/$NEW/g" "$MIRRORS"
  else
    echo "ci_install_pocl: $MIRRORS not found, mirror unchanged"
  fi
}

ok=0
n=1
while [ "$n" -le "$ATTEMPTS" ]; do
  if [ "$n" -eq 2 ]; then
    switch_mirror
  fi
  echo "ci_install_pocl: attempt $n of $ATTEMPTS (limit ${LIMIT}s)"
  start=$SECONDS
  timeout -k 10 "$LIMIT" bash "$0" --one-attempt
  rc=$?
  echo "ci_install_pocl: attempt $n finished with status $rc after $((SECONDS - start))s"
  if [ "$rc" -eq 0 ]; then
    ok=1
    break
  fi
  n=$((n + 1))
done

if [ "$ok" -ne 1 ]; then
  echo "ci_install_pocl: all $ATTEMPTS attempts failed" >&2
  exit 1
fi

if [ -n "$ARCHIVES" ]; then
  # Hand the archive directory back to the current user so the cache step can read it.
  # shellcheck disable=SC2086
  $SUDO chown -R "$(id -u):$(id -g)" "$ARCHIVES"
fi

timeout 60 clinfo -l
