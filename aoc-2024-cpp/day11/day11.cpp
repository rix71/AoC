#include <algorithm>
#include <array>
#include <chrono>
#include <concepts>
#include <cstddef>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <iterator>
#include <map>
#include <optional>
#include <ranges>
#include <regex>
#include <set>
#include <string>
#include <string_view>
#include <vector>

#include <fmt/base.h>
#include <fmt/ranges.h>
#include <flux.hpp>

namespace sr = std::ranges;
namespace sv = std::views;
namespace fs = std::filesystem;

using fmt::print;
using fmt::println;

#define MEASURE(f)                                                             \
  {                                                                            \
    auto start = std::chrono::high_resolution_clock::now();                    \
    f;                                                                         \
    auto end = std::chrono::high_resolution_clock::now();                      \
    println("{} took {} µs", #f,                                               \
            std::chrono::duration_cast<std::chrono::microseconds>(end - start) \
                .count());                                                     \
  }

auto read_input(fs::path const& file_name) -> std::vector<std::string> {
  std::fstream infile(fs::path{BASE_PATH} / file_name);
  std::vector<std::string> lines;

  std::string line;
  while (std::getline(infile, line)) {
    lines.push_back(line);
  }
  infile.close();
  return lines;
}

auto blink(std::uint64_t num, int d) {
  static auto cache =
      std::map<std::pair<std::uint64_t, std::uint64_t>, std::uint64_t>{};
  if (d == 0) {
    return 1UL;
  }
  if (!cache.contains({num, d})) {
    std::uint64_t result{0};
    if (num == 0UL) {
      result += blink(1UL, d - 1);
    } else {
      auto num_digits = static_cast<int>(std::log10(num)) + 1;
      if (num_digits % 2 == 0) {
        auto num_str = std::to_string(num);
        auto first = num_str.substr(0, num_digits / 2);
        auto second = num_str.substr(num_digits / 2);
        result += blink(std::stoul(first), d - 1);
        result += blink(std::stoul(second), d - 1);
      } else {
        result += blink(num * 2024, d - 1);
      }
    }
    cache[{num, d}] = result;
  }
  return cache[{num, d}];
};

void part1(std::vector<std::uint64_t> const& stones) {
  auto total = 0UL;
  for (auto num : stones) {
    total += blink(num, 25);
  }
  println("{}", total);
}

void part2(std::vector<std::uint64_t> const& stones) {
  auto total = 0UL;
  for (auto num : stones) {
    total += blink(num, 75);
  }
  println("{}", total);
}

int main() {
  using namespace std::literals;
  auto lines = read_input("day11/in.txt");
  auto stones =
      flux::ref(lines[0])
          .split_string(" "sv)
          .map([](auto const& s) { return std::stoul(std::string(s)); })
          .to<std::vector>();

  println("{}", stones);
  
  MEASURE(part1(stones))
  MEASURE(part2(stones))
}
