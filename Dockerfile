FROM rust:1.69 as build-stage

# Build
ADD . /app/
WORKDIR /app
RUN cargo build -r

# Production
FROM rust:1.69
WORKDIR /app
COPY --from=build-stage /app/target/release ./build
EXPOSE 3030
CMD ["/app/build/parkour-api"]
