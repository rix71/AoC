cmake_minimum_required(VERSION 3.28)

project(AoC2024)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(fmt REQUIRED)
find_package(flux REQUIRED)
find_package(Boost REQUIRED)
find_package(GTest REQUIRED)

# Link to all
link_libraries(fmt::fmt flux::flux gtest::gtest Boost::boost)

add_compile_options(-DBASE_PATH="${CMAKE_SOURCE_DIR}")

# Solutions
add_executable(day01 day01/day01.cpp)
add_executable(day02 day02/day02.cpp)
add_executable(day03 day03/day03.cpp)
add_executable(day04 day04/day04.cpp)
add_executable(day05 day05/day05.cpp)
add_executable(day06 day06/day06.cpp)
add_executable(day09 day09/day09.cpp)
add_executable(day10 day10/day10.cpp)
add_executable(day11 day11/day11.cpp)
add_executable(day12 day12/day12.cpp)
add_executable(day17 day17/day17.cpp)
add_executable(day22 day22/day22.cpp)
add_executable(day25 day25/day25.cpp)

# ----------------------------------------
execute_process(
  COMMAND ${CMAKE_COMMAND} -E create_symlink
    ${CMAKE_BINARY_DIR}/compile_commands.json
    ${CMAKE_SOURCE_DIR}/compile_commands.json
)
