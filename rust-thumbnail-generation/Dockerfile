# ---- Builder Stage ----
FROM rust:1.89-slim as builder

# 1. Install system dependencies first
RUN apt-get update && apt-get install -y \
    pkg-config \
    cmake \
    make \
    g++ \
    git \
    libde265-dev \
    libx265-dev

# 2. Compile and install a modern version of libheif
WORKDIR /usr/src/libheif
RUN git clone https://github.com/strukturag/libheif.git . && \
    git checkout v1.20.0 && \
    mkdir build && cd build && \
    # THIS IS THE FIX: Explicitly disable plugin loading and enable decoders
    cmake .. -DCMAKE_BUILD_TYPE=Release -DENABLE_PLUGIN_LOADING=OFF -DWITH_DE265=ON -DWITH_X265=ON && \
    make -j$(nproc) && \
    make install

# 3. Build our Rust application
WORKDIR /usr/src/app
COPY . .
# Build the final, optimized application binary in one clean step
RUN cargo build --release

# ---- Runner Stage ----
FROM debian:12-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y libde265-0 libx265-199 && rm -rf /var/lib/apt/lists/*

# Copy the compiled C library and update the linker cache
COPY --from=builder /usr/local/lib/libheif.so* /usr/local/lib/
RUN ldconfig

# Copy our compiled Rust application
COPY --from=builder /usr/src/app/target/release/rust-thumbnail-generation /usr/local/bin/rust-thumbnail-generation

# Set the command to run our binary
CMD ["rust-thumbnail-generation"]