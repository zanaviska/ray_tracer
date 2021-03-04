#include <algorithm>
#include <array>
#include <cmath>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

const double eps = 1e-7;

struct point
{
    double x;
    double y;
    double z;
};

point operator-(point lhs, point rhs)
{
    return {lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z};
}

point operator*(point lhs, double rhs)
{
    return {lhs.x * rhs, lhs.y * rhs, lhs.z * rhs};
}

struct triangle
{
    std::array<point, 3> vertexes;
    point &operator[](size_t idx) { return vertexes[idx]; }
};

struct color
{
    unsigned char blue;
    unsigned char green;
    unsigned char red;
};

void save_to_file(const std::vector<std::vector<color>> &image, const std::string &file)
{
    using namespace std;
    size_t width = image.size();
    size_t height = image[0].size();

    size_t filesize = 54 + 3 * width * height;
    array<unsigned char, 14> bmp_file_header = {'B', 'M', 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0};
    array<unsigned char, 40> bmp_info_header = {40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 24};

    bmp_file_header[2] = (unsigned char)(filesize);
    bmp_file_header[3] = (unsigned char)(filesize >> 8);
    bmp_file_header[4] = (unsigned char)(filesize >> 16);
    bmp_file_header[5] = (unsigned char)(filesize >> 24);

    bmp_info_header[4] = (unsigned char)(width);
    bmp_info_header[5] = (unsigned char)(width >> 8);
    bmp_info_header[6] = (unsigned char)(width >> 16);
    bmp_info_header[7] = (unsigned char)(width >> 24);
    bmp_info_header[8] = (unsigned char)(height);
    bmp_info_header[9] = (unsigned char)(height >> 8);
    bmp_info_header[10] = (unsigned char)(height >> 16);
    bmp_info_header[11] = (unsigned char)(height >> 24);

    ofstream fout(file);
    copy(bmp_file_header.begin(), bmp_file_header.end(), ostream_iterator<unsigned char>(fout));
    copy(bmp_info_header.begin(), bmp_info_header.end(), ostream_iterator<unsigned char>(fout));
    std::for_each(image.rbegin(), image.rend(), [&fout](const auto &line) {
        for (auto &i : line)
            fout << i.blue << i.green << i.red;
        for (size_t i = 0; i < 4 - 3 * line.size() % 4; i++)
            fout << 0;
    });
}

int main()
{
    // intersect({point{0, 0, 0}, {0, 0, 1}, {0, 1, 0}}, {100, 0, 0}, {0, 5, 5});
    // return 0;
    std::ifstream fin("cow.obj");
    std::string line;
    std::vector<point> vertexes;
    std::vector<triangle> triangles;
    char c;
    while (fin >> line)
    {
        if (line == "v")
        {
            double x, y, z;
            fin >> x >> y >> z;
            vertexes.push_back({x, y, z});
        }
        if (line == "f")
        {
            size_t ver1, ver2, ver3;
            fin >> ver1 >> c >> c >> ver2 >> ver2 >> c >> c >> ver3 >> ver3 >> line;
            triangles.push_back({{vertexes[ver1], vertexes[ver2], vertexes[ver3]}});
        }
    }
}
