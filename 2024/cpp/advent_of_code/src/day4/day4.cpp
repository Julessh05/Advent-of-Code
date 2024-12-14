//
// Created by Julian Schumacher on 10.12.24.
//

#include <fstream>
#include <vector>
using namespace std;

class FourthDay {
  static vector<vector<char> > getInput() {
    ifstream file("../data/test_input_day4.txt");
    string content;
    vector<string> lines;
    string line;
    while (getline(file, line)) {
      lines.push_back(line);
    }
    vector<vector<char> > result;
    for (auto &l: lines) {
      vector<char> chars;
      for (char c: l) {
        chars.push_back(c);
      }
      result.push_back(chars);
    }
    return result;
  }

public:
  static void first() {
    const vector<vector<char> > lines = getInput();
    uint32_t result = 0;
    // Because all lines have the same length, length is calculated here
    const uint32_t number_of_lines = lines.size();
    const uint32_t line_length = lines[0].size();
    for (int line_count = 0; line_count < number_of_lines; line_count++) {
      vector<char> line = lines[line_count];
      for (int char_count = 0; char_count < line_length; char_count++) {
        if (line[char_count] == 'X') {
          // Same line, next char
          if (char_count + 1 < line_length && line[char_count + 1] == 'M') {
            if (char_count + 2 < line_length && line[char_count + 2] == 'A') {
              if (char_count + 3 < line_length && line[char_count + 3] == 'S') {
                printf("line count : %i, char count: %i\n", line_count, char_count);
                result++;
              }
            }
          }
          // Same line, previous char
          else if (char_count - 1 > 0 && line[char_count - 1] == 'M') {
            if (char_count - 2 > 0 && line[char_count - 2] == 'A') {
              if (char_count - 3 > 0 && line[char_count - 3] == 'S') {
                printf("line count : %i, char count: %i\n", line_count, char_count);
                result++;
              }
            }
          }
          // Next line, same char
          else if (line_count + 1 < number_of_lines && lines[line_count + 1][char_count] == 'M') {
            if (line_count + 2 < number_of_lines && lines[line_count + 2][char_count] == 'A') {
              if (line_count + 3 < number_of_lines && lines[line_count + 3][char_count] == 'S') {
                printf("line count : %i, char count: %i\n", line_count, char_count);
                result++;
              }
            }
          }
          // Next line, previous char
          else if (
            line_count + 1 < number_of_lines &&
            char_count - 1 > 0 &&
            lines[line_count + 1][char_count - 1] == 'M'
          ) {
            if (
              line_count + 2 < number_of_lines &&
              char_count - 2 > 0 &&
              lines[line_count + 2][char_count - 2] == 'A'
            ) {
              if (
                line_count + 3 < number_of_lines &&
                char_count - 3 > 0 &&
                lines[line_count + 3][char_count - 3] == 'S'
              ) {
                printf("line count : %i, char count: %i\n", line_count, char_count);
                result++;
              }
            }
          }
          // Next line, next char
          else if (
            line_count + 1 < number_of_lines &&
            char_count + 1 < line_length &&
            lines[line_count + 1][char_count + 1] == 'M'
          ) {
            if (
              line_count + 2 < number_of_lines &&
              char_count + 2 < line_length &&
              lines[line_count + 2][char_count + 2] == 'A'
            ) {
              if (
                line_count + 3 < number_of_lines &&
                char_count + 3 < line_length &&
                lines[line_count + 3][char_count + 3] == 'S'
              ) {
                printf("line count : %i, char count: %i\n", line_count, char_count);
                result++;
              }
            }
          }
          // Previous line, same char
          else if (line_count - 1 < number_of_lines && lines[line_count - 1][char_count] == 'M') {
            if (line_count - 2 < number_of_lines && lines[line_count - 2][char_count] == 'A') {
              if (line_count - 3 < number_of_lines && lines[line_count - 3][char_count] == 'S') {
                printf("line count : %i, char count: %i\n", line_count, char_count);
                result++;
              }
            }
          }
          // Previous line, previous char
          else if (
            line_count - 1 < number_of_lines &&
            char_count - 1 > 0 &&
            lines[line_count - 1][char_count - 1] == 'M'
          ) {
            if (
              line_count - 2 < number_of_lines &&
              char_count - 2 > 0 &&
              lines[line_count - 2][char_count - 2] == 'A'
            ) {
              if (
                line_count - 3 < number_of_lines &&
                char_count - 3 > 0 &&
                lines[line_count - 3][char_count - 3] == 'S'
              ) {
                printf("line count : %i, char count: %i\n", line_count, char_count);
                result++;
              }
            }
          }
          // Previous line, next char
          else if (
            line_count - 1 < number_of_lines &&
            char_count + 1 < line_length &&
            lines[line_count - 1][char_count + 1] == 'M'
          ) {
            if (
              line_count - 2 < number_of_lines &&
              char_count + 2 < line_length &&
              lines[line_count - 2][char_count + 2] == 'A'
            ) {
              if (
                line_count - 3 < number_of_lines &&
                char_count + 3 < line_length &&
                lines[line_count - 3][char_count + 3] == 'S'
              ) {
                printf("line count : %i, char count: %i\n", line_count, char_count);
                result++;
              }
            }
          }
        }
      }
    }
    printf("Result: %i\n", result);
  }

  static void second() {
  }
};
