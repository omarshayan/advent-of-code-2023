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

    map<int, pair<int,int>> nums_to_add;
    map<coord, tuple<coord, int>> number_coords;
    vector<coord> gears;
    for(int row = 0; row < lines.size(); row++) {
        regex digits("\\d+");

        {
            auto begin = sregex_iterator(lines[row].begin(), lines[row].end(), digits);
            auto end = sregex_iterator();

            for (sregex_iterator i = begin; i != end; ++i) {
                int number = stoi(i->str());

                size_t col = i->position();
                size_t num_digits = i->str().length();

                for (int dig = 0; dig < num_digits; dig++) {
                    number_coords[{row, col + dig}] = make_tuple(make_pair(row, col), number);
                }
            }
        }

        {
            for (int j = 0; j < lines[row].size(); j++) {
                if (lines[row][j] == '*') {
                    gears.push_back({row, j});
                }
            }
        }
    }

    for (auto gear : gears) {
        set<coord> nearby_nums;
        for (int i = max(gear.first - 1, 0); i <= min(gear.first + 1, (int)lines.size()); i++) {
            for (int j = max(gear.second - 1, 0); j <= min(gear.second + 1, (int)lines[i].size()); j++) {
                    
                if (isdigit(lines[i][j])) {
                    coord num_coord = get<0>(number_coords[{i, j}]);
                    nearby_nums.insert(num_coord);
                }
            }
        }
        if (nearby_nums.size() == 2) {
            int product = 1;
            for (auto& nearby_num : nearby_nums) {
                product *= get<1>(number_coords[nearby_num]);
            }
            sum += product;
        }
    }

    cout << "here: " << sum << endl;
}

