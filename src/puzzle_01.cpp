#include <string>
#include <vector>

#include <fmt/core.h>

#include "helper.hpp"

void puzzle_01_01(std::string input_file)
{
    std::vector<std::string> input_lines;
    
    read_lines(input_file, input_lines);

    int calories{ 0 };
    int highest_calories{ 0 };

    for (const auto& s : input_lines)
    {
        if (s == "")
        {
            calories = 0;
        }
        else
        {
            calories += std::stoi(s);
        }

        if (calories > highest_calories)
            highest_calories = calories;
    }

    fmt::print("solution 01 01: {}\n", highest_calories);
}

void puzzle_01_02(std::string input_file)
{
    std::vector<std::string> input_lines;
    
    read_lines(input_file, input_lines);

    int calories{ 0 };
    int highest_calories{ 0 };

    for (const auto& s : input_lines)
    {
        if (s == "")
        {
            calories = 0;
        }
        else
        {
            calories += std::stoi(s);
        }

        if (calories > highest_calories)
            highest_calories = calories;
    }

    fmt::print("solution 01 01: {}\n", highest_calories);
}
