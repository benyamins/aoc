#include <fstream>
#include <filesystem>

#include "helper.hpp"

void read_lines(const std::string& input_file, std::vector<std::string>& input_lines)
{
    std::ifstream ifile { input_file };

	// or !ifile.good()
	if (!std::filesystem::exists(input_file))
	{
		throw std::invalid_argument("File not found!");
	}

    for (std::string line; getline(ifile, line);)
    {
        input_lines.push_back(line);
    }
}

std::vector<std::string> str_split(const std::string& str, const std::string& split_on)
{
	std::vector<std::string> split {};
	
	std::size_t start_pos { 0 };
	std::size_t sep_pos { str.find(split_on, start_pos) };
	while (true)
	{
		split.push_back(str.substr(start_pos, sep_pos));
		if (sep_pos == std::string::npos)
			break;
		else
		{
			start_pos = sep_pos + 1;
			sep_pos = str.find(split_on, start_pos);
		}
	}

	return split;
}
