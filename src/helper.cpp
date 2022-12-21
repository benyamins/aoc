#include <fstream>

#include "helper.hpp"

void read_lines(const std::string& input_file, std::vector<std::string>& input_lines)
{
    std::ifstream ifile { input_file };

    for (std::string line; getline(ifile, line);)
    {
        input_lines.push_back(line);
    }
}

