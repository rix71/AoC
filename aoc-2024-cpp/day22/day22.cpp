#include <algorithm>
#include <cassert>
#include <chrono>
#include <cstddef>
#include <filesystem>
#include <flux/source/iota.hpp>
#include <flux/source/range.hpp>
#include <fstream>
#include <map>
#include <optional>
#include <ranges>
#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

#include <fmt/base.h>
#include <fmt/ranges.h>
#include <gtest/gtest.h>
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

auto sequence(std::int64_t n) -> std::int64_t {
  n = ((n << 6) ^ n) % 16777216;
  n = ((n >> 5) ^ n) % 16777216;
  n = ((n << 11) ^ n) % 16777216;
  return n;
}

auto part1(std::vector<std::int64_t> const& initial) -> std::int64_t {
  std::int64_t ans{0};
  for (auto n : initial) {
    flux::iota(0, 2000).for_each([&n](auto) { n = sequence(n); });
    ans += n;
  }
  println("{}", ans);
  return ans;
};

auto part2(std::vector<std::int64_t> const& initial) -> std::int64_t {
  using change_seq_t =
      std::tuple<std::int64_t, std::int64_t, std::int64_t, std::int64_t>;

  auto total_bananas = std::map<change_seq_t, std::int64_t>{};

  for (auto n : initial) {
    auto seq = std::vector<std::int64_t>{};
    auto changes = std::vector<std::int64_t>{};
    seq.reserve(2001);
    changes.reserve(2000);
    flux::iota(0, 2000).for_each([&](auto) {
      seq.push_back(n);
      auto new_n = sequence(n);
      changes.push_back((new_n % 10) - (n % 10));
      n = new_n;
    });
    seq.push_back(n);

    auto seen = std::set<change_seq_t>{};
    for (auto i : flux::iota(0, 2000 - 3)) {
      auto chsub = std::tuple{changes[i], changes[i + 1], changes[i + 2],
                              changes[i + 3]};
      if (!seen.contains(chsub)) {
        total_bananas[chsub] += seq[i + 4] % 10;
        seen.insert(std::move(chsub));
      }
    }
  }
  auto res = flux::from_range(total_bananas | sv::values).max().value();
  println("{}", res);
  return res;
}



int main() {
  using namespace std::literals;

  auto lines = read_input("day22/in.txt");
  auto initial = flux::ref(lines)
                     .map([](auto const& line) { return std::stol(line); })
                     .to<std::vector>();

  MEASURE(part1(initial))
  MEASURE(part2(initial))
}
