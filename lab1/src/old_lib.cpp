#include <filesystem>
#include <iostream>

#include "old_lib.hpp"

namespace fs = std::filesystem;
using std::cout, std::endl;

FilesystemElement list_directory(std::string dir_path)
{
    FilesystemElement directory_listing(dir_path, true);
    for (const auto &entry : fs::directory_iterator(dir_path))
    {
        if (entry.is_directory())
        {
            FilesystemElement dir = list_directory(entry.path());
            directory_listing.elements.push_back(dir);
        }
        else
        {
            FilesystemElement element(entry.path(), false);
            directory_listing.elements.push_back(element);
        }
    }

    return directory_listing;
}

void count_and_print_files(FilesystemElement directory, int *prev_dirs = nullptr, int *prev_files = nullptr)
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
    for (auto &el : directory.elements)
    {
        if (el.is_directory())
        {
            (*prev_dirs)++;
            count_and_print_files(el, prev_dirs, prev_files);
        }
        else
        {
            (*prev_files)++;
        }
    }
    if (is_first)
    {
        cout << "Files: " << *prev_files << ", Folders: " << *prev_dirs << endl;
        delete prev_dirs;
        delete prev_files;
    }
}

void print_directory_recursive(FilesystemElement listing, int layer = 0)
{
    for (auto &el : listing.elements)
    {
        for (int i = 0; i < layer; i++)
            cout << "| ";

        bool is_dir = el.is_directory();
        cout << el.get_name();

        if (is_dir)
            cout << "/";

        cout << endl;

        if (is_dir)
            print_directory_recursive(el, layer + 1);
    }
    if (layer != 0)
    {
        for (int i = 0; i < layer - 1; i++)
            cout << "| ";
        cout << "╰---" << endl;
    }
}
