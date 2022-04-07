FROM rust:1.59 as builder
WORKDIR /usr/src/blip-back
COPY . .
RUN cargo install --path .

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/blip-back /usr/local/bin/blip-back
# COPY --from=builder /usr/src/blip-back/web ./web
EXPOSE 8080
CMD ["blip-back"]