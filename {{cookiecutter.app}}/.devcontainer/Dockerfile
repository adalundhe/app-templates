ARG VERSION_VARIANT="{{cookiecutter.version}}"

FROM corpheus91/devcontainers:python-${VERSION_VARIANT}-bookworm

RUN apt update && \
    apt install -y libpq-dev

EXPOSE 5000
