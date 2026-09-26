cmake_minimum_required(VERSION 3.16)
project(systemManager CXX)

set(CMAKE_CXX_STANDARD 17)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTKMM REQUIRED IMPORTED_TARGET gtkmm-4.0)

add_executable(systemManager
 src/ui/main.cpp)
target_link_libraries(systemManager PRIVATE PkgConfig::GTKMM)   