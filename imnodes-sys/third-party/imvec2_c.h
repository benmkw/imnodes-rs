// ImVec2_c definition for cimnodes compatibility
//
// The cimnodes generator expects cimgui to be a sibling folder and uses
// its `ImVec2_c` type. When using standalone with imgui-sys (which uses
// a different cimgui version that defines ImVec2 instead), we need to
// provide this definition.
//
// This header should be included before cimnodes.h

#ifndef IMVEC2_C_H
#define IMVEC2_C_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ImVec2_c {
    float x;
    float y;
} ImVec2_c;

#ifdef __cplusplus
}
#endif

#endif // IMVEC2_C_H
