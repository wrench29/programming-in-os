#include <iostream>
#include <filesystem>

#include "lib.hpp"

using std::cout, std::endl;

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

    return 0;
}
