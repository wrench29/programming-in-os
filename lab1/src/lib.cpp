#include <iostream>
#include <filesystem>
#include <string>
#include <vector>

#include "lib.hpp"

namespace fs = std::filesystem;

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
