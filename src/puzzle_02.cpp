#include <string>
#include <vector>
#include <unordered_map>
#include <fmt/core.h>

#include "helper.hpp"
/*
    A-X: ROCK (1)
    B-Y: PAPER (2)
    C-Z: SCISSOR (3)
    l: 0, d: 3, w: 6
 */

// should be of the form `{ "A X", 4 }`.
std::unordered_map<char, std::unordered_map<char, int>>
    result_map
{
        {
            'A', { {'X', 4}, {'Y', 8}, {'Z', 3} }
        },
        {
            'B', { {'X', 1}, {'Y', 5}, {'Z', 9} }
        },
        {
            'C', { {'X', 7}, {'Y', 2}, {'Z', 6} }
        }
};

void puzzle_02_01(std::string input_file)
{
    std::vector<std::string> input_lines;

    read_lines(input_file, input_lines);

    int score {};

    for (auto& game : input_lines)
        score += result_map[game[0]][game[2]];

    fmt::print("solution 02 01: {}\n", score);
}

std::unordered_map<char, std::unordered_map<char, int>>
    result_map2
{
        {
            'A', { {'X', 3}, {'Y', 4}, {'Z', 8} }
        },
        {
            'B', { {'X', 1}, {'Y', 5}, {'Z', 9} }
        },
        {
            'C', { {'X', 2}, {'Y', 6}, {'Z', 7} }
        }
};

void puzzle_02_02(std::string input_file)
{
  std::vector<std::string> input_lines;

  read_lines(input_file, input_lines);

  int score {};

  for (auto& game : input_lines)
    score += result_map2[game[0]][game[2]];

  fmt::print("solution 02 02: {}\n", score);
}
