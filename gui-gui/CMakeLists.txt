cmake_minimum_required(VERSION 3.20.0)
project(atrofac-gui-gui VERSION 0.1.0)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR})

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_EXTENSIONS off)

file(GLOB_RECURSE SRC src/*.cpp)

# add files
add_executable(atrofac-gui-gui WIN32 main.cpp ${SRC})

include_directories(include)
include_directories(ext_include)
include_directories(FL)

target_link_libraries(atrofac-gui-gui fltk)
