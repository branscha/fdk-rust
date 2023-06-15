#!/usr/bin/env bash
#
# Copyright (c) 2019, 2020 Oracle and/or its affiliates. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#


set -xe

if [ -z "$1" ];then
  echo "Please supply Rust version as argument to build image." >> /dev/stderr
  exit 2
fi

rustversion=$1

pushd images/build/${rustversion} && docker build -t fnproject/rust:${rustversion}-build . && popd
pushd images/runtime/${rustversion} && docker build -t fnproject/rust:${rustversion}-rt . && popd
pushd images/init && docker build -t fnproject/rust:init . && popd
