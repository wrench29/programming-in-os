#pragma once

#include <string>
#include <vector>

class BasicFilesystemElement
{
private:
    std::string name;

public:
    BasicFilesystemElement(std::string name) : name(name) {}
    std::string get_name() const { return name; }
};

class DirectoryListing : BasicFilesystemElement
{
public:
    DirectoryListing(std::string name) : BasicFilesystemElement(name) {}
    std::vector<std::string> elements;
};

DirectoryListing list_directory(std::string dir_path);
