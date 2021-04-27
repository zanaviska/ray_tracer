#pragma once

#include <vector>
#include <string>

struct color
{
    unsigned char blue;
    unsigned char green;
    unsigned char red;
};

const bool operator==(const color lhs, const color rhs);
void save_to_file(const std::vector<std::vector<color>> &image, const std::string &file);