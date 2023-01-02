#include <string>
#include <vector>
#include <unordered_map>
#include <fmt/core.h>

#include "helper.hpp"

void puzzle_04_01(std::string input_file)
{
    std::vector<std::string> elf_pair_list;
    read_lines(input_file, elf_pair_list);

    for (const auto& elf_pair : elf_pair_list)
    {
        fmt::print("{} {}, {} {}\n", elf_pair.substr(0, 1), elf_pair.substr(2, 1), elf_pair.substr(4, 1), elf_pair.substr(6, 1));
    }

    fmt::print("solution 04 01: {}\n", 1);
}

void puzzle_04_02(std::string input_file)
{
    std::vector<std::string> input_lines;
    read_lines(input_file, input_lines);

    fmt::print("solution 04 02: {}\n", 2);
}
