#include <algorithm>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

bool isPalindrome(std::string str) {
  std::string rev = str;
  std::reverse(rev.begin(), rev.end());

  return str == rev;
}

bool checkLine(std::vector<char> *line, int i, char character) {
  char prevChar = line->at(i - 1);
  char currChar = line->at(i);
  char nextChar = line->at(i + 1);
  return prevChar == currChar && currChar == nextChar && currChar == character;
}

int main() {
  std::ifstream symbols("symbole.txt");

  int palindromeCount = 0;

  std::vector<std::vector<char>> vec;

  int i = 0;
  std::string line;
  while (std::getline(symbols, line)) {
    line.erase(line.length() - 1);

    if (isPalindrome(line)) {
      std::cout << line << "\n";
      palindromeCount++;
    }

    for (int j = 0; j < line.length(); j++) {
      if (vec.size() <= i) {
        vec.push_back({line.at(j)});
      } else {
        vec.at(i).push_back(line.at(j));
      }
    }

    i++;
    if (symbols.eof()) {
      break;
    }
  }

  for (int i = 1; i < vec.size() - 1; i++) {
    std::vector<char> prevLine = vec.at(i - 1);
    std::vector<char> line = vec.at(i);
    std::vector<char> nextLine = vec.at(i + 1);
    for (int j = 1; j < line.size() - 1; j++) {
      char character = line.at(j);
      bool ch1 = checkLine(&prevLine, j, character);
      bool ch2 = checkLine(&line, j, character);
      bool ch3 = checkLine(&nextLine, j, character);

      if (ch1 && ch2 && ch3) {
        printf("(%i,%i)\n", i + 1, j + 1);
      }
    }
  }

  int total = 0;
  int biggest = 0;

  std::string value = "";
  for (int i = 0; i < vec.size(); i++) {
    value.clear();
    for (int j = 0; j < vec.at(i).size(); j++) {
      char character = vec.at(i).at(j);
      if (character == 'o') {
        value += "0";
      } else if (character == '+') {
        value += "1";
      } else if (character == '*') {
        value += "2";
      }
    }

    int result = 0;
    for (char c : value) {
      result = result * 3 + (c - '0');
    }

    total += result;
    if (result > biggest) {
      biggest = result;
    }
  }

  symbols.close();

  std::cout << "Palindrome count: " << palindromeCount << "\n";
  std::cout << "Biggest: " << biggest << "\n";
  std::cout << "Total: " << total << "\n";

  return 0;
}
