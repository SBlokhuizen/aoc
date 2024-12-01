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

std::vector<std::string> findStringsStartingWithSubstring(const std::string &substr)
{
    std::vector<std::string> strings = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
    std::vector<std::string> matches;
    for (const std::string &str : strings)
    {
        if (str.substr(0, substr.length()) == substr)
        {
            matches.push_back(str);
        }
    }
    return matches;
}

std::vector<std::string> findStringsEndingWithSubstring(const std::string &substr)
{
    std::vector<std::string> strings = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
    std::vector<std::string> matches;
    for (const std::string &str : strings)
    {
        // Check if the substring exists at the end of the string
        if (str.length() >= substr.length() && str.rfind(substr) == str.length() - substr.length())
        {
            matches.push_back(str);
        }
    }
    return matches;
}

bool StringExistsInVector(const std::string &target, const std::vector<std::string> &strings)
{
    for (const auto &str : strings)
    {
        if (str == target)
        {
            return true;
        }
    }
    return false;
}

int ConvertNumberWordToInt(const std::string &word)
{
    if (word == "zero")
    {
        return 0;
    }
    else if (word == "one")
    {
        return 1;
    }
    else if (word == "two")
    {
        return 2;
    }
    else if (word == "three")
    {
        return 3;
    }
    else if (word == "four")
    {
        return 4;
    }
    else if (word == "five")
    {
        return 5;
    }
    else if (word == "six")
    {
        return 6;
    }
    else if (word == "seven")
    {
        return 7;
    }
    else if (word == "eight")
    {
        return 8;
    }
    else if (word == "nine")
    {
        return 9;
    }
    else
    {
        return -1; // Indicate invalid input
    }
}

int FindFirstNumber(std::string row)
{
    int rowSize = row.size();
    char firstLetter;
    int firstNumber;
    std::string currentWord;
    std::vector<std::string> possibleNumbers;

    for (size_t i = 0; i < rowSize; i++)
    {
        firstLetter = row[i];
        if (isdigit(firstLetter))
        {
            firstNumber = firstLetter - '0';
            return firstNumber;
        }
        else
        {
            currentWord.push_back(firstLetter);
            // std::cout << currentWord << std::endl;
            possibleNumbers = findStringsStartingWithSubstring(currentWord);
            if (possibleNumbers.empty())
            {
                currentWord.erase(0, 1);
            }
            else if (StringExistsInVector(currentWord, possibleNumbers))
            {
                if (possibleNumbers.size() == 1)
                {
                    firstNumber = ConvertNumberWordToInt(currentWord);
                    return firstNumber;
                }
            }
        }
    }
    std::cout << "error" << std::endl;
    return -1;
}

int FindLastNumber(std::string row)
{
    int rowSize = row.size();
    char lastLetter;
    int lastNumber;
    std::string currentWord;
    std::vector<std::string> possibleNumbers;

    for (size_t i = 0; i < rowSize; i++)
    {
        // First letter
        lastLetter = row[rowSize - 1 - i];
        if (isdigit(lastLetter))
        {
            lastNumber = lastLetter - '0';
            return lastNumber;
        }
        else
        {
            currentWord.insert(currentWord.begin(), lastLetter);
            possibleNumbers = findStringsEndingWithSubstring(currentWord);
            if (possibleNumbers.empty())
            {
                currentWord.pop_back();
            }
            else if (StringExistsInVector(currentWord, possibleNumbers))
            {
                if (possibleNumbers.size() == 1)
                {
                    lastNumber = ConvertNumberWordToInt(currentWord);
                    return lastNumber;
                }
            }
        }
    }
    std::cout << "error" << std::endl;
    return -1;
}

int SumFirstLastDigit(std::vector<std::string> array)
{
    int totalSum = 0;
    int firstNumber, lastNumber, combinedNumber;

    for (const auto &row : array)
    {
        // Find numbers
        firstNumber = FindFirstNumber(row);
        lastNumber = FindLastNumber(row);

        // Combine numbers
        combinedNumber = CombineNumbers(firstNumber, lastNumber);
        totalSum += combinedNumber;
    }
    return totalSum;
}

int main(int argc, char const *argv[])
{
    std::vector<std::string> csvData = readCSV("day1.csv");
    int totalSum = SumFirstLastDigit(csvData);
    std::cout << totalSum << std::endl;

    return 0;
}
