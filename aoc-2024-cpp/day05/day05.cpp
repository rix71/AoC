#include <algorithm>
#include <array>
#include <chrono>
#include <cstddef>
#include <filesystem>
#include <fstream>
#include <map>
#include <ranges>
#include <regex>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include <fmt/base.h>
#include <fmt/ranges.h>
#include <flux.hpp>
#include <flux/op/inplace_reverse.hpp>

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

using update_t = std::vector<int>;
using rules_t = std::map<int, std::vector<int>>;

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

auto is_correct_order(update_t const& update, rules_t const& rules) -> bool {
  for (auto [idx, page] : update | sv::enumerate) {
    if (!rules.contains(page)) {
      if (idx != 0) {
        return false;
      } else {
        continue;
      }
    }
    auto const& page_rules = rules.at(page);
    for (auto page_before : update | sv::take(idx)) {
      if (!sr::contains(page_rules, page_before)) {
        return false;
      }
    }
  }
  return true;
};

auto sort_pages(update_t update, rules_t& rules) {
  sr::sort(update, [&rules](auto lhs, auto rhs) {
    return sr::contains(rules[rhs], lhs);
  });
  return update;
}

void part1(std::vector<update_t> const& updates, rules_t const& rules) {
  auto result = flux::ref(updates)
                    .filter([&rules](auto update) {
                      return is_correct_order(update, rules);
                    })
                    .map([](auto update) { return update[update.size() / 2]; })
                    .sum();

  println("{}", result);
}

void part2(std::vector<update_t> const& updates, rules_t& rules) {
  auto result = flux::ref(updates)
                    .filter([&rules](auto update) {
                      return !is_correct_order(update, rules);
                    })
                    .map([&rules](auto update) {
                      return sort_pages(std::move(update), rules);
                    })
                    .map([](auto update) { return update[update.size() / 2]; })
                    .sum();

  println("{}", result);
}

int main() {
  using namespace std::literals;
  auto lines = read_input("day05/in.txt");

  auto updates = std::vector<update_t>{};
  auto rules = rules_t{};

  bool reading_rules = true;
  for (auto const& line : lines) {
    if (reading_rules) {
      if (line == "") {
        reading_rules = false;
        continue;
      }
      auto rule = flux::split_string(std::string_view{line}, "|"sv)
                      .map([](auto s) { return std::stoi(std::string(s)); })
                      .to<std::vector>();
      rules[rule[1]].push_back(rule[0]);
    } else {
      auto update = flux::split_string(std::string_view{line}, ","sv)
                        .map([](auto s) { return std::stoi(std::string(s)); })
                        .to<update_t>();
      updates.push_back(std::move(update));
    }
  }

  MEASURE(part1(updates, rules))
  MEASURE(part2(updates, rules))
}
