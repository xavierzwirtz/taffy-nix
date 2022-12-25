#include <stdio.h>
#include <taffy.h>
#include <math.h>
#include <string.h>

typedef struct Layout {
    float x;
    float y;
    float width;
    float height;
    int childCount;
    void** children;
} Layout;

void* create_layout_inner(float** f)
{
    Layout* layout = malloc(sizeof(struct Layout));

    layout->x = **f;
    (*f)++;
    layout->y = **f;
    (*f)++;
    layout->width = **f;
    (*f)++;
    layout->height = **f;
    (*f)++;
    layout->childCount = **f;
    (*f)++;

    layout->children = malloc(sizeof(void *) * layout->childCount);
    for (int i = 0; i < layout->childCount; i++) {
        layout->children[i] = create_layout_inner(f);
    }

    return layout;
}

void* create_layout(const float* f)
{
    Layout* layout = create_layout_inner((float **)&f);
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

    TaffyStyleDimension defaultTaffyStyleDimension = {
        2, // auto
        0
    };

    TaffyStyleDimension taffyLengthPercentageZero = {
        0, // points
        0
    };

    TaffyStyleRect defaultTaffyStyleRect = {
        defaultTaffyStyleDimension,
        defaultTaffyStyleDimension,
        defaultTaffyStyleDimension,
        defaultTaffyStyleDimension
    };
    TaffyStyleRect defaultTaffyStyleRect_Zero = {
        taffyLengthPercentageZero,
        taffyLengthPercentageZero,
        taffyLengthPercentageZero,
        taffyLengthPercentageZero
    };

    TaffyStyleSize defaultTaffyStyleSize = {
        defaultTaffyStyleDimension,
        defaultTaffyStyleDimension
    };

    TaffyStyleSize defaultTaffyStyleSize_Zero = {
        taffyLengthPercentageZero,
        taffyLengthPercentageZero
    };


    void* taffy = taffy_init();

    void* child_style1 = taffy_style_create(
                                             0, // display
                                             0, // position_type
                                             0, // flex_direction
                                             0, // flex_wrap
                                             0, // align_items
                                             0, // align_self
                                             0, // align_content
                                             0, // justify_content
                                             defaultTaffyStyleRect, // position
                                             defaultTaffyStyleRect_Zero, // margin
                                             defaultTaffyStyleRect_Zero, // padding
                                             defaultTaffyStyleRect_Zero, // border
                                             defaultTaffyStyleSize_Zero, // gap
                                             0, // flex_grow
                                             1, // flex_shrink
                                             defaultTaffyStyleDimension, // flex_basis
                                             (TaffyStyleSize) {
                                                 (TaffyStyleDimension){0, 10},
                                                 (TaffyStyleDimension){0, 10},
                                             }, // style
                                             defaultTaffyStyleSize, // min_size
                                             defaultTaffyStyleSize, // max_size,
                                             (TaffyStyleDimension) { 0, 0 }); // aspect_ratio

    void* child1 = taffy_node_create(taffy, child_style1);

    void* child_style2 = taffy_style_create(
                                             0, // display
                                             0, // position_type
                                             0, // flex_direction
                                             0, // flex_wrap
                                             0, // align_items
                                             0, // align_self
                                             0, // align_content
                                             0, // justify_content
                                             defaultTaffyStyleRect, // position
                                             defaultTaffyStyleRect_Zero, // margin
                                             defaultTaffyStyleRect_Zero, // padding
                                             defaultTaffyStyleRect_Zero, // border
                                             defaultTaffyStyleSize_Zero, // gap
                                             0, // flex_grow
                                             1, // flex_shrink
                                             defaultTaffyStyleDimension, // flex_basis
                                             (TaffyStyleSize) {
                                                 (TaffyStyleDimension){0, 20},
                                                 (TaffyStyleDimension){0, 20},
                                             }, // style
                                             defaultTaffyStyleSize, // min_size
                                             defaultTaffyStyleSize, // max_size,
                                             (TaffyStyleDimension) { 0, 0 }); // aspect_ratio

    void* child2 = taffy_node_create(taffy, child_style2);

    void* node_style = taffy_style_create(
                                           0, // display
                                           0, // position_type
                                           0, // flex_direction
                                           0, // flex_wrap
                                           0, // align_items
                                           0, // align_self
                                           0, // align_content
                                           2, // justify_content
                                           defaultTaffyStyleRect, // position
                                           defaultTaffyStyleRect_Zero, // margin
                                           defaultTaffyStyleRect_Zero, // padding
                                           defaultTaffyStyleRect_Zero, // border
                                           defaultTaffyStyleSize_Zero, // gap
                                           0, // flex_grow
                                           1, // flex_shrink
                                           defaultTaffyStyleDimension, // flex_basis
                                           (TaffyStyleSize) {
                                               (TaffyStyleDimension){0, 100},
                                               (TaffyStyleDimension){0, 100},
                                           }, // style
                                           defaultTaffyStyleSize, // min_size
                                           defaultTaffyStyleSize, // max_size,
                                           (TaffyStyleDimension) { 0, 0 }
                                           ); // aspect_ratio

    void* node = taffy_node_create(taffy, node_style);

    taffy_node_add_child(taffy, node, child1);
    taffy_node_add_child(taffy, node, child2);

    void* output = taffy_node_compute_layout(taffy,
                                             node,
                                             (TaffyStyleDimension) { 1, 100 },
                                             (TaffyStyleDimension) { 1, 100 },
                                             create_layout);

    Layout* layout = (Layout*) output;
    print_layout(output, 0);
    return 0;
}
