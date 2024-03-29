cmake_minimum_required(VERSION 3.1)
project(lixchess_client C)

set(CMAKE_C_STANDARD 99)

set(SOURCES src/main.c src/render.c src/ui.c src/util.c src/shader.c glad/src/glad.c)
add_executable(lixchess ${SOURCES})

target_include_directories(lixchess PUBLIC include)

target_include_directories(lixchess PUBLIC ../core/include)

target_include_directories(lixchess PUBLIC nuklear glad/include)

target_link_directories(lixchess PUBLIC ../core/target/release)
target_link_libraries(lixchess chess)

find_package(cstructure)

if (NOT cstructure_FOUND)
    add_subdirectory(cstructure)
endif()

find_package(glfw3 REQUIRED)

target_link_libraries(lixchess cstructure_shared)
target_link_libraries(lixchess glfw GL X11 pthread Xrandr Xi dl)