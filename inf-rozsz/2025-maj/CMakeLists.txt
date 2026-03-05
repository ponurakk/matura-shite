cmake_minimum_required(VERSION 3.16)
project(zadanko LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/target/cpp)

add_executable(zadanko
    src/main.cpp
)
