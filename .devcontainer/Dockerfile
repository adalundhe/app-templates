ARG VERSION_VARIANT=3.11
ARG OS_VARIANT=bullseye

FROM python:${VERSION_VARIANT}-slim-${OS_VARIANT}
ARG USERNAME
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME


RUN apt install -y git build-essential libpq-dev 
RUN pip install poetry awscli

COPY pyproject.toml ./

EXPOSE 5000