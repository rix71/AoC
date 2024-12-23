#include <cassert>
#include <chrono>
#include <cstddef>
#include <filesystem>
#include <fstream>
#include <map>
#include <optional>
#include <ranges>
#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

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

using Registers = std::map<char, std::size_t>;
using Program = std::vector<std::size_t>;

auto combo(std::size_t operand, Registers& regs) {
  return operand > 3 ? regs.at('A' + (operand - 4)) : operand;
}

// 0
auto adv(std::size_t operand, Registers& regs) -> void {
  operand = combo(operand, regs);
  auto num = regs.at('A');
  regs['A'] = num >> operand;
}

// 1
auto bxl(std::size_t operand, Registers& regs) -> void {
  auto b = regs.at('B');
  regs['B'] = b ^ operand;
}

// 2
auto bst(std::size_t operand, Registers& regs) -> void {
  operand = combo(operand, regs);
  regs['B'] = operand % 8;
}

// 3
[[nodiscard]] auto jnz(std::size_t operand,
                       Registers& regs,
                       std::size_t instr_ptr) -> std::size_t {
  return regs.at('A') != 0 ? operand : instr_ptr + 2;
}

// 4
auto bxc(std::size_t, Registers& regs) -> void {
  auto b = regs.at('B');
  auto c = regs.at('C');
  regs['B'] = b ^ c;
}

// 5
[[nodiscard]] auto out(std::size_t operand, Registers& regs) -> std::size_t {
  operand = combo(operand, regs);
  return operand % 8;
}

// 6
auto bdv(std::size_t operand, Registers& regs) -> void {
  operand = combo(operand, regs);
  auto num = regs.at('A');
  regs['B'] = num >> operand;
}

// 7
auto cdv(std::size_t operand, Registers& regs) -> void {
  operand = combo(operand, regs);
  auto num = regs.at('A');
  regs['C'] = num >> operand;
}

std::string part1(Registers regs, Program program) {
  using namespace std::literals;

  std::vector<std::size_t> output;
  auto instr_ptr = 0UL;
  while (true) {
    if (instr_ptr >= program.size()) {
      break;
    }
    auto opcode = program.at(instr_ptr);
    auto operand = program.at(instr_ptr + 1);

    switch (opcode) {
      case 0:
        adv(operand, regs);
        instr_ptr += 2;
        break;
      case 1:
        bxl(operand, regs);
        instr_ptr += 2;
        break;
      case 2:
        bst(operand, regs);
        instr_ptr += 2;
        break;
      case 3:
        instr_ptr = jnz(operand, regs, instr_ptr);
        break;
      case 4:
        bxc(operand, regs);
        instr_ptr += 2;
        break;
      case 5:
        output.push_back(out(operand, regs));
        instr_ptr += 2;
        break;
      case 6:
        bdv(operand, regs);
        instr_ptr += 2;
        break;
      case 7:
        cdv(operand, regs);
        instr_ptr += 2;
        break;
      default:
        throw std::logic_error(fmt::format("Unknown opcode: {}", opcode));
    }
  }

  auto result = fmt::format("{}", fmt::join(output, ","sv));
  fmt::println("{}", result);
  return result;
}

std::size_t part2(auto&& solver, Program program) {
  auto answer = program.back();
  auto result = solver(std::move(program), answer).value();
  fmt::println("{}", result);
  return result;
}

auto solve_test2(Program program,
                 std::size_t ans) -> std::optional<std::size_t> {
  if (program.empty()) {
    return ans;
  }
  auto prog_size = program.size();
  for (auto cand : flux::iota(0UL, 8UL)) {
    auto a = ans << 3 | cand;
    auto ta = a >> 3;
    if (ta % 8 == program.back()) {
      auto sub = solve_test2(
          program | sv::take(prog_size - 1) | sr::to<std::vector>(), a);
      if (!sub) {
        continue;
      }
      return sub;
    }
  }
  return std::nullopt;
}

auto solve_part2(Program program,
                 std::size_t ans) -> std::optional<std::size_t> {
  if (program.empty()) {
    return ans;
  }
  auto prog_size = program.size();
  for (auto cand : flux::iota(0UL, 8UL)) {
    auto a = ans << 3 | cand;
    auto b = 0UL;
    auto c = 0UL;
    b = a % 8 ^ 5;
    c = a >> b;
    b = b ^ 6 ^ c;
    if (b % 8 == program.back()) {
      auto sub = solve_part2(
          program | sv::take(prog_size - 1) | sr::to<std::vector>(), a);
      if (!sub) {
        continue;
      }
      return sub;
    }
  }
  return std::nullopt;
}

auto parse(std::vector<std::string> const& lines) {
  using namespace std::literals;
  auto regs = std::map<char, std::size_t>{};
  for (auto const& line : lines | sv::take(3)) {
    auto res =
        flux::split_string(std::string_view{line}, ": "sv).to<std::vector>();
    auto reg = res[0].back();
    auto val = std::stoul(std::string{res[1]});
    regs[reg] = val;
  }
  auto prog_line = flux::split_string(std::string_view{lines.back()}, ": "sv);
  auto cur = prog_line.first();
  auto program =
      flux::split_string(flux::read_at(prog_line, flux::next(prog_line, cur)),
                         ","sv)
          .map([](auto c) { return std::stoul(std::string{c}); })
          .to<std::vector>();
  return std::pair{std::move(regs), std::move(program)};
}

TEST(Case1, Part1) {
  auto lines = read_input("day17/test1.txt");
  auto [regs, program] = parse(lines);
  // fmt::println("Registers: {}\nProgram: {}", regs, program);
  auto res = part1(regs, program);
  EXPECT_EQ(res, "4,6,3,5,6,3,5,2,1,0");
}

TEST(Case2, Part2) {
  auto lines = read_input("day17/test2.txt");
  auto [regs, program] = parse(lines);
  // fmt::println("Registers: {}\nProgram: {}", regs, program);
  auto res = part2(solve_test2, program);
  EXPECT_EQ(res, 117440);
}

int main(int argc, char** argv) {
  using namespace std::literals;

  testing::InitGoogleTest(&argc, argv);

  if (RUN_ALL_TESTS()) {
    println("Some tests failed\n");
    std::abort();
  }

  auto lines = read_input("day17/in.txt");
  auto [regs, program] = parse(lines);

  // fmt::println("{}", regs);
  // fmt::println("{}", program);

  MEASURE(part1(regs, program))
  MEASURE(part2(solve_part2, program))
}
