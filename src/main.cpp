#include <algorithm>
#include <array>
#include <cmath>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <iomanip>
#include <chrono>
#include <thread>

using namespace std::chrono;

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

point operator+(point lhs, point rhs)
{
    return {lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z};
}

point operator*(point lhs, double rhs)
{
    return {lhs.x * rhs, lhs.y * rhs, lhs.z * rhs};
}

std::ostream& operator <<(std::ostream& out, point p)
{
    return out << '{' << p.x << ' ' << p.y << ' ' << p.z << '}';
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
    /*std::for_each(image.rbegin(), image.rend(), [&fout](const auto &line) {
        for (auto &i : line)
            fout << i.blue << i.green << i.red;
        std::cout << line.size() << '\n';
        for (size_t i = 0; i < (4 - 3 * line.size() % 4)%4; i++)
            fout << 0;
    });*/
    for(int j = 0; j < image[0].size(); j++)
    {
        for(int i = 0; i < image.size(); i++)
            fout << image[i][j].blue << image[i][j].green << image[i][j].red;
        for (size_t i = 0; i < (4 - 3 * image.size() % 4)%4; i++)
            fout << 0;
    }
}

double sqr(double arg)
{
    return arg*arg;
}

point cross_product(point lhs, point rhs)
{
    return {lhs.y * rhs.z - lhs.z * rhs.y, lhs.z * rhs.x - lhs.x * rhs.z, lhs.x * rhs.y - lhs.y * rhs.x};
}

double area(triangle arg)
{
    auto diamond = cross_product(arg[0]-arg[1], arg[0]-arg[2]);
    return sqrtl(sqr(diamond.x) + sqr(diamond.y) + sqr(diamond.z))/2;
}

double dot_product(point lhs, point rhs)
{
    return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z;
}

//first -- distance to intersect
//second -- degree of intersect
std::pair<double, double> intersect(triangle trik, point start, point middle)
{
    point plane_normal = cross_product((trik[0] - trik[1]), (trik[2] - trik[1]));
    point ray_normal = middle - start;

    // does ray and plane intersect?
    double prod1 = dot_product(plane_normal, ray_normal);
    if (std::fabs(prod1) < eps) return {10000, -2};

    // find intersect of ray and plane
    double t = dot_product(plane_normal, trik[0] - start) / prod1;
    point inter = ray_normal * t + start;


    // does triangle contain intersect
    if (std::fabs(area(trik) - area({trik[0], trik[1], inter}) - area({trik[0], trik[2], inter}) - area({trik[2], trik[1], inter})) > eps) 
        return {10000, -2};
    // std::cout << "------------------------------------------------------------\n";
    // std::cout << start << '\n' << middle << '\n';
    // std::cout << std::setprecision(15) << std::fabs(area(trik) - area({trik[0], trik[1], inter}) - area({trik[0], trik[2], inter}) - area({trik[2], trik[1], inter})) << '\n';
    // std::cout << trik[0] << ' ' << trik[1] << ' ' << trik[2] << '\n';
    // std::cout << "intersect: (" << inter.x << ' ' << inter.y << ' ' << inter.z << ")\n";
    // std::cout << area(trik) << ' ' << area({trik[0], trik[1], inter})  << ' ' << area({trik[0], trik[2], inter})  << ' ' <<  area({trik[2], trik[1], inter}) << '\n';
    // std::cin.get();
    
    return {std::sqrt(sqr(start.x - inter.x) + sqr(start.y - inter.y) + sqr(start.z - inter.z)), std::fabs(prod1)/(sqrt(sqr(plane_normal.x) + sqr(plane_normal.y) + sqr(plane_normal.z))*sqrt(sqr(ray_normal.x) + sqr(ray_normal.y) + sqr(ray_normal.z)))};
}

int main()
{
    std::ifstream fin("cow.obj");
    std::string line;
    std::vector<point> vertexes;
    std::vector<triangle> triangles;
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
            triangles.push_back({{vertexes[ver1-1], vertexes[ver2-1], vertexes[ver3-1]}});
            if(triangles.size() == 4047) std::cout << ver1 << ' ' << ver2 << ' ' << ver3 << '\n';
        }
    }
    const int64_t height = 720;
    const int64_t width = 720;
    std::vector<std::vector<color>> image(width, std::vector<color>(height, {0, 0, 0}));
    std::cout << "image was generated\n";
    const size_t proc_num = 11;
    std::vector<std::thread> thrds;
    for(int64_t i = 0; i < width; i++)
        thrds.emplace_back([&triangles,&image, i]
        {
            for(int64_t j = 0; j < height; j++)
            {
                std::pair<double, double> res = {100000, -2};
                // std::cout << point({(2.0*i - height)/height, (j*2.0 - width)/width, 0}) << '\n';
                // for(int o = 0; o < triangles.size(); o++)
                for(auto &trik: triangles)
                {
                    // auto trik = triangles[o];
                    // std::cout << o << '\n';
                    auto inter = intersect(trik, {0, 100, 00}, {(2.0*i - width)/width, 0, (j*2.0 - height)/height});
                    res = std::min(res, inter);
                    // image[i][j] = {(inter+2)*100, (inter+2)*100, (inter+2)*100};
                }
                if(res.second == -2) res.second = -1;
                unsigned char color = (1-std::fabs(res.second))*255;
                image[i][j] = {color, color, color};
                // image[i][j] = {(unsigned char)(255*(res.second != -2)), (unsigned char)(255*(res.second != -2)), (unsigned char)(255*(res.second != -2))};
                // image[i][j] = {(unsigned char)((res.second + 2)*80), (unsigned char)((res.second + 2)*80), (unsigned char)((res.second + 2)*80)};


                // std::cout << i << ' ' << j << ' ' << (2.0*i - height)/height << ' ' << res.first << ' ' << res.second << '\n';

                // std::cin.get();
            }
        });
    for(auto &i: thrds)
        i.join();
    auto end = high_resolution_clock::now();
    std::cout << duration_cast<seconds>(end-start).count() << '\n';
    save_to_file(image, "output.bmp");
}
