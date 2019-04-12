set(CMAKE_BUILD_TYPE "Debug")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++14 -pthread -Wall -O0 -ggdb ${SANITIZE}")

##############################
add_executable(scale_codec_cpp scale_codec/scale_codec.cpp)

target_link_libraries(scale_codec_cpp scale)

                                      
