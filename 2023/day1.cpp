#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>

// Function to read CSV file into a vector of strings
std::vector<std::string> readCSV(const std::string &filename)
{
    std::vector<std::string> data;
    std::ifstream file(filename);
    if (!file.is_open())
    {
        std::cerr << "Error opening file: " << filename << std::endl;
        return data;
    }

    std::string line;
    while (std::getline(file, line))
    {
        data.push_back(line);
    }

    file.close();
    return data;
}

int CombineNumbers(int a, int b)
{
    std::string ab = std::to_string(a) + std::to_string(b);
    int combinedNumber = std::stoi(ab);
    return combinedNumber;
}

int SumFirstLastDigit(std::vector<std::string> array)
{
    int firstNumber, lastNumber;
    char firstLetter, lastLetter;
    bool firstFound, lastFound;
    int rowSize;
    int combinedNumber;
    int totalSum = 0;
    for (const auto &row : array)
    {
        rowSize = row.size();
        // std::cout << row << std::endl;
        firstFound = false;
        lastFound = false;
        for (size_t i = 0; i < rowSize; i++)
        {
            firstLetter = row[i];
            lastLetter = row[rowSize - 1 - i];
            if (isdigit(firstLetter) && firstFound == false)
            {
                firstFound = true;
                firstNumber = firstLetter - '0';
            }
            if (isdigit(lastLetter) && lastFound == false)
            {
                lastFound = true;
                lastNumber = lastLetter - '0';
            }
        }
        combinedNumber = CombineNumbers(firstNumber, lastNumber);
        // std::cout << firstNumber << "," << lastNumber << std::endl;
        // std::cout << combinedNumber << std::endl;
        totalSum += combinedNumber;
    }
    return totalSum;
}

int main(int argc, char const *argv[])
{
    std::vector<std::string> test{"ninefourone1",
                                  "pqr3stu8vwx",
                                  "a1b2c3d4e5f",
                                  "treb7uchet"};
    std::vector<std::string> csvData = readCSV("day1.csv");
    int totalSum = SumFirstLastDigit(csvData);
    std::cout << totalSum << std::endl;

    return 0;
}
