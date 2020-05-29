# Build Stage
FROM ekidd/rust-musl-builder AS builder

RUN USER=root cargo new sonar-cli
WORKDIR ./sonar-cli

ADD --chown=rust:rust ./Cargo.toml ./
ADD --chown=rust:rust ./Cargo.lock ./
RUN cargo build --release

ADD --chown=rust:rust ./src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM openjdk:11-jre-slim

ARG SONAR_SCANNER_HOME=/opt/sonar-scanner
ARG NODEJS_HOME=/opt/nodejs

ARG UID=1000
ARG GID=1000

ENV SONAR_SCANNER_VERSION=4.3.0.2102 \
    SONAR_SCANNER_HOME=${SONAR_SCANNER_HOME} \
    NODEJS_HOME=${NODEJS_HOME} \
    NODEJS_VERSION=v10.16.3 \
    PATH=${SONAR_SCANNER_HOME}/bin:${NODEJS_HOME}/bin:${PATH} \
    NODE_PATH=${NODEJS_HOME}/lib/node_modules

RUN apt-get update \
    && apt-get install -y --no-install-recommends unzip xz-utils \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --gid ${GID} scanner-cli \
    && useradd --uid ${UID} --gid scanner-cli --shell /bin/bash --create-home scanner-cli
COPY --chown=scanner-cli:scanner-cli bin /usr/bin/

# Install Sonar CLI
WORKDIR /tmp
ADD https://bintray.com/sonarsource/SonarQube/download_file?file_path=org%2Fsonarsource%2Fscanner%2Fcli%2Fsonar-scanner-cli%2F${SONAR_SCANNER_VERSION}%2Fsonar-scanner-cli-${SONAR_SCANNER_VERSION}.zip /tmp/sonar-scanner.zip
RUN \
    unzip /tmp/sonar-scanner.zip \
    && mkdir -p ${SONAR_SCANNER_HOME} && mv -fv /tmp/sonar-scanner-${SONAR_SCANNER_VERSION}/* ${SONAR_SCANNER_HOME} \
    && rm -rf /tmp/*

# Install Node to scan node modules
ADD https://nodejs.org/dist/${NODEJS_VERSION}/node-${NODEJS_VERSION}-linux-x64.tar.xz /tmp/nodejs.tar.xz
RUN tar Jxf nodejs.tar.xz \
    && rm nodejs.tar.xz \
    && mv node-${NODEJS_VERSION}-linux-x64 ${NODEJS_HOME} \
    && npm install -g typescript@3.8.3

COPY --from=builder --chown=scanner-cli:scanner-cli /home/rust/.cargo/bin/ /usr/bin/

WORKDIR /usr/src
USER scanner-cli