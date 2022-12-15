#include <stdio.h>
#include <taffy.h>
#include <math.h>

typedef struct Layout {
    float x;
    float y;
    float width;
    float height;
    int childCount;
    void** children;
} Layout;

void* create_layout(const float* f)
{
    Layout* layout = malloc(sizeof(struct Layout));

    layout->x = *f;
    f++;
    layout->y = *f;
    f++;
    layout->width = *f;
    f++;
    layout->height = *f;
    f++;
    layout->childCount = *f;
    f++;

    layout->children = malloc(sizeof(void *) * layout->childCount);
    for (int i = 0; i < layout->childCount; i++) {
        layout->children[i] = create_layout(f);
    }

    return layout;
}

void print_layout(Layout* layout, int level)
{
    char *prefix = malloc(level * 2);
    memset(prefix, ' ', level * 2);
    prefix[level * 2] = 0;

    printf(
           "%sx: %f, y: %f, width: %f, height: %f, childCount: %i\n",
           prefix,
           layout->x,
           layout->y,
           layout->width,
           layout->height,
           layout->childCount);
    for (int i = 0; i < layout->childCount; i++) {
        print_layout(layout->children[i], level+1);
    }
}

int main(int argc, char const *argv[])
{

    StretchStyleDimension defaultStretchStyleDimension = {
        3, //undefined
        0
    };

    StretchStyleRect defaultStretchStyleRect = {
        defaultStretchStyleDimension,
        defaultStretchStyleDimension,
        defaultStretchStyleDimension,
        defaultStretchStyleDimension
    };
    StretchStyleSize defaultStretchStyleSize = {
        defaultStretchStyleDimension,
        defaultStretchStyleDimension
    };

    void* stretch2 = stretch_init();

    void* child_style = stretch_style_create(
                                             0, // display
                                             0, // position_type
                                             0, // direction
                                             0, // flex_direction
                                             0, // flex_wrap
                                             0, // overflow
                                             0, // align_items
                                             0, // align_self
                                             0, // align_content
                                             0, // justify_content
                                             defaultStretchStyleRect, // position
                                             defaultStretchStyleRect, // margin
                                             defaultStretchStyleRect, // padding
                                             defaultStretchStyleRect, // border
                                             0, // flex_grow
                                             0, // flex_shrink
                                             defaultStretchStyleDimension, // flex_basis
                                             (StretchStyleSize) {
                                                 (StretchStyleDimension){1, 0.5},
                                                 (StretchStyleDimension){2, 0.0},
                                             }, // style
                                             defaultStretchStyleSize, // min_size
                                             defaultStretchStyleSize, // max_size,
                                             NAN); // aspect_ratio

    void* child = stretch_node_create(stretch2, child_style);

    void* node_style = stretch_style_create(
                                           0, // display
                                           0, // position_type
                                           0, // direction
                                           0, // flex_direction
                                           0, // flex_wrap
                                           0, // overflow
                                           0, // align_items
                                           0, // align_self
                                           0, // align_content
                                           2, // justify_content
                                           defaultStretchStyleRect, // position
                                           defaultStretchStyleRect, // margin
                                           defaultStretchStyleRect, // padding
                                           defaultStretchStyleRect, // border
                                           0, // flex_grow
                                           0, // flex_shrink
                                           defaultStretchStyleDimension, // flex_basis
                                           (StretchStyleSize) {
                                               (StretchStyleDimension){0, 100},
                                               (StretchStyleDimension){0, 100},
                                           }, // style
                                           defaultStretchStyleSize, // min_size
                                           defaultStretchStyleSize, // max_size,
                                           NAN); // aspect_ratio

    void* node = stretch_node_create(stretch2, node_style);

    stretch_node_add_child(stretch2, node, child);

    void* output = stretch_node_compute_layout(stretch2, node, NAN, NAN, create_layout);
    /*     let node = stretch.new_node( */
    /*         Style { */
    /*             size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) }, */
    /*             justify_content: JustifyContent::Center, */
    /*             ..Default::default() */
    /*         }, */
    /*         vec![child], */
    /*     )?; */

    /*     stretch.compute_layout(node, Size::undefined())?; */
    /*     dbg!(stretch.layout(node)?); */
    /* } */

    Layout* layout = (Layout*) output;
    print_layout(output, 0);
    return 0;
}
