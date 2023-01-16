cmake_minimum_required(VERSION 3.20)
project(unzip_tool)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Use submodule zlib/minizip on windows, and system lib on Linux
IF (WIN32)
  include_directories(zlib/contrib zlib ${CMAKE_CURRENT_BINARY_DIR})

  add_subdirectory(zlib)

  file(RENAME zlib/zconf.h.included zlib/zconf.h)

  add_library(minizip
          zlib/contrib/minizip/ioapi.c
          zlib/contrib/minizip/miniunz.c
          zlib/contrib/minizip/minizip.c
          zlib/contrib/minizip/mztools.c
          zlib/contrib/minizip/unzip.c
          zlib/contrib/minizip/zip.c)

  target_link_libraries(minizip zlibstatic)
ENDIF (WIN32)

add_library(unzip_tool unzip_tool.cpp)
target_link_libraries(unzip_tool minizip zlibstatic)
