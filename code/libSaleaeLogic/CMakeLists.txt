cmake_minimum_required(VERSION 3.0)
project(libSaleaeLogic C)

add_library(saleaeLogic STATIC foo.c)

install(TARGETS saleaeLogic DESTINATION .)
