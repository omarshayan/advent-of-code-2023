#include "../deps.h"
//
using namespace std;

int main() {

    ifstream reader("input.txt");
    string line;
    vector<string>lines;
    while(getline(reader, line)) {lines.push_back(line);} 

    cout << "input: " << endl;
    cout << lines << endl;

    int sum = 0;
    for(int i = 0; i < lines.size(); i++) {

    }
    cout << "here: " << sum << endl;
}

