#include <algorithm>
#include <array>
#include <filesystem>
#include <fstream>
#include <map>
#include <ranges>
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

using report_t = std::vector<int>;

auto is_neg = [](int n) { return n < 0; };
auto is_pos = [](int n) { return n > 0; };
auto small_step = [](int n) { return (std::abs(n) > 0) && (std::abs(n) <= 3); };

auto is_safe(report_t const& report) -> bool {
  auto diffs = flux::pairwise_map(flux::ref(report), std::minus{});
  return (flux::all(diffs, is_pos) || flux::all(diffs, is_neg)) &&
         (flux::all(diffs, small_step));
}

void part1(std::vector<report_t> const& reports) {
  auto result =
      flux::ref(reports)
          .map([](report_t const& report) { return is_safe(report) ? 1 : 0; })
          .sum();
  println("{}", result);
}

void part2(std::vector<report_t> const& reports) {
  auto result = flux::ref(reports)
                    .map([](report_t const& report) {
                      if (is_safe(report)) {
                        return 1;
                      }
                      for (auto bl : sv::iota(0UL, report.size())) {
                        auto new_report = report;
                        new_report.erase(new_report.begin() + bl);
                        if (is_safe(new_report)) {
                          return 1;
                        }
                      }
                      return 0;
                    })
                    .sum();
  println("{}", result);
}

int main() {
  using namespace std::literals;
  auto lines = read_input("day02/in.txt");

  std::vector<report_t> reports;
  reports.reserve(lines.size());
  for (auto const& line : lines) {
    auto report = flux::split_string(std::string_view{line}, " "sv)
                      .map([](auto s) { return std::stoi(std::string(s)); })
                      .to<report_t>();
    reports.push_back(std::move(report));
  }

  part1(reports);
  part2(reports);
}
