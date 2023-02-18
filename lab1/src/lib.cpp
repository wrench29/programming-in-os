#include <iostream>
#include <filesystem>
#include <string>
#include <vector>

#include "lib.hpp"

void FilesystemTools::count_and_print_files_recursive(std::filesystem::directory_entry directory, int *prev_dirs, int *prev_files)
{
    bool is_first = false;
    if (prev_dirs == nullptr && prev_files == nullptr)
    {
        is_first = true;
        prev_dirs = new int;
        *prev_dirs = 0;
        prev_files = new int;
        *prev_files = 0;
    }
    for (auto &el : std::filesystem::directory_iterator(directory))
    {
        if (el.is_directory())
        {
            (*prev_dirs)++;
            count_and_print_files_recursive(el, prev_dirs, prev_files);
        }
        else
        {
            (*prev_files)++;
        }
    }
    if (is_first)
    {
        std::cout << "Files: " << *prev_files << ", Folders: " << *prev_dirs << std::endl;
        delete prev_dirs;
        delete prev_files;
    }
}

void FilesystemTools::print_directory_recursive(std::filesystem::directory_entry directory, int printing_layer)
{
    for (const auto &entry : std::filesystem::directory_iterator(directory))
    {
        for (int i = 0; i < printing_layer; i++)
            std::cout << "| ";

        std::cout << entry.path().filename().string();
        if (entry.is_directory())
            std::cout << "/";

        std::cout << std::endl;

        if (entry.is_directory())
        {
            print_directory_recursive(entry, printing_layer + 1);
        }
    }

    if (printing_layer != 0)
    {
        for (int i = 0; i < printing_layer - 1; i++)
            std::cout << "| ";
        std::cout << "╰---" << std::endl;
    }
}

void FilesystemTools::print_directory(std::string dir_path)
{
    auto entry = std::filesystem::directory_entry(dir_path);
    print_directory_recursive(entry, 0);
}
void FilesystemTools::count_and_print_files(std::string dir_path)
{
    auto entry = std::filesystem::directory_entry(dir_path);
    count_and_print_files_recursive(entry);
}
