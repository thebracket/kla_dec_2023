cmake_minimum_required(VERSION 3.5)
project(HelloWorld)

set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")
add_executable(hello hello.cpp)