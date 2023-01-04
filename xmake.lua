set_project("advent-of-code-cpp")
add_rules("mode.debug", "mode.release")

add_requires("fmt")

if is_mode("debug") then
    add_cxxflags("-std=c++2b", "-Wall", "-pedantic", "-pedantic-errors", "-Wconversion", "-O1")
else
    add_cxxflags("-std=c++2b", "-Wall", "-Werror", "-pedantic", "-pedantic-errors", "-Wconversion", "-Wextra", "-O2")
end

target("aoc")
	set_kind("binary")
    add_headerfiles("src/*.hpp")
	add_files("src/*.cpp")
	add_packages("fmt")
