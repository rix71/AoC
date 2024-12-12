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

auto find_region_area_and_edges(
    std::map<std::size_t, std::vector<std::pair<int, int>>> patches) {
  auto area =
      sr::fold_left(patches | sv::values |
                        sv::transform([](auto const& v) { return v.size(); }),
                    0UL, std::plus{});

  // auto vertical_edges = 2UL;
  // auto horizontal_edges = 2UL;
  for (auto& [_, patch_row] : patches) {
    sr::sort(patch_row,
             [](auto const& a, auto const& b) { return a.second < b.second; });
  }

  auto left = patches.begin()->second.front();
  auto right = patches.begin()->second.back();
  auto edges = 4UL;
  for (auto& [_, patch_row] : patches | sv::drop(1)) {
    auto left_next = patch_row.front();
    auto right_next = patch_row.back();
    if (left_next.second != left.second) {
      edges += 2;
    }
    if (right_next.second != right.second) {
      edges += 2;
    }

    left = left_next;
    right = right_next;
  }
  auto holes = std::set<std::pair<int, int>>{};
  auto bays = std::set<std::pair<int, int>>{};
  for (auto& [_, patch_row] : patches) {
    flux::for_each(
        flux::ref(patch_row).pairwise(),
        flux::unpack([&holes, &bays, &patches](auto const& a, auto const& b) {
          if (std::abs(a.second - b.second) > 1) {
            auto hole = std::pair{a.first, a.second + 1};
            int dd = 0;
            bool new_hole = true;
            for (auto [dr, dc] : directions) {
              for (auto i : sv::iota(0, (int)patches.size())) {
                auto nr = hole.first + dr * i;
                auto nc = hole.second + dc * i;
                if (holes.contains(std::pair{nr, nc})) {
                  new_hole = false;
                }
                if (patches.contains(nr) &&
                    sr::contains(patches[nr], std::pair{nr, nc})) {
                  dd++;
                  break;
                }
              }
            }
            if (dd >= 4) {
              if (new_hole) {
                println("Found hole at ({}, {})", hole.first, hole.second);
                holes.insert(hole);
              }
            } else {
              println("Found bay at ({}, {})", hole.first, hole.second);
              bays.insert(hole);
            }
          }
        }));
  }
  edges += 4 * holes.size();
  edges += 2 * bays.size();

  return std::make_pair(area, edges);
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
        // println("Region {} ({}, {}) area: {}, perimeter {}", garden[r][c], r,
        // c,
        //         region_area, region_perimeter);
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
        auto [region_area, region_edges] = find_region_area_and_edges(
            get_patches(garden[r][c], r, c, garden, visited));
        println("Region {} ({}, {}) area: {}, edges {}", garden[r][c], r, c,
                region_area, region_edges);
        total_cost += region_area * region_edges;
      }
    }
  }
  println("Total cost: {}", total_cost);
  return total_cost;
}

#define assert_eq(a, b)                                                       \
  {                                                                           \
    bool success = (a == b);                                                  \
    if (!success) {                                                           \
      fmt::print(fg(fmt::color::red), "Assertion failed: {} != {} ({}:{})\n", \
                 a, b, __FUNCTION__, __LINE__);                               \
    } else {                                                                  \
      fmt::print(fg(fmt::color::green),                                       \
                 "Assertion passed: {} == {} ({}:{})\n", a, b, __FUNCTION__,  \
                 __LINE__);                                                   \
    }                                                                         \
  }

void test1() {
  auto lines = read_input("day12/test1.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  fmt::println("Test 1 garden:\n{}", garden);
  auto res1 = part1(garden);
  assert_eq(res1, 140);
  auto res2 = part2(garden);
  assert_eq(res2, 80);
}
void test2() {
  auto lines = read_input("day12/test2.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  fmt::println("Test 2 garden:\n{}", garden);
  auto res1 = part1(garden);
  assert_eq(res1, 772);
  auto res2 = part2(garden);
  assert_eq(res2, 436);
}
void test3() {
  auto lines = read_input("day12/test3.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  fmt::println("Test 3 garden:\n{}", garden);
  auto res1 = part1(garden);
  assert_eq(res1, 1930);
  auto res2 = part2(garden);
  assert_eq(res2, 1206);
}
void test4() {
  auto lines = read_input("day12/test4.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  fmt::println("Test 4 garden:\n{}", garden);
  auto res2 = part2(garden);
  assert_eq(res2, 236);
}
void test5() {
  auto lines = read_input("day12/test5.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();
  fmt::println("Test 5 garden:\n{}", garden);
  auto res2 = part2(garden);
  assert_eq(res2, 368);
}

int main() {
#if 0
  test1();
  test2();
  test3();
  test4();
  test5();

#else
  using namespace std::literals;
  auto lines = read_input("day12/in.txt");
  auto garden = lines | sv::transform([](auto const& line) {
                  return line | sr::to<std::vector>();
                }) |
                sr::to<std::vector>();

  fmt::println("{}", garden);

  MEASURE(part1(garden))
  MEASURE(part2(garden))  // Not working
#endif
}
