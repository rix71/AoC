#include <fmt/base.h>
#include <algorithm>
#include <array>
#include <chrono>
#include <cstddef>
#include <filesystem>
#include <flux/op/inplace_reverse.hpp>
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

auto diagonals(std::vector<std::string> const& grid) {
  const auto width = (int)grid[0].size();
  const auto height = (int)grid.size();
  std::vector<std::string> diagonals;
  for (int i = 0; i < height; ++i) {
    std::string diagonal;
    for (int j = 0; j < width; ++j) {
      if (i + j < height) {
        diagonal.push_back(grid[i + j][j]);
      }
    }
    diagonals.push_back(diagonal);
  }
  for (int j = 1; j < width; ++j) {
    std::string diagonal;
    for (int i = 0; i < height; ++i) {
      if (i + j < width) {
        diagonal.push_back(grid[i][i + j]);
      }
    }
    diagonals.push_back(diagonal);
  }
  return diagonals;
}

void part1_flux(std::vector<std::string> const& grid) {
  using namespace std::literals;

  auto sum_of_line = [](auto line) {
    return flux::adjacent_map<4>(
               flux::ref(line),
               [](auto x, auto m, auto a, auto s) {
                 std::string xmas{};
                 xmas.push_back(x);
                 xmas.push_back(m);
                 xmas.push_back(a);
                 xmas.push_back(s);
                 return xmas == "XMAS"sv || xmas == "SAMX"sv ? 1 : 0;
               })
        .sum();
  };

  auto count = flux::ref(grid).map(sum_of_line).sum();
  auto transposed_grid = transpose(grid);
  count += flux::ref(transposed_grid).map(sum_of_line).sum();
  auto diag = diagonals(grid);
  count += flux::ref(diag)
               .filter([](auto line) { return line.size() >= 4; })
               .map(sum_of_line)
               .sum();
  auto reversed_grid = grid;
  sr::reverse(reversed_grid);
  auto diag2 = diagonals(reversed_grid);
  count += flux::ref(diag2)
               .filter([](auto line) { return line.size() >= 4; })
               .map(sum_of_line)
               .sum();

  println("{}", count);
}

void part1(std::vector<std::string> const& grid) {
  using namespace std::literals;
  const auto width = (int)grid[0].size();
  const auto height = (int)grid.size();

  const auto max_width = width - 1;
  const auto max_height = height - 1;

  int count = 0;

  for (int i = 0; i < height; ++i) {
    for (int j = 0; j < width; ++j) {
      bool width_ok = j + 3 <= max_width;
      bool height_ok = i + 3 <= max_height;
      if (grid[i][j] == 'X' || grid[i][j] == 'S') {
        if (width_ok) {
          bool ok = (grid[i][j] == 'X' && grid[i][j + 1] == 'M' &&
                     grid[i][j + 2] == 'A' && grid[i][j + 3] == 'S') ||
                    (grid[i][j] == 'S' && grid[i][j + 1] == 'A' &&
                     grid[i][j + 2] == 'M' && grid[i][j + 3] == 'X');
          if (ok) {
            count++;
          }
        }
        if (height_ok) {
          bool ok = (grid[i][j] == 'X' && grid[i + 1][j] == 'M' &&
                     grid[i + 2][j] == 'A' && grid[i + 3][j] == 'S') ||
                    (grid[i][j] == 'S' && grid[i + 1][j] == 'A' &&
                     grid[i + 2][j] == 'M' && grid[i + 3][j] == 'X');
          if (ok) {
            count++;
          }
        }
      }
      if (width_ok && height_ok) {
        bool d1ok = (grid[i][j] == 'X' && grid[i + 1][j + 1] == 'M' &&
                     grid[i + 2][j + 2] == 'A' && grid[i + 3][j + 3] == 'S') ||
                    (grid[i][j] == 'S' && grid[i + 1][j + 1] == 'A' &&
                     grid[i + 2][j + 2] == 'M' && grid[i + 3][j + 3] == 'X');
        if (d1ok) {
          count++;
        }
        bool d2ok = (grid[i][j + 3] == 'X' && grid[i + 1][j + 2] == 'M' &&
                     grid[i + 2][j + 1] == 'A' && grid[i + 3][j] == 'S') ||
                    (grid[i][j + 3] == 'S' && grid[i + 1][j + 2] == 'A' &&
                     grid[i + 2][j + 1] == 'M' && grid[i + 3][j] == 'X');
        if (d2ok) {
          count++;
        }
      }
    }
  }
  println("{}", count);
}

void part2(std::vector<std::string> const& grid) {
  using namespace std::literals;
  const auto width = (int)grid[0].size();
  const auto height = (int)grid.size();

  const auto max_width = width - 1;
  const auto max_height = height - 1;

  int count = 0;

  for (int i = 0; i < height; ++i) {
    for (int j = 0; j < width; ++j) {
      if (i + 2 <= max_height && j + 2 <= max_width) {
        std::string diag1{};
        std::string diag2{};
        for (int k = 0; k < 3; ++k) {
          diag1.push_back(grid[i + k][j + k]);
          diag2.push_back(grid[i + k][j + 2 - k]);
        }
        auto diag1_ok = (diag1 == "MAS"sv) || (diag1 == "SAM"sv);
        auto diag2_ok = (diag2 == "MAS"sv) || (diag2 == "SAM"sv);
        if (diag1_ok && diag2_ok) {
          count++;
        }
      }
    }
  }
  println("{}", count);
}

int main() {
  using namespace std::literals;
  auto lines = read_input("day04/in.txt");
  // println("{}", lines);
  MEASURE(part1_flux(lines))
  MEASURE(part1(lines))
  MEASURE(part2(lines))
}
