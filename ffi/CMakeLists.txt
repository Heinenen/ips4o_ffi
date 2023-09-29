cmake_minimum_required(VERSION 3.10)

project(ips4o_ffi)
SET(IPS4O_USE_OPENMP TRUE)
add_subdirectory(../ips4o build)

add_library(cpp_ips4o SHARED)
target_sources(cpp_ips4o PRIVATE ./example.cpp)
target_link_libraries(cpp_ips4o PRIVATE ips4o)
install(TARGETS cpp_ips4o DESTINATION .)
