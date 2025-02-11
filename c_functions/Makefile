default_target: all
OUTPUT_DIR = ../lib
CC = gcc
TRIPLET = x64-windows
SOURCE_FILE = c_functions.c
INCLUDE_FILES = -Ivcpkg_installed/${TRIPLET}/include -I./
LINK_DIRS := vcpkg_installed/${TRIPLET}/bin  vcpkg_installed/${TRIPLET}/lib 
OPTIONS= -c -fPIC
SDL_LINK= -llibmingwex -lSDL2
all:
	vcpkg install --triplet $(TRIPLET)
	$(CC) $(SOURCE_FILE) $(OPTIONS) $(INCLUDE_FILES) $(foreach dir, $(LINK_DIRS), -L$(dir)) $(SDL_LINK)  -o $(OUTPUT_DIR)/c_functions.o
	ar crs $(OUTPUT_DIR)/c_functions.lib $(OUTPUT_DIR)/c_functions.o $(foreach dir, $(LINK_DIRS), $(wildcard $(dir)/*.*))