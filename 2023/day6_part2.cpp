#include <iostream>
#include <vector>
#include <sstream>
#include <fstream>
#include <string>

std::pair<long long int, long long int> ReadDataFromFile(const std::string &file_path)
{
    std::ifstream file(file_path);
    std::pair<long long int, long long int> result;
    std::string line;

    while (std::getline(file, line))
    {
        std::istringstream iss(line);
        std::string label;
        std::string concatenated_numbers;

        iss >> label; // Extract label, but we don't need it for this version

        // Concatenate all numbers without spaces
        std::string number;
        while (iss >> number)
        {
            concatenated_numbers += number;
        }

        // Store the concatenated numbers as integers in result
        if (label == "Time:")
            result.first = std::stoll(concatenated_numbers);
        else if (label == "Distance:")
            result.second = std::stoll(concatenated_numbers);
    }

    return result;
}

long long int NumberOfWinningTimes(long long int time,long long int distanceToBeat)
{
    long long int distance;
    long long int numberWinningTimes = 0;
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

long long int ProductOfWinningTimes(std::string filename)
{
    auto data = ReadDataFromFile(filename);
    long long int time = data.first;
    long long int distance = data.second;
    std::cout << time << "," << distance << std::endl;

    long long int numberWinningTimes = 0;
    long long int productWinningGames = 1;

    numberWinningTimes = NumberOfWinningTimes(time, distance);
    productWinningGames *= numberWinningTimes;
    std::cout << numberWinningTimes << std::endl;

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
    long long int product = ProductOfWinningTimes(filename);
    std::cout << product << std::endl;
    return 0;
}
