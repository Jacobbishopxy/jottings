/**
 * @file:	const_set_iter.cpp
 * @author:	Jacob Xie
 * @date:	2023/02/08 17:04:57 Wednesday
 * @brief:
 **/

#include <algorithm>
#include <iostream>
#include <iterator>
#include <set>
#include <string>

int main(int argc, char** argv)
{

  std::set<int> lhs{5, 2, 3};

  std::set<int> rhs{4, 3};

  std::set<int> result{};

  std::set_union(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(), std::inserter(result, result.end()));

  for (auto& e : result)
  {
    std::cout << e << std::endl;
  }

  return 0;
}
