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

void part1(std::string const& disk_map) {
  std::vector<std::int64_t> extended_map{};
  bool file_block = true;
  std::int64_t file_id = 0;
  for (auto c : disk_map) {
    if (file_block) {
      auto file_size = c - '0';
      for (int i = 0; i < file_size; ++i) {
        extended_map.push_back(file_id);
      }
      file_id++;
      file_block = false;
    } else {
      auto block_size = c - '0';
      for (int i = 0; i < block_size; ++i) {
        extended_map.push_back(-1);
      }
      file_block = true;
    }
  }

  auto ret = sr::partition(extended_map, [](auto x) { return x >= 0; });
  extended_map.erase(ret.begin(), ret.end());

  std::size_t checksum = 0;
  for (auto [i, x] : sv::enumerate(extended_map)) {
    checksum += i * x;
  }

  println("{}", checksum);
}

void part2(std::string const& disk_map) {
  std::vector<std::int64_t> extended_map{};
  std::vector<std::pair<std::size_t, std::size_t>> file_info{};
  std::vector<std::pair<std::size_t, std::size_t>> slot_info{};
  bool file_block = true;
  std::int64_t file_id = -1;
  for (auto c : disk_map) {
    if (file_block) {
      file_id++;
      auto file_size = c - '0';
      auto idx = extended_map.size();
      for (int i = 0; i < file_size; ++i) {
        extended_map.push_back(file_id);
      }
      file_info.emplace_back(file_size, idx);
      file_block = false;
    } else {
      auto block_size = c - '0';
      auto idx = extended_map.size();
      for (int i = 0; i < block_size; ++i) {
        extended_map.push_back(-1);
      }
      slot_info.emplace_back(block_size, idx);
      file_block = true;
    }
  }
  while (file_id >= 0) {
    auto [file_size, file_idx] = file_info.at(file_id);
    if (auto it = sr::find_if(slot_info,
                              [&](auto s) {
                                return (s.first >= file_size) &&
                                       (s.second < file_idx);
                              });
        it != slot_info.end()) {
      auto [slot_size, slot_idx] = *it;
      for (int i = 0; i < file_size; ++i) {
        sr::iter_swap(extended_map.begin() + file_idx + i,
                      extended_map.begin() + slot_idx + i);
      }
      it->first -= file_size;
      it->second += file_size;
    }
    file_id--;
  }
  std::size_t checksum = 0;
  for (auto [i, x] : sv::enumerate(extended_map)) {
    if (x >= 0) {
      checksum += i * x;
    }
  }
  println("{}", checksum);
}

int main() {
  using namespace std::literals;
  auto lines = read_input("day09/in.txt");
  auto disk_map = lines[0];
  println("{}", disk_map);
  MEASURE(part1(disk_map))
  MEASURE(part2(disk_map))
}
