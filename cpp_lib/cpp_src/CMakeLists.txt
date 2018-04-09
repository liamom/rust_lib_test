add_library(cpp_lib STATIC cpp_lib.h cpp_lib.c)

install(TARGETS cpp_lib DESTINATION ${CMAKE_BINARY_DIR}/..)