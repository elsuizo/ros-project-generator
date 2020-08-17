cmake_minimum_required(VERSION 3.0)
project({{ node }})

find_package(catkin REQUIRED)
catkin_package()

add_custom_target({{node}}
   ALL
   COMMAND cargo build --release -p {{node}}
   COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_BINARY_DIR}/cargo/release/{{node}} ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/{{node}}
   COMMENT "Building {{node}}!!!"
)
