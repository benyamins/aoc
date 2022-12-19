#include <iostream>
#include <vector>
#include <optional>
#include <unordered_map>
#include <functional>

#include <fmt/core.h>

#include "consoleapp.hpp"
#include "puzzle.hpp"


const char* HELP {
R"(
   _      _             _          __    ___         _     
  /_\  __| |_ _____ _ _| |_   ___ / _|  / __|___  __| |___ 
 / _ \/ _` \ V / -_) ' \  _| / _ \  _| | (__/ _ \/ _` / -_)
/_/ \_\__,_|\_/\___|_||_\__| \___/_|    \___\___/\__,_\___|
puzzle completions in `c++`

usage:
    aoc [ day ] [ puzzle-number ]
    aoc --help
)"
};


void capp::print_help()
{
	fmt::print("{}", HELP);
}


std::optional<capp::Args> capp::proc_args(int argc, char** argv)
{
	if (argc != 4)
	{
		fmt::print("Wrong number of parameters."
                   " Only 2 positve integers and an input file requiered\n");
		print_help();
		return {};
	}

	Args args {};

	try
	{
		args.day = std::atoi(argv[1]);
		args.puzzle_number = std::atoi(argv[2]);
		args.input_file = argv[3];
	}
	catch (const std::exception& e)
    {
		fmt::print("{}", e.what());
		return {};
	}

	if (args.day < 1 || args.puzzle_number < 1 || args.puzzle_number > 3)
	{
		fmt::print("Invalid numbers for day and puzzle number\n");
		return {};
	}

	return args;
}


std::unordered_map<int, std::unordered_map<int, std::function<void(std::string)>>>
puzzle_funcs {
    {1, { {1, puzzle_01_01} }}
};


int capp::call_puzzle_result(int day, int puzzle_number, std::string input_file)
{
    puzzle_funcs[day][puzzle_number](input_file);
    return 0;
}

