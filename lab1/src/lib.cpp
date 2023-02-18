#include <iostream>
#include <vector>

#include "lib.hpp"

DirectoryListing list_directory(std::string dir_path)
{
    return DirectoryListing(dir_path);
}
