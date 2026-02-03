FROM rust:1.93.0 as build-env
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

FROM debian:bookworm-slim@sha256:98f4b71de414932439ac6ac690d7060df1f27161073c5036a7553723881bffbe

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/nichijid-rs/CREDITS \
	/usr/src/nichijid-rs/LICENSE \
	/usr/share/licenses/nichijid-rs/

COPY --chown=root:root --from=build-env \
	/usr/src/nichijid-rs/target/release/nichijid-rs \
	/usr/bin/nichijid-rs

CMD ["/usr/bin/nichijid-rs"]
