#pragma once

#include <string>
#include <ostream>

struct point
{
    double x;
    double y;
    double z;
};

bool operator!=(const point lhs, const point rhs);
std::ostream &operator<<(std::ostream &out, point p);
point operator-(point lhs, point rhs);
point operator+(point lhs, point rhs);
point operator*(point lhs, double rhs);
double dot_product(point lhs, point rhs);
point cross_product(point lhs, point rhs);

namespace std
{
string to_string(point p);
point min(point lhs, point rhs);
point max(point lhs, point rhs);
} // namespace std