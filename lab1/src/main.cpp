#include <iostream>
#include <filesystem>

#include "lib.hpp"

using std::cout, std::endl;

int main()
{
    auto current_path = std::filesystem::current_path().string();

    cout << "Path: " << current_path << endl
         << endl;

    FilesystemTools::print_directory(current_path);
    FilesystemTools::count_and_print_files(current_path);

    return 0;
}
