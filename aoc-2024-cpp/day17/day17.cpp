#include <algorithm>
#include <array>
#include <cassert>
#include <chrono>
#include <concepts>
#include <cstddef>
#include <cstdint>
#include <filesystem>
#include <flux/op/adjacent.hpp>
#include <fstream>
#include <iostream>
#include <iterator>
#include <map>
#include <optional>
#include <ranges>
#include <regex>
#include <set>
#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

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

using Registers = std::map<char, std::size_t>;
using Program = std::vector<int>;

auto adv(int operand, Registers& regs) -> std::pair<bool, int>;
auto bxl(int operand, Registers& regs) -> std::pair<bool, int>;
auto bst(int operand, Registers& regs) -> std::pair<bool, int>;
auto jnz(int operand, Registers& regs) -> std::pair<bool, int>;
auto bxc(int operand, Registers& regs) -> std::pair<bool, int>;
auto out(int operand, Registers& regs) -> std::pair<bool, int>;
auto bdv(int operand, Registers& regs) -> std::pair<bool, int>;
auto cdv(int operand, Registers& regs) -> std::pair<bool, int>;

std::string part1(Registers regs, Program program) {
  auto program_out =
      flux::pairwise_map(flux::ref(program),
                         [&](auto opcode, auto operand) {
                           switch (opcode) {
                             case 0: {
                               return adv(operand, regs);
                             }
                             case 1: {
                               return bxl(operand, regs);
                             }
                             case 2: {
                               return bst(operand, regs);
                             }
                             case 3: {
                               return jnz(operand, regs);
                             }
                             case 4: {
                               return bxc(operand, regs);
                             }
                             case 5: {
                               return out(operand, regs);
                             }
                             case 6: {
                               return bdv(operand, regs);
                             }
                             case 7: {
                               return cdv(operand, regs);
                             }
                             default: {
                               throw std::logic_error{"Invalid opcode"};
                             }
                           }
                         })
          .filter([](auto ret) { return ret.first; })
          .map([](auto ret) { return std::to_string(ret.second); })
          .fold([](std::string&& acc, auto o) {
            acc += o + ",";
            return std::move(acc);
          }, std::string{});
  return "0";
}
std::uint64_t part2(Registers regs, Program program) {
  return 0;
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

auto parse(std::vector<std::string> const& lines) {
  using namespace std::literals;
  auto regs = std::map<char, std::size_t>{};
  for (auto line : lines | sv::take(3)) {
    auto res =
        flux::split_string(std::string_view{line}, ": "sv).to<std::vector>();
    auto reg = res[0].back();
    auto val = std::stoull(std::string{res[1]});
    regs[reg] = val;
  }
  auto prog_line = flux::split_string(std::string_view{lines.back()}, ": "sv);
  auto cur = prog_line.first();
  auto program =
      flux::split_string(flux::read_at(prog_line, flux::next(prog_line, cur)),
                         ","sv)
          .map([](auto c) { return std::stoi(std::string{c}); })
          .to<std::vector>();
  return std::pair{std::move(regs), std::move(program)};
}

void test1() {
  auto lines = read_input("day17/test.txt");
  auto [regs, program] = parse(lines);
  fmt::println("Registers: {}\nProgram: {}", regs, program);
  auto res1 = part1(regs, program);
  assert_eq(res1, "4,6,3,5,6,3,5,2,1,0");
  auto res2 = part2(regs, program);
  assert_eq(res2, 1206);
}

int main() {
#if 1
  test1();

#else
  using namespace std::literals;
  auto lines = read_input("day17/in.txt");
  auto [regs, program] = parse(lines);

  fmt::println("{}", regs);
  fmt::println("{}", program);

  // MEASURE(part1(garden))
  // MEASURE(part2(garden))
#endif
}
