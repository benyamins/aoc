#include <string>
#include <vector>
#include <array>
#include <algorithm>
#include <iterator>

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

std::size_t min_pos_arr(const std::array<int, 3>& arr)
{
    std::size_t min_pos{ 0 };
    int min_value{ arr[0] };
    for (std::size_t i{ 1 }; i < 3; ++i)
    {
        if (arr[i] > min_value)
        {
            continue;
        }
        else
        {
            min_value = arr[i];
            min_pos = i;
        }
    }
    return min_pos;
}

void puzzle_01_02(std::string input_file)
{
    std::vector<std::string> input_lines;
    
    read_lines(input_file, input_lines);

    int calories{ 0 };
    std::array<int, 3> highest_calories{ 0, 0, 0 };
    int sum_highest{ 0 };
    std::size_t min_pos{ 0 };

    for (size_t pos{ 0 }; const auto& s : input_lines)
    {
        if (s != "" || pos + 1 == input_lines.size())
        {
            calories += std::stoi(s);
        }
        else if (s == "" || pos + 1 == input_lines.size())
        {
            fmt::print("pos: {}, cal: {}\n", pos, calories);
            min_pos = min_pos_arr(highest_calories);

            if (calories > highest_calories[min_pos])
            {
                highest_calories[min_pos] = calories;
            }
            calories = 0;
        }
        pos += 1;

    }

    for (const int& ele : highest_calories)
    {
        sum_highest += ele;
    }

    fmt::print("solution 01 02: {}\n", sum_highest);
}
