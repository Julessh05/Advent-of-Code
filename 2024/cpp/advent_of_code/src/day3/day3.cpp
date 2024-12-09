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
        const regex reg("mul(\\d{1,3}, \\d{1,3})");
        const regex inner_regex("\\d{1,3}");
        smatch match;
        if (regex_search(input, match, reg)) {
            auto m = Multiply();
            smatch inner_match;
            regex_search(match[0].str(), match[1].str(), inner_match, inner_regex);
            m.first = stoi(inner_match.str());
            regex_search(match[0].str(), match[1].str(), inner_match, inner_regex);
            m.second = stoi(inner_match.str());
            multiplies.push_back(m);
        }
        for (Multiply multiply: multiplies) {
            result += multiply.first * multiply.second;
        }
        printf("Result: %i\n", result);
    }

    void second() {
        const string input = getInput();
    }
};
