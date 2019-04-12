# Kagome pre-requisits
########################
find_package(Boost REQUIRED)

set(KAGOME_DIR "cpp/kagome")
set(KAGOME_GSL_DIR "${KAGOME_DIR}/deps/GSL")
set(KAGOME_SCALE_DIR "${KAGOME_DIR}/core/scale/")

include_directories("${KAGOME_DIR}/core" "${KAGOME_DIR}/deps/outcome" "${KAGOME_GSL_DIR}/include")

add_subdirectory("${KAGOME_SCALE_DIR}")
