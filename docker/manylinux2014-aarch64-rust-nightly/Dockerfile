# Copyright (c) 2017, 2020 ADLINK Technology Inc.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ADLINK zenoh team, <zenoh@adlink-labs.tech>

###
### Dockerfile for a Linux 64 bits container with all required tools to build Eclipse zenoh and its APIs
###

# List of installed tools:
#  * for zenoh:
#    - Rust with nightly toolchain
#    - dpkg + cargo-deb for Debian packages
#    - rpm-build + cargo-rpm for RedHat packages
#  * for zenoh-python
#    - maturin
#  * for zenoh-c:
#    - cbindgen


###
### Dockerfile for a Linux 32 bits container with all required tools to build Eclipse zenoh and its APIs
###

# List of installed tools:
#  * for zenoh:
#    - Rust with nightly toolchain
#    - dpkg + cargo-deb for Debian packages
#    - rpm-build + cargo-rpm for RedHat packages
#  * for zenoh-python
#    - maturin
#  * for zenoh-c:
#    - cbindgen

FROM quay.io/pypa/manylinux2014_aarch64:latest

# NOTE: dkpg for aarch64 is provided by EPEL
RUN yum install -y https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm
RUN yum install -y dpkg rpm-build

ENV PATH /root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH /opt/python/cp35-cp35m/bin/:/opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:$PATH
# Otherwise `cargo new` errors
ENV USER root

###### WARNING: issue installing maturin wheel via pip on quay.io/pypa/manylinux2014_aarch64:latest : https://github.com/PyO3/maturin/issues/369
### Probable cause similar to https://stackoverflow.com/questions/39744926/how-can-i-compile-a-rust-program-so-it-doesnt-use-cxa-thread-atexit-impl
### Workaround below: clone and build maturin

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && rustup toolchain install nightly \
    && rustup default nightly \
    && cargo install cbindgen cargo-deb cargo-rpm \
    && git clone -b v0.8.3 https://github.com/PyO3/maturin.git \
    && cd maturin \
    && cargo build --release \
    && cp target/release/maturin /opt/python/cp35-cp35m/bin/
