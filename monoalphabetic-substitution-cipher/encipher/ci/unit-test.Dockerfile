FROM rust:1.89.0-alpine3.21@sha256:5ad7315e97170fe9bb1cc3b1f1499db65d721937da5bbff45d619a5d1c4561de
RUN apk add --no-cache \
	musl-dev=1.2.5-r9

ENTRYPOINT ["cargo", "test", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked"]
