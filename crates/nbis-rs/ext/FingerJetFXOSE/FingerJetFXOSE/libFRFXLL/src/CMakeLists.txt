project( FRFXLL )

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# add include directories
include_directories("${FRFXLL_PATH}/include")
include_directories("${CMAKE_CURRENT_SOURCE_DIR}/algorithm")

set(SOURCE_FILES "")
file( GLOB SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/lib/*.cpp" )

add_library( "${PROJECT_NAME}_static" STATIC
  ${SOURCE_FILES}
)

add_library( ${PROJECT_NAME} SHARED
  ${SOURCE_FILES}
)

foreach(target_name "${PROJECT_NAME}_static" ${PROJECT_NAME})
  if(MSVC)
    target_compile_definitions(${target_name} PRIVATE
      _CRT_SECURE_NO_WARNINGS
      _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING
    )
    target_compile_options(${target_name} PRIVATE /wd4146 /wd4267 /wd4996)
  endif()
endforeach()

if( USE_SANITIZER )
  target_link_libraries( ${PROJECT_NAME} "asan" )
endif()
