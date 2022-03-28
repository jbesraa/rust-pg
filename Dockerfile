####################################################################################################
## Builder
####################################################################################################
FROM rust:latest

RUN update-ca-certificates

# Create appuser
ENV USER=myip
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /myip

COPY ./ .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

EXPOSE 8080

CMD ["/myip/target/release/image-upload"]

# ####################################################################################################
# ## Final image
# ####################################################################################################
# FROM debian:buster-slim

# # Import from builder.
# COPY --from=builder /etc/passwd /etc/passwd
# COPY --from=builder /etc/group /etc/group

# WORKDIR /myip

# # Copy our build
# COPY --from=builder /myip/target/release/image-upload ./
# COPY --from=builder /myip/example.crt ./
# COPY --from=builder /myip/example.key ./

# # Use an unprivileged user.
# USER myip:myip

# CMD ["/myip/image-upload"]
