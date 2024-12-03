#include <algorithm>
#include <array>
#include <filesystem>
#include <fstream>
#include <map>
#include <ranges>
#include <string>
#include <string_view>
#include <vector>

#include <fmt/ranges.h>
#include <flux.hpp>

namespace sr = std::ranges;
namespace sv = std::views;
namespace fs = std::filesystem;

using fmt::print;
using fmt::println;

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

void part1(std::vector<int> l1, std::vector<int> l2) {
  sr::sort(l1);
  sr::sort(l2);
  int result =
      flux::zip(flux::ref(l1), flux::ref(l2))
          .map(flux::unpack([](auto a, auto b) { return std::abs(a - b); }))
          .sum();
  println("{}", result);
}

// void part1(auto l1, auto l2) {
//   sr::sort(l1);
//   sr::sort(l2);
//   auto diffs =
//       sv::zip_transform([](auto a, auto b) { return std::abs(a - b); }, l1,
//       l2);
//   auto result = sr::fold_left(diffs, 0, std::plus{});
//   println("{}", result);
// }

void part2(std::vector<int> l1, std::vector<int> l2) {
  auto l2_counts = std::map<int, int>{};
  flux::ref(l2).for_each([&l2_counts](auto i) { l2_counts[i]++; });
  auto result = flux::ref(l1)
                    .map([&l2_counts](auto i) { return i * l2_counts[i]; })
                    .sum();
  println("{}", result);
}

// void part2(auto l1, auto l2) {
//   int result = 0;
//   sr::for_each(l1, [&result, &l2](auto i) {
//     auto c = sr::count(l2, i);
//     result += i * c;
//   });
//   println("{}", result);
// }

int main() {
  using namespace std::literals;
  auto lines = read_input("day1/test.txt");

  auto l1 = std::vector<int>{};
  auto l2 = std::vector<int>{};
  for (auto const& line : lines) {
    auto num_pairs = flux::split_string(std::string_view{line}, "   "sv)
                         .map([](auto s) { return std::stoi(std::string(s)); })
                         .to<std::vector>();
    l1.push_back(num_pairs[0]);
    l2.push_back(num_pairs[1]);
  }

  part1(l1, l2);
  part2(l1, l2);
}
