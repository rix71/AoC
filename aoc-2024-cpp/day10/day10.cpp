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

constexpr static std::array<std::pair<int, int>, 4> directions = {
    {{-1, 0}, {0, 1}, {1, 0}, {0, -1}}};

void part1(std::vector<std::vector<int>> const& topo) {
  const auto rows = topo.size();
  const auto cols = topo[0].size();
  std::size_t score = 0;

  auto trace_path = [&](std::size_t r, std::size_t c) {
    std::vector<std::pair<int, std::pair<int, int>>> trails;
    std::set<std::pair<int, int>> destinations;

    trails.push_back({0, {r, c}});

    while (!trails.empty()) {
      auto [trail_val, trail_pos] = trails.back();
      trails.pop_back();
      auto [tr, tc] = trail_pos;
      for (auto [dr, dc] : directions) {
        auto nr = tr + dr;
        auto nc = tc + dc;
        if (nr < 0 || nr >= rows || nc < 0 || nc >= cols) {
          continue;
        }
        if (trail_val == 8 && topo[nr][nc] == 9) {
          destinations.insert({nr, nc});
          continue;
        }
        if (topo[nr][nc] == trail_val + 1) {
          trails.push_back({trail_val + 1, {nr, nc}});
        }
      }
    }
    auto trail_score = destinations.size();
    return trail_score;
  };

  for (auto r : sv::iota(0UL, rows)) {
    for (auto c : sv::iota(0UL, cols)) {
      if (topo[r][c] == 0) {
        score += trace_path(r, c);
      }
    }
  }
  println("{}", score);
}

void part2(std::vector<std::vector<int>> const& topo) {
  const auto rows = topo.size();
  const auto cols = topo[0].size();
  std::size_t score = 0;

  auto trace_path = [&](std::size_t r, std::size_t c) {
    std::size_t trail_score = 0;
    std::vector<std::pair<int, std::pair<int, int>>> trails;

    trails.push_back({0, {r, c}});

    while (!trails.empty()) {
      auto [trail_val, trail_pos] = trails.back();
      trails.pop_back();
      auto [tr, tc] = trail_pos;
      for (auto [dr, dc] : directions) {
        auto nr = tr + dr;
        auto nc = tc + dc;
        if (nr < 0 || nr >= rows || nc < 0 || nc >= cols) {
          continue;
        }
        if (trail_val == 8 && topo[nr][nc] == 9) {
          trail_score++;
          continue;
        }
        if (topo[nr][nc] == trail_val + 1) {
          trails.push_back({trail_val + 1, {nr, nc}});
        }
      }
    }
    return trail_score;
  };

  for (auto r : sv::iota(0UL, rows)) {
    for (auto c : sv::iota(0UL, cols)) {
      if (topo[r][c] == 0) {
        score += trace_path(r, c);
      }
    }
  }
  println("{}", score);
}

int main() {
  using namespace std::literals;
  auto lines = read_input("day10/in.txt");
  auto topo = flux::ref(lines)
                  .map([](std::string const& line) {
                    return flux::ref(line)
                        .map([](char c) { return c - '0'; })
                        .to<std::vector>();
                  })
                  .to<std::vector>();
  // println("{}", topo);
  MEASURE(part1(topo))
  MEASURE(part2(topo))
}
