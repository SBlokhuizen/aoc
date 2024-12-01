#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

struct Map
{
    std::vector<std::vector<long long int>> seedToSoil;
    std::vector<std::vector<long long int>> soilToFertilizer;
    std::vector<std::vector<long long int>> fertilizerToWater;
    std::vector<std::vector<long long int>> waterToLight;
    std::vector<std::vector<long long int>> lightToTemperature;
    std::vector<std::vector<long long int>> temperatureToHumidity;
    std::vector<std::vector<long long int>> humidityToLocation;
};

std::vector<long long int> parseHeader(const std::string &input_string)
{
    std::vector<long long int> result_vector;
    std::istringstream iss(input_string);

    // Tokenize the input string based on spaces
    std::string token;
    while (std::getline(iss, token, ' '))
    {
        // Convert each token to a long long int
        try
        {
            long long int num = std::stoll(token);
            result_vector.push_back(num);
        }
        catch (const std::invalid_argument &e)
        {
            // Handle invalid argument (non-numeric tokens)
            // You can choose to ignore or handle this accordingly
            // For simplicity, I'll ignore and continue to the next token
            continue;
        }
        catch (const std::out_of_range &e)
        {
            // Handle out of range error if the number is too large
            // You can choose to ignore or handle this accordingly
            // For simplicity, I'll ignore and continue to the next token
            continue;
        }
    }

    return result_vector;
}

std::vector<long long int> parseNumbers(const std::string &input_string)
{
    std::vector<long long int> result_vector;
    std::istringstream iss(input_string);
    long long int num;

    // Read integers from the input string stream
    while (iss >> num)
    {
        result_vector.push_back(num);
    }

    return result_vector;
}

std::vector<long long int> GetSeeds(std::vector<std::string> data)
{
    std::vector<long long int> seeds = parseHeader(data[0]);
    return seeds;
}

std::vector<std::string> ReadData(std::string filename)
{
    std::vector<std::string> lines;
    std::ifstream file(filename);
    std::string line;

    if (file.is_open())
    {
        while (std::getline(file, line))
        {
            lines.push_back(line);
        }
        file.close();
    }
    else
    {
        std::cerr << "Unable to open file: " << filename << std::endl;
    }

    return lines;
}

void fillMapFromData(Map &map, const std::vector<std::string> &data)
{
    int lineNumber = 3;
    // Fill seedToSoil map
    map.seedToSoil.clear();
    while (lineNumber != data.size() && data[lineNumber] != "soil-to-fertilizer map:" && data[lineNumber] != "")
    {
        std::vector<long long int> rowData = parseNumbers(data[lineNumber]);
        map.seedToSoil.push_back(rowData);
        lineNumber++;
    }
    lineNumber = lineNumber + 2;

    //
    map.soilToFertilizer.clear();
    while (lineNumber != data.size() && data[lineNumber] != "fertilizer-to-water map:" && data[lineNumber] != "")
    {
        std::vector<long long int> rowData = parseNumbers(data[lineNumber]);
        map.soilToFertilizer.push_back(rowData);
        lineNumber++;
    }
    lineNumber = lineNumber + 2;

    //
    map.fertilizerToWater.clear();
    while (lineNumber != data.size() && data[lineNumber] != "water-to-light map:" && data[lineNumber] != "")
    {
        std::vector<long long int> rowData = parseNumbers(data[lineNumber]);
        map.fertilizerToWater.push_back(rowData);
        lineNumber++;
    }
    lineNumber = lineNumber + 2;

    //
    map.waterToLight.clear();
    while (lineNumber != data.size() && data[lineNumber] != "light-to-temperature map:" && data[lineNumber] != "")
    {
        std::vector<long long int> rowData = parseNumbers(data[lineNumber]);
        map.waterToLight.push_back(rowData);
        lineNumber++;
    }
    lineNumber = lineNumber + 2;

    //
    map.lightToTemperature.clear();
    while (lineNumber != data.size() && data[lineNumber] != "temperature-to-humidity map:" && data[lineNumber] != "")
    {
        std::vector<long long int> rowData = parseNumbers(data[lineNumber]);
        map.lightToTemperature.push_back(rowData);
        lineNumber++;
    }
    lineNumber = lineNumber + 2;

    //
    map.temperatureToHumidity.clear();
    while (lineNumber != data.size() && data[lineNumber] != "humidity-to-location map:" && data[lineNumber] != "")
    {
        std::vector<long long int> rowData = parseNumbers(data[lineNumber]);
        map.temperatureToHumidity.push_back(rowData);
        lineNumber++;
    }
    lineNumber = lineNumber + 2;

    //
    map.humidityToLocation.clear();
    while (lineNumber != data.size())
    {
        std::vector<long long int> rowData = parseNumbers(data[lineNumber]);
        map.humidityToLocation.push_back(rowData);
        lineNumber++;
    }
}

void printMap(const std::vector<std::vector<long long int>> &map)
{
    for (const auto &row : map)
    {
        for (long long int val : row)
        {
            std::cout << val << " ";
        }
        std::cout << std::endl;
    }
    std::cout << std::endl;
}

void PrintMaps(const Map &conversionMap)
{
    // Print out the loaded maps to verify
    std::cout << "Seed to Soil map:" << std::endl;
    printMap(conversionMap.seedToSoil);

    std::cout << "Soil to Fertilizer map:" << std::endl;
    printMap(conversionMap.soilToFertilizer);

    std::cout << "Fertilizer to Water map:" << std::endl;
    printMap(conversionMap.fertilizerToWater);

    std::cout << "Water to Light map:" << std::endl;
    printMap(conversionMap.waterToLight);

    std::cout << "Light to Temperature map:" << std::endl;
    printMap(conversionMap.lightToTemperature);

    std::cout << "Temperature to Humidity map:" << std::endl;
    printMap(conversionMap.temperatureToHumidity);

    std::cout << "Humidity to Location map:" << std::endl;
    printMap(conversionMap.humidityToLocation);
}

long long int ConvertValues(const std::vector<std::vector<long long int>> &map, long long int seed)
{
    bool rangeFound = false;
    long long int convertedValue;
    for (std::vector<long long int> singleMap : map)
    {
        if (seed >= singleMap[1] && seed < (singleMap[1] + singleMap[2]))
        {
            rangeFound = true;
            // std::cout << "between" << singleMap[1] << " & " << singleMap[1] + singleMap[2] << std::endl;
            convertedValue = singleMap[0] + (seed - singleMap[1]);
        }
    }
    if (rangeFound == false)
    {
        // std::cout << "no range found" << std::endl;
        convertedValue = seed;
    }
    return convertedValue;
}

long long int LowestLocationNumber(std::string filename)
{
    std::vector<std::string> data = ReadData(filename);
    std::vector<long long int> seeds = GetSeeds(data);
    std::cout << "---- SEEDS ----" << std::endl;
    for (auto seed : seeds)
    {
        std::cout << seed << std::endl;
    }

    Map map;
    fillMapFromData(map, data);
    std::cout << "---- MAPS ----" << std::endl;

    PrintMaps(map);

    long long int conversion, location, minLocation;
    int loopNum = 0;
    std::cout << "---- CONVERSION ----" << std::endl;

    for (auto seed : seeds)
    {
        std::cout << "SEED: " << seed << std::endl;
        conversion = ConvertValues(map.seedToSoil, seed);
        std::cout << conversion << std::endl;
        conversion = ConvertValues(map.soilToFertilizer, conversion);
        std::cout << conversion << std::endl;
        conversion = ConvertValues(map.fertilizerToWater, conversion);
        std::cout << conversion << std::endl;
        conversion = ConvertValues(map.waterToLight, conversion);
        std::cout << conversion << std::endl;
        conversion = ConvertValues(map.lightToTemperature, conversion);
        std::cout << conversion << std::endl;
        conversion = ConvertValues(map.temperatureToHumidity, conversion);
        std::cout << conversion << std::endl;
        location = ConvertValues(map.humidityToLocation, conversion);
        std::cout << location << std::endl;

        // std::cout << location << std::endl;
        if (location < minLocation || loopNum == 0)
        {
            minLocation = location;
        }
        loopNum++;
        std::cout << "min LOCATION " << minLocation << std::endl;
    }
    return minLocation;
}

int main(int argc, char const *argv[])
{
    std::string filename = "day5.txt";

    long long int lowest = LowestLocationNumber(filename);
    std::cout << "-----" << std::endl
              << lowest << std::endl;
    return 0;
}