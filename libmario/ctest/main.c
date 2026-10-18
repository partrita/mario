#include "../include/libmario.h"

#include <stdatomic.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

static atomic_int wakeups = 0;

static void on_wakeup(void *userdata, mario_surface_id_t surface) {
  (void)userdata;
  (void)surface;
  atomic_fetch_add(&wakeups, 1);
}

static int wait_for(mario_render_state_t *state, const char *needle) {
  for (int attempt = 0; attempt < 200; attempt++) {
    mario_render_state_update(state);
    uint16_t lines = mario_render_state_lines(state);
    uint16_t cols = mario_render_state_columns(state);
    for (uint16_t line = 0; line < lines; line++) {
      char text[512] = {0};
      uint16_t limit = cols < 511 ? cols : 511;
      for (uint16_t col = 0; col < limit; col++) {
        mario_cell_s cell = mario_render_state_cell(state, line, col);
        text[col] = (cell.codepoint >= 32 && cell.codepoint < 127)
                        ? (char)cell.codepoint
                        : ' ';
      }
      if (strstr(text, needle)) {
        printf("row %u: %s\n", line, text);
        return 1;
      }
    }
    usleep(25 * 1000);
  }
  return 0;
}

int main(void) {
  mario_runtime_config_s config = {0};
  config.wakeup_cb = on_wakeup;

  mario_engine_t *engine = mario_engine_new(&config);
  if (!engine) {
    fprintf(stderr, "engine creation failed\n");
    return 1;
  }

  mario_surface_config_s surface_config = {0};
  surface_config.cols = 80;
  surface_config.rows = 24;
  surface_config.pixel_width = 720;
  surface_config.pixel_height = 432;
  surface_config.scrollback = 1000;

  mario_surface_t *surface = mario_surface_new(engine, &surface_config);
  if (!surface) {
    fprintf(stderr, "surface creation failed\n");
    return 1;
  }

  usleep(400 * 1000);
  const char *cmd = "printf '%s%s\\n' li bmario-cgate\r";
  mario_surface_text(surface, cmd, strlen(cmd));

  mario_render_state_t *state = mario_render_state_new(surface);
  int found = wait_for(state, "libmario-cgate");

  /* Paste into a cat child: no line editor in the way, so the check does
   * not depend on the host's shell. The tty's own echo puts it on screen. */
  mario_surface_config_s paste_config = surface_config;
  paste_config.shell = "/bin/cat";
  mario_surface_t *paste_surface = mario_surface_new(engine, &paste_config);
  mario_render_state_t *paste_state =
      paste_surface ? mario_render_state_new(paste_surface) : NULL;
  int found_paste = 0;
  int found_bracketed = 0;
  if (paste_surface) {
    const char *pasted = "libmario-pgate";
    mario_surface_paste(paste_surface, pasted, strlen(pasted));
    found_paste = wait_for(paste_state, "libmario-pgate");

    /* Enable mode 2004 through the child: cat copies the sequence back to
     * its stdout, which the terminal parses. The next paste must then wrap
     * in markers, echoed as ^[[200~..^[[201~. */
    const char *decset = "\x1b[?2004h\n";
    mario_surface_text(paste_surface, decset, strlen(decset));
    for (int attempt = 0; attempt < 200; attempt++) {
      if (mario_surface_mode_bits(paste_surface) & (1u << 3)) {
        break;
      }
      usleep(25 * 1000);
    }
    const char *bracketed = "libmario-bgate";
    mario_surface_paste(paste_surface, bracketed, strlen(bracketed));
    found_bracketed = wait_for(paste_state, "[200~libmario-bgate");
  }

  mario_cursor_s cursor = mario_render_state_cursor(state);
  printf("wakeups=%d cursor=%u,%u found=%d found_paste=%d found_bracketed=%d\n",
         atomic_load(&wakeups), cursor.line, cursor.column, found,
         found_paste, found_bracketed);

  if (paste_state) {
    mario_render_state_free(paste_state);
  }
  if (paste_surface) {
    mario_surface_free(paste_surface);
  }
  mario_render_state_free(state);
  mario_surface_free(surface);
  mario_engine_free(engine);

  if (!found || !found_paste || !found_bracketed ||
      atomic_load(&wakeups) == 0) {
    fprintf(stderr, "gate failed\n");
    return 1;
  }
  printf("libmario c gate passed\n");
  return 0;
}
