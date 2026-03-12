# Stage 1: Build
FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen clang openssl-dev openssl-libs-static pkgconfig

RUN cargo install dioxus-cli --locked
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

# Build Tailwind CSS
RUN npm install && npx @tailwindcss/cli -i tailwind.css -o assets/tailwind.css --minify

# Build fullstack app
RUN dx bundle --release --fullstack

# Stage 2: Runtime
FROM rustlang/rust:nightly-alpine AS runner

RUN apk update && \
    apk add --no-cache sed bash

# Rustfmt is needed for code formatting in sessions
RUN rustup component add rustfmt

WORKDIR /app

# Dioxus fullstack bundle output
COPY --from=builder /work/target/dx/tryrust/release/web /app

ENV RUST_LOG="info"
ENV IP="0.0.0.0"
ENV PORT="8080"

EXPOSE 8080
ENTRYPOINT ["/app/tryrust"]
