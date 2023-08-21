cmake_minimum_required(VERSION 3.20)

set(LINKS_PLATFORM_EXTRA_FLAGS "" CACHE STRING "Extra compiler flags")

project(PSQLBench CXX)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

find_package(benchmark)
find_package(Platform.Data.Doublets)
find_package(libpqxx)

add_library(${PROJECT_NAME}.Library INTERFACE)
target_include_directories(${PROJECT_NAME}.Library INTERFACE LinksPSQL)
target_link_libraries(${PROJECT_NAME}.Library INTERFACE Platform.Data.Doublets::Platform.Data.Doublets)
target_link_libraries(${PROJECT_NAME}.Library INTERFACE libpqxx::pqxx)
target_link_libraries(${PROJECT_NAME}.Library INTERFACE benchmark::benchmark)

target_compile_options(${PROJECT_NAME}.Library INTERFACE ${LINKS_PLATFORM_EXTRA_FLAGS})

add_executable(${PROJECT_NAME} Benchmarks/AllBenchmarks.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE ${PROJECT_NAME}.Library)

set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD 20)
