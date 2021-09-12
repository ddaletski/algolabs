cmake_minimum_required(VERSION 3.1)

project(bst)

SET(CMAKE_CXX_STANDARD 11)
find_package(Threads REQUIRED)

add_executable(bst "bst.cpp")

enable_testing()
add_executable(run_tests "test.cpp")
target_link_libraries(run_tests "-lgtest" ${CMAKE_THREAD_LIBS_INIT})
add_test(test1 "run_tests")

