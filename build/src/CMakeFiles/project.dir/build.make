# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.19

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Disable VCS-based implicit rules.
% : %,v


# Disable VCS-based implicit rules.
% : RCS/%


# Disable VCS-based implicit rules.
% : RCS/%,v


# Disable VCS-based implicit rules.
% : SCCS/s.%


# Disable VCS-based implicit rules.
% : s.%


.SUFFIXES: .hpux_make_needs_suffix_list


# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/local/bin/cmake

# The command to remove a file.
RM = /usr/local/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /mnt/c/Users/user/Desktop/proj/plus

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /mnt/c/Users/user/Desktop/proj/plus/build

# Include any dependencies generated for this target.
include src/CMakeFiles/project.dir/depend.make

# Include the progress variables for this target.
include src/CMakeFiles/project.dir/progress.make

# Include the compile flags for this target's objects.
include src/CMakeFiles/project.dir/flags.make

src/CMakeFiles/project.dir/main.cpp.o: src/CMakeFiles/project.dir/flags.make
src/CMakeFiles/project.dir/main.cpp.o: ../src/main.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/mnt/c/Users/user/Desktop/proj/plus/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object src/CMakeFiles/project.dir/main.cpp.o"
	cd /mnt/c/Users/user/Desktop/proj/plus/build/src && /usr/bin/g++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/project.dir/main.cpp.o -c /mnt/c/Users/user/Desktop/proj/plus/src/main.cpp

src/CMakeFiles/project.dir/main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/project.dir/main.cpp.i"
	cd /mnt/c/Users/user/Desktop/proj/plus/build/src && /usr/bin/g++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /mnt/c/Users/user/Desktop/proj/plus/src/main.cpp > CMakeFiles/project.dir/main.cpp.i

src/CMakeFiles/project.dir/main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/project.dir/main.cpp.s"
	cd /mnt/c/Users/user/Desktop/proj/plus/build/src && /usr/bin/g++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /mnt/c/Users/user/Desktop/proj/plus/src/main.cpp -o CMakeFiles/project.dir/main.cpp.s

# Object files for target project
project_OBJECTS = \
"CMakeFiles/project.dir/main.cpp.o"

# External object files for target project
project_EXTERNAL_OBJECTS =

../bin/project: src/CMakeFiles/project.dir/main.cpp.o
../bin/project: src/CMakeFiles/project.dir/build.make
../bin/project: src/CMakeFiles/project.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/mnt/c/Users/user/Desktop/proj/plus/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX executable ../../bin/project"
	cd /mnt/c/Users/user/Desktop/proj/plus/build/src && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/project.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
src/CMakeFiles/project.dir/build: ../bin/project

.PHONY : src/CMakeFiles/project.dir/build

src/CMakeFiles/project.dir/clean:
	cd /mnt/c/Users/user/Desktop/proj/plus/build/src && $(CMAKE_COMMAND) -P CMakeFiles/project.dir/cmake_clean.cmake
.PHONY : src/CMakeFiles/project.dir/clean

src/CMakeFiles/project.dir/depend:
	cd /mnt/c/Users/user/Desktop/proj/plus/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /mnt/c/Users/user/Desktop/proj/plus /mnt/c/Users/user/Desktop/proj/plus/src /mnt/c/Users/user/Desktop/proj/plus/build /mnt/c/Users/user/Desktop/proj/plus/build/src /mnt/c/Users/user/Desktop/proj/plus/build/src/CMakeFiles/project.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : src/CMakeFiles/project.dir/depend

