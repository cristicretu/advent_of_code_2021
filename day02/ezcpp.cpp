#include <fstream>
#include <iostream>

using namespace std;

ifstream fin("src/input.txt");

int main() {
  int x(0), y(0);
  string s;
  int dist;

  while (fin >> s >> dist) {
    if (s == "forward")
      x += dist;
    else if (s == "down")
      y += dist;
    else if (s == "up")
      y -= dist;
  }

  cout << x * y;
}
