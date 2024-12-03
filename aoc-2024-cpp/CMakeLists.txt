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
