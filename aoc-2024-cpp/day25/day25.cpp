#include <algorithm>
#include <cassert>
#include <chrono>
#include <cstddef>
#include <filesystem>
#include <fstream>
#include <map>
#include <optional>
#include <ranges>
#include <stdexcept>
#include <string>
#include <string_view>
#include <tuple>
#include <unordered_set>
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

auto read_whole_file(fs::path const& file_name) -> std::string {
  std::ifstream infile(fs::path{BASE_PATH} / file_name);
  std::string content((std::istreambuf_iterator<char>(infile)),
                      std::istreambuf_iterator<char>());
  infile.close();
  return content;
}

using schema_t = std::vector<std::vector<int>>;

auto part1(std::vector<schema_t> const& keys,
           std::vector<schema_t> const& locks) {
  const auto h = static_cast<int>(keys[0].size());
  const auto w = static_cast<int>(keys[0][0].size());

  auto column_sum = sv::transform([w, h](auto const& schema) {
                      return sv::iota(0, w) | sv::transform([&](auto j) {
                               auto sum = 0;
                               for (auto i = 0; i < h; ++i) {
                                 sum += schema[i][j];
                               }
                               return sum - 1;
                             }) |
                             sr::to<std::vector>();
                    }) |
                    sr::to<std::vector>();

  const auto lock_heights = locks | column_sum;
  const auto key_heights = keys | column_sum;

  auto matches = 0;
  for (auto [l, k] : flux::cartesian_product(flux::ref(lock_heights),
                                             flux::ref(key_heights))) {
    if (flux::zip(flux::ref(l), flux::ref(k))
            .map(flux::unpack(std::plus{}))
            .all([h](auto x) { return x < h - 1; })) {
      ++matches;
    }
  }

  std::println("matches: {}", matches);
};

int main() {
  using namespace std::literals;
  auto content = read_whole_file("day25/in.txt");

  auto keys = std::vector<schema_t>{};
  auto locks = std::vector<schema_t>{};

  for (auto schema : flux::split_string(std::string_view{content}, "\n\n"sv)) {
    auto s = flux::split_string(schema, "\n"sv)
                 .map([](auto const& line) {
                   return flux::ref(line)
                       .map([](auto const& x) { return x == '#' ? 1 : 0; })
                       .template to<std::vector>();
                 })
                 .to<std::vector>();
    if (flux::all(s[0], [](auto x) { return x == 0; })) {
      keys.push_back(std::move(s));
    } else {
      locks.push_back(std::move(s));
    }
  }

  MEASURE(part1(locks, keys))
}
