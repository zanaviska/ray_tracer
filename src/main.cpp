#include <chrono>
#include <cmath>
#include <fstream>
#include <iostream>
// #include <thread>
#include <vector>

#include <file_representation.h>
#include <point.h>
#include <tree.h>
// #include <triangle.h>

using namespace std::chrono;

int main()
{
    tree tr;
    std::ifstream fin("cow.obj");
    std::string line;
    std::vector<point> vertexes;
    char c;
    auto start = high_resolution_clock::now();
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
            // triangles.push_back({{vertexes[ver1 - 1], vertexes[ver2 - 1], vertexes[ver3 - 1]}});
            tr.insert({{vertexes[ver1 - 1], vertexes[ver2 - 1], vertexes[ver3 - 1]}});
        }
    }
    auto read_end = high_resolution_clock::now();
    std::cout << "read and insert in tree took " << duration_cast<milliseconds>(read_end - start).count() << "ms\n";

    const int64_t height = 720;
    const int64_t width = 720;
    std::vector<std::vector<color>> image(width, std::vector<color>(height, {0, 0, 0}));

    const point light = {5, 5, 5};
    const point camera = {3, 10, 0};
    for (int64_t i = 0; i < width; i++)
        for (int64_t j = 0; j < height; j++)
        {
            double res = tr.intersect(camera, {(2.0 * i - width) / width, 0, (j * 2.0 - height) / height}, light);

            if (res == -2) continue;
            unsigned char color = (std::fabs(res) + 0.5) / 1.5 * 255;
            image[i][j] = {0, 0, color};
        }
    auto end = high_resolution_clock::now();
    std::cout << "ray tracing itself took " << duration_cast<milliseconds>(end - read_end).count() << "ms\n";
    save_to_file(image, "output.bmp");
    auto total_end = high_resolution_clock::now();
    std::cout << "write to file took " << duration_cast<milliseconds>(total_end - end).count() << "ms\n";
    std::cout << "the whole program took " << duration_cast<milliseconds>(total_end - start).count() << "ms\n";
}
