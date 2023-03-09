cmake_minimum_required(VERSION 3.0)
project(libSaleaeLogic C)

set(CMAKE_C_STANDARD 99)

add_library(saleaeLogic STATIC main.c decode.c device.c input.c options.c output.c parsers.c session.c show.c anykey.c sigrok-cli.h wrapper.h)

find_package(PkgConfig REQUIRED)
pkg_check_modules(deps REQUIRED IMPORTED_TARGET glib-2.0 libsigrok libsigrokdecode)
target_link_libraries(saleaeLogic PkgConfig::deps)

install(TARGETS saleaeLogic DESTINATION .)
