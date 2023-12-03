#include "../deps.h"
//
using namespace std;

int main() {

    string symbols = "+-~!@#$%^&*()_=/";

    ifstream reader("input.txt");
    string line;
    vector<string>lines;
    while(getline(reader, line)) {lines.push_back(line);} 

    cout << "input: " << endl;
    cout << lines << endl;

    int sum = 0;

    vector<pair<int,int>> symbol_coords; 
    map<int, pair<int,int>> nums_to_add;
    vector<tuple<int, size_t, size_t, size_t>> number_coords;
    for(int row = 0; row < lines.size(); row++) {
        regex digits("\\d+");
        regex symbols("[^0-9.]");

        std::vector<int> numbers;
        {
            auto begin = sregex_iterator(lines[row].begin(), lines[row].end(), digits);
            auto end = sregex_iterator();

            for (sregex_iterator i = begin; i != end; ++i) {
                int number = stoi(i->str());

                size_t col = i->position();
                size_t numDigits = i->str().length();

                number_coords.push_back(std::make_tuple(number, row, col, numDigits));
            }
        }
    }

    for (auto number_coord : number_coords) {
        for (int digit = 0; digit < get<3>(number_coord); digit++) {
            
            if (check_surrounding(lines, get<1>(number_coord), get<2>(number_coord) + digit, regex("[^0-9.]"))) {
                sum += get<0>(number_coord);

                break;
            }

        }
    }
    cout << "here: " << sum << endl;
}

