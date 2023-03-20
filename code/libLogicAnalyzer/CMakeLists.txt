cmake_minimum_required(VERSION 3.0)
project(libLogicAnalyzer C)

set(CMAKE_C_STANDARD 99)

add_library(logicAnalyzer STATIC libLogicAnalyzer.c decode.c device.c session.c libLogicAnalyzer.h input.c)

find_package(PkgConfig REQUIRED)
pkg_check_modules(deps REQUIRED IMPORTED_TARGET glib-2.0 libsigrok libsigrokdecode)
target_link_libraries(logicAnalyzer PkgConfig::deps)

install(TARGETS logicAnalyzer DESTINATION .)
