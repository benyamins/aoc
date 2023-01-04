#include <string>
#include <vector>
#include <unordered_map>

#include <fmt/core.h>
#include <fmt/ranges.h>

#include "helper.hpp"

void puzzle_04_01(std::string input_file)
{
    std::vector<std::string> elf_pair_list;
    read_lines(input_file, elf_pair_list);

	int pairs_contained { 0 };
    for (const auto& elf_pair : elf_pair_list)
    {
		std::vector<std::string> elf_pair_split = str_split(elf_pair, ",");

		int right_elf_from { std::stoi(str_split(elf_pair_split[0], "-")[0]) };
		int right_elf_to { std::stoi(str_split(elf_pair_split[0], "-")[1]) };
		int left_elf_from { std::stoi(str_split(elf_pair_split[1], "-")[0]) };
		int left_elf_to { std::stoi(str_split(elf_pair_split[1], "-")[1]) };

		if ((right_elf_from >= left_elf_from && right_elf_to <= left_elf_to) ||
			(left_elf_from >= right_elf_from && left_elf_to <= right_elf_to))
			pairs_contained += 1;
    }

    fmt::print("solution 04 01: {}\n", pairs_contained);
}

void puzzle_04_02(std::string input_file)
{
    std::vector<std::string> elf_pair_list;
    read_lines(input_file, elf_pair_list);

	int pairs_intersect { 0 };
    for (const auto& elf_pair : elf_pair_list)
    {
		std::vector<std::string> elf_pair_split = str_split(elf_pair, ",");

		int right_elf_from { std::stoi(str_split(elf_pair_split[0], "-")[0]) };
		int right_elf_to { std::stoi(str_split(elf_pair_split[0], "-")[1]) };
		int left_elf_from { std::stoi(str_split(elf_pair_split[1], "-")[0]) };
		int left_elf_to { std::stoi(str_split(elf_pair_split[1], "-")[1]) };

		if ((right_elf_to >= left_elf_from && right_elf_to <= left_elf_to)
				|| (left_elf_to >= right_elf_from && left_elf_to <= right_elf_to))
		{
			pairs_intersect += 1;
		}
    }

    fmt::print("solution 04 02: {}\n", pairs_intersect);
}
