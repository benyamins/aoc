#include <string>
#include <vector>
#include <unordered_map>
#include <fmt/core.h>

#include "helper.hpp"

void puzzle_03_01(std::string input_file)
{
    std::vector<std::string> input_lines;

    read_lines(input_file, input_lines);

    int item_priority{0};

    for (const auto &bp : input_lines)
    {
        /*
            fmt::print("size: {}, line {}, p1: {}, p2: {}\n",
                       bp.size(),
                       bp,
                       bp.substr(0, bp.size() / 2),
                       bp.substr(bp.size() / 2, bp.size()));
        */

        std::string first_compartment{bp.substr(0, bp.size() / 2)};
        std::string second_compartment{bp.substr(bp.size() / 2, bp.size())};

        for (const auto &item : first_compartment)
        {
            if (second_compartment.contains(item) > 0)
            {
                if (item >= 'A' && item <= 'Z')
                    item_priority += item - 'A' + 27;
                else
                    item_priority += item - 'a' + 1;

                break;
            }
        }
    }

    fmt::print("solution 03 01: {}\n", item_priority);
}

void puzzle_03_02(std::string input_file)
{
    std::vector<std::string> rucksacks;

    read_lines(input_file, rucksacks);
    
    int item_priority { 0 };

    for (std::size_t i{0}; i < rucksacks.size(); i += 3)
    {
        for (const auto &item : rucksacks[i])
        {
            if (rucksacks[i+1].contains(item) > 0 && rucksacks[i+2].contains(item) > 0)
            {
                // fmt::print("{}\n", item);
                if (item >= 'A' && item <= 'Z')
                    item_priority += item - 'A' + 27;
                else
                    item_priority += item - 'a' + 1;
                break;
            }
        }
    }

    fmt::print("solution 03 02: {}\n", item_priority);
}