FROM fedora:33
WORKDIR /usr/local/bin
COPY ./target/release/purchase_microservice /usr/local/bin/purchase_microservice
RUN dnf install curl -y
STOPSIGNAL SIGINT
ENTRYPOINT ["purchase_microservice"]
