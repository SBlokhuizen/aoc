#include <iostream>
#include <vector>
#include <sstream>
#include <fstream>
#include <string>

std::pair<std::vector<int>, std::vector<int>> ReadDataFromFile(const std::string &file_path)
{
    std::ifstream file(file_path);
    std::vector<int> time;
    std::vector<int> distance;
    std::string line;

    while (std::getline(file, line))
    {
        std::istringstream iss(line);
        std::string label;
        int value;

        iss >> label;
        if (label == "Time:")
        {
            while (iss >> value)
            {
                time.push_back(value);
            }
        }
        else if (label == "Distance:")
        {
            while (iss >> value)
            {
                distance.push_back(value);
            }
        }
    }

    return std::make_pair(time, distance);
}

int NumberOfWinningTimes(int time, int distanceToBeat)
{
    int distance;
    int numberWinningTimes = 0;
    // Skip 0 and time
    for (size_t i = 1; i < time; i++)
    {
        distance = (time - i) * i;
        if (distance > distanceToBeat)
        {
            numberWinningTimes++;
        }
    }
    return numberWinningTimes;
}

int ProductOfWinningTimes(std::string filename)
{
    auto data = ReadDataFromFile(filename);
    std::vector<int> time = data.first;
    std::vector<int> distance = data.second;
    int numberWinningTimes = 0;
    int productWinningGames = 1;
    for (size_t i = 0; i < time.size(); i++)
    {
        numberWinningTimes = NumberOfWinningTimes(time[i], distance[i]);
        productWinningGames *= numberWinningTimes;
        std::cout << numberWinningTimes << std::endl;
    }

    // // Printing the vectors
    // std::cout << "Time: ";
    // for (int t : time)
    // {
    //     std::cout << t << " ";
    // }
    // std::cout << std::endl;

    // std::cout << "Distance: ";
    // for (int d : distance)
    // {
    //     std::cout << d << " ";
    // }
    // std::cout << std::endl;

    return productWinningGames;
}

int main(int argc, char const *argv[])
{
    std::string filename = "day6.txt";
    int product = ProductOfWinningTimes(filename);
    std::cout << product << std::endl;
    return 0;
}
