cmake_minimum_required(VERSION 3.2)

set(SOURCE_FILES
    lib.hpp
    lib.cpp
    old_lib.hpp
    old_lib.cpp
)

add_library(lab1_lib STATIC ${SOURCE_FILES})
add_executable(lab1 main.cpp)

target_link_libraries(lab1 lab1_lib)
