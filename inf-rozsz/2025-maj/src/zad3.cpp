#include <array>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

int niewd(int a, int b) {
  while (a != b) {
    if (a > b) {
      a -= b;
    } else {
      b -= a;
    }
  }
  return a;
}

bool checkCoords(std::vector<std::array<int, 2>> *coords, int i, int j) {
  return (coords->at(i)[0] + coords->at(j)[0]) / 2 ==
             coords->at((i + j) / 2)[0] &&
         (coords->at(i)[1] + coords->at(j)[1]) / 2 ==
             coords->at((i + j) / 2)[1];
}

int main() {
  int x = 0;
  int y = 0;

  std::ifstream file("dron.txt");

  std::string str = "";

  int total = 0;
  int totalInSquare = 0;

  std::vector<std::array<int, 2>> coords;

  while (std::getline(file, str)) {
    std::stringstream ss(str);
    std::string moveXStr;
    std::string moveYStr;
    std::getline(ss, moveXStr, ' ');
    std::getline(ss, moveYStr, ' ');
    int moveX = std::stoi(moveXStr);
    int moveY = std::stoi(moveYStr);
    x += moveX;
    y += moveY;

    if (moveY != 0) {
      if (moveY < 0) {
        moveY = moveY * -1;
      }
      if (niewd(moveX, moveY) > 1) {
        total++;
      }
    } else if (moveX > 1) {
      total++;
    }

    if ((x > 0 && x < 5000) && (y > 0 && y < 5000)) {
      totalInSquare++;
    }

    coords.push_back({x, y});

    if (file.eof()) {
      break;
    }
  }

  bool run = true;
  while (run) {
    for (int i = 1; i < coords.size(); i++) {
      for (int j = i + 1; j < coords.size() - 1; j++) {
        if (checkCoords(&coords, i, j)) {
          printf("(%d, %d) ", coords.at(i)[0], coords.at(i)[1]);
          printf("(%d, %d) ", coords.at((i + j) / 2)[0],
                 coords.at((i + j) / 2)[1]);
          printf("(%d, %d)\n", coords.at(j)[0], coords.at(j)[1]);
          run = false;
        }
      }
    }
  }

  std::cout << "Total: " << total << "\n";
  std::cout << "TotalInSquare: " << totalInSquare << "\n";

  return 0;
}
