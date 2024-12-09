cmake_minimum_required(VERSION 3.28)

project(AoC2024)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(fmt REQUIRED)
find_package(flux REQUIRED)

# Link to all
link_libraries(fmt::fmt flux::flux)

add_compile_options(-DBASE_PATH="${CMAKE_SOURCE_DIR}")

# Solutions
add_executable(day01 day01/day01.cpp)
add_executable(day02 day02/day02.cpp)
add_executable(day03 day03/day03.cpp)
add_executable(day04 day04/day04.cpp)
add_executable(day05 day05/day05.cpp)
add_executable(day06 day06/day06.cpp)
add_executable(day09 day09/day09.cpp)

# ----------------------------------------
execute_process(
  COMMAND ${CMAKE_COMMAND} -E create_symlink
    ${CMAKE_BINARY_DIR}/compile_commands.json
    ${CMAKE_SOURCE_DIR}/compile_commands.json
)