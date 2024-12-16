# マルチステージビルドを使用、Rustのプログラムをビルド
FROM rust:1.78-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 不要なソフトウェアは必要ないので、軽量なbookworm-slimを使用
FROM debian:bookworm-slim
WORKDIR /app

#ユーザーの作成
RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/app ./target/release/app

# ポートを解放してアプリを起動
ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./target/release/app"]