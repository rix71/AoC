#include <algorithm>
#include <array>
#include <filesystem>
#include <fstream>
#include <map>
#include <ranges>
#include <regex>
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

auto sum_instructions(std::string const& memory) -> std::uint64_t {
  std::regex re("mul\\((\\d{1,3},\\d{1,3})\\)");
  auto it = std::sregex_iterator(sr::begin(memory), sr::end(memory), re);
  auto end_it = std::sregex_iterator();

  std::uint64_t sum = 0;
  for (auto m = it; m != end_it; ++m) {
    std::smatch match = *m;
    auto nums =
        sv::split(match[1].str(), ',') | sv::transform([](auto const& s) {
          return std::stoul(std::string(sr::begin(s), sr::end(s)));
        });
    auto mult = sr::fold_left(nums, 1UL, std::multiplies{});
    sum += mult;
  }
  return sum;
}

void part1(std::string const& memory) {
  print("{}\n", sum_instructions(memory));
}

void part2(std::string const& memory) {
  using namespace std::literals;
  bool enabled = true;
  std::uint64_t total = 0;
  sr::for_each(
      memory | sv::split("don't()"sv), [&enabled, &total](auto const& m) {
        if (enabled) {
          total += sum_instructions(std::string(sr::begin(m), sr::end(m)));
          enabled = false;
        } else {
          total += sr::fold_left(std::string_view(sr::begin(m), sr::end(m)) |
                                     sv::split(std::string_view{"do()"}) |
                                     sv::drop(1) |
                                     sv::transform([](auto const& sub_m) {
                                       return sum_instructions(std::string(
                                           sr::begin(sub_m), sr::end(sub_m)));
                                     }),
                                 0UL, std::plus{});
        }
      });
  print("{}\n", total);
}

int main() {
  using namespace std::literals;
  auto lines = read_input("day03/in.txt");
  auto flattened = lines | sv::join | sv::common;
  std::string memory(flattened.begin(), flattened.end());

  part1(memory);
  part2(memory);
}
