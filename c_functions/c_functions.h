#ifndef C_FUNCTIONS
#define C_FUNCTIONS
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

enum SegmentDirection {
    UP = 0,
    DOWN = 1,
    LEFT = 2,
    RIGHT = 3
};

void init(int grid_width, int grid_height, uint8_t grid_unit_size, uint8_t segment_size, uint8_t apple_size);
uint32_t get_time_milis();
void clear_screen();
void draw_segment(unsigned int segment_x, unsigned int segment_y, enum SegmentDirection dir);
void draw_apple(unsigned int apple_x, unsigned int apple_y);
void render();
void update_SDL();
void finish_game();
void set_title(char * title);
bool up, down, left, right, game_over;
#endif