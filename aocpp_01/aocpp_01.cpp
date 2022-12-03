#include <iostream>
#include <fstream>
#include <ranges>
#include <iomanip>
#include <algorithm>
#include <string_view>
#include <experimental/filesystem>

struct Line {
    operator std::string_view() const noexcept { return m_line; }
    operator std::string() const noexcept { return m_line; }
    operator std::string&() noexcept { return m_line; }
    operator const std::string&() const noexcept { return m_line; }

    std::string_view as_string_view() const noexcept { return m_line; }

    std::string m_line;
};

std::istream& operator>>(std::istream& is, Line& line) {
    std::istream::sentry sentry(is);
    for (char c = is.get(); is; c = is.get())
    {
        if (c == '\n')
        {
            break;
        }
        line.m_line += c;
    }
    // may need to trim trailing whitespace...
    if (is.flags() & std::ios_base::skipws)
        while (!line.m_line.empty() && std::isspace(line.m_line.back()))
            line.m_line.pop_back();
    return is;
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
    auto input_view = std::ranges::istream_view<Line>(input_stream);

    std::ranges::for_each(
        std::views::counted(input_view.begin(), 5)
            | std::views::filter([](auto& s){ return true; }),
        [](auto& s){ std::cout << std::quoted(s.as_string_view(), '"') << ' '; }
    );

    return 0;
}
