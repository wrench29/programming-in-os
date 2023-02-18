#include <iostream>
#include <filesystem>

#include "lib.hpp"

using std::cout, std::endl;

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

int main()
{
    auto current_path = std::filesystem::current_path().string();

    cout << "Path: " << current_path << endl
         << endl;

    auto listing = list_directory(current_path);

    print_directory_recursive(listing);
    count_and_print_files(listing);

    return 0;
}
