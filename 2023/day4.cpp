#include <iostream>
#include <vector>
#include <sstream>
#include <fstream>
#include <string>

std::string readNthLine(const std::string &filename, int n)
{
    std::ifstream file(filename);
    std::string line;

    if (!file.is_open())
    {
        std::cerr << "Error: Unable to open file " << filename << std::endl;
        return "";
    }

    // Read lines until reaching the nth line
    for (int i = 1; i <= n; ++i)
    {
        if (!std::getline(file, line))
        {
            file.close();
            return ""; // Return empty string if file ends before reaching nth line
        }
    }

    file.close();
    return line;
}

// Function to split string into vector of integers
std::vector<int> splitToInt(const std::string &s)
{
    std::vector<int> result;
    std::istringstream iss(s);
    int num;
    while (iss >> num)
    {
        result.push_back(num);
    }
    return result;
}

// Function to split the input string into two vectors before and after '|'
std::pair<std::vector<int>, std::vector<int>> splitString(const std::string &str)
{
    std::pair<std::vector<int>, std::vector<int>> result;

    // Find the position of '|'
    size_t pos = str.find('|');
    size_t start = str.find(':') + 1;
    if (pos != std::string::npos)
    {
        // Extract substring before '|' and after '|'
        std::string before = str.substr(start, pos - start);
        std::string after = str.substr(pos + 1);

        // Split the substrings into vectors of integers
        result.first = splitToInt(before);
        result.second = splitToInt(after);
    }

    return result;
}

int GetScore(int winningCount)
{
    int powerOfTwo = 1;
    for (size_t i = 1; i < winningCount; i++)
    {
        powerOfTwo = powerOfTwo * 2;
    }
    return powerOfTwo;
}

int GetNumberWinningNumbers(std::vector<int> winningNumbers, std::vector<int> numbersYouHave)
{
    int totalWin = 0;
    for (int winningNumber : winningNumbers)
    {
        for (int numberYouHave : numbersYouHave)
        {
            if (numberYouHave == winningNumber)
            {
                totalWin++;
            }
        }
    }
    return totalWin;
}

int CountLines(const std::string &filename)
{
    std::ifstream file(filename);
    std::string line;
    int lineCount = 0;

    if (!file.is_open())
    {
        std::cerr << "Error: Unable to open file " << filename << std::endl;
        return -1; // Return -1 to indicate error
    }

    // Count lines in the file
    while (std::getline(file, line))
    {
        lineCount++;
    }

    file.close();
    return lineCount;
}

int SumOfWinningGames(std::string fileName)
{
    int numLines = CountLines(fileName);
    int scorePerGame, winningNumbers;
    std::string line;
    std::pair<std::vector<int>, std::vector<int>> result;
    int sumScores = 0;

    for (size_t i = 1; i <= numLines; i++)
    {
        line = readNthLine(fileName, i);
        result = splitString(line);
        winningNumbers = GetNumberWinningNumbers(result.first, result.second);
        if (winningNumbers == 0)
        {
            scorePerGame = 0;
        }
        else
        {
            scorePerGame = GetScore(winningNumbers);
        }
        sumScores += scorePerGame;
    }
    return sumScores;
}
int main()
{
    std::string fileName = "day4.txt";
    int totalSum = SumOfWinningGames(fileName);
    std::cout << totalSum << std::endl;

    // std::cout << "Before: ";
    // for (int num : result.first)
    // {
    //     std::cout << num << " ";
    // }
    // std::cout << std::endl;

    // std::cout << "After: ";
    // for (int num : result.second)
    // {
    //     std::cout << num << " ";
    // }
    // std::cout << std::endl;

    return 0;
}
