#include <vector>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <deque>
#include <map>
#include <iostream>
#include <regex>
#include <tuple>
#include <algorithm>
#include <set>

using namespace std;
using coord = pair<int, int>;

template <typename T>
ostream& operator<<(ostream& os, const vector<T>& v)
{
    os << "[";
    for (int i = 0; i < v.size(); ++i) {
        os << v[i];
        if (i != v.size() - 1)
            os << ", ";
    }
    os << "]\n";
    return os;
}

vector<string> read_input(string filename) {
    ifstream reader("input.txt");
    string line;
    vector<string>lines;
    while(getline(reader, line)) {lines.push_back(line);} 

    cout << "input: " << endl;
    cout << lines << endl;
    return lines;
}

bool check_surrounding(vector<string> lines, int row, int col, regex pattern) {
    for (int i = max(row - 1, 0); i <= min(row + 1, (int)lines.size()); i++) {
        for (int j = max(col - 1, 0); j <= min(col + 1, (int)lines[i].size()); j++) {
            if (regex_match(string(1, lines[i][j]), pattern)) return true;
             
        }
    }
    return false;
}
