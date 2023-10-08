# ####################################################################################################
# ## Builder
# ####################################################################################################
# FROM rust:latest AS builder

# RUN rustup target add x86_64-unknown-linux-musl
# RUN apt update && apt install -y musl-tools musl-dev
# RUN update-ca-certificates

# # Create appuser
# ENV USER=myip
# ENV UID=10001

# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     "${USER}"


# WORKDIR /app

# COPY ./ .

# RUN cargo build --target x86_64-unknown-linux-musl --release

# ####################################################################################################
# ## Final image
# ####################################################################################################
# FROM alpine:latest AS final

# # Import from builder.
# COPY --from=builder /etc/passwd /etc/passwd
# COPY --from=builder /etc/group /etc/group

# WORKDIR /app

# # Copy our build
# COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/book-service-app ./

# # Use an unprivileged user.
# USER myip:myip

# CMD ["/book-service-app"]


FROM rust:latest AS final

# Copy the binary from the previous stage or location
COPY ./target/release/book-service-app /bin

WORKDIR /bin

EXPOSE 8090

# Run the binary
CMD ["./book-service-app"]
