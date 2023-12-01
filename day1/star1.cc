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
        cout << "line: " << lines[i] << endl;
        string cal_val = "";
        int idx1 = 0;
        while (!isdigit(lines[i][idx1])) {
            idx1++;
        }
        cal_val += lines[i][idx1];
        cout << "cal_val: " << cal_val << endl;

        int idx2 = lines[i].size() - 1;
        while (!isdigit(lines[i][idx2])) {
            idx2--;
        }
        cal_val += lines[i][idx2];
        cout << "cal_val: " << cal_val << endl;

        cout << "cal_val: " << cal_val << endl;



        sum += stoi(cal_val);
    }
    cout << "here: " << sum << endl;
}

