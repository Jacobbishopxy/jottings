# Eigen Demo

## Installation

1. Download [Eigen](https://eigen.tuxfamily.org/index.php?title=Main_Page#Download)

1. unzip download file, and `cd ./eigen-3.4.0 && mkdir build_dir && cd build_dir`

1. `cmake ..` and `sudo make install`

## Compilation

Ubuntu: `g++ -I /usr/local/include/eigen3/ simple.cc -o simple`

Mac: `clang++ -I /usr/local/include/eigen3/ simple.cc -o simple`
