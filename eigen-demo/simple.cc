/**
 * @file:	simple.cc
 * @author:	Jacob Xie
 * @date:	2023/12/22 22:49:20 Friday
 * @brief:
 *
 * compile:
 * g++ -I /opt/eigen-3.4.0 simple.cc -o simple
 **/

#include <Eigen/Dense>
#include <iostream>

using Eigen::MatrixXd;

int main(int argc, char** argv)
{
  MatrixXd m(2, 2);
  m(0, 0) = 3;
  m(1, 0) = 2.5;
  m(0, 1) = -1;
  m(1, 1) = m(1, 0) + m(0, 1);

  std::cout << m << std::endl;

  return 0;
}
