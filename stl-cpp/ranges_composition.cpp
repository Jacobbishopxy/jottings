/**
 * @file:		ranges_composition.cpp
 * @author:	Jacob Xie
 * @date:		2023/04/11 14:18:16 Tuesday
 * @brief:
 **/

#include <algorithm>
#include <iostream>
#include <ranges>
#include <vector>

// Regular STL C++
// (using <algorithm>)
//
// a list of numbers, selects even numbers, skips the first one and then prints them
// in the reverse order
void select_skip_reverse()
{
  const std::vector numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

  auto even = [](int i)
  { return 0 == i % 2; };

  std::vector<int> tmp;
  std::copy_if(begin(numbers), end(numbers), std::back_inserter(tmp), even);
  std::vector<int> tmp2(begin(tmp) + 1, end(tmp));

  for (auto iter = rbegin(tmp2); iter != rend(tmp2); ++iter)
  {
    std::cout << *iter << std::endl;
  }
}

// Ranges C++
// (using <ranges>, and no longer requires <algorithm>)
void select_skip_reverse_by_ranges()
{
  const std::vector numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

  auto even = [](int i)
  { return 0 == i % 2; };

  using namespace std::views;
  auto rv = reverse(drop(filter(numbers, even), 1));

  for (auto& i : rv)
  {
    std::cout << i << std::endl;
  }
}

// Ranges C++
// using `|` operator
void select_skip_reverse_advanced()
{
  const std::vector numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

  auto even = [](int i)
  { return 0 == i % 2; };

  using namespace std::views;
  auto rv = numbers | filter(even) | drop(1) | reverse;

  for (auto& i : rv)
  {
    std::cout << i << std::endl;
  }
}

int main(int argc, char** argv)
{

  select_skip_reverse();
  select_skip_reverse_by_ranges();
  select_skip_reverse_advanced();

  return 0;
}
