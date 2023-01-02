#include <iostream>
#include <fmt/core.h>

#include "consoleapp.hpp"

int main(int argc, char** argv)
{
	auto op_args { capp::proc_args(argc, argv) };
	auto& args = *op_args;

	if (!op_args.has_value()) {
		return 1;
	}

    return capp::call_puzzle_result(args.day, args.puzzle_number, args.input_file);
}
