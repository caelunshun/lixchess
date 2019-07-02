cmake_minimum_required(VERSION 3.1)
project(lixchess_client C)

set(CMAKE_C_STANDARD 90)

set(SOURCES src/main.c)
add_executable(lixchess ${SOURCES})

target_include_directories(lixchess PUBLIC include)

target_include_directories(lixchess PUBLIC ../core/include)

target_link_directories(lixchess PUBLIC ../core/target/release)
target_link_libraries(lixchess chess)