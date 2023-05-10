/**
 * @file:	read_csv.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/10 10:43:11 Wednesday
 * @brief:
 **/

#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

int main()
{
  std::string fname;
  std::cout << "Enter the file name: ";
  std::cin >> fname;

  std::vector<std::vector<std::string>> content;
  std::vector<std::string> row;
  std::string line, word;

  std::fstream file(fname, std::ios::in);
  if (file.is_open())
  {
    while (std::getline(file, line))
    {
      row.clear();

      std::stringstream str(line);

      while (getline(str, word, ','))
        row.push_back(word);
      content.push_back(row);
    }
  }
  else
    std::cout << "Could not open the file\n";

  for (int i = 0; i < content.size(); i++)
  {
    for (int j = 0; j < content[i].size(); j++)
    {
      std::cout << content[i][j] << " ";
    }
    std::cout << "\n";
  }

  return 0;
}
