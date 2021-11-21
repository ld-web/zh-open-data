FROM rust:1.56 as builder

WORKDIR /zh-open-data
COPY . .
RUN cargo build --release


FROM debian:bullseye-slim
ARG APP=/app

RUN apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 8900

ENV TZ=Etc/UTC \
  APP_USER=appuser \
  LOAD_DIR=data

RUN groupadd $APP_USER \
  && useradd -g $APP_USER $APP_USER \
  && mkdir -p ${APP}

COPY --from=builder /zh-open-data/target/release/zh_open_data ${APP}/zh_open_data

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["sh", "-c", "./zh_open_data -l ${LOAD_DIR}"]