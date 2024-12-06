#include <fmt/base.h>
#include <algorithm>
#include <array>
#include <chrono>
#include <cstddef>
#include <filesystem>
#include <flux/op/inplace_reverse.hpp>
#include <fstream>
#include <iterator>
#include <map>
#include <optional>
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

auto transpose(std::vector<std::string> const& grid) {
  const auto width = (int)grid[0].size();
  const auto height = (int)grid.size();
  std::vector<std::string> transposed_grid(width, std::string(height, ' '));
  for (int i = 0; i < height; ++i) {
    for (int j = 0; j < width; ++j) {
      transposed_grid[j][i] = grid[i][j];
    }
  }
  return transposed_grid;
}

using direction_t = std::pair<int, int>;

constexpr std::array<direction_t, 4> directions = {{
    {-1, 0},  // UP
    {0, 1},   // RIGHT
    {1, 0},   // DOWN
    {0, -1}   // LEFT
}};
constexpr static std::string dirs_chars{"^>V<"};

void part1(std::vector<std::string> const& grid) {
  const auto width = (int)grid[0].size();
  const auto height = (int)grid.size();

  const auto max_width = width - 1;
  const auto max_height = height - 1;

  auto ci = 0;
  auto cj = 0;
  std::size_t dir_idx;

  [&] {
    auto is_staring_point = [](char c) -> std::optional<std::size_t> {
      if (auto it = sr::find(dirs_chars, c); it != dirs_chars.end()) {
        return sr::distance(dirs_chars.begin(), it);
      }
      return std::nullopt;
    };
    for (int i = 0; i < height; ++i) {
      for (int j = 0; j < width; ++j) {
        if (auto d = is_staring_point(grid[i][j])) {
          ci = i;
          cj = j;
          dir_idx = d.value();
          return;
        }
      }
    }
  }();

  println("Starting point: {}, {}, direction: {}", ci, cj, dir_idx);

  std::vector<std::vector<int>> visited(height, std::vector<int>(width));

  visited[ci][cj] = 1;

  bool gone_out = false;
  while (!gone_out) {
    auto [di, dj] = directions[dir_idx];
    ci += di;
    cj += dj;
    if ((ci < 0) || (ci > max_height) || (cj < 0) || (cj > max_width)) {
      gone_out = true;
    } else {
      if (grid[ci][cj] == '#') {
        ci -= di;
        cj -= dj;
        dir_idx = ++dir_idx % 4;
      } else {
        visited[ci][cj]++;
      }
    }
  }

  int count = 0;
  for (auto const& r : visited) {
    for (auto const& v : r) {
      count += v > 0;
    }
  }

  println("{}", count);
}

void part2(std::vector<std::string> const& grid) {
  const auto width = (int)grid[0].size();
  const auto height = (int)grid.size();

  const auto max_width = width - 1;
  const auto max_height = height - 1;

  auto ci = 0;
  auto cj = 0;
  std::size_t dir_idx;

  [&] {
    auto is_staring_point = [](char c) -> std::optional<std::size_t> {
      if (auto it = sr::find(dirs_chars, c); it != dirs_chars.end()) {
        return sr::distance(dirs_chars.begin(), it);
      }
      return std::nullopt;
    };
    for (int i = 0; i < height; ++i) {
      for (int j = 0; j < width; ++j) {
        if (auto d = is_staring_point(grid[i][j])) {
          ci = i;
          cj = j;
          dir_idx = d.value();
          return;
        }
      }
    }
  }();

  const int start_i = ci;
  const int start_j = cj;

  auto is_out = [&max_height, &max_width](int i, int j) {
    return (i < 0) || (i > max_height) || (j < 0) || (j > max_width);
  };

  std::vector<std::vector<std::vector<int>>> came_from(
      height, std::vector<std::vector<int>>(width, std::vector<int>{}));
  auto initial_path = [&] {
    std::vector<std::vector<int>> visited(height, std::vector<int>(width));
    visited[ci][cj] = 1;
    bool gone_out = false;
    while (!gone_out) {
      auto [di, dj] = directions[dir_idx];
      ci += di;
      cj += dj;
      if (is_out(ci, cj)) {
        gone_out = true;
      } else {
        if (grid[ci][cj] == '#') {
          ci -= di;
          cj -= dj;
          dir_idx = ++dir_idx % 4;
        } else {
          visited[ci][cj]++;
          came_from[ci][cj].push_back(dir_idx);
        }
      }
    }
    return visited;
  }();

  auto consider = [&](int oi, int oj) {
    println("checking ({}, {})", oi, oj);
    if (initial_path[oi][oj] == 0) {
      return 0;
    }
    auto new_grid = grid;
    new_grid[oi][oj] = '#';
    int possible_loops = 0;
    for (std::size_t obs_dir_idx = 0; obs_dir_idx < directions.size();
         ++obs_dir_idx) {
      auto current_dir_idx = obs_dir_idx;
      println("Checking direction: {} -> ", directions[current_dir_idx]);
      auto i = oi + directions[current_dir_idx].second;
      auto j = oj - directions[current_dir_idx].first;
      const int here_i = i;
      const int here_j = j;
      if (is_out(i, j) || initial_path.at(i).at(j) == 0) {
        continue;
      }
      auto prev_dir_idx = (current_dir_idx + 3) % 4;
      if (!sr::contains(came_from[i][j], prev_dir_idx)) {
        println("({}, {}) -> not possible from {}", i, j,
                directions[obs_dir_idx]);
        continue;
      }
      bool here_again = false;
      int cancel = 9999;
      while (true) {
        auto [di, dj] = directions[current_dir_idx];
        i += di;
        j += dj;
        if (is_out(i, j)) {
          break;
        }
        if (i == here_i && j == here_j) {
          here_again = true;
          break;
        }
        if (new_grid[i][j] == '#') {
          i -= di;
          j -= dj;
          current_dir_idx = ++current_dir_idx % 4;
        }
        if (cancel-- == 0) {
          break;
        }
      }
      if (here_again) {
        println("({}, {}) -> possible from {}, starting pos: ({}, {})", oi, oj,
                directions[obs_dir_idx], here_i, here_j);
        possible_loops++;
      }
    }
    println("({}, {}) -> {}", oi, oj, possible_loops);
    return possible_loops;
  };

  int count = 0;

  for (int i = 0; i < height; ++i) {
    for (int j = 0; j < width; ++j) {
      if (grid[i][j] == '#' || (i == start_i && j == start_j)) {
        continue;
      }
      count += consider(i, j);
    }
  }

  println("{}", count);
}

int main() {
  using namespace std::literals;
  auto lines = read_input("day06/in.txt");
  // println("{}", lines);
  part1(lines);
  // part2(lines);  // not working
}
