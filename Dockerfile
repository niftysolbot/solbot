FROM rust:1.57 as builder

WORKDIR solbot-app
COPY . ./

# Build Rust executable
RUN cargo build --release

# Remove Rust code as it is now no longer needed after building
RUN rm src/*.rs


# Build base image with Rust executable on top
FROM debian:buster-slim
ARG APP=/usr/src/app

# This is needed for a potential webapp
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# This is needed for a potential webapp
#EXPOSE 8000

# Set Timezone
ENV TZ=Etc/UTC \
    APP_USER=appuser

# Run as non-root
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# Copy executable from the Rust builder image
COPY --from=builder /solbot-app/target/release/solbot ${APP}/solbot

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./solbot"]
