#include "../deps.h"

using namespace std;


int main() {

    map<int, string> dig_int2str = 
    {
        {0, "zero"},
        {1, "one"},
        {2, "two"},
        {3, "three"},
        {4, "four"},
        {5, "five"},
        {6, "six"},
        {7, "seven"},
        {8, "eight"},
        {9, "nine"},
    };

    map<string, int> dig_str2int = 
    {
        {"zero"	,0},
        {"one"	,1},
        {"two"	,2},
        {"three",3},
        {"four"	,4},
        {"five"	,5},
        {"six"	,6},
        {"seven",7},
        {"eight",8},
        {"nine"	,9},
    };

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

        int digidx1 = lines[i].size();
        int strdig1 = 0;
        for (int digit = 0; digit < 10; digit++) {
           if (digidx1 > static_cast<int>(lines[i].find(dig_int2str[digit])) && static_cast<int>(lines[i].find(dig_int2str[digit])) != string::npos) {
//               cout << "found a " << digit << endl;
                digidx1 = static_cast<int>(lines[i].find(dig_int2str[digit]));
                strdig1 = digit;
           }
        }

        if (idx1 < digidx1) {
            cal_val += lines[i][idx1];
        } else {
            cal_val += to_string(strdig1);
        }
//        cout << lines[i][idx1] << "vs" << strdig1 << endl;

//        cout << "cal_val: " << cal_val << endl;

        int idx2 = lines[i].size() - 1;
        while (!isdigit(lines[i][idx2])) {
            idx2--;
        }

        int digidx2 = -1;
        int strdig2 = 0;
        for (int digit = 0; digit < 10; digit++) {
//           if (digidx2 < static_cast<int>(lines[i].rfind(dig_int2str[digit]))) cout << "am i insane " << endl;
         
           if (digidx2 <  static_cast<int>(lines[i].rfind(dig_int2str[digit])) && static_cast<int>(lines[i].rfind(dig_int2str[digit])) != string::npos) {
//                cout << "found a " << digit << endl;
               digidx2 = static_cast<int>(lines[i].rfind(dig_int2str[digit]));
               strdig2 = digit;
           }

        }

//         cout << lines[i][idx2] << "vs" << strdig2 << endl;
        if (idx2 > digidx2) {
            cal_val += lines[i][idx2];
        } else {
            cal_val += to_string(strdig2);
        }


        cout << "cal_val: " << cal_val << endl;

        sum += stoi(cal_val);
    }
    cout << "here: " << sum << endl;
}

