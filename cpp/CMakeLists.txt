cmake_minimum_required(VERSION 3.20)
project(PSQLBench)
include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup(TARGETS)

set(LINKS_PLATFORM_EXTRA_FLAGS "" CACHE STRING "Extra compiler flags")
list(APPEND LINKS_PLATFORM_EXTRA_FLAGS ${CONAN_USER_PLATFORM.HASHING_suggested_flags})

add_library(${PROJECT_NAME}.Library INTERFACE)
target_link_libraries(${PROJECT_NAME}.Library INTERFACE CONAN_PKG::gtest)
target_link_libraries(${PROJECT_NAME}.Library INTERFACE CONAN_PKG::benchmark)
target_link_libraries(${PROJECT_NAME}.Library INTERFACE CONAN_PKG::platform.data)
target_link_libraries(${PROJECT_NAME}.Library INTERFACE CONAN_PKG::platform.ranges)
target_link_libraries(${PROJECT_NAME}.Library INTERFACE CONAN_PKG::platform.setters)
target_link_libraries(${PROJECT_NAME}.Library INTERFACE CONAN_PKG::libpqxx)

target_compile_options(${PROJECT_NAME}.Library INTERFACE ${LINKS_PLATFORM_EXTRA_FLAGS})

add_executable(${PROJECT_NAME} ${PROJECT_NAME}.cpp Client.h Transaction.h)
target_link_libraries(${PROJECT_NAME} PRIVATE ${PROJECT_NAME}.Library)

set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD 20)
