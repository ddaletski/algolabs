cmake_minimum_required(VERSION 3.10)
project(BrokenGlass)

set(CMAKE_CXX_STANDARD 17)
find_package(OpenCV REQUIRED)

add_executable(main main.cpp)

target_link_directories(main PUBLIC ${OpenCV_LIB_DIRS})
target_link_libraries(main ${OpenCV_LIBS})
target_include_directories(main PUBLIC ${OpenCV_INCLUDE_DIRS})
