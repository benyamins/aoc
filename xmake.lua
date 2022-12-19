set_project("advent-of-code-cpp")
add_rules("mode.debug", "mode.release")

add_requires("fmt")

if is_mode("debug") then
	add_cxxflags("-std=c++20", "-Wall", "-pedantic", "-pedantic-errors", "-Wconversion")
else
	add_cxxflags("-std=c++20", "-Wall", "-Werror", "-pedantic", "-pedantic-errors", "-Wconversion", "-Wextra")
end

target("aoc")
	set_kind("binary")
    add_headerfiles("src/consoleapp.hpp")
    add_headerfiles("src/puzzle.hpp")
	add_files("src/main.cpp")
	add_files("src/consoleapp.cpp")
    add_files("src/puzzle_01.cpp")
	add_packages("fmt")
