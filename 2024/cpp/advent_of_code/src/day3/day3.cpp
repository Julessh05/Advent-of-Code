//
// Created by Julian Schumacher on 08.12.24.
//

#include <fstream>
#include <sstream>
#include <regex>
using namespace std;

class ThirdDay {
    static string getInput() {
        // From here: https://stackoverflow.com/questions/2602013/read-whole-ascii-file-into-c-stdstring
        ifstream t("../data/input_day3.txt");
        stringstream stream;
        stream << t.rdbuf();
        return stream.str();
    }

    class Multiply {
    public:
        int first;
        int second;
    };

public:
    void first() {
        const string input = getInput();
        int result = 0;
        vector<Multiply> multiplies;
        const string regex_string = "";
        const regex regex(regex_string);
        // TODO: update
        regex_search(input, regex);
        for (Multiply multiply: multiplies) {
            result += multiply.first * multiply.second;
        }
        printf("Result: %i\n", result);
    }

    void second() {
        const string input = getInput();
    }
};
