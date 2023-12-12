cmake_minimum_required(VERSION 3.5)
project(TypeChanges)

set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")
add_executable(array_bounds array_bounds.cpp)