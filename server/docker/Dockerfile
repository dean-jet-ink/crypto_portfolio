FROM rust:1.78-bullseye

WORKDIR /workspace

RUN apt-get update && apt-get install -y \
    git \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# リンカー moldの設定
RUN git clone https://github.com/rui314/mold.git \
    && mkdir mold/build \
    && cd mold/build \
    && git checkout v1.7.1 \
    && ../install-build-deps.sh \
    && cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ .. \
    && cmake --build . -j $(nproc) \
    && cmake --install .

RUN cargo install cargo-watch

COPY . .

RUN cargo build


CMD ["cargo", "watch", "-x", "run -- serve", "--poll"]