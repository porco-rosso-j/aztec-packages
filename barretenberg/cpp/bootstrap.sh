#!/usr/bin/env bash
set -eu

# Navigate to script folder
cd "$(dirname "$0")"

CMD=${1:-}

OS=linux

if [ -n "$CMD" ]; then
  if [ "$CMD" = "clean" ]; then
    git clean -ffdx
    exit 0
  elif [ "$CMD" = "ios" ]; then
    OS=ios
  elif [ "$CMD" = "android" ]; then
    OS=android
  else
    echo "Unknown command: $CMD"
    exit 1
  fi
fi

# Determine system.
if [ "$OS" != "ios" ] && [ "$OS" != "android" ]; then
  if [[ "$OSTYPE" == "darwin"* ]]; then
    OS=macos
  elif [[ "$OSTYPE" == "linux-gnu" ]]; then
    OS=linux
  elif [[ "$OSTYPE" == "linux-musl" ]]; then
    OS=linux
  else
    echo "Unknown OS: $OSTYPE"
    exit 1
  fi
fi

# Download ignition transcripts.
(cd ./srs_db && ./download_ignition.sh 0)

# Attempt to just pull artefacts from CI and exit on success.
[ -n "${USE_CACHE:-}" ] && ./bootstrap_cache.sh && exit

# Pick native toolchain file.
ARCH=$(uname -m)
if [ "$OS" == "macos" ]; then
  PRESET=default
elif [ "$OS" == "ios" ]; then
  PRESET=ios
elif [ "$OS" == "android" ]; then
  PRESET=android
else
  if [ "$(which clang++-16)" != "" ]; then
    PRESET=clang16
  else
    PRESET=default
  fi
fi

# Remove cmake cache files.
rm -f {build,build-wasm,build-wasm-threads,build-ios,build-android}/CMakeCache.txt

echo "#################################"
echo "# Building with preset: $PRESET"
echo "# When running cmake directly, remember to use: --build --preset $PRESET"
echo "#################################"


if [ "$OS" == "ios" ]; then
  function build_native {
    cmake --preset $PRESET -DCMAKE_BUILD_TYPE=RelWithAssert -DPLATFORM=OS64 -DDEPLOYMENT_TARGET=14.0
    cmake --build --preset $PRESET --target bb
  }
elif [ "$OS" == "android" ]; then
  function build_native {
    cmake --preset $PRESET -DCMAKE_BUILD_TYPE=RelWithAssert -DANDROID_ABI=arm64-v8a -DANDROID_PLATFORM=android-33
    cmake --build --preset $PRESET --target bb
  }
else
  function build_native {
    cmake --preset $PRESET -DCMAKE_BUILD_TYPE=RelWithAssert
    cmake --build --preset $PRESET --target bb
  }
fi

function build_wasm {
  cmake --preset wasm
  cmake --build --preset wasm
}

function build_wasm_threads {
  cmake --preset wasm-threads
  cmake --build --preset wasm-threads
}

g="\033[32m"  # Green
b="\033[34m"  # Blue
p="\033[35m"  # Purple
r="\033[0m"   # Reset

AVAILABLE_MEMORY=0

case "$(uname)" in
  Linux*)
    # Check available memory on Linux
    AVAILABLE_MEMORY=$(awk '/MemTotal/ { printf $2 }' /proc/meminfo)
    ;;
  *)
    echo "Parallel builds not supported on this operating system"
    ;;
esac
# This value may be too low.
# If builds fail with an amount of free memory greater than this value then it should be increased.
MIN_PARALLEL_BUILD_MEMORY=32854492

if [[ AVAILABLE_MEMORY -lt MIN_PARALLEL_BUILD_MEMORY ]]; then
  echo "System does not have enough memory for parallel builds, falling back to sequential"
  build_native
  build_wasm
  build_wasm_threads
else
  (build_native > >(awk -v g="$g" -v r="$r" '{print g "native: " r $0}')) &
  (build_wasm > >(awk -v b="$b" -v r="$r" '{print b "wasm: " r $0}')) &
  (build_wasm_threads > >(awk -v p="$p" -v r="$r" '{print p "wasm_threads: "r $0}')) &

  for job in $(jobs -p); do
    wait $job || exit 1
  done
fi

if [ ! -d ./srs_db/grumpkin ]; then
  # The Grumpkin SRS is generated manually at the moment, only up to a large enough size for tests
  # If tests require more points, the parameter can be increased here.
  cd ./build && cmake --build . --parallel --target grumpkin_srs_gen && ./bin/grumpkin_srs_gen 8192
fi
