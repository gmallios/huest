# RUSTFLAGS="-C linker=arm-linux-gnueabihf-ld" 
TARGET=arm-unknown-linux-gnueabihf
export TARGET_CC=$TARGET-gcc
export TARGET_AR=$TARGET-ar
export CC_armv7_unknown_linux_gnu=$TARGET-gcc
export CXX_armv7_unknown_linux_gnu=$TARGET-g++
export AR_armv7_unknown_linux_gnu=$TARGET-ar
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=$TARGET-gcc
export CMAKE_TOOLCHAIN_FILE_armv7_unknown_linux_gnueabihf=$(pwd)/wip/armv7.cmake
cargo build --release --target $TARGET
# cross build --target arm-unknown-linux-gnueabihf