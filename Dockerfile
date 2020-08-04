ARG BASE_IMAGE=alpine:3.12.0

FROM $BASE_IMAGE

ARG USER=rustmap
ARG UID=1003

LABEL MATAINRE="luanngominh <ngominhluanbox@gmai.com>"



ENTRYPOINT ["/bin/sh"]