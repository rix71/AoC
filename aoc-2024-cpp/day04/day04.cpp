#include <algorithm>
#include <array>
#include <cstddef>
#include <filesystem>
#include <flux/op/inplace_reverse.hpp>
#include <fmt/base.h>
#include <fstream>
#include <map>
#include <ranges>
#include <regex>
#include <string>
#include <string_view>
#include <vector>

#include <flux.hpp>
#include <fmt/ranges.h>

namespace sr = std::ranges;
namespace sv = std::views;
namespace fs = std::filesystem;

using fmt::print;
using fmt::println;

auto read_input(fs::path const &file_name) -> std::vector<std::string> {
  std::fstream infile(fs::path{BASE_PATH} / file_name);
  std::vector<std::string> lines;

  std::string line;
  while (std::getline(infile, line)) {
    lines.push_back(line);
  }
  infile.close();
  return lines;
}

auto transpose_grid(std::vector<std::string> const &grid) {
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

auto diagonals(std::vector<std::string> const &grid) {
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
  for (int i = height - 1; i >= 0; --i) {
    std::string diagonal;
    for (int j = 0; j < width; ++j) {
      if (i - j >= 0) {
        diagonal.push_back(grid[i - j][j]);
      }
    }
    diagonals.push_back(diagonal);
  }
  for (int i = 1; i < height; ++i) {
    std::string diagonal;
    for (int j = 0; j < width; ++j) {
      if (i + j < width) {
        diagonal.push_back(grid[(height - 1) - j][i + j]);
      }
    }
    diagonals.push_back(diagonal);
  }
  return diagonals;
}

void part1(std::vector<std::string> const &grid) {
  using namespace std::literals;

  auto sum_of_line = [](auto line) {
    return flux::adjacent_map<4>(flux::ref(line),
                                 [](auto x, auto m, auto a, auto s) {
                                   std::string xmas{};
                                   xmas.push_back(x);
                                   xmas.push_back(m);
                                   xmas.push_back(a);
                                   xmas.push_back(s);
                                   return xmas == "XMAS"sv ? 1 : 0;
                                 })
        .sum();
  };

  auto reverse_line = [](auto line) {
    flux::inplace_reverse(line);
    return line;
  };

  auto count = flux::ref(grid).map(sum_of_line).sum();
  count += flux::ref(grid).map(reverse_line).map(sum_of_line).sum();

  auto transposed_grid = transpose_grid(grid);
  count += flux::ref(transposed_grid).map(sum_of_line).sum();
  count += flux::ref(transposed_grid).map(reverse_line).map(sum_of_line).sum();

  auto diag = diagonals(grid);
  count += flux::ref(diag)
               .filter([](auto line) { return line.size() >= 4; })
               .map(sum_of_line)
               .sum();
  count += flux::ref(diag)
               .filter([](auto line) { return line.size() >= 4; })
               .map(reverse_line)
               .map(sum_of_line)
               .sum();

  println("{}", count);
}

void part2(std::vector<std::string> const &grid) {
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
  println("{}", lines);
  part1(lines);
  part2(lines);
}
