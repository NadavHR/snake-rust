#include "c_functions.h"
#include <SDL2/SDL.h>


static uint8_t _grid_unit_size;
static uint8_t _segment_size;
static uint8_t _segment_connection_size;
static uint8_t _apple_size;
static int _width_grid, _height_grid;
static int _width_px, _height_px;
static SDL_Rect _rect;

static SDL_Event window_event;
static SDL_Window *win;
static SDL_Renderer * renderer;

bool game_over = false;

bool up   = false;
bool down = false;
bool left   = false;
bool right = false;

void clear_screen() {
    SDL_RenderClear(renderer);
    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0);
    SDL_RenderFillRect(renderer, &_rect);
}

void draw_segment(unsigned int segment_x, unsigned int segment_y, enum SegmentDirection dir)
{
    SDL_SetRenderDrawColor(renderer, 140, 255, 50, 0);
    int x_extra = (dir == LEFT) ? 0 : (dir == RIGHT) ? -_segment_connection_size : 0;
    int y_extra = (dir == UP) ? 0 : (dir == DOWN) ? -_segment_connection_size : 0;
    SDL_Rect b = {
        .h = _segment_size + ((dir == UP) ? _segment_connection_size : (dir == DOWN) ? _segment_connection_size : 0),
        .w = _segment_size + ((dir == LEFT) ? _segment_connection_size : (dir == RIGHT) ? _segment_connection_size : 0),
        .x = (int)(segment_x * _grid_unit_size) + x_extra,
        .y = (int)(segment_y * _grid_unit_size) + y_extra
    };
    

    SDL_RenderFillRect(renderer, &b);
    SDL_SetRenderDrawColor(renderer, 30, 190, 20, 0);
    SDL_Rect t = {
        .h = _segment_size,
        .w = _segment_size,
        .x = (int)(segment_x * _grid_unit_size),
        .y = (int)(segment_y * _grid_unit_size)
    };

    SDL_RenderFillRect(renderer, &t);

}

void draw_apple(unsigned int apple_x, unsigned int apple_y)
{
    SDL_SetRenderDrawColor(renderer, 255, 0, 0, 0);
    SDL_Rect b = {
        .h = _apple_size,
        .w = _apple_size,
        .x = (int)(apple_x * _grid_unit_size),
        .y = (int)(apple_y * _grid_unit_size)
    };

    SDL_RenderFillRect(renderer, &b);
}

void render() {
    SDL_RenderPresent(renderer);
}


void init(int grid_width, int grid_height, uint8_t grid_unit_size, uint8_t segment_size, uint8_t apple_size) {
    _grid_unit_size = grid_unit_size;
    _apple_size = apple_size;
    _segment_size = segment_size;
    _segment_connection_size = (grid_unit_size - segment_size);
    _width_grid = grid_width;
    _height_grid = grid_height;
    _width_px = _width_grid * _grid_unit_size - _segment_connection_size;
    _height_px = _height_grid * _grid_unit_size - _segment_connection_size; 
    _rect = (SDL_Rect){.h = _height_px, .w = _width_px};


    SDL_Init(SDL_INIT_EVERYTHING);
    SDL_CreateWindowAndRenderer(_width_px, _height_px, 0, &win, &renderer);

    SDL_RenderSetScale(renderer, 1, 1);
}

uint32_t get_time_milis() {
    return SDL_GetTicks();
}

void finish_game() {
    SDL_DestroyWindow(win);
    SDL_Quit();
}

void set_title(char * title) {
    SDL_SetWindowTitle(win, title);
}

void update_SDL(){
    if (SDL_PollEvent(&window_event))
    {
        switch (window_event.type)
        {
        case SDL_QUIT:
            game_over = true;
            break;
        case SDL_KEYDOWN:
            switch (window_event.key.keysym.scancode)
            {
            case SDL_SCANCODE_UP:
                up = true;
                break;
            case SDL_SCANCODE_DOWN:
                down = true;
                break;
            case SDL_SCANCODE_LEFT:
                left = true;
                break;
            case SDL_SCANCODE_RIGHT:
                right = true;
                break;
            case SDL_SCANCODE_W:
                up = true;
                break;
            case SDL_SCANCODE_S:
                down = true;
                break;
            case SDL_SCANCODE_D:
                right = true;
                break;
            case SDL_SCANCODE_A:
                left = true;
                break;
            default:
                break;
            }
            break;
        case SDL_KEYUP:
            switch (window_event.key.keysym.scancode)
            {
            case SDL_SCANCODE_UP:
                up = false;
                break;
            case SDL_SCANCODE_DOWN:
                down = false;
                break;
            case SDL_SCANCODE_LEFT:
                left = false;
                break;
            case SDL_SCANCODE_RIGHT:
                right = false;
                break;
            case SDL_SCANCODE_W:
                up = false;
                break;
            case SDL_SCANCODE_S:
                down = false;
                break;
            case SDL_SCANCODE_D:
                right = false;
                break;
            case SDL_SCANCODE_A:
                left = false;
                break;
            default:
                break;
            }
            break;
        default:
            break;
        }
    }
}