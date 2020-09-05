initSidebarItems({"fn":[["fill_blank","Fill a blank text area (left, top, right, bottom) with color. The tuple specifies the location of the area relative to the origin(top-left point) of the framebuffer."],["print_ascii_character","Prints a character to the framebuffer at position (line, column) of all characters in the text area. # Arguments * `framebuffer`: the framebuffer to display in. * `character`: the ASCII code of the character to display. * `fg_pixel`: the value of every pixel in the character. * `bg_color`: the value of every pixel in the background. * `coordinate`: the left top coordinate of the text block relative to the origin(top-left point) of the framebuffer. * `column`, `line`: the location of the character in the text block as symbols."],["print_string","Prints a string in a framebuffer. Returns (column, line, rectangle), i.e. the position of the next symbol and an rectangle which covers the updated area. A block item (index, width) represents the index of line number and the width of charaters in this line as pixels. It can be viewed as a framebuffer block which is described in the `framebuffer_compositor` crate. # Arguments * `framebuffer`: the framebuffer to display in. * `coordinate`: the left top coordinate of the text block relative to the origin(top-left point) of the framebuffer. * `width`, `height`: the size of the text block in number of pixels. * `slice`: the string to display. * `fg_pixel`: the value of pixels in the foreground. * `bg_pixel` the value of pixels in the background. * `column`, `line`: the location of the text in the text block in number of characters."]]});