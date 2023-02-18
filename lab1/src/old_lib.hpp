#pragma once

#include <string>
#include <vector>

class FilesystemElement
{
private:
    std::string path;
    bool is_dir;

public:
    FilesystemElement(std::string path, bool is_directory) : path(path), is_dir(is_directory) {}

    std::string get_path() const { return path; }
    std::string get_name() const
    {
        int pos = path.find_last_of('/') + 1;
        return path.substr(pos, path.length() - 1);
    }
    std::vector<FilesystemElement> elements;
    bool is_directory() const { return is_dir; };
};

FilesystemElement
list_directory(std::string dir_path);