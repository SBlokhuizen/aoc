#include <iostream>
#include <vector>
#include <sstream>
#include <fstream>
#include <string>
#include <map>

struct Node
{
    std::string left;
    std::string right;
};

std::vector<std::string> ReadLine(const std::string &input)
{
    std::vector<std::string> result;
    result.push_back(input.substr(0, 3));
    // Find the position of '(' and ')'
    size_t openParenPos = input.find('(');
    size_t closeParenPos = input.find(')');

    if (openParenPos != std::string::npos && closeParenPos != std::string::npos)
    {
        // Extract the substring between '(' and ')'
        std::string mainString = input.substr(openParenPos + 1, closeParenPos - openParenPos - 1);

        // Use stringstream to split the substring by comma
        std::stringstream ss(mainString);
        std::string item;
        while (std::getline(ss, item, ','))
        {
            // Trim the whitespace from each item
            size_t start = item.find_first_not_of(" \t");
            size_t end = item.find_last_not_of(" \t");
            if (start != std::string::npos && end != std::string::npos)
                result.push_back(item.substr(start, end - start + 1));
        }
    }

    return result;
}

std::string FillStringGraph(std::string filename, std::map<std::string, Node> &nodeMap)
{
    std::ifstream file(filename);
    std::string line, firstLine;
    std::vector<std::string> vertices;

    int cntr = 0;
    while (std::getline(file, line))
    {
        if (cntr == 0)
        {
            firstLine = line;
            cntr++;
            continue;
        }
        else if (cntr == 1)
        {
            cntr++;
            continue;
        }
        // first node
        else
        {
            vertices = ReadLine(line);
            Node node;
            node.left = vertices[1];
            node.right = vertices[2];
            nodeMap.insert(std::make_pair(vertices[0], node));
            cntr++;
        }
    }
    return firstLine;
}

std::vector<std::string> FindKeysEndingWithA(const std::map<std::string, Node> &nodeMap)
{
    std::vector<std::string> keysEndingWithA;
    for (auto it = nodeMap.begin(); it != nodeMap.end(); ++it)
    {
        if (it->first.size() > 0 && it->first.back() == 'A')
        {
            keysEndingWithA.push_back(it->first);
        }
    }
    return keysEndingWithA;
}

bool IsAllTrue(std::vector<bool> endReached)
{
    for (bool key : endReached)
    {
        if (key == false)
        {
            return false;
        }
    }
    return true;
}

int CountSteps(std::string filename)
{
    std::map<std::string, Node> nodeMap;
    std::string sequence = FillStringGraph(filename, nodeMap);
    std::cout << "sequence: " << sequence << std::endl;

    Node node;
    node.left = "";
    node.right = "";

    std::vector<std::string> keysEndingWithA = FindKeysEndingWithA(nodeMap);
    int numberStart = keysEndingWithA.size();
    std::vector<std::map<std::string, Node>::iterator> it(numberStart);
    std::vector<bool> endReached(numberStart, false);
    bool keepGoing = true;

    for (size_t i = 0; i < numberStart; i++)
    {
        std::cout << keysEndingWithA[i] << std::endl;
        it[i] = nodeMap.find(keysEndingWithA[i]);
    }

    // Start search
    int cntr = 0;
    int sequenceNumber;

    while (keepGoing)
    {
        std::cout << "----------- " << cntr << " ----------" << std::endl;
        sequenceNumber = cntr % sequence.size();
        for (size_t i = 0; i < numberStart; i++)
        {
            // Get the Node object
            node = it[i]->second;
            if (sequence[sequenceNumber] == 'L')
            {
                it[i] = nodeMap.find(node.left);
            }
            else if (sequence[sequenceNumber] == 'R')
            {
                it[i] = nodeMap.find(node.right);
            }
            else
            {
                std::cout << "error" << std::endl;
                exit(1);
            }
            if ((sequence[sequenceNumber] == 'L' && node.left[2] == 'Z') || (sequence[sequenceNumber] == 'R' && node.right[2] == 'Z'))
            {
                endReached[i] = true;
            }
            // std::cout << "node : " << i << " " << sequence[sequenceNumber] << " | left: " << node.left << " | right: " << node.right << std::endl;
        }

        if (IsAllTrue(endReached))
        {
            std::cout << "All have reached end" << std::endl;
            keepGoing = false;
        }
        else
        {
            for (size_t i = 0; i < endReached.size(); i++)
            {
                endReached[i] = false;
            }
        }

        cntr++;
    }
    std::cout << cntr << std::endl;
    return 69;
}

int main(int argc, char const *argv[])
{
    std::string filename = "day8.txt";
    int steps = CountSteps(filename);

    return 0;
}
