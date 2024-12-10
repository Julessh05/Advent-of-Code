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

    static int calculate_for_input(string input) {
        vector<Multiply> multiplies;
        const regex reg(R"(mul\(\d{1,3},\d{1,3}\))");
        const regex number_regex("\\d{1,3}");
        // Get iterator of mul(d, d)
        sregex_iterator begin(input.begin(), input.end(), reg);
        sregex_iterator end;
        // Loop over "multiply" instructions
        for (sregex_iterator it = begin; it != end; ++it) {
            // Get single instruction
            const smatch &match = *it;
            auto m = Multiply();
            // Get numbers
            sregex_iterator number_begin(match[0].str().begin(), match[0].str().end(), number_regex);
            sregex_iterator number_end;
            m.first = stoi(number_begin->str());
            m.second = stoi(next(number_begin, 1)->str());
            multiplies.push_back(m);
        }
        int result = 0;
        for (auto multiply: multiplies) {
            result += multiply.first * multiply.second;
        }
        return result;
    }

public:
    static void first() {
        const string input = getInput();
        const int result = calculate_for_input(input);
        printf("Result: %i\n", result);
    }

    static void second() {
        const string input = getInput();
        vector<string> substrings;
        string working_input = input;
        // Get first substring to apply (instructions are enabled by default)
        const uint32_t first_stop = working_input.find("don't()");
        substrings.push_back(working_input.substr(0, first_stop));
        working_input.erase(0, first_stop);
        // Get the following substrings to apply
        while (!working_input.empty()) {
            const uint32_t to_remove = working_input.find("do()");
            working_input.erase(0, to_remove);
            const uint32_t stop = working_input.find(("don't()"));
            substrings.push_back(working_input.substr(0, stop));
            working_input.erase(0, stop);
        }
        int result = 0;
        // Extract numbers and add up
        for (const string &substring: substrings) {
            result += calculate_for_input(substring);
        }
        printf("Result: %i\n", result);
    }
};
