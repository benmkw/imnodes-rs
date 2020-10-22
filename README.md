
# imnodes-rs

bacially a copy of implot-rs but for imnodes

# TODO

odes/cimnodes.cpp"
  cargo:warning=third-party/cimnodes/cimnodes.cpp:44:12: error: cannot initialize return object of type 'EditorContext *' with an rvalue of type 'imnodes::EditorContext *'
  cargo:warning=    return imnodes::EditorContextCreate();
  cargo:warning=           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  cargo:warning=third-party/cimnodes/cimnodes.cpp:48:39: error: cannot initialize a parameter of type 'imnodes::EditorContext *' with an lvalue of type 'EditorContext *'
  cargo:warning=    return imnodes::EditorContextFree(noname1);
  cargo:warning=                                      ^~~~~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:148:38: note: passing argument to parameter here
  cargo:warning=void EditorContextFree(EditorContext*);
  cargo:warning=                                     ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:52:38: error: cannot initialize a parameter of type 'imnodes::EditorContext *' with an lvalue of type 'EditorContext *'
  cargo:warning=    return imnodes::EditorContextSet(noname1);
  cargo:warning=                                     ^~~~~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:149:37: note: passing argument to parameter here
  cargo:warning=void EditorContextSet(EditorContext*);
  cargo:warning=                                    ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:76:12: error: cannot initialize return object of type 'IO *' with an rvalue of type 'imnodes::IO *'
  cargo:warning=    return &imnodes::GetIO();
  cargo:warning=           ^~~~~~~~~~~~~~~~~
  cargo:warning=third-party/cimnodes/cimnodes.cpp:80:12: error: cannot initialize return object of type 'Style *' with an rvalue of type 'imnodes::Style *'
  cargo:warning=    return &imnodes::GetStyle();
  cargo:warning=           ^~~~~~~~~~~~~~~~~~~~
  cargo:warning=third-party/cimnodes/cimnodes.cpp:104:36: error: cannot initialize a parameter of type 'imnodes::ColorStyle' with an lvalue of type 'ColorStyle'
  cargo:warning=    return imnodes::PushColorStyle(item,color);
  cargo:warning=                                   ^~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:173:32: note: passing argument to parameter 'item' here
  cargo:warning=void PushColorStyle(ColorStyle item, unsigned int color);
  cargo:warning=                               ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:112:34: error: cannot initialize a parameter of type 'imnodes::StyleVar' with an lvalue of type 'StyleVar'
  cargo:warning=    return imnodes::PushStyleVar(style_item,value);
  cargo:warning=                                 ^~~~~~~~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:175:28: note: passing argument to parameter 'style_item' here
  cargo:warning=void PushStyleVar(StyleVar style_item, float value);
  cargo:warning=                           ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:140:44: error: cannot initialize a parameter of type 'imnodes::PinShape' with an lvalue of type 'PinShape'
  cargo:warning=    return imnodes::BeginInputAttribute(id,shape);
  cargo:warning=                                           ^~~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:199:43: note: passing argument to parameter 'shape' here
  cargo:warning=void BeginInputAttribute(int id, PinShape shape = PinShape_CircleFilled);
  cargo:warning=                                          ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:148:45: error: cannot initialize a parameter of type 'imnodes::PinShape' with an lvalue of type 'PinShape'
  cargo:warning=    return imnodes::BeginOutputAttribute(id,shape);
  cargo:warning=                                            ^~~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:202:44: note: passing argument to parameter 'shape' here
  cargo:warning=void BeginOutputAttribute(int id, PinShape shape = PinShape_CircleFilled);
  cargo:warning=                                           ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:164:39: error: cannot initialize a parameter of type 'imnodes::AttributeFlags' with an lvalue of type 'AttributeFlags'
  cargo:warning=    return imnodes::PushAttributeFlag(flag);
  cargo:warning=                                      ^~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:211:39: note: passing argument to parameter 'flag' here
  cargo:warning=void PushAttributeFlag(AttributeFlags flag);
  cargo:warning=                                      ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:268:48: error: cannot initialize a parameter of type 'const imnodes::EditorContext *' with an lvalue of type 'const EditorContext *'
  cargo:warning=    return imnodes::SaveEditorStateToIniString(editor,data_size);
  cargo:warning=                                               ^~~~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:293:61: note: passing argument to parameter 'editor' here
  cargo:warning=const char* SaveEditorStateToIniString(const EditorContext* editor, size_t* data_size = NULL);
  cargo:warning=                                                            ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:276:50: error: cannot initialize a parameter of type 'imnodes::EditorContext *' with an lvalue of type 'EditorContext *'
  cargo:warning=    return imnodes::LoadEditorStateFromIniString(editor,data,data_size);
  cargo:warning=                                                 ^~~~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:296:50: note: passing argument to parameter 'editor' here
  cargo:warning=void LoadEditorStateFromIniString(EditorContext* editor, const char* data, size_t data_size);
  cargo:warning=                                                 ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:284:46: error: cannot initialize a parameter of type 'const imnodes::EditorContext *' with an lvalue of type 'const EditorContext *'
  cargo:warning=    return imnodes::SaveEditorStateToIniFile(editor,file_name);
  cargo:warning=                                             ^~~~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:299:52: note: passing argument to parameter 'editor' here
  cargo:warning=void SaveEditorStateToIniFile(const EditorContext* editor, const char* file_name);
  cargo:warning=                                                   ^
  cargo:warning=third-party/cimnodes/cimnodes.cpp:292:48: error: cannot initialize a parameter of type 'imnodes::EditorContext *' with an lvalue of type 'EditorContext *'
  cargo:warning=    return imnodes::LoadEditorStateFromIniFile(editor,file_name);
  cargo:warning=                                               ^~~~~~
  cargo:warning=third-party/cimnodes/./imnodes/imnodes.h:302:48: note: passing argument to parameter 'editor' here
  cargo:warning=void LoadEditorStateFromIniFile(EditorContext* editor, const char* file_name);
  cargo:warning=                                               ^
  cargo:warning=14 errors generated.
  exit code: 1
