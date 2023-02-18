#pragma once

#include <string>

class FilesystemTools
{
private:
    static void print_directory_recursive(std::filesystem::directory_entry directory, int printing_layer);
    static void count_and_print_files_recursive(std::filesystem::directory_entry directory, int *prev_dirs = nullptr, int *prev_files = nullptr);

public:
    static void print_directory(std::string dir_path);
    static void count_and_print_files(std::string dir_path);
};
