#include "deps.h"

using namespace std;

int main() {

    ifstream reader("input.txt");
    string line;
    vector<vector<int>>grid;
    vector<string>commands;
    while(getline(reader, line)) {commands.push_back(line);} 
    cout << commands << endl;

    for(int i = 0; i < commands.size(); i++) {
        if (i < commands.size()){
            string line = commands[i];
            stringstream ss(line);
            string str;
            while (getline(ss, str, ' ')) {command.push_back(str);}
            if (command[0] != "noop"){
                int addant = stoi(command[1]);
        //        cout <<   "add " << addant << endl;
                addantbuffer.push_back(addant);
                addxcounters.push_back(0);
            }

        }
        cout << "cycle: " << cycle << "\t x: " << x << endl; 
        cout << "counters\t";
        for (auto& c : addxcounters){
            c++;
       //     cout << c <<", ";
        }
    //    cout << "\taddants\t";
        for (auto c : addantbuffer){
     //       cout << c <<", ";
        }
      //  cout << endl;
    }
    */
}

