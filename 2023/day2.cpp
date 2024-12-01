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

void RemoveGameID(std::string &line)
{
    int spaceCntr = 0;
    int index = 0;
    for (char letter : line)
    {
        if (letter == ' ')
        {
            spaceCntr++;
        }
        index++;
        if (spaceCntr == 2)
        {
            line = line.erase(0, index);
            return;
        }
    }
}

std::vector<std::string> GetSubstringsBetweenDelimiter(const std::string &input, char delimiter)
{
    std::vector<std::string> substrings;
    std::stringstream ss(input);
    std::string item;

    // Iterate through each substring separated by semicolons
    while (std::getline(ss, item, delimiter))
    {
        // Remove leading and trailing whitespaces from each substring
        size_t firstNonSpace = item.find_first_not_of(" \t");
        size_t lastNonSpace = item.find_last_not_of(" \t");

        if (firstNonSpace != std::string::npos && lastNonSpace != std::string::npos)
        {
            substrings.push_back(item.substr(firstNonSpace, lastNonSpace - firstNonSpace + 1));
        }
    }

    return substrings;
}

bool containsSubstring(const std::string &string, const std::string &substring)
{
    return string.find(substring) != std::string::npos;
}

int extractNumber(const std::string &str)
{
    std::stringstream ss(str);
    int number;
    ss >> number;
    return number;
}

// RGB
std::vector<int> CountMax(std::string substrings)
{
    std::vector<int> rgbCount{0, 0, 0};
    std::vector<int> maxCount{0, 0, 0};
    std::vector<std::string> roundScores;
    int pullNumber;

    roundScores = GetSubstringsBetweenDelimiter(substrings, ',');
    for (std::string roundScore : roundScores)
    {
        pullNumber = extractNumber(roundScore);
        if (containsSubstring(roundScore, "red"))
        {
            rgbCount[0] += pullNumber;
        }
        else if (containsSubstring(roundScore, "green"))
        {
            rgbCount[1] += pullNumber;
        }
        else if ((containsSubstring(roundScore, "blue")))
        {
            rgbCount[2] += pullNumber;
        }
        else
        {
            exit(1);
        }
    }

    return rgbCount;
}

// Maximum RGB vector
std::vector<int> MaxRGB(std::string line)
{
    std::vector<std::string> substrings = GetSubstringsBetweenDelimiter(line, ';');
    std::vector<int> currentRoundRGB{0, 0, 0};
    std::vector<int> maxRGB{0, 0, 0};
    for (std::string substring : substrings)
    {
        currentRoundRGB = CountMax(substring);
        for (size_t i = 0; i < 3; i++)
        {
            if (currentRoundRGB[i] > maxRGB[i])
            {
                maxRGB[i] = currentRoundRGB[i];
            }
        }
    }
    return maxRGB;
}

int SumOfPossibleGames(std::vector<std::string> csv, std::vector<int> maxPossibleRGB)
{
    std::vector<int> maxRGB;

    // For each line
    int gameID = 1;
    int totalGameSum = 0;
    for (std::string line : csv)
    {
        RemoveGameID(line);
        maxRGB = MaxRGB(line);
        if (maxRGB[0] <= maxPossibleRGB[0] && maxRGB[1] <= maxPossibleRGB[1] && maxRGB[2] <= maxPossibleRGB[2])
        {
            totalGameSum += gameID;
            std::cout << "Game " << gameID << " added: " << gameID << " |";
            std::cout << maxRGB[0] << ","
                      << maxRGB[1] << ","
                      << maxRGB[2] << ",";
            std::cout << std::endl;
        }
        else
        {
            std::cout << "Game " << gameID << " not added: " << gameID << " |";
            std::cout << maxRGB[0] << ","
                      << maxRGB[1] << ","
                      << maxRGB[2] << ",";
            std::cout << std::endl;
        }

        gameID++;
    }

    return totalGameSum;
}

int SumOfLeastColors(std::vector<std::string> csv)
{
    std::vector<int> maxRGB;
    int product;

    // For each line
    int totalGameSum = 0;
    for (std::string line : csv)
    {
        RemoveGameID(line);
        maxRGB = MaxRGB(line);
        product = maxRGB[0] * maxRGB[1] * maxRGB[2];
        std::cout << product << std::endl;
        totalGameSum += product;
    }

    return totalGameSum;
}
int main(int argc, char const *argv[])
{
    std::vector<int> maxPossibleRGB{12, 13, 14};
    std::vector<std::string> csvData = readCSV("day2.csv");
    // int sumOfGames = SumOfPossibleGames(csvData, maxPossibleRGB);
    int sumOfGames = SumOfLeastColors(csvData);
    std::cout << sumOfGames << std::endl;
    return 0;
}
