#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>

std::map<std::string, int> get_str_map() {
    const std::string digit_strings[] = {
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
    std::map<std::string, int> str_to_digit;
    int i = 1;
    for (auto s: digit_strings) {
        str_to_digit[s] = i++;
    }
    return str_to_digit;
}

int main() {
    std::ifstream file("../d1-1.txt");

    if (!file.is_open()) {
        throw;
    }

    long part1_sum = 0;
    long part2_sum = 0;

    const std::map<std::string, int> str_to_digit = get_str_map();


    std::string line;
    while (std::getline(file, line)) {
        std::vector<int> digits;
        std::vector<std::pair<int, int>> digit_positions;

        int i = 0;
        for (const auto c: line) {
            const auto digit = c - '0';
            if (digit >= 0 && digit <= 9) {
                digits.push_back(digit);
                digit_positions.emplace_back(i, digit);
            }
            i++;
        }
        for (const auto&[digit_str, digit]: str_to_digit) {
            auto pos = line.find(digit_str, 0);
            while (pos != std::string::npos) {
                digit_positions.emplace_back(pos, digit);
                pos = line.find(digit_str, pos + 1);
            }
        }

        assert(!digits.empty());
        part1_sum += digits[0] * 10 + digits[digits.size() - 1];

        std::sort(digit_positions.begin(), digit_positions.end());
        part2_sum += digit_positions[0].second * 10 + digit_positions[digit_positions.size() - 1].second;
    }
    file.close();

    std::cout << "Part 1 sum: " << part1_sum << std::endl;
    std::cout << "Part 2 sum: " << part2_sum << std::endl;
}
