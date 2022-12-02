#include <iostream>
#include <fstream>
#include <ranges>
#include <iomanip>
#include <algorithm>
#include <string_view>
#include <experimental/filesystem>

template<class T>
constexpr std::string read_all(std::basic_istream<T>& is)
{
    std::istreambuf_iterator<T> it{is}, end;
    std::string s{it, end};
    return s;
}

int main(int argc, char* argv[])
{
    if(argc < 2) {
        std::cerr << "Expected a path to input data as an argmument" << std::endl;
        return 1;
    }

    auto input_filepath = std::experimental::filesystem::path(argv[1]);

    if(!std::experimental::filesystem::exists(input_filepath)) {
        std::cerr << "File not found" << std::endl;
        return 1;
    }

    if(std::experimental::filesystem::is_directory(input_filepath)) {
        std::cerr << "Expected a file, not a directory" << std::endl;
        return 1;
    }

    // Create an input stream over the file
    auto input_stream = std::ifstream(input_filepath.c_str());
    auto input = read_all(input_stream);
    auto input_view = std::string_view(input);

    std::cout << input_view << std::endl;

    // constexpr std::string_view delim{"\n"};
    // auto splitter = std::ranges::views::split(input_view, delim);
    
    // Iterate over the input
    // std::ranges::for_each(
    //     splitter,
    //     [](std::string& s){ std::cout << std::quoted(s, '"') << ' '; }
    // );
    // std::cout << '\n';

    // for (std::string_view s: splitter) {
    //     std::cout << s << ' ';
    // }
    // std::cout << '\n';

    return 0;
}
