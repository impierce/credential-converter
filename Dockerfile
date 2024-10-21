FROM rust:1.82 AS build

# 1. create a workdir to start working from
WORKDIR /credential-converter
# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# # 3. build depedencies for the project 
# RUN cargo build --release

# 4. copy over the origninal source and reference files
COPY ./src ./src
COPY ./json ./json

# 5. build the project 
RUN cargo build --release



# our final base
FROM rust:1.82-slim

# copy the build artifact from the build stage
COPY --from=build /credential-converter/target/release/credential-converter .
COPY --from=build /credential-converter/json ./json

# set the startup command to run your binary
CMD ["./credential-converter", "-w", "0.0.0.0:3000"]

EXPOSE 3000
