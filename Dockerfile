FROM fedora:33
RUN dnf update -y && dnf clean all -y
WORKDIR /usr/local/bin
COPY ./target/release/sku_imgprocesser_microservice /usr/local/bin/sku_imgprocesser_microservice
COPY ./gz_sku_img_resize /usr/local/bin/resize_script
COPY ./img_add_watermark.sh /usr/local/bin/img_add_watermark.sh
COPY ./img_process.sh /usr/local/bin/img_process.sh
COPY ./watermark.svg /usr/local/bin/watermark.svg
RUN dnf install curl -y && dnf clean all -y
RUN dnf install ImageMagick -y && dnf clean all -y
STOPSIGNAL SIGINT
ENTRYPOINT ["sku_imgprocesser_microservice"]
