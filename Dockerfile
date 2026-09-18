FROM rust:1.98 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

ENV token default_token_value
ENV ip default_ip_value
ENV port default_port_value
env password default_password_value

HEALTHCHECK --interval=5m --timeout=3s --start-period=5s \
  CMD curl -f http://127.0.0.1:3030/ || exit 1

COPY --from=builder /usr/local/cargo/bin/wardogs_server_status /usr/local/bin/wardogs_server_status
RUN apt-get update && apt-get install --assume-yes curl && apt-get clean
CMD ["wardogs_server_status"]
