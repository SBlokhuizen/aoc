#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
#include <set>

class Point
{
private:
public:
    int row_;
    int col_;
    char symbol_;
    Point(int row, int col, char symbol);
};

Point::Point(int row, int col, char symbol) : row_(row), col_(col), symbol_(symbol)
{
}

std::vector<std::vector<char>> GetCharMatrix(std::string fileName)
{
    std::ifstream file(fileName);
    if (!file.is_open())
    {
        std::cerr << "Error: Unable to open file " << fileName << std::endl;
        return {};
    }

    std::vector<std::vector<char>> data;
    std::string line;
    while (std::getline(file, line))
    {
        std::vector<char> row;
        for (char c : line)
        {
            row.push_back(c);
        }
        data.push_back(row);
    }

    file.close();
    return data;
}

void PrintMatrix(std::vector<std::vector<char>> charMatrix)
{
    for (const auto &row : charMatrix)
    {
        for (char c : row)
        {
            std::cout << c;
        }
        std::cout << std::endl;
    }
}

bool isSymbol(char c)
{
    // Define the set of symbols you want to check for
    const std::string symbols = "!@#$%^&*()-_+=~`[]{}\\|;:'\",<>?/";

    // Check if the character is in the set of symbols
    return symbols.find(c) != std::string::npos;
}

std::vector<Point> GetSymbolIndices(std::vector<std::vector<char>> charMatrix)
{
    int numRows = charMatrix.size();
    int numCols = charMatrix[0].size();
    std::vector<Point> symbolIndices;
    for (size_t i = 0; i < numCols; i++)
    {
        for (size_t j = 0; j < numRows; j++)
        {
            if (isSymbol(charMatrix[i][j]))
            {
                symbolIndices.push_back(Point(i, j, charMatrix[i][j]));
            }
        }
    }
    return symbolIndices;
}
std::vector<Point> CheckIndicesWithNumbers(std::vector<std::vector<char>> charMatrix, std::vector<Point> symbolIndices)
{
    std::vector<Point> indicesWithNumbers;

    for (Point symbol : symbolIndices)
    {
        int numberChars = 0;
        int count = 0;
        for (int i = symbol.row_ - 1; i <= symbol.row_ + 1; i++)
        {
            for (int j = symbol.col_ - 1; j <= symbol.col_ + 1; j++)
            {
                // skip central point
                if (i == symbol.row_ && j == symbol.col_)
                {
                    continue;
                }
                // Check boundaries
                if (i >= 0 && i < charMatrix.size() && j >= 0 && j < charMatrix[0].size())
                {
                    // Check if the cell contains a digit
                    if (std::isdigit(charMatrix[i][j]))
                    {
                        // Count the entire number sequence
                        int number = 0;
                        int k = j; // Start from the current column
                        while (k < charMatrix[0].size() && std::isdigit(charMatrix[i][k]))
                        {
                            number = number * 10 + (charMatrix[i][k] - '0');
                            k++;
                        }
                        count++;   // Increment count for the current symbol
                        j = k - 1; // Move to the last digit of the current number
                    }
                }
            }
        }
        if (count == 2)
        {
            for (int i = symbol.row_ - 1; i <= symbol.row_ + 1; i++)
            {
                for (int j = symbol.col_ - 1; j <= symbol.col_ + 1; j++)
                {
                    // skip central point
                    if (i == symbol.row_ && j == symbol.col_)
                    {
                        continue;
                    }
                    // Check boundaries
                    if (i >= 0 && i < charMatrix.size() && j >= 0 && j < charMatrix[0].size())
                    {
                        // Check if the cell contains a digit
                        if (std::isdigit(charMatrix[i][j]))
                        {
                            indicesWithNumbers.push_back(Point(i, j, charMatrix[i][j]));
                        }
                    }
                }
            }
        }
    }

    return indicesWithNumbers;
}

int GetNumber(int row, int col, std::vector<std::vector<char>> charMatrix, std::vector<Point> &seenIndices)
{
    if (col >= 0)
    {
        char leftValue = charMatrix[row][col];
        int leftNumber;
        if (std::isdigit(leftValue))
        {
            leftNumber = leftValue - '0';
            seenIndices.push_back(Point(row, col, leftNumber));
            return leftNumber;
        }
        else
        {
            return -1;
        }
    }
    else
    {
        return -1;
    }
}

std::string AppendFront(const std::string &firstNumber, const std::string &secondNumber)
{
    // Concatenate the second number in front of the first number
    std::string resultStr = secondNumber + firstNumber;

    return resultStr;
}

std::string AppendBack(const std::string &firstNumber, const std::string &secondNumber)
{
    // Concatenate the second number at the back of the first number
    std::string resultStr = firstNumber + secondNumber;

    return resultStr;
}

bool isPointInVector(const std::vector<Point> &points, int row, int col)
{
    for (const Point &p : points)
    {
        if (p.row_ == row && p.col_ == col)
        {
            return true;
        }
    }
    return false;
}

int SumOfAllAdjecentNumbers(std::vector<int> listOfNumbers)
{
    int sum = 0;
    for (int number : listOfNumbers)
    {
        sum += number;
        std::cout << number << std::endl;
    }
    std::cout << "----" << std::endl;
    return sum;
}

std::vector<int> GetListOfNumbers(std::vector<std::vector<char>> charMatrix, std::vector<Point> indicesWithNumbers)
{
    std::vector<int> listOfNumbers;
    std::vector<Point> seenIndices;
    int tmpNumber;
    int currentRow, currentCol;
    std::string fullNumber;

    for (Point number : indicesWithNumbers)
    {
        currentRow = number.row_;
        currentCol = number.col_;
        tmpNumber = 0;

        // Skip search if already in list
        if (isPointInVector(seenIndices, currentRow, currentCol))
        {
            continue;
        }
        // Search left
        while (tmpNumber != -1)
        {
            // Stop if Point already registered
            if (isPointInVector(seenIndices, currentRow, currentCol))
            {
                break;
            }
            else
            {

                // Get number
                tmpNumber = GetNumber(currentRow, currentCol, charMatrix, seenIndices);

                // If start of
                if (currentRow == number.row_ && currentCol == number.col_)
                {
                    fullNumber = std::to_string(tmpNumber);
                }

                else if (tmpNumber != -1)
                {
                    fullNumber = AppendFront(fullNumber, std::to_string(tmpNumber));
                }
                currentCol--;
            }
        }

        // Search right
        currentRow = number.row_;
        currentCol = number.col_ + 1;
        tmpNumber = 0;

        while (tmpNumber != -1)
        {
            if (isPointInVector(seenIndices, currentRow, currentCol))
            {
                break;
            }
            else
            {
                // Get number
                tmpNumber = GetNumber(currentRow, currentCol, charMatrix, seenIndices);

                if (tmpNumber != -1)
                {
                    fullNumber = AppendBack(fullNumber, std::to_string(tmpNumber));
                }
                currentCol++;
            }
        }
        listOfNumbers.push_back(std::stoi(fullNumber));
        std::cout << fullNumber <<std::endl;
        // seenIndices.clear();
    }

    return listOfNumbers;
}



int GetSumOfNumbers(std::vector<std::vector<char>> charMatrix)
{
    int numRows = charMatrix.size();
    int numCols = charMatrix[0].size();
    int gears = 0;
    std::vector<Point> symbolIndices = GetSymbolIndices(charMatrix);
    // Iterate through the vector in reverse to avoid invalidating iterators
    for (auto it = symbolIndices.rbegin(); it != symbolIndices.rend(); ++it)
    {
        if (it->symbol_ != '*')
        {
            // Erase the element from the vector
            symbolIndices.erase(std::next(it).base()); // .base() converts reverse iterator to forward iterator
        }
    }

    std::vector<Point> indicesWithNumbers = CheckIndicesWithNumbers(charMatrix, symbolIndices);
    std::vector<int> listOfNumbers = GetListOfNumbers(charMatrix, indicesWithNumbers);

    // for (int list : listOfNumbers)
    // {
    //     std::cout <<"a" <<  list << std::endl;
    // }   
    
    for (size_t i = 0; i < listOfNumbers.size() / 2; i++)
    {
        gears += (listOfNumbers[2*i] * listOfNumbers[2*i+1]);
        std::cout << listOfNumbers[2*i] << "," << listOfNumbers[2*i+1] << std::endl;
    }
    std::cout << gears << std::endl;
    return gears;
}

int main(int argc, char const *argv[])
{
    std::string fileName = "day3.csv";
    std::vector<std::vector<char>> charMatrix = GetCharMatrix(fileName);

    int sumNumbers = GetSumOfNumbers(charMatrix);
    // std::cout << sumNumbers << std::endl;
    return 0;
}
