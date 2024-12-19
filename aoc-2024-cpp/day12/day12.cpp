#include <algorithm>
#include <array>
#include <cassert>
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
#include <fmt/color.h>
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

constexpr static std::array<std::pair<int, int>, 4> directions = {
    {{-1, 0}, {0, 1}, {1, 0}, {0, -1}}};

auto get_patches(char plant_type,
                 std::size_t r,
                 std::size_t c,
                 std::vector<std::vector<char>> const& garden,
                 std::vector<std::vector<bool>>& visited) {
  const auto rows = garden.size();
  const auto cols = garden[0].size();

  std::vector<std::pair<int, int>> search;
  std::map<std::size_t, std::vector<std::pair<int, int>>> patches{};

  search.push_back({r, c});
  patches[r].push_back({r, c});

  while (!search.empty()) {
    auto [cr, cc] = search.back();
    search.pop_back();
    for (auto [dr, dc] : directions) {
      auto nr = cr + dr;
      auto nc = cc + dc;
      if (nr < 0 || nr >= (int)rows || nc < 0 || nc >= (int)cols) {
        continue;
      }
      if (garden[nr][nc] == plant_type && !visited[nr][nc]) {
        visited[nr][nc] = true;
        search.push_back({nr, nc});
        patches[nr].push_back({nr, nc});
        continue;
      }
    }
  }
  return patches;
}

auto find_region_area_and_perimeter(
    std::map<std::size_t, std::vector<std::pair<int, int>>> patches) {
  auto area =
      sr::fold_left(patches | sv::values |
                        sv::transform([](auto const& v) { return v.size(); }),
                    0UL, std::plus{});

  auto vertical_edges = 0UL;
  auto horizontal_edges = 0UL;
  for (auto& [_, patch_row] : patches) {
    sr::sort(patch_row,
             [](auto const& a, auto const& b) { return a.second < b.second; });
  }
  for (const auto& [_, patch_row] : patches) {
    vertical_edges += 2;
    vertical_edges +=
        flux::ref(patch_row)
            .pairwise_map([](auto const& a, auto const& b) {
              return a.first == b.first && std::abs(a.second - b.second) == 1
                         ? 0
                         : 2;
            })
            .sum();
    for (const auto& [nr, nc] : patch_row) {
      if (patches.contains(nr + 1)) {
        if (!sr::contains(patches[nr + 1], std::pair{nr + 1, nc})) {
          horizontal_edges++;
        }
      } else {
        horizontal_edges++;
      }
      if (patches.contains(nr - 1)) {
        if (!sr::contains(patches[nr - 1], std::pair{nr - 1, nc})) {
          horizontal_edges++;
        }
      } else {
        horizontal_edges++;
      }
    }
  }

  std::size_t perimeter = vertical_edges + horizontal_edges;

  return std::make_pair(area, perimeter);
}

auto find_region_area_and_sides(
    std::map<std::size_t, std::vector<std::pair<int, int>>> patches) {
  auto area =
      sr::fold_left(patches | sv::values |
                        sv::transform([](auto const& v) { return v.size(); }),
                    0UL, std::plus{});

  for (auto& [_, patch_row] : patches) {
    sr::sort(patch_row,
             [](auto const& a, auto const& b) { return a.second < b.second; });
  }

  auto edges = std::set<std::pair<std::pair<int, int>, std::pair<int, int>>>{};

  for (auto& [_, patch_row] : patches) {
    for (auto& [r, c] : patch_row) {
      for (auto [dr, dc] : directions) {
        auto nr = r + dr;
        auto nc = c + dc;
        if (patches.contains(nr) &&
            sr::contains(patches[nr], std::pair{nr, nc})) {
          continue;
        }
        edges.insert({{r, c}, {nr, nc}});
      }
    }
  }

  auto sides = std::set<std::pair<std::pair<int, int>, std::pair<int, int>>>{};
  for (auto [p1, p2] : edges) {
    bool keep = true;
    for (auto [dr, dc] : std::vector<std::pair<int, int>>{{{1, 0}, {0, 1}}}) {
      auto p1n = std::pair{p1.first + dr, p1.second + dc};
      auto p2n = std::pair{p2.first + dr, p2.second + dc};
      if (edges.contains({p1n, p2n})) {
        keep = false;
      }
    }
    if (keep) {
      sides.insert({p1, p2});
    }
  }

  auto num_sides = sides.size();

  return std::make_pair(area, num_sides);
}

std::uint64_t part1(std::vector<std::vector<char>> const& garden) {
  const auto rows = garden.size();
  const auto cols = garden[0].size();

  std::vector<std::vector<bool>> visited(rows, std::vector<bool>(cols, false));

  auto total_cost = 0UL;

  for (auto r : sv::iota(0UL, rows)) {
    for (auto c : sv::iota(0UL, cols)) {
      if (!visited[r][c]) {
        visited[r][c] = true;
        auto [region_area, region_perimeter] = find_region_area_and_perimeter(
            get_patches(garden[r][c], r, c, garden, visited));
        total_cost += region_area * region_perimeter;
      }
    }
  }
  println("Total cost: {}", total_cost);
  return total_cost;
}

std::uint64_t part2(std::vector<std::vector<char>> const& garden) {
  const auto rows = garden.size();
  const auto cols = garden[0].size();

  std::vector<std::vector<bool>> visited(rows, std::vector<bool>(cols, false));

  auto total_cost = 0UL;

  for (auto r : sv::iota(0UL, rows)) {
    for (auto c : sv::iota(0UL, cols)) {
      if (!visited[r][c]) {
        visited[r][c] = true;
        auto [region_area, region_edges] = find_region_area_and_sides(
            get_patches(garden[r][c], r, c, garden, visited));

        total_cost += region_area * region_edges;
      }
    }
  }
  println("Total cost: {}", total_cost);
  return total_cost;
}

TEST(Case1, Part1) {
  auto lines = read_input("day12/test1.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  // fmt::println("Test 1 garden:\n{}", garden);
  auto res = part1(garden);
  EXPECT_EQ(res, 140);
}

TEST(Case1, Part2) {
  auto lines = read_input("day12/test1.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  auto res = part2(garden);
  EXPECT_EQ(res, 80);
}

TEST(Case2, Part1) {
  auto lines = read_input("day12/test2.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  // fmt::println("Test 2 garden:\n{}", garden);
  auto res = part1(garden);
  EXPECT_EQ(res, 772);
}

TEST(Case2, Part2) {
  auto lines = read_input("day12/test2.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  // fmt::println("Test 2 garden:\n{}", garden);
  auto res = part2(garden);
  EXPECT_EQ(res, 436);
}

TEST(Case3, Part1) {
  auto lines = read_input("day12/test3.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  // fmt::println("Test 3 garden:\n{}", garden);
  auto res = part1(garden);
  EXPECT_EQ(res, 1930);
}

TEST(Case3, Part2) {
  auto lines = read_input("day12/test3.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  // fmt::println("Test 3 garden:\n{}", garden);
  auto res = part2(garden);
  EXPECT_EQ(res, 1206);
}

TEST(Case4, Part2) {
  auto lines = read_input("day12/test4.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  // fmt::println("Test 4 garden:\n{}", garden);
  auto res = part2(garden);
  EXPECT_EQ(res, 236);
}

TEST(Case5, Part2) {
  auto lines = read_input("day12/test5.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  // fmt::println("Test 5 garden:\n{}", garden);
  auto res = part2(garden);
  EXPECT_EQ(res, 368);
}

int main(int argc, char** argv) {
  using namespace std::literals;

  testing::InitGoogleTest(&argc, argv);

  if (RUN_ALL_TESTS()) {
    println("Some tests failed\n");
    std::abort();
  }

  auto lines = read_input("day12/in.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();

  // fmt::println("{}", garden);

  MEASURE(part1(garden))
  MEASURE(part2(garden))
}
