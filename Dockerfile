FROM rust:1.72.1 as build-env
LABEL maintainer="yanorei32"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

WORKDIR /usr/src
RUN cargo new nichijid-rs
COPY LICENSE Cargo.toml Cargo.lock /usr/src/nichijid-rs/
WORKDIR /usr/src/nichijid-rs
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN	cargo install cargo-license && cargo license \
	--authors \
	--do-not-bundle \
	--avoid-dev-deps \
	--avoid-build-deps \
	--filter-platform "$(rustc -vV | sed -n 's|host: ||p')" \
	> CREDITS

RUN cargo build --release
COPY src/ /usr/src/nichijid-rs/src/

RUN touch src/* && cargo build --release

FROM debian:bullseye-slim@sha256:0cdee24a156b67861e0ec34a0784be80f3c6ec00f0d1708434395aa58d427d44

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/nichijid-rs/CREDITS \
	/usr/src/nichijid-rs/LICENSE \
	/usr/share/licenses/nichijid-rs/

COPY --chown=root:root --from=build-env \
	/usr/src/nichijid-rs/target/release/nichijid-rs \
	/usr/bin/nichijid-rs

CMD ["/usr/bin/nichijid-rs"]
