FROM openjdk:11-jre-slim

ARG SONAR_SCANNER_HOME=/opt/sonar-scanner
ARG UID=1000
ARG GID=1000

ENV SONAR_SCANNER_VERSION=4.3.0.2102 \
    SONAR_SCANNER_HOME=${SONAR_SCANNER_HOME} \
    PATH=${SONAR_SCANNER_HOME}/bin:${PATH}

RUN addgroup -g ${GID} -S scanner-cli \
    && adduser -u ${UID} -S scanner-cli -G scanner-cli -s /bin/bash
COPY --chown=scanner-cli:scanner-cli bin /usr/bin/

ADD https://bintray.com/sonarsource/SonarQube/download_file?file_path=org%2Fsonarsource%2Fscanner%2Fcli%2Fsonar-scanner-cli%2F${SONAR_SCANNER_VERSION}%2Fsonar-scanner-cli-${SONAR_SCANNER_VERSION}.zip /tmp/sonar-scanner.zip

WORKDIR /tmp

RUN \
    unzip /tmp/sonar-scanner.zip && \
    mv -fv /tmp/sonar-scanner-${SONAR_SCANNER_VERSION}/bin/sonar-scanner /usr/bin && \
    mv -fv /tmp/sonar-scanner-${SONAR_SCANNER_VERSION}/lib/* /usr/lib

RUN \
    apk add --no-cache nodejs && \
    ls -lha /usr/bin/sonar* && \
    ln -s /usr/bin/sonar-scanner-run.sh /usr/bin/gitlab-sonar-scanner

WORKDIR /usr/src

USER scanner-cli

# FROM alpine:edge
# # unzip issue with 3.11 : https://gitlab.alpinelinux.org/alpine/aports/issues/11221

# ARG SONAR_SCANNER_HOME=/opt/sonar-scanner
# ARG UID=1000
# ARG GID=1000

# ENV SONAR_SCANNER_HOME=${SONAR_SCANNER_HOME} \
#     SONAR_SCANNER_VERSION=4.2.0.1873 \
#     PATH=${SONAR_SCANNER_HOME}/bin:${PATH}

# RUN apk add --update \
#         wget \
#         git \
#         bash \
#     && rm -rf /var/cache/apk/*

# RUN addgroup -g ${GID} -S scanner-cli \
#     && adduser -u ${UID} -S scanner-cli -G scanner-cli -s /bin/bash

# WORKDIR /opt
# RUN wget -U "scannercli" -q -O /opt/sonar-scanner-cli.zip https://binaries.sonarsource.com/Distribution/sonar-scanner-cli/sonar-scanner-cli-${SONAR_SCANNER_VERSION}-linux.zip

# RUN unzip --help
# RUN CF="${CF} -DNO_LCHMOD"
# RUN CFLAGS="${CFLAGS} -DNO_LCHMOD"
# RUN unzip -q sonar-scanner-cli.zip \
#     && rm sonar-scanner-cli.zip \
#     && mv sonar-scanner-${SONAR_SCANNER_VERSION}-linux ${SONAR_SCANNER_HOME}
#
#COPY --chown=scanner-cli:scanner-cli bin /usr/bin/
#
#WORKDIR /usr/src
#
#USER scanner-cli
