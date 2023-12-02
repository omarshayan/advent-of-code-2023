#include "../deps.h"
//
using namespace std;

int main() {

    ifstream reader("input.txt");
    string line;
    vector<string>lines;
    while(getline(reader, line)) {lines.push_back(line.substr(line.find(":") + 1));}

    cout << "input: " << endl;
    cout << lines << endl;

    int gamesum = 0;

    for(int i = 0; i < lines.size(); i++) {
        stringstream ss(lines[i]);
        int max_blue = 0;
        int max_red = 0;
        int max_green = 0;


        string game_desc;
        while(getline(ss, game_desc, ';')) {
            stringstream ss(game_desc);
            string pull_desc;
            while(getline(ss, pull_desc, ',')) {
                cout << pull_desc << endl;
                cout << stoi(pull_desc) << endl;

                if (pull_desc.find("blue") != string::npos) {
                    max_blue = max(max_blue, stoi(pull_desc));
                }
                if (pull_desc.find("red") != string::npos) {
                    max_red = max(max_red, stoi(pull_desc));
                }
                if (pull_desc.find("green") != string::npos) {
                    max_green = max(max_green, stoi(pull_desc));
                }
            }
        }
        cout << "max_blue: " << max_blue << endl;
        cout << "max_green: " << max_green << endl;
        cout << "max_red: " << max_red << endl;
        if (!(max_blue > 14 || max_red > 12 || max_green > 13)) {
            gamesum += i+1;
            cout << " game " << i+1 << " possible!\n";
        }
    }
    cout << gamesum << endl;
}

