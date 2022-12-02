cmake_minimum_required(VERSION 3.24)

project(aocpp_01)

add_executable(${PROJECT_NAME}
    aocpp_01.cpp
)

set_target_properties(${PROJECT_NAME}
    PROPERTIES
        CXX_STANDARD 20
        CXX_STANDARD_REQUIRED ON
)

target_link_libraries(${PROJECT_NAME}
    stdc++fs
)

target_compile_options(${PROJECT_NAME}
    PRIVATE
        -Wall
)
