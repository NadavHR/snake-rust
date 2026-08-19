default_target: all
OUTPUT_DIR = ../lib
CC = gcc
TRIPLET = x64-linux
SOURCE_FILE = c_functions.c
INCLUDE_FILES = -Ivcpkg_installed/${TRIPLET}/include -I./
LINK_DIRS := vcpkg_installed/${TRIPLET}/lib 
OPTIONS = -c -fPIC
SDL_LINK = -lSDL2
all:
	vcpkg install --triplet $(TRIPLET)
	$(CC) $(SOURCE_FILE) $(OPTIONS) $(INCLUDE_FILES) -o $(OUTPUT_DIR)/c_functions.o
	ar crs $(OUTPUT_DIR)/libc_functions.a $(OUTPUT_DIR)/c_functions.o $(foreach dir, $(LINK_DIRS), $(wildcard $(dir)/*.a))
clean:
	rm -f $(OUTPUT_DIR)/c_functions.o $(OUTPUT_DIR)/libc_functions.a