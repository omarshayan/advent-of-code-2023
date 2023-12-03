#include "./deps.h"
//
using namespace std;

int main() {

    vector my_input = read_input("input.txt");

    int sum = 0;

    for(int row = 0; row < my_input.size(); row++) {
        regex digits("\\d+");

        std::vector<int> numbers;
        {
            auto begin = sregex_iterator(my_input[row].begin(), my_input[row].end(), digits);
            auto end = sregex_iterator();

            for (sregex_iterator i = begin; i != end; ++i) {
            }
        }
    }

    cout << "here: " << sum << endl;
}

