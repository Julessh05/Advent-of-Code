//
// Created by Julian Schumacher on 07.12.24.
//

#include <vector>
#include <sstream>
#include <cstdint>
#include <fstream>
using namespace std;

class SecondDay {
    static vector<string> getInput() {
        ifstream file("../data/second_input.txt");
        string content;
        vector<string> lines;
        string line;
        while (getline(file, line)) {
            lines.push_back(line);
        }
        return lines;
    }

    static vector<uint8_t> getLevels(const string& line) {
        vector<uint8_t> result;
        result.reserve(line.size());
        stringstream stream(line);
        string temp;
        while (getline(stream, temp, ' ')) {
            result.push_back(stoi(temp));
        }
        return result;
    }

    enum Tendency {
        Increase,
        Decrease,
        Undefined
    };

    Tendency tend = Undefined;

    bool compare_levels(const uint8_t first, const uint8_t second) {
        if (tend == Undefined) {
            if (1 <= first - second && first - second <= 3) {
                tend = Increase;
                return true;
            }
            if (1 <= second - first && second - first <= 3) {
                tend = Decrease;
                return true;
            }
            return false;
        }
        if (tend == Increase) {
            return 1 <= first - second && first - second <= 3;
        }
        if (tend == Decrease) {
            return 1 <= second - first && second - first <= 3;
        }
        return false;
    }

    bool check_line(const string& line) {
        tend = Undefined;
        const vector<uint8_t> levels = getLevels(line);
        for (int i = 0; i < levels.size() - 1; i++) {
            if (!compare_levels(levels[i], levels[i + 1])) {
                return false;
            }
        }
        return true;
    }

    bool check_line_with_buffer(const string& line) {
        tend = Undefined;
        const vector<uint8_t> levels = getLevels(line);
        bool did_remove = false;
        for (int i = 0; i < levels.size() - 1; i++) {
            if (!compare_levels(levels[i], levels[i + 1])) {
                if (!did_remove) {
                    did_remove = true;
                    // Applicable if level i + 1 can be removed to make the report safe
                    const bool first_levels = compare_levels(levels[i], levels[i + 2]); // TODO i + 2 gegeben?
                    // applicable if level i can be removed to make the report safe
                    const bool second_levels = compare_levels(levels[i + 1], levels[i + 2]);
                    if (first_levels || second_levels) {
                        continue;
                    }
                    return false;
                }
                return false;
            }
        }
        return true;
    }

public:
    void first() {
        const vector<string> lines = getInput();
        uint16_t safe_count = lines.size();
        for (const string& line : lines) {
            if (!check_line(line)) {
                safe_count--;
            }
        }
        printf("Safe count: %i\n", safe_count);
    }

    void second() {
        const vector<string> lines = getInput();
        uint16_t safe_count = lines.size();
        for (const string& line : lines) {
            if (!check_line_with_buffer(line)) {
                safe_count--;
            }
        }
        printf("Safe count with buffer: %i", safe_count);
    }
};