FROM fedora:33
WORKDIR /usr/local/bin
COPY ./target/release/sku_imgprocesser_microservice /usr/local/bin/sku_imgprocesser_microservice
RUN dnf install curl -y
RUN dnf install ImageMagick -y
STOPSIGNAL SIGINT
ENTRYPOINT ["sku_imgprocesser_microservice"]
