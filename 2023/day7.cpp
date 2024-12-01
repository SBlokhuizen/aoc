#include <iostream>
#include <vector>
#include <sstream>
#include <fstream>
#include <string>
#include <unordered_map>
#include <map>
#include <optional>

const std::map<char, int> rankValues = {
    {'A', 14}, {'K', 13}, {'Q', 12}, {'J', 11}, {'T', 10}, {'9', 9}, {'8', 8}, {'7', 7}, {'6', 6}, {'5', 5}, {'4', 4}, {'3', 3}, {'2', 2}};

struct PokerHand
{
    std::string hand;
    int score;

    // Constructor with initializer list
    PokerHand(const std::string &stringValue, int intValue)
        : hand(stringValue), score(intValue) {}
};

struct HandTypes
{
    std::vector<PokerHand> highCards;
    std::vector<PokerHand> onePairs;
    std::vector<PokerHand> twoPairs;
    std::vector<PokerHand> threeOfAKinds;
    std::vector<PokerHand> fullHouses;
    std::vector<PokerHand> fourOfAKinds;
    std::vector<PokerHand> fiveOfAKinds;
};

PokerHand SplitStringAndInt(const std::string &input)
{
    std::istringstream iss(input);
    std::string strPart;
    int intPart;

    iss >> strPart >> intPart;

    return PokerHand(strPart, intPart);
}

void FillHands(std::vector<PokerHand> &pokerHands, std::string filename)
{
    std::ifstream file(filename);
    std::string line;
    while (std::getline(file, line))
    {
        PokerHand pokerHand = SplitStringAndInt(line);
        pokerHands.push_back(pokerHand);
    }
}

bool ContainsNSameNumbers(const std::string &str, int N, std::string &remainder)
{
    std::unordered_map<char, int> countMap;
    for (char ch : str)
    {
        countMap[ch]++;
        if (countMap[ch] == N)
        {
            remainder.clear();
            for (char c : str)
            {
                if (countMap[c] != N)
                {
                    remainder += c;
                }
            }
            return true;
        }
    }
    return false;
}

int GetHandType(PokerHand pokerHand)
{
    int handType;
    std::string remainder;
    // five of a kind
    if (ContainsNSameNumbers(pokerHand.hand, 5, remainder))
    {
        std::cout << "five of a kind " << std::endl;
        return 6;
    }
    // four of a kind
    else if (ContainsNSameNumbers(pokerHand.hand, 4, remainder))
    {
        std::cout << "four of a kind " << std::endl;
        return 5;
    }
    // three of a kind / full house
    else if (ContainsNSameNumbers(pokerHand.hand, 3, remainder))
    {
        if (ContainsNSameNumbers(remainder, 2, remainder))
        {

            std::cout << "full house" << std::endl;
            return 4;
            /* code */
        }
        else
        {
            std::cout << "three of a kind" << std::endl;

            return 3;
        }
    }
    // one pair / two pair
    else if (ContainsNSameNumbers(pokerHand.hand, 2, remainder))
    {
        if (ContainsNSameNumbers(remainder, 2, remainder))
        {

            std::cout << "two pair" << std::endl;
            return 2;
        }
        else
        {
            std::cout << "one pair" << std::endl;

            return 1;
        }
        std::cout << "one pair / two pair " << std::endl;
    }
    std::cout << "high card " << std::endl;

    return 0;
    // 0 : high card
    // 1 : one pair
    // 2 : two pair
    // 3 : three of a kind
    // 4 : full house
    // 5 : four of a kind
    // 6 : five of a kind
}

void FilterHandByType(HandTypes &handTypes, const std::vector<PokerHand> &pokerHands)
{
    int handType;
    for (auto currentHand : pokerHands)
    {
        std::cout << currentHand.hand << "," << currentHand.score << std::endl;
        handType = GetHandType(currentHand);
        if (handType == 0)
        {
            handTypes.highCards.push_back(currentHand);
        }
        else if (handType == 1)
        {
            handTypes.onePairs.push_back(currentHand);
        }
        else if (handType == 2)
        {
            handTypes.twoPairs.push_back(currentHand);
        }
        else if (handType == 3)
        {
            handTypes.threeOfAKinds.push_back(currentHand);
        }
        else if (handType == 4)
        {
            handTypes.fullHouses.push_back(currentHand);
        }
        else if (handType == 5)
        {
            handTypes.fourOfAKinds.push_back(currentHand);
        }
        else if (handType == 6)
        {
            handTypes.fiveOfAKinds.push_back(currentHand);
        }
        else
        {
            exit(1);
            std::cout << "ERROR" << std::endl;
        }
    }
}

std::optional<char> GetKeyByValue(const std::map<char, int> &myMap, int value)
{
    for (const auto &pair : myMap)
    {
        if (pair.second == value)
        {
            return pair.first;
        }
    }
    return std::nullopt; // Value not found
}

void SortSeparateHandTypes(std::vector<PokerHand> &pokerHands)
{
    // std::vector<PokerHand> sortedHands;
    // sortedHands.push_back(pokerHands[0]);
    char c1, c2;
    int cntr;
    // gebruik map
    for (size_t i = 0; i < pokerHands.size(); i++)
    {
        for (size_t j = i; j < pokerHands.size(); j++)
        {
            cntr = 0;
            if (i != j)
            {
                while (cntr < 5)
                {
                    c1 = pokerHands[i].hand[cntr];
                    c2 = pokerHands[j].hand[cntr];
                    if (rankValues.at(c1) < rankValues.at(c2))
                    {
                        std::swap(pokerHands[i], pokerHands[j]);
                        break;
                    }
                    else if (rankValues.at(c1) == rankValues.at(c2))
                    {
                        cntr++;
                    }
                    else
                    {
                        break;
                    }
                    
                }
            }
        }
    }
}

std::vector<PokerHand> SortHands(const std::vector<PokerHand> &pokerHands)
{
    HandTypes handTypes;
    std::vector<PokerHand> sortedHands;
    FilterHandByType(handTypes, pokerHands);
    SortSeparateHandTypes(handTypes.fiveOfAKinds);
    SortSeparateHandTypes(handTypes.fourOfAKinds);
    SortSeparateHandTypes(handTypes.fullHouses);
    SortSeparateHandTypes(handTypes.threeOfAKinds);
    SortSeparateHandTypes(handTypes.twoPairs);
    SortSeparateHandTypes(handTypes.onePairs);
    SortSeparateHandTypes(handTypes.highCards);

    sortedHands = handTypes.fiveOfAKinds;
    sortedHands.insert(sortedHands.end(), handTypes.fourOfAKinds.begin(), handTypes.fourOfAKinds.end());
    sortedHands.insert(sortedHands.end(), handTypes.fullHouses.begin(), handTypes.fullHouses.end());
    sortedHands.insert(sortedHands.end(), handTypes.threeOfAKinds.begin(), handTypes.threeOfAKinds.end());
    sortedHands.insert(sortedHands.end(), handTypes.twoPairs.begin(), handTypes.twoPairs.end());
    sortedHands.insert(sortedHands.end(), handTypes.onePairs.begin(), handTypes.onePairs.end());
    sortedHands.insert(sortedHands.end(), handTypes.highCards.begin(), handTypes.highCards.end());
    return sortedHands;
}

int ProductOfGames(std::string filename)
{
    std::vector<PokerHand> pokerHands;
    FillHands(pokerHands, filename);
    std::vector<PokerHand> sortedHands = SortHands(pokerHands);

    std::cout << "----" << std::endl;
    int totalWinnings = 0;
    for (size_t i = 0; i < sortedHands.size(); i++)
    {
        totalWinnings += sortedHands[i].score * (sortedHands.size() - i);
    }
    
    return totalWinnings;
}

int main(int argc, char const *argv[])
{
    std::string filename = "day7.txt";
    int product = ProductOfGames(filename);
    std::cout << "total winnings: " << product << std::endl;
    return 0;
}
