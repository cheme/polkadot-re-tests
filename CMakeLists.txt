set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

cmake_minimum_required (VERSION 3.0)
project (PolkadotRuntimeEnvironmentTests)

# The various Polkadot RE implementations submodules
###################################################
add_subdirectory(implementations)

add_custom_target(git_submodule_init ALL COMMAND git submodule update --init --recursive)

# The test helpers in various languages
#######################################
add_subdirectory(test/testers)

# The Julia tester
##############################
# TBA
