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
add_executable(day1 day1/day1.cpp)
add_executable(day2 day2/day2.cpp)
add_executable(day3 day3/day3.cpp)
