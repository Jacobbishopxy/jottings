#!/usr/bin/env bash
# author:	Jacob Xie
# @date:	2023/12/25 15:36:16 Monday
# @brief:

sudo apt-get install \
     build-essential \
     ninja-build \
     cmake

git clone https://github.com/apache/arrow.git
cd arrow/cpp

mkdir build-release
# other options, see <https://arrow.apache.org/docs/developers/cpp/building.html#optional-components>
cmake -DCMAKE_INSTALL_PREFIX=/usr/local \
     -DARROW_CSV=ON \
     -DARROW_DATASET=ON \
     -DARROW_FLIGHT=ON \
     -DARROW_PARQUET=ON \
     .
sudo make all install -sj


