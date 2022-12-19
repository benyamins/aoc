#include <optional>
#include <string>

namespace capp
{

struct Args
{
	int day;
	int puzzle_number;
	std::string input_file;
};

void print_help();

std::optional<Args> proc_args(int argc, char** argv);

int call_puzzle_result(int day, int puzzle_number, std::string input_file);

}
