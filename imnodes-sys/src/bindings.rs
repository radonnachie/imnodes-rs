/* automatically generated by rust-bindgen 0.60.1 */

pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type size_t = __darwin_size_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImGuiContext {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImVec2 {
    pub x: f32,
    pub y: f32,
}
#[test]
fn bindgen_test_layout_ImVec2() {
    assert_eq!(
        ::std::mem::size_of::<ImVec2>(),
        8usize,
        concat!("Size of: ", stringify!(ImVec2))
    );
    assert_eq!(
        ::std::mem::align_of::<ImVec2>(),
        4usize,
        concat!("Alignment of ", stringify!(ImVec2))
    );
    fn test_field_x() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImVec2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(ImVec2), "::", stringify!(x))
        );
    }
    test_field_x();
    fn test_field_y() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImVec2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize
            },
            4usize,
            concat!("Offset of field: ", stringify!(ImVec2), "::", stringify!(y))
        );
    }
    test_field_y();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImNodesContext {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImNodesEditorContext {
    _unused: [u8; 0],
}
pub type ImNodesCol = ::std::os::raw::c_int;
pub type ImNodesStyleVar = ::std::os::raw::c_int;
pub type ImNodesStyleFlags = ::std::os::raw::c_int;
pub type ImNodesPinShape = ::std::os::raw::c_int;
pub type ImNodesAttributeFlags = ::std::os::raw::c_int;
pub type ImNodesMiniMapLocation = ::std::os::raw::c_int;
pub const ImNodesCol__ImNodesCol_NodeBackground: ImNodesCol_ = 0;
pub const ImNodesCol__ImNodesCol_NodeBackgroundHovered: ImNodesCol_ = 1;
pub const ImNodesCol__ImNodesCol_NodeBackgroundSelected: ImNodesCol_ = 2;
pub const ImNodesCol__ImNodesCol_NodeOutline: ImNodesCol_ = 3;
pub const ImNodesCol__ImNodesCol_TitleBar: ImNodesCol_ = 4;
pub const ImNodesCol__ImNodesCol_TitleBarHovered: ImNodesCol_ = 5;
pub const ImNodesCol__ImNodesCol_TitleBarSelected: ImNodesCol_ = 6;
pub const ImNodesCol__ImNodesCol_Link: ImNodesCol_ = 7;
pub const ImNodesCol__ImNodesCol_LinkHovered: ImNodesCol_ = 8;
pub const ImNodesCol__ImNodesCol_LinkSelected: ImNodesCol_ = 9;
pub const ImNodesCol__ImNodesCol_Pin: ImNodesCol_ = 10;
pub const ImNodesCol__ImNodesCol_PinHovered: ImNodesCol_ = 11;
pub const ImNodesCol__ImNodesCol_BoxSelector: ImNodesCol_ = 12;
pub const ImNodesCol__ImNodesCol_BoxSelectorOutline: ImNodesCol_ = 13;
pub const ImNodesCol__ImNodesCol_GridBackground: ImNodesCol_ = 14;
pub const ImNodesCol__ImNodesCol_GridLine: ImNodesCol_ = 15;
pub const ImNodesCol__ImNodesCol_GridLinePrimary: ImNodesCol_ = 16;
pub const ImNodesCol__ImNodesCol_MiniMapBackground: ImNodesCol_ = 17;
pub const ImNodesCol__ImNodesCol_MiniMapBackgroundHovered: ImNodesCol_ = 18;
pub const ImNodesCol__ImNodesCol_MiniMapOutline: ImNodesCol_ = 19;
pub const ImNodesCol__ImNodesCol_MiniMapOutlineHovered: ImNodesCol_ = 20;
pub const ImNodesCol__ImNodesCol_MiniMapNodeBackground: ImNodesCol_ = 21;
pub const ImNodesCol__ImNodesCol_MiniMapNodeBackgroundHovered: ImNodesCol_ = 22;
pub const ImNodesCol__ImNodesCol_MiniMapNodeBackgroundSelected: ImNodesCol_ = 23;
pub const ImNodesCol__ImNodesCol_MiniMapNodeOutline: ImNodesCol_ = 24;
pub const ImNodesCol__ImNodesCol_MiniMapLink: ImNodesCol_ = 25;
pub const ImNodesCol__ImNodesCol_MiniMapLinkSelected: ImNodesCol_ = 26;
pub const ImNodesCol__ImNodesCol_MiniMapCanvas: ImNodesCol_ = 27;
pub const ImNodesCol__ImNodesCol_MiniMapCanvasOutline: ImNodesCol_ = 28;
pub const ImNodesCol__ImNodesCol_COUNT: ImNodesCol_ = 29;
pub type ImNodesCol_ = ::std::os::raw::c_uint;
pub const ImNodesStyleVar__ImNodesStyleVar_GridSpacing: ImNodesStyleVar_ = 0;
pub const ImNodesStyleVar__ImNodesStyleVar_NodeCornerRounding: ImNodesStyleVar_ = 1;
pub const ImNodesStyleVar__ImNodesStyleVar_NodePadding: ImNodesStyleVar_ = 2;
pub const ImNodesStyleVar__ImNodesStyleVar_NodeBorderThickness: ImNodesStyleVar_ = 3;
pub const ImNodesStyleVar__ImNodesStyleVar_LinkThickness: ImNodesStyleVar_ = 4;
pub const ImNodesStyleVar__ImNodesStyleVar_LinkLineSegmentsPerLength: ImNodesStyleVar_ = 5;
pub const ImNodesStyleVar__ImNodesStyleVar_LinkHoverDistance: ImNodesStyleVar_ = 6;
pub const ImNodesStyleVar__ImNodesStyleVar_PinCircleRadius: ImNodesStyleVar_ = 7;
pub const ImNodesStyleVar__ImNodesStyleVar_PinQuadSideLength: ImNodesStyleVar_ = 8;
pub const ImNodesStyleVar__ImNodesStyleVar_PinTriangleSideLength: ImNodesStyleVar_ = 9;
pub const ImNodesStyleVar__ImNodesStyleVar_PinLineThickness: ImNodesStyleVar_ = 10;
pub const ImNodesStyleVar__ImNodesStyleVar_PinHoverRadius: ImNodesStyleVar_ = 11;
pub const ImNodesStyleVar__ImNodesStyleVar_PinOffset: ImNodesStyleVar_ = 12;
pub const ImNodesStyleVar__ImNodesStyleVar_MiniMapPadding: ImNodesStyleVar_ = 13;
pub const ImNodesStyleVar__ImNodesStyleVar_MiniMapOffset: ImNodesStyleVar_ = 14;
pub const ImNodesStyleVar__ImNodesStyleVar_COUNT: ImNodesStyleVar_ = 15;
pub type ImNodesStyleVar_ = ::std::os::raw::c_uint;
pub const ImNodesStyleFlags__ImNodesStyleFlags_None: ImNodesStyleFlags_ = 0;
pub const ImNodesStyleFlags__ImNodesStyleFlags_NodeOutline: ImNodesStyleFlags_ = 1;
pub const ImNodesStyleFlags__ImNodesStyleFlags_GridLines: ImNodesStyleFlags_ = 4;
pub const ImNodesStyleFlags__ImNodesStyleFlags_GridLinesPrimary: ImNodesStyleFlags_ = 8;
pub const ImNodesStyleFlags__ImNodesStyleFlags_GridSnapping: ImNodesStyleFlags_ = 16;
pub type ImNodesStyleFlags_ = ::std::os::raw::c_uint;
pub const ImNodesPinShape__ImNodesPinShape_Circle: ImNodesPinShape_ = 0;
pub const ImNodesPinShape__ImNodesPinShape_CircleFilled: ImNodesPinShape_ = 1;
pub const ImNodesPinShape__ImNodesPinShape_Triangle: ImNodesPinShape_ = 2;
pub const ImNodesPinShape__ImNodesPinShape_TriangleFilled: ImNodesPinShape_ = 3;
pub const ImNodesPinShape__ImNodesPinShape_Quad: ImNodesPinShape_ = 4;
pub const ImNodesPinShape__ImNodesPinShape_QuadFilled: ImNodesPinShape_ = 5;
pub type ImNodesPinShape_ = ::std::os::raw::c_uint;
pub const ImNodesAttributeFlags__ImNodesAttributeFlags_None: ImNodesAttributeFlags_ = 0;
pub const ImNodesAttributeFlags__ImNodesAttributeFlags_EnableLinkDetachWithDragClick:
    ImNodesAttributeFlags_ = 1;
pub const ImNodesAttributeFlags__ImNodesAttributeFlags_EnableLinkCreationOnSnap:
    ImNodesAttributeFlags_ = 2;
pub type ImNodesAttributeFlags_ = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmulateThreeButtonMouse {
    pub Modifier: *const bool,
}
#[test]
fn bindgen_test_layout_EmulateThreeButtonMouse() {
    assert_eq!(
        ::std::mem::size_of::<EmulateThreeButtonMouse>(),
        8usize,
        concat!("Size of: ", stringify!(EmulateThreeButtonMouse))
    );
    assert_eq!(
        ::std::mem::align_of::<EmulateThreeButtonMouse>(),
        8usize,
        concat!("Alignment of ", stringify!(EmulateThreeButtonMouse))
    );
    fn test_field_Modifier() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<EmulateThreeButtonMouse>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Modifier) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(EmulateThreeButtonMouse),
                "::",
                stringify!(Modifier)
            )
        );
    }
    test_field_Modifier();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LinkDetachWithModifierClick {
    pub Modifier: *const bool,
}
#[test]
fn bindgen_test_layout_LinkDetachWithModifierClick() {
    assert_eq!(
        ::std::mem::size_of::<LinkDetachWithModifierClick>(),
        8usize,
        concat!("Size of: ", stringify!(LinkDetachWithModifierClick))
    );
    assert_eq!(
        ::std::mem::align_of::<LinkDetachWithModifierClick>(),
        8usize,
        concat!("Alignment of ", stringify!(LinkDetachWithModifierClick))
    );
    fn test_field_Modifier() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<LinkDetachWithModifierClick>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Modifier) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(LinkDetachWithModifierClick),
                "::",
                stringify!(Modifier)
            )
        );
    }
    test_field_Modifier();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MultipleSelectModifier {
    pub Modifier: *const bool,
}
#[test]
fn bindgen_test_layout_MultipleSelectModifier() {
    assert_eq!(
        ::std::mem::size_of::<MultipleSelectModifier>(),
        8usize,
        concat!("Size of: ", stringify!(MultipleSelectModifier))
    );
    assert_eq!(
        ::std::mem::align_of::<MultipleSelectModifier>(),
        8usize,
        concat!("Alignment of ", stringify!(MultipleSelectModifier))
    );
    fn test_field_Modifier() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MultipleSelectModifier>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Modifier) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MultipleSelectModifier),
                "::",
                stringify!(Modifier)
            )
        );
    }
    test_field_Modifier();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImNodesIO {
    pub EmulateThreeButtonMouse: EmulateThreeButtonMouse,
    pub LinkDetachWithModifierClick: LinkDetachWithModifierClick,
    pub MultipleSelectModifier: MultipleSelectModifier,
    pub AltMouseButton: ::std::os::raw::c_int,
    pub AutoPanningSpeed: f32,
}
#[test]
fn bindgen_test_layout_ImNodesIO() {
    assert_eq!(
        ::std::mem::size_of::<ImNodesIO>(),
        32usize,
        concat!("Size of: ", stringify!(ImNodesIO))
    );
    assert_eq!(
        ::std::mem::align_of::<ImNodesIO>(),
        8usize,
        concat!("Alignment of ", stringify!(ImNodesIO))
    );
    fn test_field_EmulateThreeButtonMouse() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesIO>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).EmulateThreeButtonMouse) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesIO),
                "::",
                stringify!(EmulateThreeButtonMouse)
            )
        );
    }
    test_field_EmulateThreeButtonMouse();
    fn test_field_LinkDetachWithModifierClick() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesIO>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).LinkDetachWithModifierClick) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesIO),
                "::",
                stringify!(LinkDetachWithModifierClick)
            )
        );
    }
    test_field_LinkDetachWithModifierClick();
    fn test_field_MultipleSelectModifier() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesIO>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).MultipleSelectModifier) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesIO),
                "::",
                stringify!(MultipleSelectModifier)
            )
        );
    }
    test_field_MultipleSelectModifier();
    fn test_field_AltMouseButton() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesIO>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).AltMouseButton) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesIO),
                "::",
                stringify!(AltMouseButton)
            )
        );
    }
    test_field_AltMouseButton();
    fn test_field_AutoPanningSpeed() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesIO>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).AutoPanningSpeed) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesIO),
                "::",
                stringify!(AutoPanningSpeed)
            )
        );
    }
    test_field_AutoPanningSpeed();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImNodesStyle {
    pub GridSpacing: f32,
    pub NodeCornerRounding: f32,
    pub NodePadding: ImVec2,
    pub NodeBorderThickness: f32,
    pub LinkThickness: f32,
    pub LinkLineSegmentsPerLength: f32,
    pub LinkHoverDistance: f32,
    pub PinCircleRadius: f32,
    pub PinQuadSideLength: f32,
    pub PinTriangleSideLength: f32,
    pub PinLineThickness: f32,
    pub PinHoverRadius: f32,
    pub PinOffset: f32,
    pub MiniMapPadding: ImVec2,
    pub MiniMapOffset: ImVec2,
    pub Flags: ImNodesStyleFlags,
    pub Colors: [::std::os::raw::c_uint; 29usize],
}
#[test]
fn bindgen_test_layout_ImNodesStyle() {
    assert_eq!(
        ::std::mem::size_of::<ImNodesStyle>(),
        192usize,
        concat!("Size of: ", stringify!(ImNodesStyle))
    );
    assert_eq!(
        ::std::mem::align_of::<ImNodesStyle>(),
        4usize,
        concat!("Alignment of ", stringify!(ImNodesStyle))
    );
    fn test_field_GridSpacing() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).GridSpacing) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(GridSpacing)
            )
        );
    }
    test_field_GridSpacing();
    fn test_field_NodeCornerRounding() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).NodeCornerRounding) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(NodeCornerRounding)
            )
        );
    }
    test_field_NodeCornerRounding();
    fn test_field_NodePadding() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).NodePadding) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(NodePadding)
            )
        );
    }
    test_field_NodePadding();
    fn test_field_NodeBorderThickness() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).NodeBorderThickness) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(NodeBorderThickness)
            )
        );
    }
    test_field_NodeBorderThickness();
    fn test_field_LinkThickness() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).LinkThickness) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(LinkThickness)
            )
        );
    }
    test_field_LinkThickness();
    fn test_field_LinkLineSegmentsPerLength() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).LinkLineSegmentsPerLength) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(LinkLineSegmentsPerLength)
            )
        );
    }
    test_field_LinkLineSegmentsPerLength();
    fn test_field_LinkHoverDistance() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).LinkHoverDistance) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(LinkHoverDistance)
            )
        );
    }
    test_field_LinkHoverDistance();
    fn test_field_PinCircleRadius() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).PinCircleRadius) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(PinCircleRadius)
            )
        );
    }
    test_field_PinCircleRadius();
    fn test_field_PinQuadSideLength() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).PinQuadSideLength) as usize - ptr as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(PinQuadSideLength)
            )
        );
    }
    test_field_PinQuadSideLength();
    fn test_field_PinTriangleSideLength() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).PinTriangleSideLength) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(PinTriangleSideLength)
            )
        );
    }
    test_field_PinTriangleSideLength();
    fn test_field_PinLineThickness() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).PinLineThickness) as usize - ptr as usize
            },
            44usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(PinLineThickness)
            )
        );
    }
    test_field_PinLineThickness();
    fn test_field_PinHoverRadius() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).PinHoverRadius) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(PinHoverRadius)
            )
        );
    }
    test_field_PinHoverRadius();
    fn test_field_PinOffset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).PinOffset) as usize - ptr as usize
            },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(PinOffset)
            )
        );
    }
    test_field_PinOffset();
    fn test_field_MiniMapPadding() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).MiniMapPadding) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(MiniMapPadding)
            )
        );
    }
    test_field_MiniMapPadding();
    fn test_field_MiniMapOffset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).MiniMapOffset) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(MiniMapOffset)
            )
        );
    }
    test_field_MiniMapOffset();
    fn test_field_Flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Flags) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(Flags)
            )
        );
    }
    test_field_Flags();
    fn test_field_Colors() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ImNodesStyle>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Colors) as usize - ptr as usize
            },
            76usize,
            concat!(
                "Offset of field: ",
                stringify!(ImNodesStyle),
                "::",
                stringify!(Colors)
            )
        );
    }
    test_field_Colors();
}
pub const ImNodesMiniMapLocation__ImNodesMiniMapLocation_BottomLeft: ImNodesMiniMapLocation_ = 0;
pub const ImNodesMiniMapLocation__ImNodesMiniMapLocation_BottomRight: ImNodesMiniMapLocation_ = 1;
pub const ImNodesMiniMapLocation__ImNodesMiniMapLocation_TopLeft: ImNodesMiniMapLocation_ = 2;
pub const ImNodesMiniMapLocation__ImNodesMiniMapLocation_TopRight: ImNodesMiniMapLocation_ = 3;
pub type ImNodesMiniMapLocation_ = ::std::os::raw::c_uint;
pub type ImNodesMiniMapNodeHoveringCallback = ::std::option::Option<
    unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: *mut ::std::os::raw::c_void),
>;
pub type ImNodesMiniMapNodeHoveringCallbackUserData = *mut ::std::os::raw::c_void;
extern "C" {
    pub fn ImNodesIO_ImNodesIO() -> *mut ImNodesIO;
}
extern "C" {
    pub fn ImNodesIO_destroy(self_: *mut ImNodesIO);
}
extern "C" {
    pub fn ImNodesStyle_ImNodesStyle() -> *mut ImNodesStyle;
}
extern "C" {
    pub fn ImNodesStyle_destroy(self_: *mut ImNodesStyle);
}
extern "C" {
    pub fn imnodes_SetImGuiContext(ctx: *mut ImGuiContext);
}
extern "C" {
    pub fn imnodes_CreateContext() -> *mut ImNodesContext;
}
extern "C" {
    pub fn imnodes_DestroyContext(ctx: *mut ImNodesContext);
}
extern "C" {
    pub fn imnodes_GetCurrentContext() -> *mut ImNodesContext;
}
extern "C" {
    pub fn imnodes_SetCurrentContext(ctx: *mut ImNodesContext);
}
extern "C" {
    pub fn imnodes_EditorContextCreate() -> *mut ImNodesEditorContext;
}
extern "C" {
    pub fn imnodes_EditorContextFree(noname1: *mut ImNodesEditorContext);
}
extern "C" {
    pub fn imnodes_EditorContextSet(noname1: *mut ImNodesEditorContext);
}
extern "C" {
    pub fn imnodes_EditorContextGetPanning(pOut: *mut ImVec2);
}
extern "C" {
    pub fn imnodes_EditorContextResetPanning(pos: ImVec2);
}
extern "C" {
    pub fn imnodes_EditorContextMoveToNode(node_id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_GetIO() -> *mut ImNodesIO;
}
extern "C" {
    pub fn imnodes_GetStyle() -> *mut ImNodesStyle;
}
extern "C" {
    pub fn imnodes_StyleColorsDark(dest: *mut ImNodesStyle);
}
extern "C" {
    pub fn imnodes_StyleColorsClassic(dest: *mut ImNodesStyle);
}
extern "C" {
    pub fn imnodes_StyleColorsLight(dest: *mut ImNodesStyle);
}
extern "C" {
    pub fn imnodes_BeginNodeEditor();
}
extern "C" {
    pub fn imnodes_EndNodeEditor();
}
extern "C" {
    pub fn imnodes_MiniMap(
        minimap_size_fraction: f32,
        location: ImNodesMiniMapLocation,
        node_hovering_callback: ImNodesMiniMapNodeHoveringCallback,
        node_hovering_callback_data: ImNodesMiniMapNodeHoveringCallbackUserData,
    );
}
extern "C" {
    pub fn imnodes_PushColorStyle(item: ImNodesCol, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn imnodes_PopColorStyle();
}
extern "C" {
    pub fn imnodes_PushStyleVar_Float(style_item: ImNodesStyleVar, value: f32);
}
extern "C" {
    pub fn imnodes_PushStyleVar_Vec2(style_item: ImNodesStyleVar, value: ImVec2);
}
extern "C" {
    pub fn imnodes_PopStyleVar(count: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_BeginNode(id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_EndNode();
}
extern "C" {
    pub fn imnodes_GetNodeDimensions(pOut: *mut ImVec2, id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_BeginNodeTitleBar();
}
extern "C" {
    pub fn imnodes_EndNodeTitleBar();
}
extern "C" {
    pub fn imnodes_BeginInputAttribute(id: ::std::os::raw::c_int, shape: ImNodesPinShape);
}
extern "C" {
    pub fn imnodes_EndInputAttribute();
}
extern "C" {
    pub fn imnodes_BeginOutputAttribute(id: ::std::os::raw::c_int, shape: ImNodesPinShape);
}
extern "C" {
    pub fn imnodes_EndOutputAttribute();
}
extern "C" {
    pub fn imnodes_BeginStaticAttribute(id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_EndStaticAttribute();
}
extern "C" {
    pub fn imnodes_PushAttributeFlag(flag: ImNodesAttributeFlags);
}
extern "C" {
    pub fn imnodes_PopAttributeFlag();
}
extern "C" {
    pub fn imnodes_Link(
        id: ::std::os::raw::c_int,
        start_attribute_id: ::std::os::raw::c_int,
        end_attribute_id: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn imnodes_SetNodeDraggable(node_id: ::std::os::raw::c_int, draggable: bool);
}
extern "C" {
    pub fn imnodes_SetNodeScreenSpacePos(node_id: ::std::os::raw::c_int, screen_space_pos: ImVec2);
}
extern "C" {
    pub fn imnodes_SetNodeEditorSpacePos(node_id: ::std::os::raw::c_int, editor_space_pos: ImVec2);
}
extern "C" {
    pub fn imnodes_SetNodeGridSpacePos(node_id: ::std::os::raw::c_int, grid_pos: ImVec2);
}
extern "C" {
    pub fn imnodes_GetNodeScreenSpacePos(pOut: *mut ImVec2, node_id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_GetNodeEditorSpacePos(pOut: *mut ImVec2, node_id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_GetNodeGridSpacePos(pOut: *mut ImVec2, node_id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_SnapNodeToGrid(node_id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_IsEditorHovered() -> bool;
}
extern "C" {
    pub fn imnodes_IsNodeHovered(node_id: *mut ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn imnodes_IsLinkHovered(link_id: *mut ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn imnodes_IsPinHovered(attribute_id: *mut ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn imnodes_NumSelectedNodes() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn imnodes_NumSelectedLinks() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn imnodes_GetSelectedNodes(node_ids: *mut ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_GetSelectedLinks(link_ids: *mut ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_ClearNodeSelection_Nil();
}
extern "C" {
    pub fn imnodes_ClearLinkSelection_Nil();
}
extern "C" {
    pub fn imnodes_SelectNode(node_id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_ClearNodeSelection_Int(node_id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_IsNodeSelected(node_id: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn imnodes_SelectLink(link_id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_ClearLinkSelection_Int(link_id: ::std::os::raw::c_int);
}
extern "C" {
    pub fn imnodes_IsLinkSelected(link_id: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn imnodes_IsAttributeActive() -> bool;
}
extern "C" {
    pub fn imnodes_IsAnyAttributeActive(attribute_id: *mut ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn imnodes_IsLinkStarted(started_at_attribute_id: *mut ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn imnodes_IsLinkDropped(
        started_at_attribute_id: *mut ::std::os::raw::c_int,
        including_detached_links: bool,
    ) -> bool;
}
extern "C" {
    pub fn imnodes_IsLinkCreated_BoolPtr(
        started_at_attribute_id: *mut ::std::os::raw::c_int,
        ended_at_attribute_id: *mut ::std::os::raw::c_int,
        created_from_snap: *mut bool,
    ) -> bool;
}
extern "C" {
    pub fn imnodes_IsLinkCreated_IntPtr(
        started_at_node_id: *mut ::std::os::raw::c_int,
        started_at_attribute_id: *mut ::std::os::raw::c_int,
        ended_at_node_id: *mut ::std::os::raw::c_int,
        ended_at_attribute_id: *mut ::std::os::raw::c_int,
        created_from_snap: *mut bool,
    ) -> bool;
}
extern "C" {
    pub fn imnodes_IsLinkDestroyed(link_id: *mut ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn imnodes_SaveCurrentEditorStateToIniString(
        data_size: *mut size_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn imnodes_SaveEditorStateToIniString(
        editor: *const ImNodesEditorContext,
        data_size: *mut size_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn imnodes_LoadCurrentEditorStateFromIniString(
        data: *const ::std::os::raw::c_char,
        data_size: size_t,
    );
}
extern "C" {
    pub fn imnodes_LoadEditorStateFromIniString(
        editor: *mut ImNodesEditorContext,
        data: *const ::std::os::raw::c_char,
        data_size: size_t,
    );
}
extern "C" {
    pub fn imnodes_SaveCurrentEditorStateToIniFile(file_name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn imnodes_SaveEditorStateToIniFile(
        editor: *const ImNodesEditorContext,
        file_name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn imnodes_LoadCurrentEditorStateFromIniFile(file_name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn imnodes_LoadEditorStateFromIniFile(
        editor: *mut ImNodesEditorContext,
        file_name: *const ::std::os::raw::c_char,
    );
}
