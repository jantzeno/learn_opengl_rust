pub use self::types::*;
pub use self::enumerations::*;
pub use self::functions::*;

use std::os::raw::c_void;


#[derive(Copy, Clone)]
struct FnPtr {
    ptr: *const c_void,
    is_loaded: bool
}

#[allow(dead_code)]
impl FnPtr {
    fn new(ptr: *const c_void) -> FnPtr {
        if !ptr.is_null() {
            FnPtr { ptr, is_loaded: true }
        } else {
            FnPtr { ptr: FnPtr::not_initialized as *const c_void, is_loaded: false }
        }
    }

    fn set_ptr(&mut self, ptr: *const c_void) {
        *self = Self::new(ptr);
    }
    
    fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            *self = *other;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! { panic!("gl: function not initialized") }
}

unsafe impl Sync for FnPtr {}
unsafe impl Send for FnPtr {}

pub mod types {
#![allow(dead_code, non_snake_case, non_camel_case_types)]

use std::os::raw;

pub type GLvoid = raw::c_void;

pub type GLbyte = raw::c_char;
pub type GLubyte = raw::c_uchar;
pub type GLchar = raw::c_char;
pub type GLboolean = raw::c_uchar;

pub type GLshort = raw::c_short;
pub type GLushort = raw::c_ushort;

pub type GLint = raw::c_int;
pub type GLuint = raw::c_uint;
pub type GLint64 = i64;
pub type GLuint64 = u64;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub type GLsizei = GLint;
pub type GLclampx = raw::c_int;
pub type GLfixed = GLint;
pub type GLhalf = raw::c_ushort;
pub type GLhalfNV = raw::c_ushort;
pub type GLhalfARB = raw::c_ushort;

pub type GLenum = raw::c_uint;
pub type GLbitfield = raw::c_uint;

pub type GLfloat = raw::c_float;
pub type GLdouble = raw::c_double;
pub type GLclampf = raw::c_float;
pub type GLclampd = raw::c_double;

pub type GLcharARB = raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = raw::c_uint;

pub enum __GLsync {}

pub type GLsync = *const __GLsync;

pub enum _cl_context {}

pub enum _cl_event {}

pub type GLvdpauSurfaceNV = GLintptr;
pub type GLeglClientBufferEXT = *const raw::c_void;
pub type GLeglImageOES = *const raw::c_void;


pub type GLDEBUGPROC = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLDEBUGPROCAMD = extern "system" fn (
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLVULKANPROCNV = extern "system" fn ();
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use std::os::raw::*;
    use super::types::*;

    pub const _1PASS_EXT: c_uint = 0x80A1;
    pub const _1PASS_SGIS: c_uint = 0x80A1;
    pub const _2PASS_0_EXT: c_uint = 0x80A2;
    pub const _2PASS_0_SGIS: c_uint = 0x80A2;
    pub const _2PASS_1_EXT: c_uint = 0x80A3;
    pub const _2PASS_1_SGIS: c_uint = 0x80A3;
    pub const _2X_BIT_ATI: c_uint = 0x00000001;
    pub const _422_AVERAGE_EXT: c_uint = 0x80CE;
    pub const _422_EXT: c_uint = 0x80CC;
    pub const _422_REV_AVERAGE_EXT: c_uint = 0x80CF;
    pub const _422_REV_EXT: c_uint = 0x80CD;
    pub const _4PASS_0_EXT: c_uint = 0x80A4;
    pub const _4PASS_0_SGIS: c_uint = 0x80A4;
    pub const _4PASS_1_EXT: c_uint = 0x80A5;
    pub const _4PASS_1_SGIS: c_uint = 0x80A5;
    pub const _4PASS_2_EXT: c_uint = 0x80A6;
    pub const _4PASS_2_SGIS: c_uint = 0x80A6;
    pub const _4PASS_3_EXT: c_uint = 0x80A7;
    pub const _4PASS_3_SGIS: c_uint = 0x80A7;
    pub const _4X_BIT_ATI: c_uint = 0x00000002;
    pub const _8X_BIT_ATI: c_uint = 0x00000004;
    pub const ABGR_EXT: c_uint = 0x8000;
    pub const ACCUM_ADJACENT_PAIRS_NV: c_uint = 0x90AD;
    pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D9;
    pub const ACTIVE_ATTRIBUTES: c_uint = 0x8B89;
    pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: c_uint = 0x8B8A;
    pub const ACTIVE_PROGRAM: c_uint = 0x8259;
    pub const ACTIVE_PROGRAM_EXT: c_uint = 0x8B8D;
    pub const ACTIVE_RESOURCES: c_uint = 0x92F5;
    pub const ACTIVE_STENCIL_FACE_EXT: c_uint = 0x8911;
    pub const ACTIVE_SUBROUTINES: c_uint = 0x8DE5;
    pub const ACTIVE_SUBROUTINE_MAX_LENGTH: c_uint = 0x8E48;
    pub const ACTIVE_SUBROUTINE_UNIFORMS: c_uint = 0x8DE6;
    pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: c_uint = 0x8E47;
    pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: c_uint = 0x8E49;
    pub const ACTIVE_TEXTURE: c_uint = 0x84E0;
    pub const ACTIVE_TEXTURE_ARB: c_uint = 0x84E0;
    pub const ACTIVE_UNIFORMS: c_uint = 0x8B86;
    pub const ACTIVE_UNIFORM_BLOCKS: c_uint = 0x8A36;
    pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: c_uint = 0x8A35;
    pub const ACTIVE_UNIFORM_MAX_LENGTH: c_uint = 0x8B87;
    pub const ACTIVE_VARIABLES: c_uint = 0x9305;
    pub const ACTIVE_VARYINGS_NV: c_uint = 0x8C81;
    pub const ACTIVE_VARYING_MAX_LENGTH_NV: c_uint = 0x8C82;
    pub const ACTIVE_VERTEX_UNITS_ARB: c_uint = 0x86A5;
    pub const ADD_ATI: c_uint = 0x8963;
    pub const ADD_SIGNED_ARB: c_uint = 0x8574;
    pub const ADD_SIGNED_EXT: c_uint = 0x8574;
    pub const ADJACENT_PAIRS_NV: c_uint = 0x90AE;
    pub const AFFINE_2D_NV: c_uint = 0x9092;
    pub const AFFINE_3D_NV: c_uint = 0x9094;
    pub const ALIASED_LINE_WIDTH_RANGE: c_uint = 0x846E;
    pub const ALLOW_DRAW_FRG_HINT_PGI: c_uint = 0x1A210;
    pub const ALLOW_DRAW_MEM_HINT_PGI: c_uint = 0x1A211;
    pub const ALLOW_DRAW_OBJ_HINT_PGI: c_uint = 0x1A20E;
    pub const ALLOW_DRAW_WIN_HINT_PGI: c_uint = 0x1A20F;
    pub const ALL_BARRIER_BITS: c_uint = 0xFFFFFFFF;
    pub const ALL_BARRIER_BITS_EXT: c_uint = 0xFFFFFFFF;
    pub const ALL_COMPLETED_NV: c_uint = 0x84F2;
    pub const ALL_PIXELS_AMD: c_uint = 0xFFFFFFFF;
    pub const ALL_SHADER_BITS: c_uint = 0xFFFFFFFF;
    pub const ALL_STATIC_DATA_IBM: c_uint = 103060;
    pub const ALPHA: c_uint = 0x1906;
    pub const ALPHA12_EXT: c_uint = 0x803D;
    pub const ALPHA16F_ARB: c_uint = 0x881C;
    pub const ALPHA16F_EXT: c_uint = 0x881C;
    pub const ALPHA16I_EXT: c_uint = 0x8D8A;
    pub const ALPHA16UI_EXT: c_uint = 0x8D78;
    pub const ALPHA16_EXT: c_uint = 0x803E;
    pub const ALPHA16_SNORM: c_uint = 0x9018;
    pub const ALPHA32F_ARB: c_uint = 0x8816;
    pub const ALPHA32F_EXT: c_uint = 0x8816;
    pub const ALPHA32I_EXT: c_uint = 0x8D84;
    pub const ALPHA32UI_EXT: c_uint = 0x8D72;
    pub const ALPHA4_EXT: c_uint = 0x803B;
    pub const ALPHA8I_EXT: c_uint = 0x8D90;
    pub const ALPHA8UI_EXT: c_uint = 0x8D7E;
    pub const ALPHA8_EXT: c_uint = 0x803C;
    pub const ALPHA8_SNORM: c_uint = 0x9014;
    pub const ALPHA_FLOAT16_APPLE: c_uint = 0x881C;
    pub const ALPHA_FLOAT16_ATI: c_uint = 0x881C;
    pub const ALPHA_FLOAT32_APPLE: c_uint = 0x8816;
    pub const ALPHA_FLOAT32_ATI: c_uint = 0x8816;
    pub const ALPHA_INTEGER_EXT: c_uint = 0x8D97;
    pub const ALPHA_MAX_CLAMP_INGR: c_uint = 0x8567;
    pub const ALPHA_MAX_SGIX: c_uint = 0x8321;
    pub const ALPHA_MIN_CLAMP_INGR: c_uint = 0x8563;
    pub const ALPHA_MIN_SGIX: c_uint = 0x8320;
    pub const ALPHA_REF_COMMAND_NV: c_uint = 0x000F;
    pub const ALPHA_SNORM: c_uint = 0x9010;
    pub const ALPHA_TO_COVERAGE_DITHER_DEFAULT_NV: c_uint = 0x934D;
    pub const ALPHA_TO_COVERAGE_DITHER_DISABLE_NV: c_uint = 0x934F;
    pub const ALPHA_TO_COVERAGE_DITHER_ENABLE_NV: c_uint = 0x934E;
    pub const ALPHA_TO_COVERAGE_DITHER_MODE_NV: c_uint = 0x92BF;
    pub const ALREADY_SIGNALED: c_uint = 0x911A;
    pub const ALWAYS: c_uint = 0x0207;
    pub const ALWAYS_FAST_HINT_PGI: c_uint = 0x1A20C;
    pub const ALWAYS_SOFT_HINT_PGI: c_uint = 0x1A20D;
    pub const AND: c_uint = 0x1501;
    pub const AND_INVERTED: c_uint = 0x1504;
    pub const AND_REVERSE: c_uint = 0x1502;
    pub const ANY_SAMPLES_PASSED: c_uint = 0x8C2F;
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: c_uint = 0x8D6A;
    pub const ARC_TO_NV: c_uint = 0xFE;
    pub const ARRAY_BUFFER: c_uint = 0x8892;
    pub const ARRAY_BUFFER_ARB: c_uint = 0x8892;
    pub const ARRAY_BUFFER_BINDING: c_uint = 0x8894;
    pub const ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8894;
    pub const ARRAY_ELEMENT_LOCK_COUNT_EXT: c_uint = 0x81A9;
    pub const ARRAY_ELEMENT_LOCK_FIRST_EXT: c_uint = 0x81A8;
    pub const ARRAY_OBJECT_BUFFER_ATI: c_uint = 0x8766;
    pub const ARRAY_OBJECT_OFFSET_ATI: c_uint = 0x8767;
    pub const ARRAY_SIZE: c_uint = 0x92FB;
    pub const ARRAY_STRIDE: c_uint = 0x92FE;
    pub const ASYNC_DRAW_PIXELS_SGIX: c_uint = 0x835D;
    pub const ASYNC_HISTOGRAM_SGIX: c_uint = 0x832C;
    pub const ASYNC_MARKER_SGIX: c_uint = 0x8329;
    pub const ASYNC_READ_PIXELS_SGIX: c_uint = 0x835E;
    pub const ASYNC_TEX_IMAGE_SGIX: c_uint = 0x835C;
    pub const ATOMIC_COUNTER_BARRIER_BIT: c_uint = 0x00001000;
    pub const ATOMIC_COUNTER_BARRIER_BIT_EXT: c_uint = 0x00001000;
    pub const ATOMIC_COUNTER_BUFFER: c_uint = 0x92C0;
    pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: c_uint = 0x92C5;
    pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: c_uint = 0x92C6;
    pub const ATOMIC_COUNTER_BUFFER_BINDING: c_uint = 0x92C1;
    pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: c_uint = 0x92C4;
    pub const ATOMIC_COUNTER_BUFFER_INDEX: c_uint = 0x9301;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x90ED;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x92CB;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x92CA;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_MESH_SHADER_NV: c_uint = 0x959E;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TASK_SHADER_NV: c_uint = 0x959F;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x92C8;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x92C9;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: c_uint = 0x92C7;
    pub const ATOMIC_COUNTER_BUFFER_SIZE: c_uint = 0x92C3;
    pub const ATOMIC_COUNTER_BUFFER_START: c_uint = 0x92C2;
    pub const ATTACHED_MEMORY_OBJECT_NV: c_uint = 0x95A4;
    pub const ATTACHED_MEMORY_OFFSET_NV: c_uint = 0x95A5;
    pub const ATTACHED_SHADERS: c_uint = 0x8B85;
    pub const ATTENUATION_EXT: c_uint = 0x834D;
    pub const ATTRIBUTE_ADDRESS_COMMAND_NV: c_uint = 0x0009;
    pub const ATTRIB_ARRAY_POINTER_NV: c_uint = 0x8645;
    pub const ATTRIB_ARRAY_SIZE_NV: c_uint = 0x8623;
    pub const ATTRIB_ARRAY_STRIDE_NV: c_uint = 0x8624;
    pub const ATTRIB_ARRAY_TYPE_NV: c_uint = 0x8625;
    pub const AUTO_GENERATE_MIPMAP: c_uint = 0x8295;
    pub const AUX_DEPTH_STENCIL_APPLE: c_uint = 0x8A14;
    pub const AVERAGE_EXT: c_uint = 0x8335;
    pub const AVERAGE_HP: c_uint = 0x8160;
    pub const BACK: c_uint = 0x0405;
    pub const BACK_LEFT: c_uint = 0x0402;
    pub const BACK_NORMALS_HINT_PGI: c_uint = 0x1A223;
    pub const BACK_PRIMARY_COLOR_NV: c_uint = 0x8C77;
    pub const BACK_RIGHT: c_uint = 0x0403;
    pub const BACK_SECONDARY_COLOR_NV: c_uint = 0x8C78;
    pub const BEVEL_NV: c_uint = 0x90A6;
    pub const BGR: c_uint = 0x80E0;
    pub const BGRA: c_uint = 0x80E1;
    pub const BGRA8_EXT: c_uint = 0x93A1;
    pub const BGRA_EXT: c_uint = 0x80E1;
    pub const BGRA_INTEGER: c_uint = 0x8D9B;
    pub const BGRA_INTEGER_EXT: c_uint = 0x8D9B;
    pub const BGR_EXT: c_uint = 0x80E0;
    pub const BGR_INTEGER: c_uint = 0x8D9A;
    pub const BGR_INTEGER_EXT: c_uint = 0x8D9A;
    pub const BIAS_BIT_ATI: c_uint = 0x00000008;
    pub const BIAS_BY_NEGATIVE_ONE_HALF_NV: c_uint = 0x8541;
    pub const BINORMAL_ARRAY_EXT: c_uint = 0x843A;
    pub const BINORMAL_ARRAY_POINTER_EXT: c_uint = 0x8443;
    pub const BINORMAL_ARRAY_STRIDE_EXT: c_uint = 0x8441;
    pub const BINORMAL_ARRAY_TYPE_EXT: c_uint = 0x8440;
    pub const BLACKHOLE_RENDER_INTEL: c_uint = 0x83FC;
    pub const BLEND: c_uint = 0x0BE2;
    pub const BLEND_ADVANCED_COHERENT_KHR: c_uint = 0x9285;
    pub const BLEND_ADVANCED_COHERENT_NV: c_uint = 0x9285;
    pub const BLEND_COLOR: c_uint = 0x8005;
    pub const BLEND_COLOR_COMMAND_NV: c_uint = 0x000B;
    pub const BLEND_COLOR_EXT: c_uint = 0x8005;
    pub const BLEND_DST: c_uint = 0x0BE0;
    pub const BLEND_DST_ALPHA: c_uint = 0x80CA;
    pub const BLEND_DST_ALPHA_EXT: c_uint = 0x80CA;
    pub const BLEND_DST_RGB: c_uint = 0x80C8;
    pub const BLEND_DST_RGB_EXT: c_uint = 0x80C8;
    pub const BLEND_EQUATION: c_uint = 0x8009;
    pub const BLEND_EQUATION_ALPHA: c_uint = 0x883D;
    pub const BLEND_EQUATION_ALPHA_EXT: c_uint = 0x883D;
    pub const BLEND_EQUATION_EXT: c_uint = 0x8009;
    pub const BLEND_EQUATION_RGB: c_uint = 0x8009;
    pub const BLEND_EQUATION_RGB_EXT: c_uint = 0x8009;
    pub const BLEND_OVERLAP_NV: c_uint = 0x9281;
    pub const BLEND_PREMULTIPLIED_SRC_NV: c_uint = 0x9280;
    pub const BLEND_SRC: c_uint = 0x0BE1;
    pub const BLEND_SRC_ALPHA: c_uint = 0x80CB;
    pub const BLEND_SRC_ALPHA_EXT: c_uint = 0x80CB;
    pub const BLEND_SRC_RGB: c_uint = 0x80C9;
    pub const BLEND_SRC_RGB_EXT: c_uint = 0x80C9;
    pub const BLOCK_INDEX: c_uint = 0x92FD;
    pub const BLUE: c_uint = 0x1905;
    pub const BLUE_BIT_ATI: c_uint = 0x00000004;
    pub const BLUE_INTEGER: c_uint = 0x8D96;
    pub const BLUE_INTEGER_EXT: c_uint = 0x8D96;
    pub const BLUE_MAX_CLAMP_INGR: c_uint = 0x8566;
    pub const BLUE_MIN_CLAMP_INGR: c_uint = 0x8562;
    pub const BLUE_NV: c_uint = 0x1905;
    pub const BOLD_BIT_NV: c_uint = 0x01;
    pub const BOOL: c_uint = 0x8B56;
    pub const BOOL_ARB: c_uint = 0x8B56;
    pub const BOOL_VEC2: c_uint = 0x8B57;
    pub const BOOL_VEC2_ARB: c_uint = 0x8B57;
    pub const BOOL_VEC3: c_uint = 0x8B58;
    pub const BOOL_VEC3_ARB: c_uint = 0x8B58;
    pub const BOOL_VEC4: c_uint = 0x8B59;
    pub const BOOL_VEC4_ARB: c_uint = 0x8B59;
    pub const BOUNDING_BOX_NV: c_uint = 0x908D;
    pub const BOUNDING_BOX_OF_BOUNDING_BOXES_NV: c_uint = 0x909C;
    pub const BUFFER: c_uint = 0x82E0;
    pub const BUFFER_ACCESS: c_uint = 0x88BB;
    pub const BUFFER_ACCESS_ARB: c_uint = 0x88BB;
    pub const BUFFER_ACCESS_FLAGS: c_uint = 0x911F;
    pub const BUFFER_BINDING: c_uint = 0x9302;
    pub const BUFFER_DATA_SIZE: c_uint = 0x9303;
    pub const BUFFER_FLUSHING_UNMAP_APPLE: c_uint = 0x8A13;
    pub const BUFFER_GPU_ADDRESS_NV: c_uint = 0x8F1D;
    pub const BUFFER_IMMUTABLE_STORAGE: c_uint = 0x821F;
    pub const BUFFER_MAPPED: c_uint = 0x88BC;
    pub const BUFFER_MAPPED_ARB: c_uint = 0x88BC;
    pub const BUFFER_MAP_LENGTH: c_uint = 0x9120;
    pub const BUFFER_MAP_OFFSET: c_uint = 0x9121;
    pub const BUFFER_MAP_POINTER: c_uint = 0x88BD;
    pub const BUFFER_MAP_POINTER_ARB: c_uint = 0x88BD;
    pub const BUFFER_OBJECT_APPLE: c_uint = 0x85B3;
    pub const BUFFER_OBJECT_EXT: c_uint = 0x9151;
    pub const BUFFER_SERIALIZED_MODIFY_APPLE: c_uint = 0x8A12;
    pub const BUFFER_SIZE: c_uint = 0x8764;
    pub const BUFFER_SIZE_ARB: c_uint = 0x8764;
    pub const BUFFER_STORAGE_FLAGS: c_uint = 0x8220;
    pub const BUFFER_UPDATE_BARRIER_BIT: c_uint = 0x00000200;
    pub const BUFFER_UPDATE_BARRIER_BIT_EXT: c_uint = 0x00000200;
    pub const BUFFER_USAGE: c_uint = 0x8765;
    pub const BUFFER_USAGE_ARB: c_uint = 0x8765;
    pub const BUFFER_VARIABLE: c_uint = 0x92E5;
    pub const BUMP_ENVMAP_ATI: c_uint = 0x877B;
    pub const BUMP_NUM_TEX_UNITS_ATI: c_uint = 0x8777;
    pub const BUMP_ROT_MATRIX_ATI: c_uint = 0x8775;
    pub const BUMP_ROT_MATRIX_SIZE_ATI: c_uint = 0x8776;
    pub const BUMP_TARGET_ATI: c_uint = 0x877C;
    pub const BUMP_TEX_UNITS_ATI: c_uint = 0x8778;
    pub const BYTE: c_uint = 0x1400;
    pub const CALLIGRAPHIC_FRAGMENT_SGIX: c_uint = 0x8183;
    pub const CAVEAT_SUPPORT: c_uint = 0x82B8;
    pub const CCW: c_uint = 0x0901;
    pub const CIRCULAR_CCW_ARC_TO_NV: c_uint = 0xF8;
    pub const CIRCULAR_CW_ARC_TO_NV: c_uint = 0xFA;
    pub const CIRCULAR_TANGENT_ARC_TO_NV: c_uint = 0xFC;
    pub const CLAMP_FRAGMENT_COLOR_ARB: c_uint = 0x891B;
    pub const CLAMP_READ_COLOR: c_uint = 0x891C;
    pub const CLAMP_READ_COLOR_ARB: c_uint = 0x891C;
    pub const CLAMP_TO_BORDER: c_uint = 0x812D;
    pub const CLAMP_TO_BORDER_ARB: c_uint = 0x812D;
    pub const CLAMP_TO_BORDER_SGIS: c_uint = 0x812D;
    pub const CLAMP_TO_EDGE: c_uint = 0x812F;
    pub const CLAMP_TO_EDGE_SGIS: c_uint = 0x812F;
    pub const CLAMP_VERTEX_COLOR_ARB: c_uint = 0x891A;
    pub const CLEAR: c_uint = 0x1500;
    pub const CLEAR_BUFFER: c_uint = 0x82B4;
    pub const CLEAR_TEXTURE: c_uint = 0x9365;
    pub const CLIENT_ACTIVE_TEXTURE_ARB: c_uint = 0x84E1;
    pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: c_uint = 0x00004000;
    pub const CLIENT_STORAGE_BIT: c_uint = 0x0200;
    pub const CLIPPING_INPUT_PRIMITIVES: c_uint = 0x82F6;
    pub const CLIPPING_INPUT_PRIMITIVES_ARB: c_uint = 0x82F6;
    pub const CLIPPING_OUTPUT_PRIMITIVES: c_uint = 0x82F7;
    pub const CLIPPING_OUTPUT_PRIMITIVES_ARB: c_uint = 0x82F7;
    pub const CLIP_DEPTH_MODE: c_uint = 0x935D;
    pub const CLIP_DISTANCE0: c_uint = 0x3000;
    pub const CLIP_DISTANCE1: c_uint = 0x3001;
    pub const CLIP_DISTANCE2: c_uint = 0x3002;
    pub const CLIP_DISTANCE3: c_uint = 0x3003;
    pub const CLIP_DISTANCE4: c_uint = 0x3004;
    pub const CLIP_DISTANCE5: c_uint = 0x3005;
    pub const CLIP_DISTANCE6: c_uint = 0x3006;
    pub const CLIP_DISTANCE7: c_uint = 0x3007;
    pub const CLIP_DISTANCE_NV: c_uint = 0x8C7A;
    pub const CLIP_FAR_HINT_PGI: c_uint = 0x1A221;
    pub const CLIP_NEAR_HINT_PGI: c_uint = 0x1A220;
    pub const CLIP_ORIGIN: c_uint = 0x935C;
    pub const CLIP_VOLUME_CLIPPING_HINT_EXT: c_uint = 0x80F0;
    pub const CLOSE_PATH_NV: c_uint = 0x00;
    pub const CMYKA_EXT: c_uint = 0x800D;
    pub const CMYK_EXT: c_uint = 0x800C;
    pub const CND0_ATI: c_uint = 0x896B;
    pub const CND_ATI: c_uint = 0x896A;
    pub const COLOR: c_uint = 0x1800;
    pub const COLOR3_BIT_PGI: c_uint = 0x00010000;
    pub const COLOR4_BIT_PGI: c_uint = 0x00020000;
    pub const COLORBURN_KHR: c_uint = 0x929A;
    pub const COLORBURN_NV: c_uint = 0x929A;
    pub const COLORDODGE_KHR: c_uint = 0x9299;
    pub const COLORDODGE_NV: c_uint = 0x9299;
    pub const COLOR_ALPHA_PAIRING_ATI: c_uint = 0x8975;
    pub const COLOR_ARRAY_ADDRESS_NV: c_uint = 0x8F23;
    pub const COLOR_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8898;
    pub const COLOR_ARRAY_COUNT_EXT: c_uint = 0x8084;
    pub const COLOR_ARRAY_EXT: c_uint = 0x8076;
    pub const COLOR_ARRAY_LENGTH_NV: c_uint = 0x8F2D;
    pub const COLOR_ARRAY_LIST_IBM: c_uint = 103072;
    pub const COLOR_ARRAY_LIST_STRIDE_IBM: c_uint = 103082;
    pub const COLOR_ARRAY_PARALLEL_POINTERS_INTEL: c_uint = 0x83F7;
    pub const COLOR_ARRAY_POINTER_EXT: c_uint = 0x8090;
    pub const COLOR_ARRAY_SIZE_EXT: c_uint = 0x8081;
    pub const COLOR_ARRAY_STRIDE_EXT: c_uint = 0x8083;
    pub const COLOR_ARRAY_TYPE_EXT: c_uint = 0x8082;
    pub const COLOR_ATTACHMENT0: c_uint = 0x8CE0;
    pub const COLOR_ATTACHMENT0_EXT: c_uint = 0x8CE0;
    pub const COLOR_ATTACHMENT1: c_uint = 0x8CE1;
    pub const COLOR_ATTACHMENT10: c_uint = 0x8CEA;
    pub const COLOR_ATTACHMENT10_EXT: c_uint = 0x8CEA;
    pub const COLOR_ATTACHMENT11: c_uint = 0x8CEB;
    pub const COLOR_ATTACHMENT11_EXT: c_uint = 0x8CEB;
    pub const COLOR_ATTACHMENT12: c_uint = 0x8CEC;
    pub const COLOR_ATTACHMENT12_EXT: c_uint = 0x8CEC;
    pub const COLOR_ATTACHMENT13: c_uint = 0x8CED;
    pub const COLOR_ATTACHMENT13_EXT: c_uint = 0x8CED;
    pub const COLOR_ATTACHMENT14: c_uint = 0x8CEE;
    pub const COLOR_ATTACHMENT14_EXT: c_uint = 0x8CEE;
    pub const COLOR_ATTACHMENT15: c_uint = 0x8CEF;
    pub const COLOR_ATTACHMENT15_EXT: c_uint = 0x8CEF;
    pub const COLOR_ATTACHMENT16: c_uint = 0x8CF0;
    pub const COLOR_ATTACHMENT17: c_uint = 0x8CF1;
    pub const COLOR_ATTACHMENT18: c_uint = 0x8CF2;
    pub const COLOR_ATTACHMENT19: c_uint = 0x8CF3;
    pub const COLOR_ATTACHMENT1_EXT: c_uint = 0x8CE1;
    pub const COLOR_ATTACHMENT2: c_uint = 0x8CE2;
    pub const COLOR_ATTACHMENT20: c_uint = 0x8CF4;
    pub const COLOR_ATTACHMENT21: c_uint = 0x8CF5;
    pub const COLOR_ATTACHMENT22: c_uint = 0x8CF6;
    pub const COLOR_ATTACHMENT23: c_uint = 0x8CF7;
    pub const COLOR_ATTACHMENT24: c_uint = 0x8CF8;
    pub const COLOR_ATTACHMENT25: c_uint = 0x8CF9;
    pub const COLOR_ATTACHMENT26: c_uint = 0x8CFA;
    pub const COLOR_ATTACHMENT27: c_uint = 0x8CFB;
    pub const COLOR_ATTACHMENT28: c_uint = 0x8CFC;
    pub const COLOR_ATTACHMENT29: c_uint = 0x8CFD;
    pub const COLOR_ATTACHMENT2_EXT: c_uint = 0x8CE2;
    pub const COLOR_ATTACHMENT3: c_uint = 0x8CE3;
    pub const COLOR_ATTACHMENT30: c_uint = 0x8CFE;
    pub const COLOR_ATTACHMENT31: c_uint = 0x8CFF;
    pub const COLOR_ATTACHMENT3_EXT: c_uint = 0x8CE3;
    pub const COLOR_ATTACHMENT4: c_uint = 0x8CE4;
    pub const COLOR_ATTACHMENT4_EXT: c_uint = 0x8CE4;
    pub const COLOR_ATTACHMENT5: c_uint = 0x8CE5;
    pub const COLOR_ATTACHMENT5_EXT: c_uint = 0x8CE5;
    pub const COLOR_ATTACHMENT6: c_uint = 0x8CE6;
    pub const COLOR_ATTACHMENT6_EXT: c_uint = 0x8CE6;
    pub const COLOR_ATTACHMENT7: c_uint = 0x8CE7;
    pub const COLOR_ATTACHMENT7_EXT: c_uint = 0x8CE7;
    pub const COLOR_ATTACHMENT8: c_uint = 0x8CE8;
    pub const COLOR_ATTACHMENT8_EXT: c_uint = 0x8CE8;
    pub const COLOR_ATTACHMENT9: c_uint = 0x8CE9;
    pub const COLOR_ATTACHMENT9_EXT: c_uint = 0x8CE9;
    pub const COLOR_BUFFER_BIT: c_uint = 0x00004000;
    pub const COLOR_CLEAR_UNCLAMPED_VALUE_ATI: c_uint = 0x8835;
    pub const COLOR_CLEAR_VALUE: c_uint = 0x0C22;
    pub const COLOR_COMPONENTS: c_uint = 0x8283;
    pub const COLOR_ENCODING: c_uint = 0x8296;
    pub const COLOR_FLOAT_APPLE: c_uint = 0x8A0F;
    pub const COLOR_INDEX12_EXT: c_uint = 0x80E6;
    pub const COLOR_INDEX16_EXT: c_uint = 0x80E7;
    pub const COLOR_INDEX1_EXT: c_uint = 0x80E2;
    pub const COLOR_INDEX2_EXT: c_uint = 0x80E3;
    pub const COLOR_INDEX4_EXT: c_uint = 0x80E4;
    pub const COLOR_INDEX8_EXT: c_uint = 0x80E5;
    pub const COLOR_LOGIC_OP: c_uint = 0x0BF2;
    pub const COLOR_MATRIX_SGI: c_uint = 0x80B1;
    pub const COLOR_MATRIX_STACK_DEPTH_SGI: c_uint = 0x80B2;
    pub const COLOR_RENDERABLE: c_uint = 0x8286;
    pub const COLOR_SAMPLES_NV: c_uint = 0x8E20;
    pub const COLOR_SUM_ARB: c_uint = 0x8458;
    pub const COLOR_SUM_CLAMP_NV: c_uint = 0x854F;
    pub const COLOR_SUM_EXT: c_uint = 0x8458;
    pub const COLOR_TABLE_ALPHA_SIZE_SGI: c_uint = 0x80DD;
    pub const COLOR_TABLE_BIAS_SGI: c_uint = 0x80D7;
    pub const COLOR_TABLE_BLUE_SIZE_SGI: c_uint = 0x80DC;
    pub const COLOR_TABLE_FORMAT_SGI: c_uint = 0x80D8;
    pub const COLOR_TABLE_GREEN_SIZE_SGI: c_uint = 0x80DB;
    pub const COLOR_TABLE_INTENSITY_SIZE_SGI: c_uint = 0x80DF;
    pub const COLOR_TABLE_LUMINANCE_SIZE_SGI: c_uint = 0x80DE;
    pub const COLOR_TABLE_RED_SIZE_SGI: c_uint = 0x80DA;
    pub const COLOR_TABLE_SCALE_SGI: c_uint = 0x80D6;
    pub const COLOR_TABLE_SGI: c_uint = 0x80D0;
    pub const COLOR_TABLE_WIDTH_SGI: c_uint = 0x80D9;
    pub const COLOR_WRITEMASK: c_uint = 0x0C23;
    pub const COMBINE4_NV: c_uint = 0x8503;
    pub const COMBINER0_NV: c_uint = 0x8550;
    pub const COMBINER1_NV: c_uint = 0x8551;
    pub const COMBINER2_NV: c_uint = 0x8552;
    pub const COMBINER3_NV: c_uint = 0x8553;
    pub const COMBINER4_NV: c_uint = 0x8554;
    pub const COMBINER5_NV: c_uint = 0x8555;
    pub const COMBINER6_NV: c_uint = 0x8556;
    pub const COMBINER7_NV: c_uint = 0x8557;
    pub const COMBINER_AB_DOT_PRODUCT_NV: c_uint = 0x8545;
    pub const COMBINER_AB_OUTPUT_NV: c_uint = 0x854A;
    pub const COMBINER_BIAS_NV: c_uint = 0x8549;
    pub const COMBINER_CD_DOT_PRODUCT_NV: c_uint = 0x8546;
    pub const COMBINER_CD_OUTPUT_NV: c_uint = 0x854B;
    pub const COMBINER_COMPONENT_USAGE_NV: c_uint = 0x8544;
    pub const COMBINER_INPUT_NV: c_uint = 0x8542;
    pub const COMBINER_MAPPING_NV: c_uint = 0x8543;
    pub const COMBINER_MUX_SUM_NV: c_uint = 0x8547;
    pub const COMBINER_SCALE_NV: c_uint = 0x8548;
    pub const COMBINER_SUM_OUTPUT_NV: c_uint = 0x854C;
    pub const COMBINE_ALPHA_ARB: c_uint = 0x8572;
    pub const COMBINE_ALPHA_EXT: c_uint = 0x8572;
    pub const COMBINE_ARB: c_uint = 0x8570;
    pub const COMBINE_EXT: c_uint = 0x8570;
    pub const COMBINE_RGB_ARB: c_uint = 0x8571;
    pub const COMBINE_RGB_EXT: c_uint = 0x8571;
    pub const COMMAND_BARRIER_BIT: c_uint = 0x00000040;
    pub const COMMAND_BARRIER_BIT_EXT: c_uint = 0x00000040;
    pub const COMPARE_REF_DEPTH_TO_TEXTURE_EXT: c_uint = 0x884E;
    pub const COMPARE_REF_TO_TEXTURE: c_uint = 0x884E;
    pub const COMPARE_R_TO_TEXTURE_ARB: c_uint = 0x884E;
    pub const COMPATIBLE_SUBROUTINES: c_uint = 0x8E4B;
    pub const COMPILE_STATUS: c_uint = 0x8B81;
    pub const COMPLETION_STATUS_ARB: c_uint = 0x91B1;
    pub const COMPLETION_STATUS_KHR: c_uint = 0x91B1;
    pub const COMPRESSED_ALPHA_ARB: c_uint = 0x84E9;
    pub const COMPRESSED_INTENSITY_ARB: c_uint = 0x84EC;
    pub const COMPRESSED_LUMINANCE_ALPHA_ARB: c_uint = 0x84EB;
    pub const COMPRESSED_LUMINANCE_ALPHA_LATC2_EXT: c_uint = 0x8C72;
    pub const COMPRESSED_LUMINANCE_ARB: c_uint = 0x84EA;
    pub const COMPRESSED_LUMINANCE_LATC1_EXT: c_uint = 0x8C70;
    pub const COMPRESSED_R11_EAC: c_uint = 0x9270;
    pub const COMPRESSED_RED: c_uint = 0x8225;
    pub const COMPRESSED_RED_GREEN_RGTC2_EXT: c_uint = 0x8DBD;
    pub const COMPRESSED_RED_RGTC1: c_uint = 0x8DBB;
    pub const COMPRESSED_RED_RGTC1_EXT: c_uint = 0x8DBB;
    pub const COMPRESSED_RG: c_uint = 0x8226;
    pub const COMPRESSED_RG11_EAC: c_uint = 0x9272;
    pub const COMPRESSED_RGB: c_uint = 0x84ED;
    pub const COMPRESSED_RGB8_ETC2: c_uint = 0x9274;
    pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: c_uint = 0x9276;
    pub const COMPRESSED_RGBA: c_uint = 0x84EE;
    pub const COMPRESSED_RGBA8_ETC2_EAC: c_uint = 0x9278;
    pub const COMPRESSED_RGBA_ARB: c_uint = 0x84EE;
    pub const COMPRESSED_RGBA_ASTC_10x10_KHR: c_uint = 0x93BB;
    pub const COMPRESSED_RGBA_ASTC_10x5_KHR: c_uint = 0x93B8;
    pub const COMPRESSED_RGBA_ASTC_10x6_KHR: c_uint = 0x93B9;
    pub const COMPRESSED_RGBA_ASTC_10x8_KHR: c_uint = 0x93BA;
    pub const COMPRESSED_RGBA_ASTC_12x10_KHR: c_uint = 0x93BC;
    pub const COMPRESSED_RGBA_ASTC_12x12_KHR: c_uint = 0x93BD;
    pub const COMPRESSED_RGBA_ASTC_4x4_KHR: c_uint = 0x93B0;
    pub const COMPRESSED_RGBA_ASTC_5x4_KHR: c_uint = 0x93B1;
    pub const COMPRESSED_RGBA_ASTC_5x5_KHR: c_uint = 0x93B2;
    pub const COMPRESSED_RGBA_ASTC_6x5_KHR: c_uint = 0x93B3;
    pub const COMPRESSED_RGBA_ASTC_6x6_KHR: c_uint = 0x93B4;
    pub const COMPRESSED_RGBA_ASTC_8x5_KHR: c_uint = 0x93B5;
    pub const COMPRESSED_RGBA_ASTC_8x6_KHR: c_uint = 0x93B6;
    pub const COMPRESSED_RGBA_ASTC_8x8_KHR: c_uint = 0x93B7;
    pub const COMPRESSED_RGBA_BPTC_UNORM: c_uint = 0x8E8C;
    pub const COMPRESSED_RGBA_BPTC_UNORM_ARB: c_uint = 0x8E8C;
    pub const COMPRESSED_RGBA_FXT1_3DFX: c_uint = 0x86B1;
    pub const COMPRESSED_RGBA_S3TC_DXT1_EXT: c_uint = 0x83F1;
    pub const COMPRESSED_RGBA_S3TC_DXT3_EXT: c_uint = 0x83F2;
    pub const COMPRESSED_RGBA_S3TC_DXT5_EXT: c_uint = 0x83F3;
    pub const COMPRESSED_RGB_ARB: c_uint = 0x84ED;
    pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: c_uint = 0x8E8E;
    pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB: c_uint = 0x8E8E;
    pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: c_uint = 0x8E8F;
    pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB: c_uint = 0x8E8F;
    pub const COMPRESSED_RGB_FXT1_3DFX: c_uint = 0x86B0;
    pub const COMPRESSED_RGB_S3TC_DXT1_EXT: c_uint = 0x83F0;
    pub const COMPRESSED_RG_RGTC2: c_uint = 0x8DBD;
    pub const COMPRESSED_SIGNED_LUMINANCE_ALPHA_LATC2_EXT: c_uint = 0x8C73;
    pub const COMPRESSED_SIGNED_LUMINANCE_LATC1_EXT: c_uint = 0x8C71;
    pub const COMPRESSED_SIGNED_R11_EAC: c_uint = 0x9271;
    pub const COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT: c_uint = 0x8DBE;
    pub const COMPRESSED_SIGNED_RED_RGTC1: c_uint = 0x8DBC;
    pub const COMPRESSED_SIGNED_RED_RGTC1_EXT: c_uint = 0x8DBC;
    pub const COMPRESSED_SIGNED_RG11_EAC: c_uint = 0x9273;
    pub const COMPRESSED_SIGNED_RG_RGTC2: c_uint = 0x8DBE;
    pub const COMPRESSED_SLUMINANCE_ALPHA_EXT: c_uint = 0x8C4B;
    pub const COMPRESSED_SLUMINANCE_EXT: c_uint = 0x8C4A;
    pub const COMPRESSED_SRGB: c_uint = 0x8C48;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: c_uint = 0x93DB;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: c_uint = 0x93D8;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: c_uint = 0x93D9;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: c_uint = 0x93DA;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: c_uint = 0x93DC;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: c_uint = 0x93DD;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: c_uint = 0x93D0;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: c_uint = 0x93D1;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: c_uint = 0x93D2;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: c_uint = 0x93D3;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: c_uint = 0x93D4;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: c_uint = 0x93D5;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: c_uint = 0x93D6;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: c_uint = 0x93D7;
    pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: c_uint = 0x9279;
    pub const COMPRESSED_SRGB8_ETC2: c_uint = 0x9275;
    pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: c_uint = 0x9277;
    pub const COMPRESSED_SRGB_ALPHA: c_uint = 0x8C49;
    pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: c_uint = 0x8E8D;
    pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB: c_uint = 0x8E8D;
    pub const COMPRESSED_SRGB_ALPHA_EXT: c_uint = 0x8C49;
    pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: c_uint = 0x8C4D;
    pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: c_uint = 0x8C4E;
    pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: c_uint = 0x8C4F;
    pub const COMPRESSED_SRGB_EXT: c_uint = 0x8C48;
    pub const COMPRESSED_SRGB_S3TC_DXT1_EXT: c_uint = 0x8C4C;
    pub const COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A3;
    pub const COMPRESSED_TEXTURE_FORMATS_ARB: c_uint = 0x86A3;
    pub const COMPUTE_PROGRAM_NV: c_uint = 0x90FB;
    pub const COMPUTE_PROGRAM_PARAMETER_BUFFER_NV: c_uint = 0x90FC;
    pub const COMPUTE_SHADER: c_uint = 0x91B9;
    pub const COMPUTE_SHADER_BIT: c_uint = 0x00000020;
    pub const COMPUTE_SHADER_INVOCATIONS: c_uint = 0x82F5;
    pub const COMPUTE_SHADER_INVOCATIONS_ARB: c_uint = 0x82F5;
    pub const COMPUTE_SUBROUTINE: c_uint = 0x92ED;
    pub const COMPUTE_SUBROUTINE_UNIFORM: c_uint = 0x92F3;
    pub const COMPUTE_TEXTURE: c_uint = 0x82A0;
    pub const COMPUTE_WORK_GROUP_SIZE: c_uint = 0x8267;
    pub const COMP_BIT_ATI: c_uint = 0x00000002;
    pub const CONDITION_SATISFIED: c_uint = 0x911C;
    pub const CONFORMANT_NV: c_uint = 0x9374;
    pub const CONIC_CURVE_TO_NV: c_uint = 0x1A;
    pub const CONJOINT_NV: c_uint = 0x9284;
    pub const CONSERVATIVE_RASTERIZATION_INTEL: c_uint = 0x83FE;
    pub const CONSERVATIVE_RASTERIZATION_NV: c_uint = 0x9346;
    pub const CONSERVATIVE_RASTER_DILATE_GRANULARITY_NV: c_uint = 0x937B;
    pub const CONSERVATIVE_RASTER_DILATE_NV: c_uint = 0x9379;
    pub const CONSERVATIVE_RASTER_DILATE_RANGE_NV: c_uint = 0x937A;
    pub const CONSERVATIVE_RASTER_MODE_NV: c_uint = 0x954D;
    pub const CONSERVATIVE_RASTER_MODE_POST_SNAP_NV: c_uint = 0x954E;
    pub const CONSERVATIVE_RASTER_MODE_PRE_SNAP_NV: c_uint = 0x9550;
    pub const CONSERVATIVE_RASTER_MODE_PRE_SNAP_TRIANGLES_NV: c_uint = 0x954F;
    pub const CONSERVE_MEMORY_HINT_PGI: c_uint = 0x1A1FD;
    pub const CONSTANT_ALPHA: c_uint = 0x8003;
    pub const CONSTANT_ALPHA_EXT: c_uint = 0x8003;
    pub const CONSTANT_ARB: c_uint = 0x8576;
    pub const CONSTANT_BORDER_HP: c_uint = 0x8151;
    pub const CONSTANT_COLOR: c_uint = 0x8001;
    pub const CONSTANT_COLOR0_NV: c_uint = 0x852A;
    pub const CONSTANT_COLOR1_NV: c_uint = 0x852B;
    pub const CONSTANT_COLOR_EXT: c_uint = 0x8001;
    pub const CONSTANT_EXT: c_uint = 0x8576;
    pub const CONST_EYE_NV: c_uint = 0x86E5;
    pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: c_uint = 0x00000002;
    pub const CONTEXT_CORE_PROFILE_BIT: c_uint = 0x00000001;
    pub const CONTEXT_FLAGS: c_uint = 0x821E;
    pub const CONTEXT_FLAG_DEBUG_BIT: c_uint = 0x00000002;
    pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: c_uint = 0x00000001;
    pub const CONTEXT_FLAG_NO_ERROR_BIT: c_uint = 0x00000008;
    pub const CONTEXT_FLAG_NO_ERROR_BIT_KHR: c_uint = 0x00000008;
    pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: c_uint = 0x00000004;
    pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT_ARB: c_uint = 0x00000004;
    pub const CONTEXT_LOST: c_uint = 0x0507;
    pub const CONTEXT_PROFILE_MASK: c_uint = 0x9126;
    pub const CONTEXT_RELEASE_BEHAVIOR: c_uint = 0x82FB;
    pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: c_uint = 0x82FC;
    pub const CONTEXT_ROBUST_ACCESS: c_uint = 0x90F3;
    pub const CONTINUOUS_AMD: c_uint = 0x9007;
    pub const CONTRAST_NV: c_uint = 0x92A1;
    pub const CONVEX_HULL_NV: c_uint = 0x908B;
    pub const CONVOLUTION_1D_EXT: c_uint = 0x8010;
    pub const CONVOLUTION_2D_EXT: c_uint = 0x8011;
    pub const CONVOLUTION_BORDER_COLOR_HP: c_uint = 0x8154;
    pub const CONVOLUTION_BORDER_MODE_EXT: c_uint = 0x8013;
    pub const CONVOLUTION_FILTER_BIAS_EXT: c_uint = 0x8015;
    pub const CONVOLUTION_FILTER_SCALE_EXT: c_uint = 0x8014;
    pub const CONVOLUTION_FORMAT_EXT: c_uint = 0x8017;
    pub const CONVOLUTION_HEIGHT_EXT: c_uint = 0x8019;
    pub const CONVOLUTION_HINT_SGIX: c_uint = 0x8316;
    pub const CONVOLUTION_WIDTH_EXT: c_uint = 0x8018;
    pub const CON_0_ATI: c_uint = 0x8941;
    pub const CON_10_ATI: c_uint = 0x894B;
    pub const CON_11_ATI: c_uint = 0x894C;
    pub const CON_12_ATI: c_uint = 0x894D;
    pub const CON_13_ATI: c_uint = 0x894E;
    pub const CON_14_ATI: c_uint = 0x894F;
    pub const CON_15_ATI: c_uint = 0x8950;
    pub const CON_16_ATI: c_uint = 0x8951;
    pub const CON_17_ATI: c_uint = 0x8952;
    pub const CON_18_ATI: c_uint = 0x8953;
    pub const CON_19_ATI: c_uint = 0x8954;
    pub const CON_1_ATI: c_uint = 0x8942;
    pub const CON_20_ATI: c_uint = 0x8955;
    pub const CON_21_ATI: c_uint = 0x8956;
    pub const CON_22_ATI: c_uint = 0x8957;
    pub const CON_23_ATI: c_uint = 0x8958;
    pub const CON_24_ATI: c_uint = 0x8959;
    pub const CON_25_ATI: c_uint = 0x895A;
    pub const CON_26_ATI: c_uint = 0x895B;
    pub const CON_27_ATI: c_uint = 0x895C;
    pub const CON_28_ATI: c_uint = 0x895D;
    pub const CON_29_ATI: c_uint = 0x895E;
    pub const CON_2_ATI: c_uint = 0x8943;
    pub const CON_30_ATI: c_uint = 0x895F;
    pub const CON_31_ATI: c_uint = 0x8960;
    pub const CON_3_ATI: c_uint = 0x8944;
    pub const CON_4_ATI: c_uint = 0x8945;
    pub const CON_5_ATI: c_uint = 0x8946;
    pub const CON_6_ATI: c_uint = 0x8947;
    pub const CON_7_ATI: c_uint = 0x8948;
    pub const CON_8_ATI: c_uint = 0x8949;
    pub const CON_9_ATI: c_uint = 0x894A;
    pub const COORD_REPLACE_ARB: c_uint = 0x8862;
    pub const COORD_REPLACE_NV: c_uint = 0x8862;
    pub const COPY: c_uint = 0x1503;
    pub const COPY_INVERTED: c_uint = 0x150C;
    pub const COPY_READ_BUFFER: c_uint = 0x8F36;
    pub const COPY_READ_BUFFER_BINDING: c_uint = 0x8F36;
    pub const COPY_WRITE_BUFFER: c_uint = 0x8F37;
    pub const COPY_WRITE_BUFFER_BINDING: c_uint = 0x8F37;
    pub const COUNTER_RANGE_AMD: c_uint = 0x8BC1;
    pub const COUNTER_TYPE_AMD: c_uint = 0x8BC0;
    pub const COUNT_DOWN_NV: c_uint = 0x9089;
    pub const COUNT_UP_NV: c_uint = 0x9088;
    pub const COVERAGE_MODULATION_NV: c_uint = 0x9332;
    pub const COVERAGE_MODULATION_TABLE_NV: c_uint = 0x9331;
    pub const COVERAGE_MODULATION_TABLE_SIZE_NV: c_uint = 0x9333;
    pub const CUBIC_CURVE_TO_NV: c_uint = 0x0C;
    pub const CUBIC_EXT: c_uint = 0x8334;
    pub const CUBIC_HP: c_uint = 0x815F;
    pub const CULL_FACE: c_uint = 0x0B44;
    pub const CULL_FACE_MODE: c_uint = 0x0B45;
    pub const CULL_FRAGMENT_NV: c_uint = 0x86E7;
    pub const CULL_MODES_NV: c_uint = 0x86E0;
    pub const CULL_VERTEX_EXT: c_uint = 0x81AA;
    pub const CULL_VERTEX_EYE_POSITION_EXT: c_uint = 0x81AB;
    pub const CULL_VERTEX_IBM: c_uint = 103050;
    pub const CULL_VERTEX_OBJECT_POSITION_EXT: c_uint = 0x81AC;
    pub const CURRENT_ATTRIB_NV: c_uint = 0x8626;
    pub const CURRENT_BINORMAL_EXT: c_uint = 0x843C;
    pub const CURRENT_FOG_COORDINATE_EXT: c_uint = 0x8453;
    pub const CURRENT_MATRIX_ARB: c_uint = 0x8641;
    pub const CURRENT_MATRIX_INDEX_ARB: c_uint = 0x8845;
    pub const CURRENT_MATRIX_NV: c_uint = 0x8641;
    pub const CURRENT_MATRIX_STACK_DEPTH_ARB: c_uint = 0x8640;
    pub const CURRENT_MATRIX_STACK_DEPTH_NV: c_uint = 0x8640;
    pub const CURRENT_OCCLUSION_QUERY_ID_NV: c_uint = 0x8865;
    pub const CURRENT_PALETTE_MATRIX_ARB: c_uint = 0x8843;
    pub const CURRENT_PROGRAM: c_uint = 0x8B8D;
    pub const CURRENT_QUERY: c_uint = 0x8865;
    pub const CURRENT_QUERY_ARB: c_uint = 0x8865;
    pub const CURRENT_RASTER_NORMAL_SGIX: c_uint = 0x8406;
    pub const CURRENT_SECONDARY_COLOR_EXT: c_uint = 0x8459;
    pub const CURRENT_TANGENT_EXT: c_uint = 0x843B;
    pub const CURRENT_TIME_NV: c_uint = 0x8E28;
    pub const CURRENT_VERTEX_ATTRIB: c_uint = 0x8626;
    pub const CURRENT_VERTEX_ATTRIB_ARB: c_uint = 0x8626;
    pub const CURRENT_VERTEX_EXT: c_uint = 0x87E2;
    pub const CURRENT_VERTEX_WEIGHT_EXT: c_uint = 0x850B;
    pub const CURRENT_WEIGHT_ARB: c_uint = 0x86A8;
    pub const CW: c_uint = 0x0900;
    pub const D3D12_FENCE_VALUE_EXT: c_uint = 0x9595;
    pub const DARKEN_KHR: c_uint = 0x9297;
    pub const DARKEN_NV: c_uint = 0x9297;
    pub const DATA_BUFFER_AMD: c_uint = 0x9151;
    pub const DEBUG_CALLBACK_FUNCTION: c_uint = 0x8244;
    pub const DEBUG_CALLBACK_FUNCTION_ARB: c_uint = 0x8244;
    pub const DEBUG_CALLBACK_USER_PARAM: c_uint = 0x8245;
    pub const DEBUG_CALLBACK_USER_PARAM_ARB: c_uint = 0x8245;
    pub const DEBUG_CATEGORY_API_ERROR_AMD: c_uint = 0x9149;
    pub const DEBUG_CATEGORY_APPLICATION_AMD: c_uint = 0x914F;
    pub const DEBUG_CATEGORY_DEPRECATION_AMD: c_uint = 0x914B;
    pub const DEBUG_CATEGORY_OTHER_AMD: c_uint = 0x9150;
    pub const DEBUG_CATEGORY_PERFORMANCE_AMD: c_uint = 0x914D;
    pub const DEBUG_CATEGORY_SHADER_COMPILER_AMD: c_uint = 0x914E;
    pub const DEBUG_CATEGORY_UNDEFINED_BEHAVIOR_AMD: c_uint = 0x914C;
    pub const DEBUG_CATEGORY_WINDOW_SYSTEM_AMD: c_uint = 0x914A;
    pub const DEBUG_GROUP_STACK_DEPTH: c_uint = 0x826D;
    pub const DEBUG_LOGGED_MESSAGES: c_uint = 0x9145;
    pub const DEBUG_LOGGED_MESSAGES_AMD: c_uint = 0x9145;
    pub const DEBUG_LOGGED_MESSAGES_ARB: c_uint = 0x9145;
    pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: c_uint = 0x8243;
    pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB: c_uint = 0x8243;
    pub const DEBUG_OUTPUT: c_uint = 0x92E0;
    pub const DEBUG_OUTPUT_SYNCHRONOUS: c_uint = 0x8242;
    pub const DEBUG_OUTPUT_SYNCHRONOUS_ARB: c_uint = 0x8242;
    pub const DEBUG_SEVERITY_HIGH: c_uint = 0x9146;
    pub const DEBUG_SEVERITY_HIGH_AMD: c_uint = 0x9146;
    pub const DEBUG_SEVERITY_HIGH_ARB: c_uint = 0x9146;
    pub const DEBUG_SEVERITY_LOW: c_uint = 0x9148;
    pub const DEBUG_SEVERITY_LOW_AMD: c_uint = 0x9148;
    pub const DEBUG_SEVERITY_LOW_ARB: c_uint = 0x9148;
    pub const DEBUG_SEVERITY_MEDIUM: c_uint = 0x9147;
    pub const DEBUG_SEVERITY_MEDIUM_AMD: c_uint = 0x9147;
    pub const DEBUG_SEVERITY_MEDIUM_ARB: c_uint = 0x9147;
    pub const DEBUG_SEVERITY_NOTIFICATION: c_uint = 0x826B;
    pub const DEBUG_SOURCE_API: c_uint = 0x8246;
    pub const DEBUG_SOURCE_API_ARB: c_uint = 0x8246;
    pub const DEBUG_SOURCE_APPLICATION: c_uint = 0x824A;
    pub const DEBUG_SOURCE_APPLICATION_ARB: c_uint = 0x824A;
    pub const DEBUG_SOURCE_OTHER: c_uint = 0x824B;
    pub const DEBUG_SOURCE_OTHER_ARB: c_uint = 0x824B;
    pub const DEBUG_SOURCE_SHADER_COMPILER: c_uint = 0x8248;
    pub const DEBUG_SOURCE_SHADER_COMPILER_ARB: c_uint = 0x8248;
    pub const DEBUG_SOURCE_THIRD_PARTY: c_uint = 0x8249;
    pub const DEBUG_SOURCE_THIRD_PARTY_ARB: c_uint = 0x8249;
    pub const DEBUG_SOURCE_WINDOW_SYSTEM: c_uint = 0x8247;
    pub const DEBUG_SOURCE_WINDOW_SYSTEM_ARB: c_uint = 0x8247;
    pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: c_uint = 0x824D;
    pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB: c_uint = 0x824D;
    pub const DEBUG_TYPE_ERROR: c_uint = 0x824C;
    pub const DEBUG_TYPE_ERROR_ARB: c_uint = 0x824C;
    pub const DEBUG_TYPE_MARKER: c_uint = 0x8268;
    pub const DEBUG_TYPE_OTHER: c_uint = 0x8251;
    pub const DEBUG_TYPE_OTHER_ARB: c_uint = 0x8251;
    pub const DEBUG_TYPE_PERFORMANCE: c_uint = 0x8250;
    pub const DEBUG_TYPE_PERFORMANCE_ARB: c_uint = 0x8250;
    pub const DEBUG_TYPE_POP_GROUP: c_uint = 0x826A;
    pub const DEBUG_TYPE_PORTABILITY: c_uint = 0x824F;
    pub const DEBUG_TYPE_PORTABILITY_ARB: c_uint = 0x824F;
    pub const DEBUG_TYPE_PUSH_GROUP: c_uint = 0x8269;
    pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: c_uint = 0x824E;
    pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB: c_uint = 0x824E;
    pub const DECODE_EXT: c_uint = 0x8A49;
    pub const DECR: c_uint = 0x1E03;
    pub const DECR_WRAP: c_uint = 0x8508;
    pub const DECR_WRAP_EXT: c_uint = 0x8508;
    pub const DEDICATED_MEMORY_OBJECT_EXT: c_uint = 0x9581;
    pub const DEFORMATIONS_MASK_SGIX: c_uint = 0x8196;
    pub const DELETE_STATUS: c_uint = 0x8B80;
    pub const DEPENDENT_AR_TEXTURE_2D_NV: c_uint = 0x86E9;
    pub const DEPENDENT_GB_TEXTURE_2D_NV: c_uint = 0x86EA;
    pub const DEPENDENT_HILO_TEXTURE_2D_NV: c_uint = 0x8858;
    pub const DEPENDENT_RGB_TEXTURE_3D_NV: c_uint = 0x8859;
    pub const DEPENDENT_RGB_TEXTURE_CUBE_MAP_NV: c_uint = 0x885A;
    pub const DEPTH: c_uint = 0x1801;
    pub const DEPTH24_STENCIL8: c_uint = 0x88F0;
    pub const DEPTH24_STENCIL8_EXT: c_uint = 0x88F0;
    pub const DEPTH32F_STENCIL8: c_uint = 0x8CAD;
    pub const DEPTH32F_STENCIL8_NV: c_uint = 0x8DAC;
    pub const DEPTH_ATTACHMENT: c_uint = 0x8D00;
    pub const DEPTH_ATTACHMENT_EXT: c_uint = 0x8D00;
    pub const DEPTH_BOUNDS_EXT: c_uint = 0x8891;
    pub const DEPTH_BOUNDS_TEST_EXT: c_uint = 0x8890;
    pub const DEPTH_BUFFER_BIT: c_uint = 0x00000100;
    pub const DEPTH_BUFFER_FLOAT_MODE_NV: c_uint = 0x8DAF;
    pub const DEPTH_CLAMP: c_uint = 0x864F;
    pub const DEPTH_CLAMP_FAR_AMD: c_uint = 0x901F;
    pub const DEPTH_CLAMP_NEAR_AMD: c_uint = 0x901E;
    pub const DEPTH_CLAMP_NV: c_uint = 0x864F;
    pub const DEPTH_CLEAR_VALUE: c_uint = 0x0B73;
    pub const DEPTH_COMPONENT: c_uint = 0x1902;
    pub const DEPTH_COMPONENT16: c_uint = 0x81A5;
    pub const DEPTH_COMPONENT16_ARB: c_uint = 0x81A5;
    pub const DEPTH_COMPONENT16_SGIX: c_uint = 0x81A5;
    pub const DEPTH_COMPONENT24: c_uint = 0x81A6;
    pub const DEPTH_COMPONENT24_ARB: c_uint = 0x81A6;
    pub const DEPTH_COMPONENT24_SGIX: c_uint = 0x81A6;
    pub const DEPTH_COMPONENT32: c_uint = 0x81A7;
    pub const DEPTH_COMPONENT32F: c_uint = 0x8CAC;
    pub const DEPTH_COMPONENT32F_NV: c_uint = 0x8DAB;
    pub const DEPTH_COMPONENT32_ARB: c_uint = 0x81A7;
    pub const DEPTH_COMPONENT32_SGIX: c_uint = 0x81A7;
    pub const DEPTH_COMPONENTS: c_uint = 0x8284;
    pub const DEPTH_FUNC: c_uint = 0x0B74;
    pub const DEPTH_RANGE: c_uint = 0x0B70;
    pub const DEPTH_RENDERABLE: c_uint = 0x8287;
    pub const DEPTH_SAMPLES_NV: c_uint = 0x932D;
    pub const DEPTH_STENCIL: c_uint = 0x84F9;
    pub const DEPTH_STENCIL_ATTACHMENT: c_uint = 0x821A;
    pub const DEPTH_STENCIL_EXT: c_uint = 0x84F9;
    pub const DEPTH_STENCIL_NV: c_uint = 0x84F9;
    pub const DEPTH_STENCIL_TEXTURE_MODE: c_uint = 0x90EA;
    pub const DEPTH_STENCIL_TO_BGRA_NV: c_uint = 0x886F;
    pub const DEPTH_STENCIL_TO_RGBA_NV: c_uint = 0x886E;
    pub const DEPTH_TEST: c_uint = 0x0B71;
    pub const DEPTH_TEXTURE_MODE_ARB: c_uint = 0x884B;
    pub const DEPTH_WRITEMASK: c_uint = 0x0B72;
    pub const DETACHED_BUFFERS_NV: c_uint = 0x95AB;
    pub const DETACHED_MEMORY_INCARNATION_NV: c_uint = 0x95A9;
    pub const DETACHED_TEXTURES_NV: c_uint = 0x95AA;
    pub const DETAIL_TEXTURE_2D_BINDING_SGIS: c_uint = 0x8096;
    pub const DETAIL_TEXTURE_2D_SGIS: c_uint = 0x8095;
    pub const DETAIL_TEXTURE_FUNC_POINTS_SGIS: c_uint = 0x809C;
    pub const DETAIL_TEXTURE_LEVEL_SGIS: c_uint = 0x809A;
    pub const DETAIL_TEXTURE_MODE_SGIS: c_uint = 0x809B;
    pub const DEVICE_LUID_EXT: c_uint = 0x9599;
    pub const DEVICE_NODE_MASK_EXT: c_uint = 0x959A;
    pub const DEVICE_UUID_EXT: c_uint = 0x9597;
    pub const DIFFERENCE_KHR: c_uint = 0x929E;
    pub const DIFFERENCE_NV: c_uint = 0x929E;
    pub const DISCARD_ATI: c_uint = 0x8763;
    pub const DISCARD_NV: c_uint = 0x8530;
    pub const DISCRETE_AMD: c_uint = 0x9006;
    pub const DISJOINT_NV: c_uint = 0x9283;
    pub const DISPATCH_INDIRECT_BUFFER: c_uint = 0x90EE;
    pub const DISPATCH_INDIRECT_BUFFER_BINDING: c_uint = 0x90EF;
    pub const DISTANCE_ATTENUATION_EXT: c_uint = 0x8129;
    pub const DISTANCE_ATTENUATION_SGIS: c_uint = 0x8129;
    pub const DITHER: c_uint = 0x0BD0;
    pub const DONT_CARE: c_uint = 0x1100;
    pub const DOT2_ADD_ATI: c_uint = 0x896C;
    pub const DOT3_ATI: c_uint = 0x8966;
    pub const DOT3_RGBA_ARB: c_uint = 0x86AF;
    pub const DOT3_RGBA_EXT: c_uint = 0x8741;
    pub const DOT3_RGB_ARB: c_uint = 0x86AE;
    pub const DOT3_RGB_EXT: c_uint = 0x8740;
    pub const DOT4_ATI: c_uint = 0x8967;
    pub const DOT_PRODUCT_AFFINE_DEPTH_REPLACE_NV: c_uint = 0x885D;
    pub const DOT_PRODUCT_CONST_EYE_REFLECT_CUBE_MAP_NV: c_uint = 0x86F3;
    pub const DOT_PRODUCT_DEPTH_REPLACE_NV: c_uint = 0x86ED;
    pub const DOT_PRODUCT_DIFFUSE_CUBE_MAP_NV: c_uint = 0x86F1;
    pub const DOT_PRODUCT_NV: c_uint = 0x86EC;
    pub const DOT_PRODUCT_PASS_THROUGH_NV: c_uint = 0x885B;
    pub const DOT_PRODUCT_REFLECT_CUBE_MAP_NV: c_uint = 0x86F2;
    pub const DOT_PRODUCT_TEXTURE_1D_NV: c_uint = 0x885C;
    pub const DOT_PRODUCT_TEXTURE_2D_NV: c_uint = 0x86EE;
    pub const DOT_PRODUCT_TEXTURE_3D_NV: c_uint = 0x86EF;
    pub const DOT_PRODUCT_TEXTURE_CUBE_MAP_NV: c_uint = 0x86F0;
    pub const DOT_PRODUCT_TEXTURE_RECTANGLE_NV: c_uint = 0x864E;
    pub const DOUBLE: c_uint = 0x140A;
    pub const DOUBLEBUFFER: c_uint = 0x0C32;
    pub const DOUBLE_MAT2: c_uint = 0x8F46;
    pub const DOUBLE_MAT2_EXT: c_uint = 0x8F46;
    pub const DOUBLE_MAT2x3: c_uint = 0x8F49;
    pub const DOUBLE_MAT2x3_EXT: c_uint = 0x8F49;
    pub const DOUBLE_MAT2x4: c_uint = 0x8F4A;
    pub const DOUBLE_MAT2x4_EXT: c_uint = 0x8F4A;
    pub const DOUBLE_MAT3: c_uint = 0x8F47;
    pub const DOUBLE_MAT3_EXT: c_uint = 0x8F47;
    pub const DOUBLE_MAT3x2: c_uint = 0x8F4B;
    pub const DOUBLE_MAT3x2_EXT: c_uint = 0x8F4B;
    pub const DOUBLE_MAT3x4: c_uint = 0x8F4C;
    pub const DOUBLE_MAT3x4_EXT: c_uint = 0x8F4C;
    pub const DOUBLE_MAT4: c_uint = 0x8F48;
    pub const DOUBLE_MAT4_EXT: c_uint = 0x8F48;
    pub const DOUBLE_MAT4x2: c_uint = 0x8F4D;
    pub const DOUBLE_MAT4x2_EXT: c_uint = 0x8F4D;
    pub const DOUBLE_MAT4x3: c_uint = 0x8F4E;
    pub const DOUBLE_MAT4x3_EXT: c_uint = 0x8F4E;
    pub const DOUBLE_VEC2: c_uint = 0x8FFC;
    pub const DOUBLE_VEC2_EXT: c_uint = 0x8FFC;
    pub const DOUBLE_VEC3: c_uint = 0x8FFD;
    pub const DOUBLE_VEC3_EXT: c_uint = 0x8FFD;
    pub const DOUBLE_VEC4: c_uint = 0x8FFE;
    pub const DOUBLE_VEC4_EXT: c_uint = 0x8FFE;
    pub const DRAW_ARRAYS_COMMAND_NV: c_uint = 0x0003;
    pub const DRAW_ARRAYS_INSTANCED_COMMAND_NV: c_uint = 0x0007;
    pub const DRAW_ARRAYS_STRIP_COMMAND_NV: c_uint = 0x0005;
    pub const DRAW_BUFFER: c_uint = 0x0C01;
    pub const DRAW_BUFFER0: c_uint = 0x8825;
    pub const DRAW_BUFFER0_ARB: c_uint = 0x8825;
    pub const DRAW_BUFFER0_ATI: c_uint = 0x8825;
    pub const DRAW_BUFFER1: c_uint = 0x8826;
    pub const DRAW_BUFFER10: c_uint = 0x882F;
    pub const DRAW_BUFFER10_ARB: c_uint = 0x882F;
    pub const DRAW_BUFFER10_ATI: c_uint = 0x882F;
    pub const DRAW_BUFFER11: c_uint = 0x8830;
    pub const DRAW_BUFFER11_ARB: c_uint = 0x8830;
    pub const DRAW_BUFFER11_ATI: c_uint = 0x8830;
    pub const DRAW_BUFFER12: c_uint = 0x8831;
    pub const DRAW_BUFFER12_ARB: c_uint = 0x8831;
    pub const DRAW_BUFFER12_ATI: c_uint = 0x8831;
    pub const DRAW_BUFFER13: c_uint = 0x8832;
    pub const DRAW_BUFFER13_ARB: c_uint = 0x8832;
    pub const DRAW_BUFFER13_ATI: c_uint = 0x8832;
    pub const DRAW_BUFFER14: c_uint = 0x8833;
    pub const DRAW_BUFFER14_ARB: c_uint = 0x8833;
    pub const DRAW_BUFFER14_ATI: c_uint = 0x8833;
    pub const DRAW_BUFFER15: c_uint = 0x8834;
    pub const DRAW_BUFFER15_ARB: c_uint = 0x8834;
    pub const DRAW_BUFFER15_ATI: c_uint = 0x8834;
    pub const DRAW_BUFFER1_ARB: c_uint = 0x8826;
    pub const DRAW_BUFFER1_ATI: c_uint = 0x8826;
    pub const DRAW_BUFFER2: c_uint = 0x8827;
    pub const DRAW_BUFFER2_ARB: c_uint = 0x8827;
    pub const DRAW_BUFFER2_ATI: c_uint = 0x8827;
    pub const DRAW_BUFFER3: c_uint = 0x8828;
    pub const DRAW_BUFFER3_ARB: c_uint = 0x8828;
    pub const DRAW_BUFFER3_ATI: c_uint = 0x8828;
    pub const DRAW_BUFFER4: c_uint = 0x8829;
    pub const DRAW_BUFFER4_ARB: c_uint = 0x8829;
    pub const DRAW_BUFFER4_ATI: c_uint = 0x8829;
    pub const DRAW_BUFFER5: c_uint = 0x882A;
    pub const DRAW_BUFFER5_ARB: c_uint = 0x882A;
    pub const DRAW_BUFFER5_ATI: c_uint = 0x882A;
    pub const DRAW_BUFFER6: c_uint = 0x882B;
    pub const DRAW_BUFFER6_ARB: c_uint = 0x882B;
    pub const DRAW_BUFFER6_ATI: c_uint = 0x882B;
    pub const DRAW_BUFFER7: c_uint = 0x882C;
    pub const DRAW_BUFFER7_ARB: c_uint = 0x882C;
    pub const DRAW_BUFFER7_ATI: c_uint = 0x882C;
    pub const DRAW_BUFFER8: c_uint = 0x882D;
    pub const DRAW_BUFFER8_ARB: c_uint = 0x882D;
    pub const DRAW_BUFFER8_ATI: c_uint = 0x882D;
    pub const DRAW_BUFFER9: c_uint = 0x882E;
    pub const DRAW_BUFFER9_ARB: c_uint = 0x882E;
    pub const DRAW_BUFFER9_ATI: c_uint = 0x882E;
    pub const DRAW_ELEMENTS_COMMAND_NV: c_uint = 0x0002;
    pub const DRAW_ELEMENTS_INSTANCED_COMMAND_NV: c_uint = 0x0006;
    pub const DRAW_ELEMENTS_STRIP_COMMAND_NV: c_uint = 0x0004;
    pub const DRAW_FRAMEBUFFER: c_uint = 0x8CA9;
    pub const DRAW_FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
    pub const DRAW_FRAMEBUFFER_BINDING_EXT: c_uint = 0x8CA6;
    pub const DRAW_FRAMEBUFFER_EXT: c_uint = 0x8CA9;
    pub const DRAW_INDIRECT_ADDRESS_NV: c_uint = 0x8F41;
    pub const DRAW_INDIRECT_BUFFER: c_uint = 0x8F3F;
    pub const DRAW_INDIRECT_BUFFER_BINDING: c_uint = 0x8F43;
    pub const DRAW_INDIRECT_LENGTH_NV: c_uint = 0x8F42;
    pub const DRAW_INDIRECT_UNIFIED_NV: c_uint = 0x8F40;
    pub const DRAW_PIXELS_APPLE: c_uint = 0x8A0A;
    pub const DRIVER_UUID_EXT: c_uint = 0x9598;
    pub const DSDT8_MAG8_INTENSITY8_NV: c_uint = 0x870B;
    pub const DSDT8_MAG8_NV: c_uint = 0x870A;
    pub const DSDT8_NV: c_uint = 0x8709;
    pub const DSDT_MAG_INTENSITY_NV: c_uint = 0x86DC;
    pub const DSDT_MAG_NV: c_uint = 0x86F6;
    pub const DSDT_MAG_VIB_NV: c_uint = 0x86F7;
    pub const DSDT_NV: c_uint = 0x86F5;
    pub const DST_ALPHA: c_uint = 0x0304;
    pub const DST_ATOP_NV: c_uint = 0x928F;
    pub const DST_COLOR: c_uint = 0x0306;
    pub const DST_IN_NV: c_uint = 0x928B;
    pub const DST_NV: c_uint = 0x9287;
    pub const DST_OUT_NV: c_uint = 0x928D;
    pub const DST_OVER_NV: c_uint = 0x9289;
    pub const DS_BIAS_NV: c_uint = 0x8716;
    pub const DS_SCALE_NV: c_uint = 0x8710;
    pub const DT_BIAS_NV: c_uint = 0x8717;
    pub const DT_SCALE_NV: c_uint = 0x8711;
    pub const DU8DV8_ATI: c_uint = 0x877A;
    pub const DUAL_ALPHA12_SGIS: c_uint = 0x8112;
    pub const DUAL_ALPHA16_SGIS: c_uint = 0x8113;
    pub const DUAL_ALPHA4_SGIS: c_uint = 0x8110;
    pub const DUAL_ALPHA8_SGIS: c_uint = 0x8111;
    pub const DUAL_INTENSITY12_SGIS: c_uint = 0x811A;
    pub const DUAL_INTENSITY16_SGIS: c_uint = 0x811B;
    pub const DUAL_INTENSITY4_SGIS: c_uint = 0x8118;
    pub const DUAL_INTENSITY8_SGIS: c_uint = 0x8119;
    pub const DUAL_LUMINANCE12_SGIS: c_uint = 0x8116;
    pub const DUAL_LUMINANCE16_SGIS: c_uint = 0x8117;
    pub const DUAL_LUMINANCE4_SGIS: c_uint = 0x8114;
    pub const DUAL_LUMINANCE8_SGIS: c_uint = 0x8115;
    pub const DUAL_LUMINANCE_ALPHA4_SGIS: c_uint = 0x811C;
    pub const DUAL_LUMINANCE_ALPHA8_SGIS: c_uint = 0x811D;
    pub const DUAL_TEXTURE_SELECT_SGIS: c_uint = 0x8124;
    pub const DUDV_ATI: c_uint = 0x8779;
    pub const DUP_FIRST_CUBIC_CURVE_TO_NV: c_uint = 0xF2;
    pub const DUP_LAST_CUBIC_CURVE_TO_NV: c_uint = 0xF4;
    pub const DYNAMIC_ATI: c_uint = 0x8761;
    pub const DYNAMIC_COPY: c_uint = 0x88EA;
    pub const DYNAMIC_COPY_ARB: c_uint = 0x88EA;
    pub const DYNAMIC_DRAW: c_uint = 0x88E8;
    pub const DYNAMIC_DRAW_ARB: c_uint = 0x88E8;
    pub const DYNAMIC_READ: c_uint = 0x88E9;
    pub const DYNAMIC_READ_ARB: c_uint = 0x88E9;
    pub const DYNAMIC_STORAGE_BIT: c_uint = 0x0100;
    pub const EDGEFLAG_BIT_PGI: c_uint = 0x00040000;
    pub const EDGE_FLAG_ARRAY_ADDRESS_NV: c_uint = 0x8F26;
    pub const EDGE_FLAG_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889B;
    pub const EDGE_FLAG_ARRAY_COUNT_EXT: c_uint = 0x808D;
    pub const EDGE_FLAG_ARRAY_EXT: c_uint = 0x8079;
    pub const EDGE_FLAG_ARRAY_LENGTH_NV: c_uint = 0x8F30;
    pub const EDGE_FLAG_ARRAY_LIST_IBM: c_uint = 103075;
    pub const EDGE_FLAG_ARRAY_LIST_STRIDE_IBM: c_uint = 103085;
    pub const EDGE_FLAG_ARRAY_POINTER_EXT: c_uint = 0x8093;
    pub const EDGE_FLAG_ARRAY_STRIDE_EXT: c_uint = 0x808C;
    pub const EFFECTIVE_RASTER_SAMPLES_EXT: c_uint = 0x932C;
    pub const EIGHTH_BIT_ATI: c_uint = 0x00000020;
    pub const ELEMENT_ADDRESS_COMMAND_NV: c_uint = 0x0008;
    pub const ELEMENT_ARRAY_ADDRESS_NV: c_uint = 0x8F29;
    pub const ELEMENT_ARRAY_APPLE: c_uint = 0x8A0C;
    pub const ELEMENT_ARRAY_ATI: c_uint = 0x8768;
    pub const ELEMENT_ARRAY_BARRIER_BIT: c_uint = 0x00000002;
    pub const ELEMENT_ARRAY_BARRIER_BIT_EXT: c_uint = 0x00000002;
    pub const ELEMENT_ARRAY_BUFFER: c_uint = 0x8893;
    pub const ELEMENT_ARRAY_BUFFER_ARB: c_uint = 0x8893;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895;
    pub const ELEMENT_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8895;
    pub const ELEMENT_ARRAY_LENGTH_NV: c_uint = 0x8F33;
    pub const ELEMENT_ARRAY_POINTER_APPLE: c_uint = 0x8A0E;
    pub const ELEMENT_ARRAY_POINTER_ATI: c_uint = 0x876A;
    pub const ELEMENT_ARRAY_TYPE_APPLE: c_uint = 0x8A0D;
    pub const ELEMENT_ARRAY_TYPE_ATI: c_uint = 0x8769;
    pub const ELEMENT_ARRAY_UNIFIED_NV: c_uint = 0x8F1F;
    pub const EMBOSS_CONSTANT_NV: c_uint = 0x855E;
    pub const EMBOSS_LIGHT_NV: c_uint = 0x855D;
    pub const EMBOSS_MAP_NV: c_uint = 0x855F;
    pub const EQUAL: c_uint = 0x0202;
    pub const EQUIV: c_uint = 0x1509;
    pub const EVAL_2D_NV: c_uint = 0x86C0;
    pub const EVAL_FRACTIONAL_TESSELLATION_NV: c_uint = 0x86C5;
    pub const EVAL_TRIANGULAR_2D_NV: c_uint = 0x86C1;
    pub const EVAL_VERTEX_ATTRIB0_NV: c_uint = 0x86C6;
    pub const EVAL_VERTEX_ATTRIB10_NV: c_uint = 0x86D0;
    pub const EVAL_VERTEX_ATTRIB11_NV: c_uint = 0x86D1;
    pub const EVAL_VERTEX_ATTRIB12_NV: c_uint = 0x86D2;
    pub const EVAL_VERTEX_ATTRIB13_NV: c_uint = 0x86D3;
    pub const EVAL_VERTEX_ATTRIB14_NV: c_uint = 0x86D4;
    pub const EVAL_VERTEX_ATTRIB15_NV: c_uint = 0x86D5;
    pub const EVAL_VERTEX_ATTRIB1_NV: c_uint = 0x86C7;
    pub const EVAL_VERTEX_ATTRIB2_NV: c_uint = 0x86C8;
    pub const EVAL_VERTEX_ATTRIB3_NV: c_uint = 0x86C9;
    pub const EVAL_VERTEX_ATTRIB4_NV: c_uint = 0x86CA;
    pub const EVAL_VERTEX_ATTRIB5_NV: c_uint = 0x86CB;
    pub const EVAL_VERTEX_ATTRIB6_NV: c_uint = 0x86CC;
    pub const EVAL_VERTEX_ATTRIB7_NV: c_uint = 0x86CD;
    pub const EVAL_VERTEX_ATTRIB8_NV: c_uint = 0x86CE;
    pub const EVAL_VERTEX_ATTRIB9_NV: c_uint = 0x86CF;
    pub const EXCLUSION_KHR: c_uint = 0x92A0;
    pub const EXCLUSION_NV: c_uint = 0x92A0;
    pub const EXCLUSIVE_EXT: c_uint = 0x8F11;
    pub const EXPAND_NEGATE_NV: c_uint = 0x8539;
    pub const EXPAND_NORMAL_NV: c_uint = 0x8538;
    pub const EXTENSIONS: c_uint = 0x1F03;
    pub const EXTERNAL_VIRTUAL_MEMORY_BUFFER_AMD: c_uint = 0x9160;
    pub const EYE_DISTANCE_TO_LINE_SGIS: c_uint = 0x81F2;
    pub const EYE_DISTANCE_TO_POINT_SGIS: c_uint = 0x81F0;
    pub const EYE_LINE_SGIS: c_uint = 0x81F6;
    pub const EYE_PLANE: c_uint = 0x2502;
    pub const EYE_PLANE_ABSOLUTE_NV: c_uint = 0x855C;
    pub const EYE_POINT_SGIS: c_uint = 0x81F4;
    pub const EYE_RADIAL_NV: c_uint = 0x855B;
    pub const E_TIMES_F_NV: c_uint = 0x8531;
    pub const FACTOR_MAX_AMD: c_uint = 0x901D;
    pub const FACTOR_MIN_AMD: c_uint = 0x901C;
    pub const FAILURE_NV: c_uint = 0x9030;
    pub const FALSE: c_uchar = 0;
    pub const FASTEST: c_uint = 0x1101;
    pub const FENCE_APPLE: c_uint = 0x8A0B;
    pub const FENCE_CONDITION_NV: c_uint = 0x84F4;
    pub const FENCE_STATUS_NV: c_uint = 0x84F3;
    pub const FIELDS_NV: c_uint = 0x8E27;
    pub const FIELD_LOWER_NV: c_uint = 0x9023;
    pub const FIELD_UPPER_NV: c_uint = 0x9022;
    pub const FILE_NAME_NV: c_uint = 0x9074;
    pub const FILL: c_uint = 0x1B02;
    pub const FILL_RECTANGLE_NV: c_uint = 0x933C;
    pub const FILTER: c_uint = 0x829A;
    pub const FILTER4_SGIS: c_uint = 0x8146;
    pub const FIRST_TO_REST_NV: c_uint = 0x90AF;
    pub const FIRST_VERTEX_CONVENTION: c_uint = 0x8E4D;
    pub const FIRST_VERTEX_CONVENTION_EXT: c_uint = 0x8E4D;
    pub const FIXED: c_uint = 0x140C;
    pub const FIXED_OES: c_uint = 0x140C;
    pub const FIXED_ONLY: c_uint = 0x891D;
    pub const FIXED_ONLY_ARB: c_uint = 0x891D;
    pub const FLOAT: c_uint = 0x1406;
    pub const FLOAT16_MAT2_AMD: c_uint = 0x91C5;
    pub const FLOAT16_MAT2x3_AMD: c_uint = 0x91C8;
    pub const FLOAT16_MAT2x4_AMD: c_uint = 0x91C9;
    pub const FLOAT16_MAT3_AMD: c_uint = 0x91C6;
    pub const FLOAT16_MAT3x2_AMD: c_uint = 0x91CA;
    pub const FLOAT16_MAT3x4_AMD: c_uint = 0x91CB;
    pub const FLOAT16_MAT4_AMD: c_uint = 0x91C7;
    pub const FLOAT16_MAT4x2_AMD: c_uint = 0x91CC;
    pub const FLOAT16_MAT4x3_AMD: c_uint = 0x91CD;
    pub const FLOAT16_NV: c_uint = 0x8FF8;
    pub const FLOAT16_VEC2_NV: c_uint = 0x8FF9;
    pub const FLOAT16_VEC3_NV: c_uint = 0x8FFA;
    pub const FLOAT16_VEC4_NV: c_uint = 0x8FFB;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: c_uint = 0x8DAD;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV_NV: c_uint = 0x8DAD;
    pub const FLOAT_CLEAR_COLOR_VALUE_NV: c_uint = 0x888D;
    pub const FLOAT_MAT2: c_uint = 0x8B5A;
    pub const FLOAT_MAT2_ARB: c_uint = 0x8B5A;
    pub const FLOAT_MAT2x3: c_uint = 0x8B65;
    pub const FLOAT_MAT2x4: c_uint = 0x8B66;
    pub const FLOAT_MAT3: c_uint = 0x8B5B;
    pub const FLOAT_MAT3_ARB: c_uint = 0x8B5B;
    pub const FLOAT_MAT3x2: c_uint = 0x8B67;
    pub const FLOAT_MAT3x4: c_uint = 0x8B68;
    pub const FLOAT_MAT4: c_uint = 0x8B5C;
    pub const FLOAT_MAT4_ARB: c_uint = 0x8B5C;
    pub const FLOAT_MAT4x2: c_uint = 0x8B69;
    pub const FLOAT_MAT4x3: c_uint = 0x8B6A;
    pub const FLOAT_R16_NV: c_uint = 0x8884;
    pub const FLOAT_R32_NV: c_uint = 0x8885;
    pub const FLOAT_RG16_NV: c_uint = 0x8886;
    pub const FLOAT_RG32_NV: c_uint = 0x8887;
    pub const FLOAT_RGB16_NV: c_uint = 0x8888;
    pub const FLOAT_RGB32_NV: c_uint = 0x8889;
    pub const FLOAT_RGBA16_NV: c_uint = 0x888A;
    pub const FLOAT_RGBA32_NV: c_uint = 0x888B;
    pub const FLOAT_RGBA_MODE_NV: c_uint = 0x888E;
    pub const FLOAT_RGBA_NV: c_uint = 0x8883;
    pub const FLOAT_RGB_NV: c_uint = 0x8882;
    pub const FLOAT_RG_NV: c_uint = 0x8881;
    pub const FLOAT_R_NV: c_uint = 0x8880;
    pub const FLOAT_VEC2: c_uint = 0x8B50;
    pub const FLOAT_VEC2_ARB: c_uint = 0x8B50;
    pub const FLOAT_VEC3: c_uint = 0x8B51;
    pub const FLOAT_VEC3_ARB: c_uint = 0x8B51;
    pub const FLOAT_VEC4: c_uint = 0x8B52;
    pub const FLOAT_VEC4_ARB: c_uint = 0x8B52;
    pub const FOG: c_uint = 0x0B60;
    pub const FOG_COORDINATE_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889D;
    pub const FOG_COORDINATE_ARRAY_EXT: c_uint = 0x8457;
    pub const FOG_COORDINATE_ARRAY_LIST_IBM: c_uint = 103076;
    pub const FOG_COORDINATE_ARRAY_LIST_STRIDE_IBM: c_uint = 103086;
    pub const FOG_COORDINATE_ARRAY_POINTER_EXT: c_uint = 0x8456;
    pub const FOG_COORDINATE_ARRAY_STRIDE_EXT: c_uint = 0x8455;
    pub const FOG_COORDINATE_ARRAY_TYPE_EXT: c_uint = 0x8454;
    pub const FOG_COORDINATE_EXT: c_uint = 0x8451;
    pub const FOG_COORDINATE_SOURCE_EXT: c_uint = 0x8450;
    pub const FOG_COORD_ARRAY_ADDRESS_NV: c_uint = 0x8F28;
    pub const FOG_COORD_ARRAY_LENGTH_NV: c_uint = 0x8F32;
    pub const FOG_DISTANCE_MODE_NV: c_uint = 0x855A;
    pub const FOG_FUNC_POINTS_SGIS: c_uint = 0x812B;
    pub const FOG_FUNC_SGIS: c_uint = 0x812A;
    pub const FOG_OFFSET_SGIX: c_uint = 0x8198;
    pub const FOG_OFFSET_VALUE_SGIX: c_uint = 0x8199;
    pub const FOG_SPECULAR_TEXTURE_WIN: c_uint = 0x80EC;
    pub const FONT_ASCENDER_BIT_NV: c_uint = 0x00200000;
    pub const FONT_DESCENDER_BIT_NV: c_uint = 0x00400000;
    pub const FONT_GLYPHS_AVAILABLE_NV: c_uint = 0x9368;
    pub const FONT_HAS_KERNING_BIT_NV: c_uint = 0x10000000;
    pub const FONT_HEIGHT_BIT_NV: c_uint = 0x00800000;
    pub const FONT_MAX_ADVANCE_HEIGHT_BIT_NV: c_uint = 0x02000000;
    pub const FONT_MAX_ADVANCE_WIDTH_BIT_NV: c_uint = 0x01000000;
    pub const FONT_NUM_GLYPH_INDICES_BIT_NV: c_uint = 0x20000000;
    pub const FONT_TARGET_UNAVAILABLE_NV: c_uint = 0x9369;
    pub const FONT_UNAVAILABLE_NV: c_uint = 0x936A;
    pub const FONT_UNDERLINE_POSITION_BIT_NV: c_uint = 0x04000000;
    pub const FONT_UNDERLINE_THICKNESS_BIT_NV: c_uint = 0x08000000;
    pub const FONT_UNINTELLIGIBLE_NV: c_uint = 0x936B;
    pub const FONT_UNITS_PER_EM_BIT_NV: c_uint = 0x00100000;
    pub const FONT_X_MAX_BOUNDS_BIT_NV: c_uint = 0x00040000;
    pub const FONT_X_MIN_BOUNDS_BIT_NV: c_uint = 0x00010000;
    pub const FONT_Y_MAX_BOUNDS_BIT_NV: c_uint = 0x00080000;
    pub const FONT_Y_MIN_BOUNDS_BIT_NV: c_uint = 0x00020000;
    pub const FORCE_BLUE_TO_ONE_NV: c_uint = 0x8860;
    pub const FORMAT_SUBSAMPLE_244_244_OML: c_uint = 0x8983;
    pub const FORMAT_SUBSAMPLE_24_24_OML: c_uint = 0x8982;
    pub const FRACTIONAL_EVEN: c_uint = 0x8E7C;
    pub const FRACTIONAL_ODD: c_uint = 0x8E7B;
    pub const FRAGMENT_COLOR_EXT: c_uint = 0x834C;
    pub const FRAGMENT_COLOR_MATERIAL_FACE_SGIX: c_uint = 0x8402;
    pub const FRAGMENT_COLOR_MATERIAL_PARAMETER_SGIX: c_uint = 0x8403;
    pub const FRAGMENT_COLOR_MATERIAL_SGIX: c_uint = 0x8401;
    pub const FRAGMENT_COVERAGE_COLOR_NV: c_uint = 0x92DE;
    pub const FRAGMENT_COVERAGE_TO_COLOR_NV: c_uint = 0x92DD;
    pub const FRAGMENT_DEPTH_EXT: c_uint = 0x8452;
    pub const FRAGMENT_INPUT_NV: c_uint = 0x936D;
    pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: c_uint = 0x8E5D;
    pub const FRAGMENT_LIGHT0_SGIX: c_uint = 0x840C;
    pub const FRAGMENT_LIGHT1_SGIX: c_uint = 0x840D;
    pub const FRAGMENT_LIGHT2_SGIX: c_uint = 0x840E;
    pub const FRAGMENT_LIGHT3_SGIX: c_uint = 0x840F;
    pub const FRAGMENT_LIGHT4_SGIX: c_uint = 0x8410;
    pub const FRAGMENT_LIGHT5_SGIX: c_uint = 0x8411;
    pub const FRAGMENT_LIGHT6_SGIX: c_uint = 0x8412;
    pub const FRAGMENT_LIGHT7_SGIX: c_uint = 0x8413;
    pub const FRAGMENT_LIGHTING_SGIX: c_uint = 0x8400;
    pub const FRAGMENT_LIGHT_MODEL_AMBIENT_SGIX: c_uint = 0x840A;
    pub const FRAGMENT_LIGHT_MODEL_LOCAL_VIEWER_SGIX: c_uint = 0x8408;
    pub const FRAGMENT_LIGHT_MODEL_NORMAL_INTERPOLATION_SGIX: c_uint = 0x840B;
    pub const FRAGMENT_LIGHT_MODEL_TWO_SIDE_SGIX: c_uint = 0x8409;
    pub const FRAGMENT_MATERIAL_EXT: c_uint = 0x8349;
    pub const FRAGMENT_NORMAL_EXT: c_uint = 0x834A;
    pub const FRAGMENT_PROGRAM_ARB: c_uint = 0x8804;
    pub const FRAGMENT_PROGRAM_BINDING_NV: c_uint = 0x8873;
    pub const FRAGMENT_PROGRAM_INTERPOLATION_OFFSET_BITS_NV: c_uint = 0x8E5D;
    pub const FRAGMENT_PROGRAM_NV: c_uint = 0x8870;
    pub const FRAGMENT_PROGRAM_PARAMETER_BUFFER_NV: c_uint = 0x8DA4;
    pub const FRAGMENT_SHADER: c_uint = 0x8B30;
    pub const FRAGMENT_SHADER_ARB: c_uint = 0x8B30;
    pub const FRAGMENT_SHADER_ATI: c_uint = 0x8920;
    pub const FRAGMENT_SHADER_BIT: c_uint = 0x00000002;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: c_uint = 0x8B8B;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT_ARB: c_uint = 0x8B8B;
    pub const FRAGMENT_SHADER_DISCARDS_SAMPLES_EXT: c_uint = 0x8A52;
    pub const FRAGMENT_SHADER_INVOCATIONS: c_uint = 0x82F4;
    pub const FRAGMENT_SHADER_INVOCATIONS_ARB: c_uint = 0x82F4;
    pub const FRAGMENT_SUBROUTINE: c_uint = 0x92EC;
    pub const FRAGMENT_SUBROUTINE_UNIFORM: c_uint = 0x92F2;
    pub const FRAGMENT_TEXTURE: c_uint = 0x829F;
    pub const FRAMEBUFFER: c_uint = 0x8D40;
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: c_uint = 0x8215;
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: c_uint = 0x8214;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: c_uint = 0x8210;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: c_uint = 0x8211;
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: c_uint = 0x8216;
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: c_uint = 0x8213;
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED: c_uint = 0x8DA7;
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED_ARB: c_uint = 0x8DA7;
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED_EXT: c_uint = 0x8DA7;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: c_uint = 0x8CD1;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_EXT: c_uint = 0x8CD1;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: c_uint = 0x8CD0;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_EXT: c_uint = 0x8CD0;
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: c_uint = 0x8212;
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: c_uint = 0x8217;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_EXT: c_uint = 0x8CD4;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_BASE_VIEW_INDEX_OVR: c_uint = 0x9632;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: c_uint = 0x8CD3;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_EXT: c_uint = 0x8CD3;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: c_uint = 0x8CD4;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER_EXT: c_uint = 0x8CD4;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: c_uint = 0x8CD2;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_EXT: c_uint = 0x8CD2;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_NUM_VIEWS_OVR: c_uint = 0x9630;
    pub const FRAMEBUFFER_BARRIER_BIT: c_uint = 0x00000400;
    pub const FRAMEBUFFER_BARRIER_BIT_EXT: c_uint = 0x00000400;
    pub const FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
    pub const FRAMEBUFFER_BINDING_EXT: c_uint = 0x8CA6;
    pub const FRAMEBUFFER_BLEND: c_uint = 0x828B;
    pub const FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5;
    pub const FRAMEBUFFER_COMPLETE_EXT: c_uint = 0x8CD5;
    pub const FRAMEBUFFER_DEFAULT: c_uint = 0x8218;
    pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: c_uint = 0x9314;
    pub const FRAMEBUFFER_DEFAULT_HEIGHT: c_uint = 0x9311;
    pub const FRAMEBUFFER_DEFAULT_LAYERS: c_uint = 0x9312;
    pub const FRAMEBUFFER_DEFAULT_SAMPLES: c_uint = 0x9313;
    pub const FRAMEBUFFER_DEFAULT_WIDTH: c_uint = 0x9310;
    pub const FRAMEBUFFER_EXT: c_uint = 0x8D40;
    pub const FRAMEBUFFER_FLIP_X_MESA: c_uint = 0x8BBC;
    pub const FRAMEBUFFER_FLIP_Y_MESA: c_uint = 0x8BBB;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: c_uint = 0x8CD6;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT_EXT: c_uint = 0x8CD6;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS_EXT: c_uint = 0x8CD9;
    pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: c_uint = 0x8CDB;
    pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_EXT: c_uint = 0x8CDB;
    pub const FRAMEBUFFER_INCOMPLETE_FORMATS_EXT: c_uint = 0x8CDA;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_ARB: c_uint = 0x8DA9;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_EXT: c_uint = 0x8DA9;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: c_uint = 0x8DA8;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_ARB: c_uint = 0x8DA8;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_EXT: c_uint = 0x8DA8;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: c_uint = 0x8CD7;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_EXT: c_uint = 0x8CD7;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: c_uint = 0x8D56;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_EXT: c_uint = 0x8D56;
    pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: c_uint = 0x8CDC;
    pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER_EXT: c_uint = 0x8CDC;
    pub const FRAMEBUFFER_INCOMPLETE_VIEW_TARGETS_OVR: c_uint = 0x9633;
    pub const FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_ARB: c_uint = 0x9342;
    pub const FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_NV: c_uint = 0x9342;
    pub const FRAMEBUFFER_RENDERABLE: c_uint = 0x8289;
    pub const FRAMEBUFFER_RENDERABLE_LAYERED: c_uint = 0x828A;
    pub const FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_ARB: c_uint = 0x9343;
    pub const FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_NV: c_uint = 0x9343;
    pub const FRAMEBUFFER_SRGB: c_uint = 0x8DB9;
    pub const FRAMEBUFFER_SRGB_CAPABLE_EXT: c_uint = 0x8DBA;
    pub const FRAMEBUFFER_SRGB_EXT: c_uint = 0x8DB9;
    pub const FRAMEBUFFER_SWAP_XY_MESA: c_uint = 0x8BBD;
    pub const FRAMEBUFFER_UNDEFINED: c_uint = 0x8219;
    pub const FRAMEBUFFER_UNSUPPORTED: c_uint = 0x8CDD;
    pub const FRAMEBUFFER_UNSUPPORTED_EXT: c_uint = 0x8CDD;
    pub const FRAMEZOOM_FACTOR_SGIX: c_uint = 0x818C;
    pub const FRAMEZOOM_SGIX: c_uint = 0x818B;
    pub const FRAME_NV: c_uint = 0x8E26;
    pub const FRONT: c_uint = 0x0404;
    pub const FRONT_AND_BACK: c_uint = 0x0408;
    pub const FRONT_FACE: c_uint = 0x0B46;
    pub const FRONT_FACE_COMMAND_NV: c_uint = 0x0012;
    pub const FRONT_LEFT: c_uint = 0x0400;
    pub const FRONT_RIGHT: c_uint = 0x0401;
    pub const FULL_RANGE_EXT: c_uint = 0x87E1;
    pub const FULL_STIPPLE_HINT_PGI: c_uint = 0x1A219;
    pub const FULL_SUPPORT: c_uint = 0x82B7;
    pub const FUNC_ADD: c_uint = 0x8006;
    pub const FUNC_ADD_EXT: c_uint = 0x8006;
    pub const FUNC_REVERSE_SUBTRACT: c_uint = 0x800B;
    pub const FUNC_REVERSE_SUBTRACT_EXT: c_uint = 0x800B;
    pub const FUNC_SUBTRACT: c_uint = 0x800A;
    pub const FUNC_SUBTRACT_EXT: c_uint = 0x800A;
    pub const GENERATE_MIPMAP_HINT_SGIS: c_uint = 0x8192;
    pub const GENERATE_MIPMAP_SGIS: c_uint = 0x8191;
    pub const GENERIC_ATTRIB_NV: c_uint = 0x8C7D;
    pub const GEOMETRY_DEFORMATION_BIT_SGIX: c_uint = 0x00000002;
    pub const GEOMETRY_DEFORMATION_SGIX: c_uint = 0x8194;
    pub const GEOMETRY_INPUT_TYPE: c_uint = 0x8917;
    pub const GEOMETRY_INPUT_TYPE_ARB: c_uint = 0x8DDB;
    pub const GEOMETRY_INPUT_TYPE_EXT: c_uint = 0x8DDB;
    pub const GEOMETRY_OUTPUT_TYPE: c_uint = 0x8918;
    pub const GEOMETRY_OUTPUT_TYPE_ARB: c_uint = 0x8DDC;
    pub const GEOMETRY_OUTPUT_TYPE_EXT: c_uint = 0x8DDC;
    pub const GEOMETRY_PROGRAM_NV: c_uint = 0x8C26;
    pub const GEOMETRY_PROGRAM_PARAMETER_BUFFER_NV: c_uint = 0x8DA3;
    pub const GEOMETRY_SHADER: c_uint = 0x8DD9;
    pub const GEOMETRY_SHADER_ARB: c_uint = 0x8DD9;
    pub const GEOMETRY_SHADER_BIT: c_uint = 0x00000004;
    pub const GEOMETRY_SHADER_EXT: c_uint = 0x8DD9;
    pub const GEOMETRY_SHADER_INVOCATIONS: c_uint = 0x887F;
    pub const GEOMETRY_SHADER_PRIMITIVES_EMITTED: c_uint = 0x82F3;
    pub const GEOMETRY_SHADER_PRIMITIVES_EMITTED_ARB: c_uint = 0x82F3;
    pub const GEOMETRY_SUBROUTINE: c_uint = 0x92EB;
    pub const GEOMETRY_SUBROUTINE_UNIFORM: c_uint = 0x92F1;
    pub const GEOMETRY_TEXTURE: c_uint = 0x829E;
    pub const GEOMETRY_VERTICES_OUT: c_uint = 0x8916;
    pub const GEOMETRY_VERTICES_OUT_ARB: c_uint = 0x8DDA;
    pub const GEOMETRY_VERTICES_OUT_EXT: c_uint = 0x8DDA;
    pub const GEQUAL: c_uint = 0x0206;
    pub const GET_TEXTURE_IMAGE_FORMAT: c_uint = 0x8291;
    pub const GET_TEXTURE_IMAGE_TYPE: c_uint = 0x8292;
    pub const GLOBAL_ALPHA_FACTOR_SUN: c_uint = 0x81DA;
    pub const GLOBAL_ALPHA_SUN: c_uint = 0x81D9;
    pub const GLYPH_HAS_KERNING_BIT_NV: c_uint = 0x100;
    pub const GLYPH_HEIGHT_BIT_NV: c_uint = 0x02;
    pub const GLYPH_HORIZONTAL_BEARING_ADVANCE_BIT_NV: c_uint = 0x10;
    pub const GLYPH_HORIZONTAL_BEARING_X_BIT_NV: c_uint = 0x04;
    pub const GLYPH_HORIZONTAL_BEARING_Y_BIT_NV: c_uint = 0x08;
    pub const GLYPH_VERTICAL_BEARING_ADVANCE_BIT_NV: c_uint = 0x80;
    pub const GLYPH_VERTICAL_BEARING_X_BIT_NV: c_uint = 0x20;
    pub const GLYPH_VERTICAL_BEARING_Y_BIT_NV: c_uint = 0x40;
    pub const GLYPH_WIDTH_BIT_NV: c_uint = 0x01;
    pub const GPU_ADDRESS_NV: c_uint = 0x8F34;
    pub const GPU_MEMORY_INFO_CURRENT_AVAILABLE_VIDMEM_NVX: c_uint = 0x9049;
    pub const GPU_MEMORY_INFO_DEDICATED_VIDMEM_NVX: c_uint = 0x9047;
    pub const GPU_MEMORY_INFO_EVICTED_MEMORY_NVX: c_uint = 0x904B;
    pub const GPU_MEMORY_INFO_EVICTION_COUNT_NVX: c_uint = 0x904A;
    pub const GPU_MEMORY_INFO_TOTAL_AVAILABLE_MEMORY_NVX: c_uint = 0x9048;
    pub const GREATER: c_uint = 0x0204;
    pub const GREEN: c_uint = 0x1904;
    pub const GREEN_BIT_ATI: c_uint = 0x00000002;
    pub const GREEN_INTEGER: c_uint = 0x8D95;
    pub const GREEN_INTEGER_EXT: c_uint = 0x8D95;
    pub const GREEN_MAX_CLAMP_INGR: c_uint = 0x8565;
    pub const GREEN_MIN_CLAMP_INGR: c_uint = 0x8561;
    pub const GREEN_NV: c_uint = 0x1904;
    pub const GUILTY_CONTEXT_RESET: c_uint = 0x8253;
    pub const GUILTY_CONTEXT_RESET_ARB: c_uint = 0x8253;
    pub const HALF_APPLE: c_uint = 0x140B;
    pub const HALF_BIAS_NEGATE_NV: c_uint = 0x853B;
    pub const HALF_BIAS_NORMAL_NV: c_uint = 0x853A;
    pub const HALF_BIT_ATI: c_uint = 0x00000008;
    pub const HALF_FLOAT: c_uint = 0x140B;
    pub const HALF_FLOAT_ARB: c_uint = 0x140B;
    pub const HALF_FLOAT_NV: c_uint = 0x140B;
    pub const HANDLE_TYPE_D3D11_IMAGE_EXT: c_uint = 0x958B;
    pub const HANDLE_TYPE_D3D11_IMAGE_KMT_EXT: c_uint = 0x958C;
    pub const HANDLE_TYPE_D3D12_FENCE_EXT: c_uint = 0x9594;
    pub const HANDLE_TYPE_D3D12_RESOURCE_EXT: c_uint = 0x958A;
    pub const HANDLE_TYPE_D3D12_TILEPOOL_EXT: c_uint = 0x9589;
    pub const HANDLE_TYPE_OPAQUE_FD_EXT: c_uint = 0x9586;
    pub const HANDLE_TYPE_OPAQUE_WIN32_EXT: c_uint = 0x9587;
    pub const HANDLE_TYPE_OPAQUE_WIN32_KMT_EXT: c_uint = 0x9588;
    pub const HARDLIGHT_KHR: c_uint = 0x929B;
    pub const HARDLIGHT_NV: c_uint = 0x929B;
    pub const HARDMIX_NV: c_uint = 0x92A9;
    pub const HIGH_FLOAT: c_uint = 0x8DF2;
    pub const HIGH_INT: c_uint = 0x8DF5;
    pub const HILO16_NV: c_uint = 0x86F8;
    pub const HILO8_NV: c_uint = 0x885E;
    pub const HILO_NV: c_uint = 0x86F4;
    pub const HISTOGRAM_ALPHA_SIZE_EXT: c_uint = 0x802B;
    pub const HISTOGRAM_BLUE_SIZE_EXT: c_uint = 0x802A;
    pub const HISTOGRAM_EXT: c_uint = 0x8024;
    pub const HISTOGRAM_FORMAT_EXT: c_uint = 0x8027;
    pub const HISTOGRAM_GREEN_SIZE_EXT: c_uint = 0x8029;
    pub const HISTOGRAM_LUMINANCE_SIZE_EXT: c_uint = 0x802C;
    pub const HISTOGRAM_RED_SIZE_EXT: c_uint = 0x8028;
    pub const HISTOGRAM_SINK_EXT: c_uint = 0x802D;
    pub const HISTOGRAM_WIDTH_EXT: c_uint = 0x8026;
    pub const HI_BIAS_NV: c_uint = 0x8714;
    pub const HI_SCALE_NV: c_uint = 0x870E;
    pub const HORIZONTAL_LINE_TO_NV: c_uint = 0x06;
    pub const HSL_COLOR_KHR: c_uint = 0x92AF;
    pub const HSL_COLOR_NV: c_uint = 0x92AF;
    pub const HSL_HUE_KHR: c_uint = 0x92AD;
    pub const HSL_HUE_NV: c_uint = 0x92AD;
    pub const HSL_LUMINOSITY_KHR: c_uint = 0x92B0;
    pub const HSL_LUMINOSITY_NV: c_uint = 0x92B0;
    pub const HSL_SATURATION_KHR: c_uint = 0x92AE;
    pub const HSL_SATURATION_NV: c_uint = 0x92AE;
    pub const IDENTITY_NV: c_uint = 0x862A;
    pub const IGNORE_BORDER_HP: c_uint = 0x8150;
    pub const IMAGE_1D: c_uint = 0x904C;
    pub const IMAGE_1D_ARRAY: c_uint = 0x9052;
    pub const IMAGE_1D_ARRAY_EXT: c_uint = 0x9052;
    pub const IMAGE_1D_EXT: c_uint = 0x904C;
    pub const IMAGE_2D: c_uint = 0x904D;
    pub const IMAGE_2D_ARRAY: c_uint = 0x9053;
    pub const IMAGE_2D_ARRAY_EXT: c_uint = 0x9053;
    pub const IMAGE_2D_EXT: c_uint = 0x904D;
    pub const IMAGE_2D_MULTISAMPLE: c_uint = 0x9055;
    pub const IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9056;
    pub const IMAGE_2D_MULTISAMPLE_ARRAY_EXT: c_uint = 0x9056;
    pub const IMAGE_2D_MULTISAMPLE_EXT: c_uint = 0x9055;
    pub const IMAGE_2D_RECT: c_uint = 0x904F;
    pub const IMAGE_2D_RECT_EXT: c_uint = 0x904F;
    pub const IMAGE_3D: c_uint = 0x904E;
    pub const IMAGE_3D_EXT: c_uint = 0x904E;
    pub const IMAGE_BINDING_ACCESS: c_uint = 0x8F3E;
    pub const IMAGE_BINDING_ACCESS_EXT: c_uint = 0x8F3E;
    pub const IMAGE_BINDING_FORMAT: c_uint = 0x906E;
    pub const IMAGE_BINDING_FORMAT_EXT: c_uint = 0x906E;
    pub const IMAGE_BINDING_LAYER: c_uint = 0x8F3D;
    pub const IMAGE_BINDING_LAYERED: c_uint = 0x8F3C;
    pub const IMAGE_BINDING_LAYERED_EXT: c_uint = 0x8F3C;
    pub const IMAGE_BINDING_LAYER_EXT: c_uint = 0x8F3D;
    pub const IMAGE_BINDING_LEVEL: c_uint = 0x8F3B;
    pub const IMAGE_BINDING_LEVEL_EXT: c_uint = 0x8F3B;
    pub const IMAGE_BINDING_NAME: c_uint = 0x8F3A;
    pub const IMAGE_BINDING_NAME_EXT: c_uint = 0x8F3A;
    pub const IMAGE_BUFFER: c_uint = 0x9051;
    pub const IMAGE_BUFFER_EXT: c_uint = 0x9051;
    pub const IMAGE_CLASS_10_10_10_2: c_uint = 0x82C3;
    pub const IMAGE_CLASS_11_11_10: c_uint = 0x82C2;
    pub const IMAGE_CLASS_1_X_16: c_uint = 0x82BE;
    pub const IMAGE_CLASS_1_X_32: c_uint = 0x82BB;
    pub const IMAGE_CLASS_1_X_8: c_uint = 0x82C1;
    pub const IMAGE_CLASS_2_X_16: c_uint = 0x82BD;
    pub const IMAGE_CLASS_2_X_32: c_uint = 0x82BA;
    pub const IMAGE_CLASS_2_X_8: c_uint = 0x82C0;
    pub const IMAGE_CLASS_4_X_16: c_uint = 0x82BC;
    pub const IMAGE_CLASS_4_X_32: c_uint = 0x82B9;
    pub const IMAGE_CLASS_4_X_8: c_uint = 0x82BF;
    pub const IMAGE_COMPATIBILITY_CLASS: c_uint = 0x82A8;
    pub const IMAGE_CUBE: c_uint = 0x9050;
    pub const IMAGE_CUBE_EXT: c_uint = 0x9050;
    pub const IMAGE_CUBE_MAP_ARRAY: c_uint = 0x9054;
    pub const IMAGE_CUBE_MAP_ARRAY_EXT: c_uint = 0x9054;
    pub const IMAGE_CUBIC_WEIGHT_HP: c_uint = 0x815E;
    pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: c_uint = 0x90C9;
    pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: c_uint = 0x90C8;
    pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: c_uint = 0x90C7;
    pub const IMAGE_MAG_FILTER_HP: c_uint = 0x815C;
    pub const IMAGE_MIN_FILTER_HP: c_uint = 0x815D;
    pub const IMAGE_PIXEL_FORMAT: c_uint = 0x82A9;
    pub const IMAGE_PIXEL_TYPE: c_uint = 0x82AA;
    pub const IMAGE_ROTATE_ANGLE_HP: c_uint = 0x8159;
    pub const IMAGE_ROTATE_ORIGIN_X_HP: c_uint = 0x815A;
    pub const IMAGE_ROTATE_ORIGIN_Y_HP: c_uint = 0x815B;
    pub const IMAGE_SCALE_X_HP: c_uint = 0x8155;
    pub const IMAGE_SCALE_Y_HP: c_uint = 0x8156;
    pub const IMAGE_TEXEL_SIZE: c_uint = 0x82A7;
    pub const IMAGE_TRANSFORM_2D_HP: c_uint = 0x8161;
    pub const IMAGE_TRANSLATE_X_HP: c_uint = 0x8157;
    pub const IMAGE_TRANSLATE_Y_HP: c_uint = 0x8158;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: c_uint = 0x8B9B;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT_OES: c_uint = 0x8B9B;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: c_uint = 0x8B9A;
    pub const IMPLEMENTATION_COLOR_READ_TYPE_OES: c_uint = 0x8B9A;
    pub const INCLUSIVE_EXT: c_uint = 0x8F10;
    pub const INCR: c_uint = 0x1E02;
    pub const INCR_WRAP: c_uint = 0x8507;
    pub const INCR_WRAP_EXT: c_uint = 0x8507;
    pub const INDEX_ARRAY_ADDRESS_NV: c_uint = 0x8F24;
    pub const INDEX_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8899;
    pub const INDEX_ARRAY_COUNT_EXT: c_uint = 0x8087;
    pub const INDEX_ARRAY_EXT: c_uint = 0x8077;
    pub const INDEX_ARRAY_LENGTH_NV: c_uint = 0x8F2E;
    pub const INDEX_ARRAY_LIST_IBM: c_uint = 103073;
    pub const INDEX_ARRAY_LIST_STRIDE_IBM: c_uint = 103083;
    pub const INDEX_ARRAY_POINTER_EXT: c_uint = 0x8091;
    pub const INDEX_ARRAY_STRIDE_EXT: c_uint = 0x8086;
    pub const INDEX_ARRAY_TYPE_EXT: c_uint = 0x8085;
    pub const INDEX_BIT_PGI: c_uint = 0x00080000;
    pub const INDEX_MATERIAL_EXT: c_uint = 0x81B8;
    pub const INDEX_MATERIAL_FACE_EXT: c_uint = 0x81BA;
    pub const INDEX_MATERIAL_PARAMETER_EXT: c_uint = 0x81B9;
    pub const INDEX_TEST_EXT: c_uint = 0x81B5;
    pub const INDEX_TEST_FUNC_EXT: c_uint = 0x81B6;
    pub const INDEX_TEST_REF_EXT: c_uint = 0x81B7;
    pub const INFO_LOG_LENGTH: c_uint = 0x8B84;
    pub const INNOCENT_CONTEXT_RESET: c_uint = 0x8254;
    pub const INNOCENT_CONTEXT_RESET_ARB: c_uint = 0x8254;
    pub const INSTRUMENT_BUFFER_POINTER_SGIX: c_uint = 0x8180;
    pub const INSTRUMENT_MEASUREMENTS_SGIX: c_uint = 0x8181;
    pub const INT: c_uint = 0x1404;
    pub const INT16_NV: c_uint = 0x8FE4;
    pub const INT16_VEC2_NV: c_uint = 0x8FE5;
    pub const INT16_VEC3_NV: c_uint = 0x8FE6;
    pub const INT16_VEC4_NV: c_uint = 0x8FE7;
    pub const INT64_ARB: c_uint = 0x140E;
    pub const INT64_NV: c_uint = 0x140E;
    pub const INT64_VEC2_ARB: c_uint = 0x8FE9;
    pub const INT64_VEC2_NV: c_uint = 0x8FE9;
    pub const INT64_VEC3_ARB: c_uint = 0x8FEA;
    pub const INT64_VEC3_NV: c_uint = 0x8FEA;
    pub const INT64_VEC4_ARB: c_uint = 0x8FEB;
    pub const INT64_VEC4_NV: c_uint = 0x8FEB;
    pub const INT8_NV: c_uint = 0x8FE0;
    pub const INT8_VEC2_NV: c_uint = 0x8FE1;
    pub const INT8_VEC3_NV: c_uint = 0x8FE2;
    pub const INT8_VEC4_NV: c_uint = 0x8FE3;
    pub const INTENSITY12_EXT: c_uint = 0x804C;
    pub const INTENSITY16F_ARB: c_uint = 0x881D;
    pub const INTENSITY16I_EXT: c_uint = 0x8D8B;
    pub const INTENSITY16UI_EXT: c_uint = 0x8D79;
    pub const INTENSITY16_EXT: c_uint = 0x804D;
    pub const INTENSITY16_SNORM: c_uint = 0x901B;
    pub const INTENSITY32F_ARB: c_uint = 0x8817;
    pub const INTENSITY32I_EXT: c_uint = 0x8D85;
    pub const INTENSITY32UI_EXT: c_uint = 0x8D73;
    pub const INTENSITY4_EXT: c_uint = 0x804A;
    pub const INTENSITY8I_EXT: c_uint = 0x8D91;
    pub const INTENSITY8UI_EXT: c_uint = 0x8D7F;
    pub const INTENSITY8_EXT: c_uint = 0x804B;
    pub const INTENSITY8_SNORM: c_uint = 0x9017;
    pub const INTENSITY_EXT: c_uint = 0x8049;
    pub const INTENSITY_FLOAT16_APPLE: c_uint = 0x881D;
    pub const INTENSITY_FLOAT16_ATI: c_uint = 0x881D;
    pub const INTENSITY_FLOAT32_APPLE: c_uint = 0x8817;
    pub const INTENSITY_FLOAT32_ATI: c_uint = 0x8817;
    pub const INTENSITY_SNORM: c_uint = 0x9013;
    pub const INTERLACE_OML: c_uint = 0x8980;
    pub const INTERLACE_READ_INGR: c_uint = 0x8568;
    pub const INTERLACE_READ_OML: c_uint = 0x8981;
    pub const INTERLACE_SGIX: c_uint = 0x8094;
    pub const INTERLEAVED_ATTRIBS: c_uint = 0x8C8C;
    pub const INTERLEAVED_ATTRIBS_EXT: c_uint = 0x8C8C;
    pub const INTERLEAVED_ATTRIBS_NV: c_uint = 0x8C8C;
    pub const INTERNALFORMAT_ALPHA_SIZE: c_uint = 0x8274;
    pub const INTERNALFORMAT_ALPHA_TYPE: c_uint = 0x827B;
    pub const INTERNALFORMAT_BLUE_SIZE: c_uint = 0x8273;
    pub const INTERNALFORMAT_BLUE_TYPE: c_uint = 0x827A;
    pub const INTERNALFORMAT_DEPTH_SIZE: c_uint = 0x8275;
    pub const INTERNALFORMAT_DEPTH_TYPE: c_uint = 0x827C;
    pub const INTERNALFORMAT_GREEN_SIZE: c_uint = 0x8272;
    pub const INTERNALFORMAT_GREEN_TYPE: c_uint = 0x8279;
    pub const INTERNALFORMAT_PREFERRED: c_uint = 0x8270;
    pub const INTERNALFORMAT_RED_SIZE: c_uint = 0x8271;
    pub const INTERNALFORMAT_RED_TYPE: c_uint = 0x8278;
    pub const INTERNALFORMAT_SHARED_SIZE: c_uint = 0x8277;
    pub const INTERNALFORMAT_STENCIL_SIZE: c_uint = 0x8276;
    pub const INTERNALFORMAT_STENCIL_TYPE: c_uint = 0x827D;
    pub const INTERNALFORMAT_SUPPORTED: c_uint = 0x826F;
    pub const INTERPOLATE_ARB: c_uint = 0x8575;
    pub const INTERPOLATE_EXT: c_uint = 0x8575;
    pub const INT_2_10_10_10_REV: c_uint = 0x8D9F;
    pub const INT_IMAGE_1D: c_uint = 0x9057;
    pub const INT_IMAGE_1D_ARRAY: c_uint = 0x905D;
    pub const INT_IMAGE_1D_ARRAY_EXT: c_uint = 0x905D;
    pub const INT_IMAGE_1D_EXT: c_uint = 0x9057;
    pub const INT_IMAGE_2D: c_uint = 0x9058;
    pub const INT_IMAGE_2D_ARRAY: c_uint = 0x905E;
    pub const INT_IMAGE_2D_ARRAY_EXT: c_uint = 0x905E;
    pub const INT_IMAGE_2D_EXT: c_uint = 0x9058;
    pub const INT_IMAGE_2D_MULTISAMPLE: c_uint = 0x9060;
    pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9061;
    pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: c_uint = 0x9061;
    pub const INT_IMAGE_2D_MULTISAMPLE_EXT: c_uint = 0x9060;
    pub const INT_IMAGE_2D_RECT: c_uint = 0x905A;
    pub const INT_IMAGE_2D_RECT_EXT: c_uint = 0x905A;
    pub const INT_IMAGE_3D: c_uint = 0x9059;
    pub const INT_IMAGE_3D_EXT: c_uint = 0x9059;
    pub const INT_IMAGE_BUFFER: c_uint = 0x905C;
    pub const INT_IMAGE_BUFFER_EXT: c_uint = 0x905C;
    pub const INT_IMAGE_CUBE: c_uint = 0x905B;
    pub const INT_IMAGE_CUBE_EXT: c_uint = 0x905B;
    pub const INT_IMAGE_CUBE_MAP_ARRAY: c_uint = 0x905F;
    pub const INT_IMAGE_CUBE_MAP_ARRAY_EXT: c_uint = 0x905F;
    pub const INT_SAMPLER_1D: c_uint = 0x8DC9;
    pub const INT_SAMPLER_1D_ARRAY: c_uint = 0x8DCE;
    pub const INT_SAMPLER_1D_ARRAY_EXT: c_uint = 0x8DCE;
    pub const INT_SAMPLER_1D_EXT: c_uint = 0x8DC9;
    pub const INT_SAMPLER_2D: c_uint = 0x8DCA;
    pub const INT_SAMPLER_2D_ARRAY: c_uint = 0x8DCF;
    pub const INT_SAMPLER_2D_ARRAY_EXT: c_uint = 0x8DCF;
    pub const INT_SAMPLER_2D_EXT: c_uint = 0x8DCA;
    pub const INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x9109;
    pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910C;
    pub const INT_SAMPLER_2D_RECT: c_uint = 0x8DCD;
    pub const INT_SAMPLER_2D_RECT_EXT: c_uint = 0x8DCD;
    pub const INT_SAMPLER_3D: c_uint = 0x8DCB;
    pub const INT_SAMPLER_3D_EXT: c_uint = 0x8DCB;
    pub const INT_SAMPLER_BUFFER: c_uint = 0x8DD0;
    pub const INT_SAMPLER_BUFFER_AMD: c_uint = 0x9002;
    pub const INT_SAMPLER_BUFFER_EXT: c_uint = 0x8DD0;
    pub const INT_SAMPLER_CUBE: c_uint = 0x8DCC;
    pub const INT_SAMPLER_CUBE_EXT: c_uint = 0x8DCC;
    pub const INT_SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900E;
    pub const INT_SAMPLER_CUBE_MAP_ARRAY_ARB: c_uint = 0x900E;
    pub const INT_SAMPLER_RENDERBUFFER_NV: c_uint = 0x8E57;
    pub const INT_VEC2: c_uint = 0x8B53;
    pub const INT_VEC2_ARB: c_uint = 0x8B53;
    pub const INT_VEC3: c_uint = 0x8B54;
    pub const INT_VEC3_ARB: c_uint = 0x8B54;
    pub const INT_VEC4: c_uint = 0x8B55;
    pub const INT_VEC4_ARB: c_uint = 0x8B55;
    pub const INVALID_ENUM: c_uint = 0x0500;
    pub const INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506;
    pub const INVALID_FRAMEBUFFER_OPERATION_EXT: c_uint = 0x0506;
    pub const INVALID_INDEX: c_uint = 0xFFFFFFFF;
    pub const INVALID_OPERATION: c_uint = 0x0502;
    pub const INVALID_VALUE: c_uint = 0x0501;
    pub const INVARIANT_DATATYPE_EXT: c_uint = 0x87EB;
    pub const INVARIANT_EXT: c_uint = 0x87C2;
    pub const INVARIANT_VALUE_EXT: c_uint = 0x87EA;
    pub const INVERSE_NV: c_uint = 0x862B;
    pub const INVERSE_TRANSPOSE_NV: c_uint = 0x862D;
    pub const INVERT: c_uint = 0x150A;
    pub const INVERTED_SCREEN_W_REND: c_uint = 0x8491;
    pub const INVERT_OVG_NV: c_uint = 0x92B4;
    pub const INVERT_RGB_NV: c_uint = 0x92A3;
    pub const IR_INSTRUMENT1_SGIX: c_uint = 0x817F;
    pub const ISOLINES: c_uint = 0x8E7A;
    pub const IS_PER_PATCH: c_uint = 0x92E7;
    pub const IS_ROW_MAJOR: c_uint = 0x9300;
    pub const ITALIC_BIT_NV: c_uint = 0x02;
    pub const IUI_N3F_V2F_EXT: c_uint = 0x81AF;
    pub const IUI_N3F_V3F_EXT: c_uint = 0x81B0;
    pub const IUI_V2F_EXT: c_uint = 0x81AD;
    pub const IUI_V3F_EXT: c_uint = 0x81AE;
    pub const KEEP: c_uint = 0x1E00;
    pub const LARGE_CCW_ARC_TO_NV: c_uint = 0x16;
    pub const LARGE_CW_ARC_TO_NV: c_uint = 0x18;
    pub const LAST_VERTEX_CONVENTION: c_uint = 0x8E4E;
    pub const LAST_VERTEX_CONVENTION_EXT: c_uint = 0x8E4E;
    pub const LAST_VIDEO_CAPTURE_STATUS_NV: c_uint = 0x9027;
    pub const LAYER_NV: c_uint = 0x8DAA;
    pub const LAYER_PROVOKING_VERTEX: c_uint = 0x825E;
    pub const LAYOUT_COLOR_ATTACHMENT_EXT: c_uint = 0x958E;
    pub const LAYOUT_DEFAULT_INTEL: c_uint = 0;
    pub const LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_EXT: c_uint = 0x9531;
    pub const LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_EXT: c_uint = 0x9530;
    pub const LAYOUT_DEPTH_STENCIL_ATTACHMENT_EXT: c_uint = 0x958F;
    pub const LAYOUT_DEPTH_STENCIL_READ_ONLY_EXT: c_uint = 0x9590;
    pub const LAYOUT_GENERAL_EXT: c_uint = 0x958D;
    pub const LAYOUT_LINEAR_CPU_CACHED_INTEL: c_uint = 2;
    pub const LAYOUT_LINEAR_INTEL: c_uint = 1;
    pub const LAYOUT_SHADER_READ_ONLY_EXT: c_uint = 0x9591;
    pub const LAYOUT_TRANSFER_DST_EXT: c_uint = 0x9593;
    pub const LAYOUT_TRANSFER_SRC_EXT: c_uint = 0x9592;
    pub const LEFT: c_uint = 0x0406;
    pub const LEQUAL: c_uint = 0x0203;
    pub const LERP_ATI: c_uint = 0x8969;
    pub const LESS: c_uint = 0x0201;
    pub const LGPU_SEPARATE_STORAGE_BIT_NVX: c_uint = 0x0800;
    pub const LIGHTEN_KHR: c_uint = 0x9298;
    pub const LIGHTEN_NV: c_uint = 0x9298;
    pub const LIGHT_ENV_MODE_SGIX: c_uint = 0x8407;
    pub const LIGHT_MODEL_COLOR_CONTROL_EXT: c_uint = 0x81F8;
    pub const LIGHT_MODEL_SPECULAR_VECTOR_APPLE: c_uint = 0x85B0;
    pub const LINE: c_uint = 0x1B01;
    pub const LINEAR: c_uint = 0x2601;
    pub const LINEARBURN_NV: c_uint = 0x92A5;
    pub const LINEARDODGE_NV: c_uint = 0x92A4;
    pub const LINEARLIGHT_NV: c_uint = 0x92A7;
    pub const LINEAR_CLIPMAP_LINEAR_SGIX: c_uint = 0x8170;
    pub const LINEAR_CLIPMAP_NEAREST_SGIX: c_uint = 0x844F;
    pub const LINEAR_DETAIL_ALPHA_SGIS: c_uint = 0x8098;
    pub const LINEAR_DETAIL_COLOR_SGIS: c_uint = 0x8099;
    pub const LINEAR_DETAIL_SGIS: c_uint = 0x8097;
    pub const LINEAR_MIPMAP_LINEAR: c_uint = 0x2703;
    pub const LINEAR_MIPMAP_NEAREST: c_uint = 0x2701;
    pub const LINEAR_SHARPEN_ALPHA_SGIS: c_uint = 0x80AE;
    pub const LINEAR_SHARPEN_COLOR_SGIS: c_uint = 0x80AF;
    pub const LINEAR_SHARPEN_SGIS: c_uint = 0x80AD;
    pub const LINEAR_TILING_EXT: c_uint = 0x9585;
    pub const LINES: c_uint = 0x0001;
    pub const LINES_ADJACENCY: c_uint = 0x000A;
    pub const LINES_ADJACENCY_ARB: c_uint = 0x000A;
    pub const LINES_ADJACENCY_EXT: c_uint = 0x000A;
    pub const LINE_LOOP: c_uint = 0x0002;
    pub const LINE_SMOOTH: c_uint = 0x0B20;
    pub const LINE_SMOOTH_HINT: c_uint = 0x0C52;
    pub const LINE_STRIP: c_uint = 0x0003;
    pub const LINE_STRIP_ADJACENCY: c_uint = 0x000B;
    pub const LINE_STRIP_ADJACENCY_ARB: c_uint = 0x000B;
    pub const LINE_STRIP_ADJACENCY_EXT: c_uint = 0x000B;
    pub const LINE_TO_NV: c_uint = 0x04;
    pub const LINE_WIDTH: c_uint = 0x0B21;
    pub const LINE_WIDTH_COMMAND_NV: c_uint = 0x000D;
    pub const LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
    pub const LINE_WIDTH_RANGE: c_uint = 0x0B22;
    pub const LINK_STATUS: c_uint = 0x8B82;
    pub const LIST_PRIORITY_SGIX: c_uint = 0x8182;
    pub const LOCAL_CONSTANT_DATATYPE_EXT: c_uint = 0x87ED;
    pub const LOCAL_CONSTANT_EXT: c_uint = 0x87C3;
    pub const LOCAL_CONSTANT_VALUE_EXT: c_uint = 0x87EC;
    pub const LOCAL_EXT: c_uint = 0x87C4;
    pub const LOCATION: c_uint = 0x930E;
    pub const LOCATION_COMPONENT: c_uint = 0x934A;
    pub const LOCATION_INDEX: c_uint = 0x930F;
    pub const LOGIC_OP_MODE: c_uint = 0x0BF0;
    pub const LOSE_CONTEXT_ON_RESET: c_uint = 0x8252;
    pub const LOSE_CONTEXT_ON_RESET_ARB: c_uint = 0x8252;
    pub const LOWER_LEFT: c_uint = 0x8CA1;
    pub const LOW_FLOAT: c_uint = 0x8DF0;
    pub const LOW_INT: c_uint = 0x8DF3;
    pub const LO_BIAS_NV: c_uint = 0x8715;
    pub const LO_SCALE_NV: c_uint = 0x870F;
    pub const LUID_SIZE_EXT: c_uint = 8;
    pub const LUMINANCE12_ALPHA12_EXT: c_uint = 0x8047;
    pub const LUMINANCE12_ALPHA4_EXT: c_uint = 0x8046;
    pub const LUMINANCE12_EXT: c_uint = 0x8041;
    pub const LUMINANCE16F_ARB: c_uint = 0x881E;
    pub const LUMINANCE16F_EXT: c_uint = 0x881E;
    pub const LUMINANCE16I_EXT: c_uint = 0x8D8C;
    pub const LUMINANCE16UI_EXT: c_uint = 0x8D7A;
    pub const LUMINANCE16_ALPHA16_EXT: c_uint = 0x8048;
    pub const LUMINANCE16_ALPHA16_SNORM: c_uint = 0x901A;
    pub const LUMINANCE16_EXT: c_uint = 0x8042;
    pub const LUMINANCE16_SNORM: c_uint = 0x9019;
    pub const LUMINANCE32F_ARB: c_uint = 0x8818;
    pub const LUMINANCE32F_EXT: c_uint = 0x8818;
    pub const LUMINANCE32I_EXT: c_uint = 0x8D86;
    pub const LUMINANCE32UI_EXT: c_uint = 0x8D74;
    pub const LUMINANCE4_ALPHA4_EXT: c_uint = 0x8043;
    pub const LUMINANCE4_EXT: c_uint = 0x803F;
    pub const LUMINANCE6_ALPHA2_EXT: c_uint = 0x8044;
    pub const LUMINANCE8I_EXT: c_uint = 0x8D92;
    pub const LUMINANCE8UI_EXT: c_uint = 0x8D80;
    pub const LUMINANCE8_ALPHA8_EXT: c_uint = 0x8045;
    pub const LUMINANCE8_ALPHA8_SNORM: c_uint = 0x9016;
    pub const LUMINANCE8_EXT: c_uint = 0x8040;
    pub const LUMINANCE8_SNORM: c_uint = 0x9015;
    pub const LUMINANCE_ALPHA16F_ARB: c_uint = 0x881F;
    pub const LUMINANCE_ALPHA16F_EXT: c_uint = 0x881F;
    pub const LUMINANCE_ALPHA16I_EXT: c_uint = 0x8D8D;
    pub const LUMINANCE_ALPHA16UI_EXT: c_uint = 0x8D7B;
    pub const LUMINANCE_ALPHA32F_ARB: c_uint = 0x8819;
    pub const LUMINANCE_ALPHA32F_EXT: c_uint = 0x8819;
    pub const LUMINANCE_ALPHA32I_EXT: c_uint = 0x8D87;
    pub const LUMINANCE_ALPHA32UI_EXT: c_uint = 0x8D75;
    pub const LUMINANCE_ALPHA8I_EXT: c_uint = 0x8D93;
    pub const LUMINANCE_ALPHA8UI_EXT: c_uint = 0x8D81;
    pub const LUMINANCE_ALPHA_FLOAT16_APPLE: c_uint = 0x881F;
    pub const LUMINANCE_ALPHA_FLOAT16_ATI: c_uint = 0x881F;
    pub const LUMINANCE_ALPHA_FLOAT32_APPLE: c_uint = 0x8819;
    pub const LUMINANCE_ALPHA_FLOAT32_ATI: c_uint = 0x8819;
    pub const LUMINANCE_ALPHA_INTEGER_EXT: c_uint = 0x8D9D;
    pub const LUMINANCE_ALPHA_SNORM: c_uint = 0x9012;
    pub const LUMINANCE_FLOAT16_APPLE: c_uint = 0x881E;
    pub const LUMINANCE_FLOAT16_ATI: c_uint = 0x881E;
    pub const LUMINANCE_FLOAT32_APPLE: c_uint = 0x8818;
    pub const LUMINANCE_FLOAT32_ATI: c_uint = 0x8818;
    pub const LUMINANCE_INTEGER_EXT: c_uint = 0x8D9C;
    pub const LUMINANCE_SNORM: c_uint = 0x9011;
    pub const MAD_ATI: c_uint = 0x8968;
    pub const MAGNITUDE_BIAS_NV: c_uint = 0x8718;
    pub const MAGNITUDE_SCALE_NV: c_uint = 0x8712;
    pub const MAJOR_VERSION: c_uint = 0x821B;
    pub const MANUAL_GENERATE_MIPMAP: c_uint = 0x8294;
    pub const MAP1_BINORMAL_EXT: c_uint = 0x8446;
    pub const MAP1_TANGENT_EXT: c_uint = 0x8444;
    pub const MAP1_VERTEX_ATTRIB0_4_NV: c_uint = 0x8660;
    pub const MAP1_VERTEX_ATTRIB10_4_NV: c_uint = 0x866A;
    pub const MAP1_VERTEX_ATTRIB11_4_NV: c_uint = 0x866B;
    pub const MAP1_VERTEX_ATTRIB12_4_NV: c_uint = 0x866C;
    pub const MAP1_VERTEX_ATTRIB13_4_NV: c_uint = 0x866D;
    pub const MAP1_VERTEX_ATTRIB14_4_NV: c_uint = 0x866E;
    pub const MAP1_VERTEX_ATTRIB15_4_NV: c_uint = 0x866F;
    pub const MAP1_VERTEX_ATTRIB1_4_NV: c_uint = 0x8661;
    pub const MAP1_VERTEX_ATTRIB2_4_NV: c_uint = 0x8662;
    pub const MAP1_VERTEX_ATTRIB3_4_NV: c_uint = 0x8663;
    pub const MAP1_VERTEX_ATTRIB4_4_NV: c_uint = 0x8664;
    pub const MAP1_VERTEX_ATTRIB5_4_NV: c_uint = 0x8665;
    pub const MAP1_VERTEX_ATTRIB6_4_NV: c_uint = 0x8666;
    pub const MAP1_VERTEX_ATTRIB7_4_NV: c_uint = 0x8667;
    pub const MAP1_VERTEX_ATTRIB8_4_NV: c_uint = 0x8668;
    pub const MAP1_VERTEX_ATTRIB9_4_NV: c_uint = 0x8669;
    pub const MAP2_BINORMAL_EXT: c_uint = 0x8447;
    pub const MAP2_TANGENT_EXT: c_uint = 0x8445;
    pub const MAP2_VERTEX_ATTRIB0_4_NV: c_uint = 0x8670;
    pub const MAP2_VERTEX_ATTRIB10_4_NV: c_uint = 0x867A;
    pub const MAP2_VERTEX_ATTRIB11_4_NV: c_uint = 0x867B;
    pub const MAP2_VERTEX_ATTRIB12_4_NV: c_uint = 0x867C;
    pub const MAP2_VERTEX_ATTRIB13_4_NV: c_uint = 0x867D;
    pub const MAP2_VERTEX_ATTRIB14_4_NV: c_uint = 0x867E;
    pub const MAP2_VERTEX_ATTRIB15_4_NV: c_uint = 0x867F;
    pub const MAP2_VERTEX_ATTRIB1_4_NV: c_uint = 0x8671;
    pub const MAP2_VERTEX_ATTRIB2_4_NV: c_uint = 0x8672;
    pub const MAP2_VERTEX_ATTRIB3_4_NV: c_uint = 0x8673;
    pub const MAP2_VERTEX_ATTRIB4_4_NV: c_uint = 0x8674;
    pub const MAP2_VERTEX_ATTRIB5_4_NV: c_uint = 0x8675;
    pub const MAP2_VERTEX_ATTRIB6_4_NV: c_uint = 0x8676;
    pub const MAP2_VERTEX_ATTRIB7_4_NV: c_uint = 0x8677;
    pub const MAP2_VERTEX_ATTRIB8_4_NV: c_uint = 0x8678;
    pub const MAP2_VERTEX_ATTRIB9_4_NV: c_uint = 0x8679;
    pub const MAP_ATTRIB_U_ORDER_NV: c_uint = 0x86C3;
    pub const MAP_ATTRIB_V_ORDER_NV: c_uint = 0x86C4;
    pub const MAP_COHERENT_BIT: c_uint = 0x0080;
    pub const MAP_FLUSH_EXPLICIT_BIT: c_uint = 0x0010;
    pub const MAP_INVALIDATE_BUFFER_BIT: c_uint = 0x0008;
    pub const MAP_INVALIDATE_RANGE_BIT: c_uint = 0x0004;
    pub const MAP_PERSISTENT_BIT: c_uint = 0x0040;
    pub const MAP_READ_BIT: c_uint = 0x0001;
    pub const MAP_TESSELLATION_NV: c_uint = 0x86C2;
    pub const MAP_UNSYNCHRONIZED_BIT: c_uint = 0x0020;
    pub const MAP_WRITE_BIT: c_uint = 0x0002;
    pub const MATERIAL_SIDE_HINT_PGI: c_uint = 0x1A22C;
    pub const MATRIX0_ARB: c_uint = 0x88C0;
    pub const MATRIX0_NV: c_uint = 0x8630;
    pub const MATRIX10_ARB: c_uint = 0x88CA;
    pub const MATRIX11_ARB: c_uint = 0x88CB;
    pub const MATRIX12_ARB: c_uint = 0x88CC;
    pub const MATRIX13_ARB: c_uint = 0x88CD;
    pub const MATRIX14_ARB: c_uint = 0x88CE;
    pub const MATRIX15_ARB: c_uint = 0x88CF;
    pub const MATRIX16_ARB: c_uint = 0x88D0;
    pub const MATRIX17_ARB: c_uint = 0x88D1;
    pub const MATRIX18_ARB: c_uint = 0x88D2;
    pub const MATRIX19_ARB: c_uint = 0x88D3;
    pub const MATRIX1_ARB: c_uint = 0x88C1;
    pub const MATRIX1_NV: c_uint = 0x8631;
    pub const MATRIX20_ARB: c_uint = 0x88D4;
    pub const MATRIX21_ARB: c_uint = 0x88D5;
    pub const MATRIX22_ARB: c_uint = 0x88D6;
    pub const MATRIX23_ARB: c_uint = 0x88D7;
    pub const MATRIX24_ARB: c_uint = 0x88D8;
    pub const MATRIX25_ARB: c_uint = 0x88D9;
    pub const MATRIX26_ARB: c_uint = 0x88DA;
    pub const MATRIX27_ARB: c_uint = 0x88DB;
    pub const MATRIX28_ARB: c_uint = 0x88DC;
    pub const MATRIX29_ARB: c_uint = 0x88DD;
    pub const MATRIX2_ARB: c_uint = 0x88C2;
    pub const MATRIX2_NV: c_uint = 0x8632;
    pub const MATRIX30_ARB: c_uint = 0x88DE;
    pub const MATRIX31_ARB: c_uint = 0x88DF;
    pub const MATRIX3_ARB: c_uint = 0x88C3;
    pub const MATRIX3_NV: c_uint = 0x8633;
    pub const MATRIX4_ARB: c_uint = 0x88C4;
    pub const MATRIX4_NV: c_uint = 0x8634;
    pub const MATRIX5_ARB: c_uint = 0x88C5;
    pub const MATRIX5_NV: c_uint = 0x8635;
    pub const MATRIX6_ARB: c_uint = 0x88C6;
    pub const MATRIX6_NV: c_uint = 0x8636;
    pub const MATRIX7_ARB: c_uint = 0x88C7;
    pub const MATRIX7_NV: c_uint = 0x8637;
    pub const MATRIX8_ARB: c_uint = 0x88C8;
    pub const MATRIX9_ARB: c_uint = 0x88C9;
    pub const MATRIX_EXT: c_uint = 0x87C0;
    pub const MATRIX_INDEX_ARRAY_ARB: c_uint = 0x8844;
    pub const MATRIX_INDEX_ARRAY_POINTER_ARB: c_uint = 0x8849;
    pub const MATRIX_INDEX_ARRAY_SIZE_ARB: c_uint = 0x8846;
    pub const MATRIX_INDEX_ARRAY_STRIDE_ARB: c_uint = 0x8848;
    pub const MATRIX_INDEX_ARRAY_TYPE_ARB: c_uint = 0x8847;
    pub const MATRIX_PALETTE_ARB: c_uint = 0x8840;
    pub const MATRIX_STRIDE: c_uint = 0x92FF;
    pub const MAT_AMBIENT_AND_DIFFUSE_BIT_PGI: c_uint = 0x00200000;
    pub const MAT_AMBIENT_BIT_PGI: c_uint = 0x00100000;
    pub const MAT_COLOR_INDEXES_BIT_PGI: c_uint = 0x01000000;
    pub const MAT_DIFFUSE_BIT_PGI: c_uint = 0x00400000;
    pub const MAT_EMISSION_BIT_PGI: c_uint = 0x00800000;
    pub const MAT_SHININESS_BIT_PGI: c_uint = 0x02000000;
    pub const MAT_SPECULAR_BIT_PGI: c_uint = 0x04000000;
    pub const MAX: c_uint = 0x8008;
    pub const MAX_3D_TEXTURE_SIZE: c_uint = 0x8073;
    pub const MAX_3D_TEXTURE_SIZE_EXT: c_uint = 0x8073;
    pub const MAX_4D_TEXTURE_SIZE_SGIS: c_uint = 0x8138;
    pub const MAX_ACTIVE_LIGHTS_SGIX: c_uint = 0x8405;
    pub const MAX_ARRAY_TEXTURE_LAYERS: c_uint = 0x88FF;
    pub const MAX_ARRAY_TEXTURE_LAYERS_EXT: c_uint = 0x88FF;
    pub const MAX_ASYNC_DRAW_PIXELS_SGIX: c_uint = 0x8360;
    pub const MAX_ASYNC_HISTOGRAM_SGIX: c_uint = 0x832D;
    pub const MAX_ASYNC_READ_PIXELS_SGIX: c_uint = 0x8361;
    pub const MAX_ASYNC_TEX_IMAGE_SGIX: c_uint = 0x835F;
    pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: c_uint = 0x92DC;
    pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: c_uint = 0x92D8;
    pub const MAX_BINDABLE_UNIFORM_SIZE_EXT: c_uint = 0x8DED;
    pub const MAX_CLIPMAP_DEPTH_SGIX: c_uint = 0x8177;
    pub const MAX_CLIPMAP_VIRTUAL_DEPTH_SGIX: c_uint = 0x8178;
    pub const MAX_CLIP_DISTANCES: c_uint = 0x0D32;
    pub const MAX_COARSE_FRAGMENT_SAMPLES_NV: c_uint = 0x955F;
    pub const MAX_COLOR_ATTACHMENTS: c_uint = 0x8CDF;
    pub const MAX_COLOR_ATTACHMENTS_EXT: c_uint = 0x8CDF;
    pub const MAX_COLOR_FRAMEBUFFER_SAMPLES_AMD: c_uint = 0x91B3;
    pub const MAX_COLOR_FRAMEBUFFER_STORAGE_SAMPLES_AMD: c_uint = 0x91B4;
    pub const MAX_COLOR_MATRIX_STACK_DEPTH_SGI: c_uint = 0x80B3;
    pub const MAX_COLOR_TEXTURE_SAMPLES: c_uint = 0x910E;
    pub const MAX_COMBINED_ATOMIC_COUNTERS: c_uint = 0x92D7;
    pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D1;
    pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: c_uint = 0x82FA;
    pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: c_uint = 0x8266;
    pub const MAX_COMBINED_DIMENSIONS: c_uint = 0x8282;
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8A33;
    pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8A32;
    pub const MAX_COMBINED_IMAGE_UNIFORMS: c_uint = 0x90CF;
    pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: c_uint = 0x8F39;
    pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS_EXT: c_uint = 0x8F39;
    pub const MAX_COMBINED_MESH_UNIFORM_COMPONENTS_NV: c_uint = 0x8E67;
    pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: c_uint = 0x8F39;
    pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: c_uint = 0x90DC;
    pub const MAX_COMBINED_TASK_UNIFORM_COMPONENTS_NV: c_uint = 0x8E6F;
    pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: c_uint = 0x8E1E;
    pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: c_uint = 0x8E1F;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8B4D;
    pub const MAX_COMBINED_UNIFORM_BLOCKS: c_uint = 0x8A2E;
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8A31;
    pub const MAX_COMPUTE_ATOMIC_COUNTERS: c_uint = 0x8265;
    pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: c_uint = 0x8264;
    pub const MAX_COMPUTE_FIXED_GROUP_INVOCATIONS_ARB: c_uint = 0x90EB;
    pub const MAX_COMPUTE_FIXED_GROUP_SIZE_ARB: c_uint = 0x91BF;
    pub const MAX_COMPUTE_IMAGE_UNIFORMS: c_uint = 0x91BD;
    pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: c_uint = 0x90DB;
    pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: c_uint = 0x8262;
    pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: c_uint = 0x91BC;
    pub const MAX_COMPUTE_UNIFORM_BLOCKS: c_uint = 0x91BB;
    pub const MAX_COMPUTE_UNIFORM_COMPONENTS: c_uint = 0x8263;
    pub const MAX_COMPUTE_VARIABLE_GROUP_INVOCATIONS_ARB: c_uint = 0x9344;
    pub const MAX_COMPUTE_VARIABLE_GROUP_SIZE_ARB: c_uint = 0x9345;
    pub const MAX_COMPUTE_WORK_GROUP_COUNT: c_uint = 0x91BE;
    pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: c_uint = 0x90EB;
    pub const MAX_COMPUTE_WORK_GROUP_SIZE: c_uint = 0x91BF;
    pub const MAX_CONVOLUTION_HEIGHT_EXT: c_uint = 0x801B;
    pub const MAX_CONVOLUTION_WIDTH_EXT: c_uint = 0x801A;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: c_uint = 0x851C;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE_ARB: c_uint = 0x851C;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE_EXT: c_uint = 0x851C;
    pub const MAX_CULL_DISTANCES: c_uint = 0x82F9;
    pub const MAX_DEBUG_GROUP_STACK_DEPTH: c_uint = 0x826C;
    pub const MAX_DEBUG_LOGGED_MESSAGES: c_uint = 0x9144;
    pub const MAX_DEBUG_LOGGED_MESSAGES_AMD: c_uint = 0x9144;
    pub const MAX_DEBUG_LOGGED_MESSAGES_ARB: c_uint = 0x9144;
    pub const MAX_DEBUG_MESSAGE_LENGTH: c_uint = 0x9143;
    pub const MAX_DEBUG_MESSAGE_LENGTH_AMD: c_uint = 0x9143;
    pub const MAX_DEBUG_MESSAGE_LENGTH_ARB: c_uint = 0x9143;
    pub const MAX_DEEP_3D_TEXTURE_DEPTH_NV: c_uint = 0x90D1;
    pub const MAX_DEEP_3D_TEXTURE_WIDTH_HEIGHT_NV: c_uint = 0x90D0;
    pub const MAX_DEFORMATION_ORDER_SGIX: c_uint = 0x8197;
    pub const MAX_DEPTH: c_uint = 0x8280;
    pub const MAX_DEPTH_STENCIL_FRAMEBUFFER_SAMPLES_AMD: c_uint = 0x91B5;
    pub const MAX_DEPTH_TEXTURE_SAMPLES: c_uint = 0x910F;
    pub const MAX_DETACHED_BUFFERS_NV: c_uint = 0x95AD;
    pub const MAX_DETACHED_TEXTURES_NV: c_uint = 0x95AC;
    pub const MAX_DRAW_BUFFERS: c_uint = 0x8824;
    pub const MAX_DRAW_BUFFERS_ARB: c_uint = 0x8824;
    pub const MAX_DRAW_BUFFERS_ATI: c_uint = 0x8824;
    pub const MAX_DRAW_MESH_TASKS_COUNT_NV: c_uint = 0x953D;
    pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: c_uint = 0x88FC;
    pub const MAX_ELEMENTS_INDICES: c_uint = 0x80E9;
    pub const MAX_ELEMENTS_INDICES_EXT: c_uint = 0x80E9;
    pub const MAX_ELEMENTS_VERTICES: c_uint = 0x80E8;
    pub const MAX_ELEMENTS_VERTICES_EXT: c_uint = 0x80E8;
    pub const MAX_ELEMENT_INDEX: c_uint = 0x8D6B;
    pub const MAX_EXT: c_uint = 0x8008;
    pub const MAX_FOG_FUNC_POINTS_SGIS: c_uint = 0x812C;
    pub const MAX_FRAGMENT_ATOMIC_COUNTERS: c_uint = 0x92D6;
    pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D0;
    pub const MAX_FRAGMENT_BINDABLE_UNIFORMS_EXT: c_uint = 0x8DE3;
    pub const MAX_FRAGMENT_IMAGE_UNIFORMS: c_uint = 0x90CE;
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: c_uint = 0x9125;
    pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: c_uint = 0x8E5C;
    pub const MAX_FRAGMENT_INTERPOLATION_OFFSET_NV: c_uint = 0x8E5C;
    pub const MAX_FRAGMENT_LIGHTS_SGIX: c_uint = 0x8404;
    pub const MAX_FRAGMENT_PROGRAM_LOCAL_PARAMETERS_NV: c_uint = 0x8868;
    pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: c_uint = 0x90DA;
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: c_uint = 0x8A2D;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8B49;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS_ARB: c_uint = 0x8B49;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: c_uint = 0x8DFD;
    pub const MAX_FRAMEBUFFER_HEIGHT: c_uint = 0x9316;
    pub const MAX_FRAMEBUFFER_LAYERS: c_uint = 0x9317;
    pub const MAX_FRAMEBUFFER_SAMPLES: c_uint = 0x9318;
    pub const MAX_FRAMEBUFFER_WIDTH: c_uint = 0x9315;
    pub const MAX_FRAMEZOOM_FACTOR_SGIX: c_uint = 0x818D;
    pub const MAX_GENERAL_COMBINERS_NV: c_uint = 0x854D;
    pub const MAX_GEOMETRY_ATOMIC_COUNTERS: c_uint = 0x92D5;
    pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CF;
    pub const MAX_GEOMETRY_BINDABLE_UNIFORMS_EXT: c_uint = 0x8DE4;
    pub const MAX_GEOMETRY_IMAGE_UNIFORMS: c_uint = 0x90CD;
    pub const MAX_GEOMETRY_INPUT_COMPONENTS: c_uint = 0x9123;
    pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: c_uint = 0x9124;
    pub const MAX_GEOMETRY_OUTPUT_VERTICES: c_uint = 0x8DE0;
    pub const MAX_GEOMETRY_OUTPUT_VERTICES_ARB: c_uint = 0x8DE0;
    pub const MAX_GEOMETRY_OUTPUT_VERTICES_EXT: c_uint = 0x8DE0;
    pub const MAX_GEOMETRY_PROGRAM_INVOCATIONS_NV: c_uint = 0x8E5A;
    pub const MAX_GEOMETRY_SHADER_INVOCATIONS: c_uint = 0x8E5A;
    pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: c_uint = 0x90D7;
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: c_uint = 0x8C29;
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8C29;
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_EXT: c_uint = 0x8C29;
    pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: c_uint = 0x8DE1;
    pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_ARB: c_uint = 0x8DE1;
    pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_EXT: c_uint = 0x8DE1;
    pub const MAX_GEOMETRY_UNIFORM_BLOCKS: c_uint = 0x8A2C;
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8DDF;
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS_ARB: c_uint = 0x8DDF;
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS_EXT: c_uint = 0x8DDF;
    pub const MAX_GEOMETRY_VARYING_COMPONENTS_ARB: c_uint = 0x8DDD;
    pub const MAX_GEOMETRY_VARYING_COMPONENTS_EXT: c_uint = 0x8DDD;
    pub const MAX_HEIGHT: c_uint = 0x827F;
    pub const MAX_IMAGE_SAMPLES: c_uint = 0x906D;
    pub const MAX_IMAGE_SAMPLES_EXT: c_uint = 0x906D;
    pub const MAX_IMAGE_UNITS: c_uint = 0x8F38;
    pub const MAX_IMAGE_UNITS_EXT: c_uint = 0x8F38;
    pub const MAX_INTEGER_SAMPLES: c_uint = 0x9110;
    pub const MAX_LABEL_LENGTH: c_uint = 0x82E8;
    pub const MAX_LAYERS: c_uint = 0x8281;
    pub const MAX_LGPU_GPUS_NVX: c_uint = 0x92BA;
    pub const MAX_MAP_TESSELLATION_NV: c_uint = 0x86D6;
    pub const MAX_MATRIX_PALETTE_STACK_DEPTH_ARB: c_uint = 0x8841;
    pub const MAX_MESH_ATOMIC_COUNTERS_NV: c_uint = 0x8E65;
    pub const MAX_MESH_ATOMIC_COUNTER_BUFFERS_NV: c_uint = 0x8E64;
    pub const MAX_MESH_IMAGE_UNIFORMS_NV: c_uint = 0x8E62;
    pub const MAX_MESH_OUTPUT_PRIMITIVES_NV: c_uint = 0x9539;
    pub const MAX_MESH_OUTPUT_VERTICES_NV: c_uint = 0x9538;
    pub const MAX_MESH_SHADER_STORAGE_BLOCKS_NV: c_uint = 0x8E66;
    pub const MAX_MESH_TEXTURE_IMAGE_UNITS_NV: c_uint = 0x8E61;
    pub const MAX_MESH_TOTAL_MEMORY_SIZE_NV: c_uint = 0x9536;
    pub const MAX_MESH_UNIFORM_BLOCKS_NV: c_uint = 0x8E60;
    pub const MAX_MESH_UNIFORM_COMPONENTS_NV: c_uint = 0x8E63;
    pub const MAX_MESH_VIEWS_NV: c_uint = 0x9557;
    pub const MAX_MESH_WORK_GROUP_INVOCATIONS_NV: c_uint = 0x95A2;
    pub const MAX_MESH_WORK_GROUP_SIZE_NV: c_uint = 0x953B;
    pub const MAX_MULTISAMPLE_COVERAGE_MODES_NV: c_uint = 0x8E11;
    pub const MAX_NAME_LENGTH: c_uint = 0x92F6;
    pub const MAX_NUM_ACTIVE_VARIABLES: c_uint = 0x92F7;
    pub const MAX_NUM_COMPATIBLE_SUBROUTINES: c_uint = 0x92F8;
    pub const MAX_OPTIMIZED_VERTEX_SHADER_INSTRUCTIONS_EXT: c_uint = 0x87CA;
    pub const MAX_OPTIMIZED_VERTEX_SHADER_INVARIANTS_EXT: c_uint = 0x87CD;
    pub const MAX_OPTIMIZED_VERTEX_SHADER_LOCALS_EXT: c_uint = 0x87CE;
    pub const MAX_OPTIMIZED_VERTEX_SHADER_LOCAL_CONSTANTS_EXT: c_uint = 0x87CC;
    pub const MAX_OPTIMIZED_VERTEX_SHADER_VARIANTS_EXT: c_uint = 0x87CB;
    pub const MAX_PALETTE_MATRICES_ARB: c_uint = 0x8842;
    pub const MAX_PATCH_VERTICES: c_uint = 0x8E7D;
    pub const MAX_PIXEL_TRANSFORM_2D_STACK_DEPTH_EXT: c_uint = 0x8337;
    pub const MAX_PN_TRIANGLES_TESSELATION_LEVEL_ATI: c_uint = 0x87F1;
    pub const MAX_PROGRAM_ADDRESS_REGISTERS_ARB: c_uint = 0x88B1;
    pub const MAX_PROGRAM_ALU_INSTRUCTIONS_ARB: c_uint = 0x880B;
    pub const MAX_PROGRAM_ATTRIBS_ARB: c_uint = 0x88AD;
    pub const MAX_PROGRAM_ATTRIB_COMPONENTS_NV: c_uint = 0x8908;
    pub const MAX_PROGRAM_CALL_DEPTH_NV: c_uint = 0x88F5;
    pub const MAX_PROGRAM_ENV_PARAMETERS_ARB: c_uint = 0x88B5;
    pub const MAX_PROGRAM_EXEC_INSTRUCTIONS_NV: c_uint = 0x88F4;
    pub const MAX_PROGRAM_GENERIC_ATTRIBS_NV: c_uint = 0x8DA5;
    pub const MAX_PROGRAM_GENERIC_RESULTS_NV: c_uint = 0x8DA6;
    pub const MAX_PROGRAM_IF_DEPTH_NV: c_uint = 0x88F6;
    pub const MAX_PROGRAM_INSTRUCTIONS_ARB: c_uint = 0x88A1;
    pub const MAX_PROGRAM_LOCAL_PARAMETERS_ARB: c_uint = 0x88B4;
    pub const MAX_PROGRAM_LOOP_COUNT_NV: c_uint = 0x88F8;
    pub const MAX_PROGRAM_LOOP_DEPTH_NV: c_uint = 0x88F7;
    pub const MAX_PROGRAM_MATRICES_ARB: c_uint = 0x862F;
    pub const MAX_PROGRAM_MATRIX_STACK_DEPTH_ARB: c_uint = 0x862E;
    pub const MAX_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: c_uint = 0x88B3;
    pub const MAX_PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB: c_uint = 0x880E;
    pub const MAX_PROGRAM_NATIVE_ATTRIBS_ARB: c_uint = 0x88AF;
    pub const MAX_PROGRAM_NATIVE_INSTRUCTIONS_ARB: c_uint = 0x88A3;
    pub const MAX_PROGRAM_NATIVE_PARAMETERS_ARB: c_uint = 0x88AB;
    pub const MAX_PROGRAM_NATIVE_TEMPORARIES_ARB: c_uint = 0x88A7;
    pub const MAX_PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB: c_uint = 0x8810;
    pub const MAX_PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB: c_uint = 0x880F;
    pub const MAX_PROGRAM_OUTPUT_VERTICES_NV: c_uint = 0x8C27;
    pub const MAX_PROGRAM_PARAMETERS_ARB: c_uint = 0x88A9;
    pub const MAX_PROGRAM_PARAMETER_BUFFER_BINDINGS_NV: c_uint = 0x8DA0;
    pub const MAX_PROGRAM_PARAMETER_BUFFER_SIZE_NV: c_uint = 0x8DA1;
    pub const MAX_PROGRAM_PATCH_ATTRIBS_NV: c_uint = 0x86D8;
    pub const MAX_PROGRAM_RESULT_COMPONENTS_NV: c_uint = 0x8909;
    pub const MAX_PROGRAM_SUBROUTINE_NUM_NV: c_uint = 0x8F45;
    pub const MAX_PROGRAM_SUBROUTINE_PARAMETERS_NV: c_uint = 0x8F44;
    pub const MAX_PROGRAM_TEMPORARIES_ARB: c_uint = 0x88A5;
    pub const MAX_PROGRAM_TEXEL_OFFSET: c_uint = 0x8905;
    pub const MAX_PROGRAM_TEXEL_OFFSET_EXT: c_uint = 0x8905;
    pub const MAX_PROGRAM_TEXEL_OFFSET_NV: c_uint = 0x8905;
    pub const MAX_PROGRAM_TEXTURE_GATHER_COMPONENTS_ARB: c_uint = 0x8F9F;
    pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: c_uint = 0x8E5F;
    pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET_ARB: c_uint = 0x8E5F;
    pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET_NV: c_uint = 0x8E5F;
    pub const MAX_PROGRAM_TEX_INDIRECTIONS_ARB: c_uint = 0x880D;
    pub const MAX_PROGRAM_TEX_INSTRUCTIONS_ARB: c_uint = 0x880C;
    pub const MAX_PROGRAM_TOTAL_OUTPUT_COMPONENTS_NV: c_uint = 0x8C28;
    pub const MAX_RASTER_SAMPLES_EXT: c_uint = 0x9329;
    pub const MAX_RATIONAL_EVAL_ORDER_NV: c_uint = 0x86D7;
    pub const MAX_RECTANGLE_TEXTURE_SIZE: c_uint = 0x84F8;
    pub const MAX_RECTANGLE_TEXTURE_SIZE_ARB: c_uint = 0x84F8;
    pub const MAX_RECTANGLE_TEXTURE_SIZE_NV: c_uint = 0x84F8;
    pub const MAX_RENDERBUFFER_SIZE: c_uint = 0x84E8;
    pub const MAX_RENDERBUFFER_SIZE_EXT: c_uint = 0x84E8;
    pub const MAX_SAMPLES: c_uint = 0x8D57;
    pub const MAX_SAMPLES_EXT: c_uint = 0x8D57;
    pub const MAX_SAMPLE_MASK_WORDS: c_uint = 0x8E59;
    pub const MAX_SAMPLE_MASK_WORDS_NV: c_uint = 0x8E59;
    pub const MAX_SERVER_WAIT_TIMEOUT: c_uint = 0x9111;
    pub const MAX_SHADER_BUFFER_ADDRESS_NV: c_uint = 0x8F35;
    pub const MAX_SHADER_COMPILER_THREADS_ARB: c_uint = 0x91B0;
    pub const MAX_SHADER_COMPILER_THREADS_KHR: c_uint = 0x91B0;
    pub const MAX_SHADER_STORAGE_BLOCK_SIZE: c_uint = 0x90DE;
    pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: c_uint = 0x90DD;
    pub const MAX_SHININESS_NV: c_uint = 0x8504;
    pub const MAX_SPARSE_3D_TEXTURE_SIZE_AMD: c_uint = 0x9199;
    pub const MAX_SPARSE_3D_TEXTURE_SIZE_ARB: c_uint = 0x9199;
    pub const MAX_SPARSE_ARRAY_TEXTURE_LAYERS: c_uint = 0x919A;
    pub const MAX_SPARSE_ARRAY_TEXTURE_LAYERS_ARB: c_uint = 0x919A;
    pub const MAX_SPARSE_TEXTURE_SIZE_AMD: c_uint = 0x9198;
    pub const MAX_SPARSE_TEXTURE_SIZE_ARB: c_uint = 0x9198;
    pub const MAX_SPOT_EXPONENT_NV: c_uint = 0x8505;
    pub const MAX_SUBPIXEL_PRECISION_BIAS_BITS_NV: c_uint = 0x9349;
    pub const MAX_SUBROUTINES: c_uint = 0x8DE7;
    pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: c_uint = 0x8DE8;
    pub const MAX_TASK_ATOMIC_COUNTERS_NV: c_uint = 0x8E6D;
    pub const MAX_TASK_ATOMIC_COUNTER_BUFFERS_NV: c_uint = 0x8E6C;
    pub const MAX_TASK_IMAGE_UNIFORMS_NV: c_uint = 0x8E6A;
    pub const MAX_TASK_OUTPUT_COUNT_NV: c_uint = 0x953A;
    pub const MAX_TASK_SHADER_STORAGE_BLOCKS_NV: c_uint = 0x8E6E;
    pub const MAX_TASK_TEXTURE_IMAGE_UNITS_NV: c_uint = 0x8E69;
    pub const MAX_TASK_TOTAL_MEMORY_SIZE_NV: c_uint = 0x9537;
    pub const MAX_TASK_UNIFORM_BLOCKS_NV: c_uint = 0x8E68;
    pub const MAX_TASK_UNIFORM_COMPONENTS_NV: c_uint = 0x8E6B;
    pub const MAX_TASK_WORK_GROUP_INVOCATIONS_NV: c_uint = 0x95A3;
    pub const MAX_TASK_WORK_GROUP_SIZE_NV: c_uint = 0x953C;
    pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: c_uint = 0x92D3;
    pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CD;
    pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: c_uint = 0x90CB;
    pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: c_uint = 0x886C;
    pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: c_uint = 0x8E83;
    pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: c_uint = 0x90D8;
    pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: c_uint = 0x8E81;
    pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: c_uint = 0x8E85;
    pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: c_uint = 0x8E89;
    pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: c_uint = 0x8E7F;
    pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: c_uint = 0x92D4;
    pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CE;
    pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: c_uint = 0x90CC;
    pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: c_uint = 0x886D;
    pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: c_uint = 0x8E86;
    pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: c_uint = 0x90D9;
    pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: c_uint = 0x8E82;
    pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: c_uint = 0x8E8A;
    pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: c_uint = 0x8E80;
    pub const MAX_TESS_GEN_LEVEL: c_uint = 0x8E7E;
    pub const MAX_TESS_PATCH_COMPONENTS: c_uint = 0x8E84;
    pub const MAX_TEXTURE_BUFFER_SIZE: c_uint = 0x8C2B;
    pub const MAX_TEXTURE_BUFFER_SIZE_ARB: c_uint = 0x8C2B;
    pub const MAX_TEXTURE_BUFFER_SIZE_EXT: c_uint = 0x8C2B;
    pub const MAX_TEXTURE_COORDS_ARB: c_uint = 0x8871;
    pub const MAX_TEXTURE_COORDS_NV: c_uint = 0x8871;
    pub const MAX_TEXTURE_IMAGE_UNITS: c_uint = 0x8872;
    pub const MAX_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8872;
    pub const MAX_TEXTURE_IMAGE_UNITS_NV: c_uint = 0x8872;
    pub const MAX_TEXTURE_LOD_BIAS: c_uint = 0x84FD;
    pub const MAX_TEXTURE_LOD_BIAS_EXT: c_uint = 0x84FD;
    pub const MAX_TEXTURE_MAX_ANISOTROPY: c_uint = 0x84FF;
    pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: c_uint = 0x84FF;
    pub const MAX_TEXTURE_SIZE: c_uint = 0x0D33;
    pub const MAX_TEXTURE_UNITS_ARB: c_uint = 0x84E2;
    pub const MAX_TIMELINE_SEMAPHORE_VALUE_DIFFERENCE_NV: c_uint = 0x95B6;
    pub const MAX_TRACK_MATRICES_NV: c_uint = 0x862F;
    pub const MAX_TRACK_MATRIX_STACK_DEPTH_NV: c_uint = 0x862E;
    pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: c_uint = 0x8E70;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: c_uint = 0x8C8A;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS_EXT: c_uint = 0x8C8A;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS_NV: c_uint = 0x8C8A;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: c_uint = 0x8C8B;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS_EXT: c_uint = 0x8C8B;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS_NV: c_uint = 0x8C8B;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: c_uint = 0x8C80;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS_EXT: c_uint = 0x8C80;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS_NV: c_uint = 0x8C80;
    pub const MAX_UNIFORM_BLOCK_SIZE: c_uint = 0x8A30;
    pub const MAX_UNIFORM_BUFFER_BINDINGS: c_uint = 0x8A2F;
    pub const MAX_UNIFORM_LOCATIONS: c_uint = 0x826E;
    pub const MAX_VARYING_COMPONENTS: c_uint = 0x8B4B;
    pub const MAX_VARYING_COMPONENTS_EXT: c_uint = 0x8B4B;
    pub const MAX_VARYING_FLOATS: c_uint = 0x8B4B;
    pub const MAX_VARYING_FLOATS_ARB: c_uint = 0x8B4B;
    pub const MAX_VARYING_VECTORS: c_uint = 0x8DFC;
    pub const MAX_VERTEX_ARRAY_RANGE_ELEMENT_NV: c_uint = 0x8520;
    pub const MAX_VERTEX_ATOMIC_COUNTERS: c_uint = 0x92D2;
    pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CC;
    pub const MAX_VERTEX_ATTRIBS: c_uint = 0x8869;
    pub const MAX_VERTEX_ATTRIBS_ARB: c_uint = 0x8869;
    pub const MAX_VERTEX_ATTRIB_BINDINGS: c_uint = 0x82DA;
    pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: c_uint = 0x82D9;
    pub const MAX_VERTEX_ATTRIB_STRIDE: c_uint = 0x82E5;
    pub const MAX_VERTEX_BINDABLE_UNIFORMS_EXT: c_uint = 0x8DE2;
    pub const MAX_VERTEX_HINT_PGI: c_uint = 0x1A22D;
    pub const MAX_VERTEX_IMAGE_UNIFORMS: c_uint = 0x90CA;
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: c_uint = 0x9122;
    pub const MAX_VERTEX_SHADER_INSTRUCTIONS_EXT: c_uint = 0x87C5;
    pub const MAX_VERTEX_SHADER_INVARIANTS_EXT: c_uint = 0x87C7;
    pub const MAX_VERTEX_SHADER_LOCALS_EXT: c_uint = 0x87C9;
    pub const MAX_VERTEX_SHADER_LOCAL_CONSTANTS_EXT: c_uint = 0x87C8;
    pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: c_uint = 0x90D6;
    pub const MAX_VERTEX_SHADER_VARIANTS_EXT: c_uint = 0x87C6;
    pub const MAX_VERTEX_STREAMS: c_uint = 0x8E71;
    pub const MAX_VERTEX_STREAMS_ATI: c_uint = 0x876B;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4C;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8B4C;
    pub const MAX_VERTEX_UNIFORM_BLOCKS: c_uint = 0x8A2B;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8B4A;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS_ARB: c_uint = 0x8B4A;
    pub const MAX_VERTEX_UNIFORM_VECTORS: c_uint = 0x8DFB;
    pub const MAX_VERTEX_UNITS_ARB: c_uint = 0x86A4;
    pub const MAX_VERTEX_VARYING_COMPONENTS_ARB: c_uint = 0x8DDE;
    pub const MAX_VERTEX_VARYING_COMPONENTS_EXT: c_uint = 0x8DDE;
    pub const MAX_VIEWPORTS: c_uint = 0x825B;
    pub const MAX_VIEWPORT_DIMS: c_uint = 0x0D3A;
    pub const MAX_VIEWS_OVR: c_uint = 0x9631;
    pub const MAX_WIDTH: c_uint = 0x827E;
    pub const MAX_WINDOW_RECTANGLES_EXT: c_uint = 0x8F14;
    pub const MEDIUM_FLOAT: c_uint = 0x8DF1;
    pub const MEDIUM_INT: c_uint = 0x8DF4;
    pub const MEMORY_ATTACHABLE_ALIGNMENT_NV: c_uint = 0x95A6;
    pub const MEMORY_ATTACHABLE_NV: c_uint = 0x95A8;
    pub const MEMORY_ATTACHABLE_SIZE_NV: c_uint = 0x95A7;
    pub const MESH_OUTPUT_PER_PRIMITIVE_GRANULARITY_NV: c_uint = 0x9543;
    pub const MESH_OUTPUT_PER_VERTEX_GRANULARITY_NV: c_uint = 0x92DF;
    pub const MESH_OUTPUT_TYPE_NV: c_uint = 0x957B;
    pub const MESH_PRIMITIVES_OUT_NV: c_uint = 0x957A;
    pub const MESH_SHADER_BIT_NV: c_uint = 0x00000040;
    pub const MESH_SHADER_NV: c_uint = 0x9559;
    pub const MESH_SUBROUTINE_NV: c_uint = 0x957C;
    pub const MESH_SUBROUTINE_UNIFORM_NV: c_uint = 0x957E;
    pub const MESH_VERTICES_OUT_NV: c_uint = 0x9579;
    pub const MESH_WORK_GROUP_SIZE_NV: c_uint = 0x953E;
    pub const MIN: c_uint = 0x8007;
    pub const MINMAX_EXT: c_uint = 0x802E;
    pub const MINMAX_FORMAT_EXT: c_uint = 0x802F;
    pub const MINMAX_SINK_EXT: c_uint = 0x8030;
    pub const MINOR_VERSION: c_uint = 0x821C;
    pub const MINUS_CLAMPED_NV: c_uint = 0x92B3;
    pub const MINUS_NV: c_uint = 0x929F;
    pub const MIN_EXT: c_uint = 0x8007;
    pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: c_uint = 0x8E5B;
    pub const MIN_FRAGMENT_INTERPOLATION_OFFSET_NV: c_uint = 0x8E5B;
    pub const MIN_LOD_WARNING_AMD: c_uint = 0x919C;
    pub const MIN_MAP_BUFFER_ALIGNMENT: c_uint = 0x90BC;
    pub const MIN_PROGRAM_TEXEL_OFFSET: c_uint = 0x8904;
    pub const MIN_PROGRAM_TEXEL_OFFSET_EXT: c_uint = 0x8904;
    pub const MIN_PROGRAM_TEXEL_OFFSET_NV: c_uint = 0x8904;
    pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: c_uint = 0x8E5E;
    pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET_ARB: c_uint = 0x8E5E;
    pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET_NV: c_uint = 0x8E5E;
    pub const MIN_SAMPLE_SHADING_VALUE: c_uint = 0x8C37;
    pub const MIN_SAMPLE_SHADING_VALUE_ARB: c_uint = 0x8C37;
    pub const MIN_SPARSE_LEVEL_AMD: c_uint = 0x919B;
    pub const MIPMAP: c_uint = 0x8293;
    pub const MIRRORED_REPEAT: c_uint = 0x8370;
    pub const MIRRORED_REPEAT_ARB: c_uint = 0x8370;
    pub const MIRRORED_REPEAT_IBM: c_uint = 0x8370;
    pub const MIRROR_CLAMP_ATI: c_uint = 0x8742;
    pub const MIRROR_CLAMP_EXT: c_uint = 0x8742;
    pub const MIRROR_CLAMP_TO_BORDER_EXT: c_uint = 0x8912;
    pub const MIRROR_CLAMP_TO_EDGE: c_uint = 0x8743;
    pub const MIRROR_CLAMP_TO_EDGE_ATI: c_uint = 0x8743;
    pub const MIRROR_CLAMP_TO_EDGE_EXT: c_uint = 0x8743;
    pub const MITER_REVERT_NV: c_uint = 0x90A7;
    pub const MITER_TRUNCATE_NV: c_uint = 0x90A8;
    pub const MIXED_DEPTH_SAMPLES_SUPPORTED_NV: c_uint = 0x932F;
    pub const MIXED_STENCIL_SAMPLES_SUPPORTED_NV: c_uint = 0x9330;
    pub const MODELVIEW0_ARB: c_uint = 0x1700;
    pub const MODELVIEW0_EXT: c_uint = 0x1700;
    pub const MODELVIEW0_MATRIX_EXT: c_uint = 0x0BA6;
    pub const MODELVIEW0_STACK_DEPTH_EXT: c_uint = 0x0BA3;
    pub const MODELVIEW10_ARB: c_uint = 0x872A;
    pub const MODELVIEW11_ARB: c_uint = 0x872B;
    pub const MODELVIEW12_ARB: c_uint = 0x872C;
    pub const MODELVIEW13_ARB: c_uint = 0x872D;
    pub const MODELVIEW14_ARB: c_uint = 0x872E;
    pub const MODELVIEW15_ARB: c_uint = 0x872F;
    pub const MODELVIEW16_ARB: c_uint = 0x8730;
    pub const MODELVIEW17_ARB: c_uint = 0x8731;
    pub const MODELVIEW18_ARB: c_uint = 0x8732;
    pub const MODELVIEW19_ARB: c_uint = 0x8733;
    pub const MODELVIEW1_ARB: c_uint = 0x850A;
    pub const MODELVIEW1_EXT: c_uint = 0x850A;
    pub const MODELVIEW1_MATRIX_EXT: c_uint = 0x8506;
    pub const MODELVIEW1_STACK_DEPTH_EXT: c_uint = 0x8502;
    pub const MODELVIEW20_ARB: c_uint = 0x8734;
    pub const MODELVIEW21_ARB: c_uint = 0x8735;
    pub const MODELVIEW22_ARB: c_uint = 0x8736;
    pub const MODELVIEW23_ARB: c_uint = 0x8737;
    pub const MODELVIEW24_ARB: c_uint = 0x8738;
    pub const MODELVIEW25_ARB: c_uint = 0x8739;
    pub const MODELVIEW26_ARB: c_uint = 0x873A;
    pub const MODELVIEW27_ARB: c_uint = 0x873B;
    pub const MODELVIEW28_ARB: c_uint = 0x873C;
    pub const MODELVIEW29_ARB: c_uint = 0x873D;
    pub const MODELVIEW2_ARB: c_uint = 0x8722;
    pub const MODELVIEW30_ARB: c_uint = 0x873E;
    pub const MODELVIEW31_ARB: c_uint = 0x873F;
    pub const MODELVIEW3_ARB: c_uint = 0x8723;
    pub const MODELVIEW4_ARB: c_uint = 0x8724;
    pub const MODELVIEW5_ARB: c_uint = 0x8725;
    pub const MODELVIEW6_ARB: c_uint = 0x8726;
    pub const MODELVIEW7_ARB: c_uint = 0x8727;
    pub const MODELVIEW8_ARB: c_uint = 0x8728;
    pub const MODELVIEW9_ARB: c_uint = 0x8729;
    pub const MODELVIEW_PROJECTION_NV: c_uint = 0x8629;
    pub const MODULATE_ADD_ATI: c_uint = 0x8744;
    pub const MODULATE_SIGNED_ADD_ATI: c_uint = 0x8745;
    pub const MODULATE_SUBTRACT_ATI: c_uint = 0x8746;
    pub const MOVE_TO_CONTINUES_NV: c_uint = 0x90B6;
    pub const MOVE_TO_NV: c_uint = 0x02;
    pub const MOVE_TO_RESETS_NV: c_uint = 0x90B5;
    pub const MOV_ATI: c_uint = 0x8961;
    pub const MULTICAST_GPUS_NV: c_uint = 0x92BA;
    pub const MULTICAST_PROGRAMMABLE_SAMPLE_LOCATION_NV: c_uint = 0x9549;
    pub const MULTIPLY_KHR: c_uint = 0x9294;
    pub const MULTIPLY_NV: c_uint = 0x9294;
    pub const MULTISAMPLE: c_uint = 0x809D;
    pub const MULTISAMPLES_NV: c_uint = 0x9371;
    pub const MULTISAMPLE_3DFX: c_uint = 0x86B2;
    pub const MULTISAMPLE_ARB: c_uint = 0x809D;
    pub const MULTISAMPLE_BIT_3DFX: c_uint = 0x20000000;
    pub const MULTISAMPLE_BIT_ARB: c_uint = 0x20000000;
    pub const MULTISAMPLE_BIT_EXT: c_uint = 0x20000000;
    pub const MULTISAMPLE_COVERAGE_MODES_NV: c_uint = 0x8E12;
    pub const MULTISAMPLE_EXT: c_uint = 0x809D;
    pub const MULTISAMPLE_FILTER_HINT_NV: c_uint = 0x8534;
    pub const MULTISAMPLE_LINE_WIDTH_GRANULARITY_ARB: c_uint = 0x9382;
    pub const MULTISAMPLE_LINE_WIDTH_RANGE_ARB: c_uint = 0x9381;
    pub const MULTISAMPLE_RASTERIZATION_ALLOWED_EXT: c_uint = 0x932B;
    pub const MULTISAMPLE_SGIS: c_uint = 0x809D;
    pub const MUL_ATI: c_uint = 0x8964;
    pub const MVP_MATRIX_EXT: c_uint = 0x87E3;
    pub const NAMED_STRING_LENGTH_ARB: c_uint = 0x8DE9;
    pub const NAMED_STRING_TYPE_ARB: c_uint = 0x8DEA;
    pub const NAME_LENGTH: c_uint = 0x92F9;
    pub const NAND: c_uint = 0x150E;
    pub const NATIVE_GRAPHICS_BEGIN_HINT_PGI: c_uint = 0x1A203;
    pub const NATIVE_GRAPHICS_END_HINT_PGI: c_uint = 0x1A204;
    pub const NATIVE_GRAPHICS_HANDLE_PGI: c_uint = 0x1A202;
    pub const NEAREST: c_uint = 0x2600;
    pub const NEAREST_CLIPMAP_LINEAR_SGIX: c_uint = 0x844E;
    pub const NEAREST_CLIPMAP_NEAREST_SGIX: c_uint = 0x844D;
    pub const NEAREST_MIPMAP_LINEAR: c_uint = 0x2702;
    pub const NEAREST_MIPMAP_NEAREST: c_uint = 0x2700;
    pub const NEGATE_BIT_ATI: c_uint = 0x00000004;
    pub const NEGATIVE_ONE_EXT: c_uint = 0x87DF;
    pub const NEGATIVE_ONE_TO_ONE: c_uint = 0x935E;
    pub const NEGATIVE_W_EXT: c_uint = 0x87DC;
    pub const NEGATIVE_X_EXT: c_uint = 0x87D9;
    pub const NEGATIVE_Y_EXT: c_uint = 0x87DA;
    pub const NEGATIVE_Z_EXT: c_uint = 0x87DB;
    pub const NEVER: c_uint = 0x0200;
    pub const NEXT_BUFFER_NV: c_int = -2;
    pub const NEXT_VIDEO_CAPTURE_BUFFER_STATUS_NV: c_uint = 0x9025;
    pub const NICEST: c_uint = 0x1102;
    pub const NONE: c_uint = 0;
    pub const NOOP: c_uint = 0x1505;
    pub const NOP_COMMAND_NV: c_uint = 0x0001;
    pub const NOR: c_uint = 0x1508;
    pub const NORMALIZED_RANGE_EXT: c_uint = 0x87E0;
    pub const NORMAL_ARRAY_ADDRESS_NV: c_uint = 0x8F22;
    pub const NORMAL_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8897;
    pub const NORMAL_ARRAY_COUNT_EXT: c_uint = 0x8080;
    pub const NORMAL_ARRAY_EXT: c_uint = 0x8075;
    pub const NORMAL_ARRAY_LENGTH_NV: c_uint = 0x8F2C;
    pub const NORMAL_ARRAY_LIST_IBM: c_uint = 103071;
    pub const NORMAL_ARRAY_LIST_STRIDE_IBM: c_uint = 103081;
    pub const NORMAL_ARRAY_PARALLEL_POINTERS_INTEL: c_uint = 0x83F6;
    pub const NORMAL_ARRAY_POINTER_EXT: c_uint = 0x808F;
    pub const NORMAL_ARRAY_STRIDE_EXT: c_uint = 0x807F;
    pub const NORMAL_ARRAY_TYPE_EXT: c_uint = 0x807E;
    pub const NORMAL_BIT_PGI: c_uint = 0x08000000;
    pub const NORMAL_MAP_ARB: c_uint = 0x8511;
    pub const NORMAL_MAP_EXT: c_uint = 0x8511;
    pub const NORMAL_MAP_NV: c_uint = 0x8511;
    pub const NOTEQUAL: c_uint = 0x0205;
    pub const NO_ERROR: c_uint = 0;
    pub const NO_RESET_NOTIFICATION: c_uint = 0x8261;
    pub const NO_RESET_NOTIFICATION_ARB: c_uint = 0x8261;
    pub const NUM_ACTIVE_VARIABLES: c_uint = 0x9304;
    pub const NUM_COMPATIBLE_SUBROUTINES: c_uint = 0x8E4A;
    pub const NUM_COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A2;
    pub const NUM_COMPRESSED_TEXTURE_FORMATS_ARB: c_uint = 0x86A2;
    pub const NUM_DEVICE_UUIDS_EXT: c_uint = 0x9596;
    pub const NUM_EXTENSIONS: c_uint = 0x821D;
    pub const NUM_FILL_STREAMS_NV: c_uint = 0x8E29;
    pub const NUM_FRAGMENT_CONSTANTS_ATI: c_uint = 0x896F;
    pub const NUM_FRAGMENT_REGISTERS_ATI: c_uint = 0x896E;
    pub const NUM_GENERAL_COMBINERS_NV: c_uint = 0x854E;
    pub const NUM_INPUT_INTERPOLATOR_COMPONENTS_ATI: c_uint = 0x8973;
    pub const NUM_INSTRUCTIONS_PER_PASS_ATI: c_uint = 0x8971;
    pub const NUM_INSTRUCTIONS_TOTAL_ATI: c_uint = 0x8972;
    pub const NUM_LOOPBACK_COMPONENTS_ATI: c_uint = 0x8974;
    pub const NUM_PASSES_ATI: c_uint = 0x8970;
    pub const NUM_PROGRAM_BINARY_FORMATS: c_uint = 0x87FE;
    pub const NUM_SAMPLE_COUNTS: c_uint = 0x9380;
    pub const NUM_SHADER_BINARY_FORMATS: c_uint = 0x8DF9;
    pub const NUM_SHADING_LANGUAGE_VERSIONS: c_uint = 0x82E9;
    pub const NUM_SPARSE_LEVELS_ARB: c_uint = 0x91AA;
    pub const NUM_SPIR_V_EXTENSIONS: c_uint = 0x9554;
    pub const NUM_SUPPORTED_MULTISAMPLE_MODES_AMD: c_uint = 0x91B6;
    pub const NUM_TILING_TYPES_EXT: c_uint = 0x9582;
    pub const NUM_VIDEO_CAPTURE_STREAMS_NV: c_uint = 0x9024;
    pub const NUM_VIRTUAL_PAGE_SIZES_ARB: c_uint = 0x91A8;
    pub const NUM_WINDOW_RECTANGLES_EXT: c_uint = 0x8F15;
    pub const OBJECT_ACTIVE_ATTRIBUTES_ARB: c_uint = 0x8B89;
    pub const OBJECT_ACTIVE_ATTRIBUTE_MAX_LENGTH_ARB: c_uint = 0x8B8A;
    pub const OBJECT_ACTIVE_UNIFORMS_ARB: c_uint = 0x8B86;
    pub const OBJECT_ACTIVE_UNIFORM_MAX_LENGTH_ARB: c_uint = 0x8B87;
    pub const OBJECT_ATTACHED_OBJECTS_ARB: c_uint = 0x8B85;
    pub const OBJECT_BUFFER_SIZE_ATI: c_uint = 0x8764;
    pub const OBJECT_BUFFER_USAGE_ATI: c_uint = 0x8765;
    pub const OBJECT_COMPILE_STATUS_ARB: c_uint = 0x8B81;
    pub const OBJECT_DELETE_STATUS_ARB: c_uint = 0x8B80;
    pub const OBJECT_DISTANCE_TO_LINE_SGIS: c_uint = 0x81F3;
    pub const OBJECT_DISTANCE_TO_POINT_SGIS: c_uint = 0x81F1;
    pub const OBJECT_INFO_LOG_LENGTH_ARB: c_uint = 0x8B84;
    pub const OBJECT_LINE_SGIS: c_uint = 0x81F7;
    pub const OBJECT_LINK_STATUS_ARB: c_uint = 0x8B82;
    pub const OBJECT_POINT_SGIS: c_uint = 0x81F5;
    pub const OBJECT_SHADER_SOURCE_LENGTH_ARB: c_uint = 0x8B88;
    pub const OBJECT_SUBTYPE_ARB: c_uint = 0x8B4F;
    pub const OBJECT_TYPE: c_uint = 0x9112;
    pub const OBJECT_TYPE_ARB: c_uint = 0x8B4E;
    pub const OBJECT_VALIDATE_STATUS_ARB: c_uint = 0x8B83;
    pub const OCCLUSION_QUERY_EVENT_MASK_AMD: c_uint = 0x874F;
    pub const OCCLUSION_TEST_HP: c_uint = 0x8165;
    pub const OCCLUSION_TEST_RESULT_HP: c_uint = 0x8166;
    pub const OFFSET: c_uint = 0x92FC;
    pub const OFFSET_HILO_PROJECTIVE_TEXTURE_2D_NV: c_uint = 0x8856;
    pub const OFFSET_HILO_PROJECTIVE_TEXTURE_RECTANGLE_NV: c_uint = 0x8857;
    pub const OFFSET_HILO_TEXTURE_2D_NV: c_uint = 0x8854;
    pub const OFFSET_HILO_TEXTURE_RECTANGLE_NV: c_uint = 0x8855;
    pub const OFFSET_PROJECTIVE_TEXTURE_2D_NV: c_uint = 0x8850;
    pub const OFFSET_PROJECTIVE_TEXTURE_2D_SCALE_NV: c_uint = 0x8851;
    pub const OFFSET_PROJECTIVE_TEXTURE_RECTANGLE_NV: c_uint = 0x8852;
    pub const OFFSET_PROJECTIVE_TEXTURE_RECTANGLE_SCALE_NV: c_uint = 0x8853;
    pub const OFFSET_TEXTURE_2D_BIAS_NV: c_uint = 0x86E3;
    pub const OFFSET_TEXTURE_2D_MATRIX_NV: c_uint = 0x86E1;
    pub const OFFSET_TEXTURE_2D_NV: c_uint = 0x86E8;
    pub const OFFSET_TEXTURE_2D_SCALE_NV: c_uint = 0x86E2;
    pub const OFFSET_TEXTURE_BIAS_NV: c_uint = 0x86E3;
    pub const OFFSET_TEXTURE_MATRIX_NV: c_uint = 0x86E1;
    pub const OFFSET_TEXTURE_RECTANGLE_NV: c_uint = 0x864C;
    pub const OFFSET_TEXTURE_RECTANGLE_SCALE_NV: c_uint = 0x864D;
    pub const OFFSET_TEXTURE_SCALE_NV: c_uint = 0x86E2;
    pub const ONE: c_uint = 1;
    pub const ONE_EXT: c_uint = 0x87DE;
    pub const ONE_MINUS_CONSTANT_ALPHA: c_uint = 0x8004;
    pub const ONE_MINUS_CONSTANT_ALPHA_EXT: c_uint = 0x8004;
    pub const ONE_MINUS_CONSTANT_COLOR: c_uint = 0x8002;
    pub const ONE_MINUS_CONSTANT_COLOR_EXT: c_uint = 0x8002;
    pub const ONE_MINUS_DST_ALPHA: c_uint = 0x0305;
    pub const ONE_MINUS_DST_COLOR: c_uint = 0x0307;
    pub const ONE_MINUS_SRC1_ALPHA: c_uint = 0x88FB;
    pub const ONE_MINUS_SRC1_COLOR: c_uint = 0x88FA;
    pub const ONE_MINUS_SRC_ALPHA: c_uint = 0x0303;
    pub const ONE_MINUS_SRC_COLOR: c_uint = 0x0301;
    pub const OPERAND0_ALPHA_ARB: c_uint = 0x8598;
    pub const OPERAND0_ALPHA_EXT: c_uint = 0x8598;
    pub const OPERAND0_RGB_ARB: c_uint = 0x8590;
    pub const OPERAND0_RGB_EXT: c_uint = 0x8590;
    pub const OPERAND1_ALPHA_ARB: c_uint = 0x8599;
    pub const OPERAND1_ALPHA_EXT: c_uint = 0x8599;
    pub const OPERAND1_RGB_ARB: c_uint = 0x8591;
    pub const OPERAND1_RGB_EXT: c_uint = 0x8591;
    pub const OPERAND2_ALPHA_ARB: c_uint = 0x859A;
    pub const OPERAND2_ALPHA_EXT: c_uint = 0x859A;
    pub const OPERAND2_RGB_ARB: c_uint = 0x8592;
    pub const OPERAND2_RGB_EXT: c_uint = 0x8592;
    pub const OPERAND3_ALPHA_NV: c_uint = 0x859B;
    pub const OPERAND3_RGB_NV: c_uint = 0x8593;
    pub const OPTIMAL_TILING_EXT: c_uint = 0x9584;
    pub const OP_ADD_EXT: c_uint = 0x8787;
    pub const OP_CLAMP_EXT: c_uint = 0x878E;
    pub const OP_CROSS_PRODUCT_EXT: c_uint = 0x8797;
    pub const OP_DOT3_EXT: c_uint = 0x8784;
    pub const OP_DOT4_EXT: c_uint = 0x8785;
    pub const OP_EXP_BASE_2_EXT: c_uint = 0x8791;
    pub const OP_FLOOR_EXT: c_uint = 0x878F;
    pub const OP_FRAC_EXT: c_uint = 0x8789;
    pub const OP_INDEX_EXT: c_uint = 0x8782;
    pub const OP_LOG_BASE_2_EXT: c_uint = 0x8792;
    pub const OP_MADD_EXT: c_uint = 0x8788;
    pub const OP_MAX_EXT: c_uint = 0x878A;
    pub const OP_MIN_EXT: c_uint = 0x878B;
    pub const OP_MOV_EXT: c_uint = 0x8799;
    pub const OP_MULTIPLY_MATRIX_EXT: c_uint = 0x8798;
    pub const OP_MUL_EXT: c_uint = 0x8786;
    pub const OP_NEGATE_EXT: c_uint = 0x8783;
    pub const OP_POWER_EXT: c_uint = 0x8793;
    pub const OP_RECIP_EXT: c_uint = 0x8794;
    pub const OP_RECIP_SQRT_EXT: c_uint = 0x8795;
    pub const OP_ROUND_EXT: c_uint = 0x8790;
    pub const OP_SET_GE_EXT: c_uint = 0x878C;
    pub const OP_SET_LT_EXT: c_uint = 0x878D;
    pub const OP_SUB_EXT: c_uint = 0x8796;
    pub const OR: c_uint = 0x1507;
    pub const OR_INVERTED: c_uint = 0x150D;
    pub const OR_REVERSE: c_uint = 0x150B;
    pub const OUTPUT_COLOR0_EXT: c_uint = 0x879B;
    pub const OUTPUT_COLOR1_EXT: c_uint = 0x879C;
    pub const OUTPUT_FOG_EXT: c_uint = 0x87BD;
    pub const OUTPUT_TEXTURE_COORD0_EXT: c_uint = 0x879D;
    pub const OUTPUT_TEXTURE_COORD10_EXT: c_uint = 0x87A7;
    pub const OUTPUT_TEXTURE_COORD11_EXT: c_uint = 0x87A8;
    pub const OUTPUT_TEXTURE_COORD12_EXT: c_uint = 0x87A9;
    pub const OUTPUT_TEXTURE_COORD13_EXT: c_uint = 0x87AA;
    pub const OUTPUT_TEXTURE_COORD14_EXT: c_uint = 0x87AB;
    pub const OUTPUT_TEXTURE_COORD15_EXT: c_uint = 0x87AC;
    pub const OUTPUT_TEXTURE_COORD16_EXT: c_uint = 0x87AD;
    pub const OUTPUT_TEXTURE_COORD17_EXT: c_uint = 0x87AE;
    pub const OUTPUT_TEXTURE_COORD18_EXT: c_uint = 0x87AF;
    pub const OUTPUT_TEXTURE_COORD19_EXT: c_uint = 0x87B0;
    pub const OUTPUT_TEXTURE_COORD1_EXT: c_uint = 0x879E;
    pub const OUTPUT_TEXTURE_COORD20_EXT: c_uint = 0x87B1;
    pub const OUTPUT_TEXTURE_COORD21_EXT: c_uint = 0x87B2;
    pub const OUTPUT_TEXTURE_COORD22_EXT: c_uint = 0x87B3;
    pub const OUTPUT_TEXTURE_COORD23_EXT: c_uint = 0x87B4;
    pub const OUTPUT_TEXTURE_COORD24_EXT: c_uint = 0x87B5;
    pub const OUTPUT_TEXTURE_COORD25_EXT: c_uint = 0x87B6;
    pub const OUTPUT_TEXTURE_COORD26_EXT: c_uint = 0x87B7;
    pub const OUTPUT_TEXTURE_COORD27_EXT: c_uint = 0x87B8;
    pub const OUTPUT_TEXTURE_COORD28_EXT: c_uint = 0x87B9;
    pub const OUTPUT_TEXTURE_COORD29_EXT: c_uint = 0x87BA;
    pub const OUTPUT_TEXTURE_COORD2_EXT: c_uint = 0x879F;
    pub const OUTPUT_TEXTURE_COORD30_EXT: c_uint = 0x87BB;
    pub const OUTPUT_TEXTURE_COORD31_EXT: c_uint = 0x87BC;
    pub const OUTPUT_TEXTURE_COORD3_EXT: c_uint = 0x87A0;
    pub const OUTPUT_TEXTURE_COORD4_EXT: c_uint = 0x87A1;
    pub const OUTPUT_TEXTURE_COORD5_EXT: c_uint = 0x87A2;
    pub const OUTPUT_TEXTURE_COORD6_EXT: c_uint = 0x87A3;
    pub const OUTPUT_TEXTURE_COORD7_EXT: c_uint = 0x87A4;
    pub const OUTPUT_TEXTURE_COORD8_EXT: c_uint = 0x87A5;
    pub const OUTPUT_TEXTURE_COORD9_EXT: c_uint = 0x87A6;
    pub const OUTPUT_VERTEX_EXT: c_uint = 0x879A;
    pub const OUT_OF_MEMORY: c_uint = 0x0505;
    pub const OVERLAY_KHR: c_uint = 0x9296;
    pub const OVERLAY_NV: c_uint = 0x9296;
    pub const PACK_ALIGNMENT: c_uint = 0x0D05;
    pub const PACK_CMYK_HINT_EXT: c_uint = 0x800E;
    pub const PACK_COMPRESSED_BLOCK_DEPTH: c_uint = 0x912D;
    pub const PACK_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x912C;
    pub const PACK_COMPRESSED_BLOCK_SIZE: c_uint = 0x912E;
    pub const PACK_COMPRESSED_BLOCK_WIDTH: c_uint = 0x912B;
    pub const PACK_IMAGE_DEPTH_SGIS: c_uint = 0x8131;
    pub const PACK_IMAGE_HEIGHT: c_uint = 0x806C;
    pub const PACK_IMAGE_HEIGHT_EXT: c_uint = 0x806C;
    pub const PACK_INVERT_MESA: c_uint = 0x8758;
    pub const PACK_LSB_FIRST: c_uint = 0x0D01;
    pub const PACK_RESAMPLE_OML: c_uint = 0x8984;
    pub const PACK_RESAMPLE_SGIX: c_uint = 0x842E;
    pub const PACK_ROW_BYTES_APPLE: c_uint = 0x8A15;
    pub const PACK_ROW_LENGTH: c_uint = 0x0D02;
    pub const PACK_SKIP_IMAGES: c_uint = 0x806B;
    pub const PACK_SKIP_IMAGES_EXT: c_uint = 0x806B;
    pub const PACK_SKIP_PIXELS: c_uint = 0x0D04;
    pub const PACK_SKIP_ROWS: c_uint = 0x0D03;
    pub const PACK_SKIP_VOLUMES_SGIS: c_uint = 0x8130;
    pub const PACK_SUBSAMPLE_RATE_SGIX: c_uint = 0x85A0;
    pub const PACK_SWAP_BYTES: c_uint = 0x0D00;
    pub const PALETTE4_R5_G6_B5_OES: c_uint = 0x8B92;
    pub const PALETTE4_RGB5_A1_OES: c_uint = 0x8B94;
    pub const PALETTE4_RGB8_OES: c_uint = 0x8B90;
    pub const PALETTE4_RGBA4_OES: c_uint = 0x8B93;
    pub const PALETTE4_RGBA8_OES: c_uint = 0x8B91;
    pub const PALETTE8_R5_G6_B5_OES: c_uint = 0x8B97;
    pub const PALETTE8_RGB5_A1_OES: c_uint = 0x8B99;
    pub const PALETTE8_RGB8_OES: c_uint = 0x8B95;
    pub const PALETTE8_RGBA4_OES: c_uint = 0x8B98;
    pub const PALETTE8_RGBA8_OES: c_uint = 0x8B96;
    pub const PARALLEL_ARRAYS_INTEL: c_uint = 0x83F4;
    pub const PARAMETER_BUFFER: c_uint = 0x80EE;
    pub const PARAMETER_BUFFER_ARB: c_uint = 0x80EE;
    pub const PARAMETER_BUFFER_BINDING: c_uint = 0x80EF;
    pub const PARAMETER_BUFFER_BINDING_ARB: c_uint = 0x80EF;
    pub const PARTIAL_SUCCESS_NV: c_uint = 0x902E;
    pub const PASS_THROUGH_NV: c_uint = 0x86E6;
    pub const PATCHES: c_uint = 0x000E;
    pub const PATCH_DEFAULT_INNER_LEVEL: c_uint = 0x8E73;
    pub const PATCH_DEFAULT_OUTER_LEVEL: c_uint = 0x8E74;
    pub const PATCH_VERTICES: c_uint = 0x8E72;
    pub const PATH_CLIENT_LENGTH_NV: c_uint = 0x907F;
    pub const PATH_COMMAND_COUNT_NV: c_uint = 0x909D;
    pub const PATH_COMPUTED_LENGTH_NV: c_uint = 0x90A0;
    pub const PATH_COORD_COUNT_NV: c_uint = 0x909E;
    pub const PATH_COVER_DEPTH_FUNC_NV: c_uint = 0x90BF;
    pub const PATH_DASH_ARRAY_COUNT_NV: c_uint = 0x909F;
    pub const PATH_DASH_CAPS_NV: c_uint = 0x907B;
    pub const PATH_DASH_OFFSET_NV: c_uint = 0x907E;
    pub const PATH_DASH_OFFSET_RESET_NV: c_uint = 0x90B4;
    pub const PATH_END_CAPS_NV: c_uint = 0x9076;
    pub const PATH_ERROR_POSITION_NV: c_uint = 0x90AB;
    pub const PATH_FILL_BOUNDING_BOX_NV: c_uint = 0x90A1;
    pub const PATH_FILL_COVER_MODE_NV: c_uint = 0x9082;
    pub const PATH_FILL_MASK_NV: c_uint = 0x9081;
    pub const PATH_FILL_MODE_NV: c_uint = 0x9080;
    pub const PATH_FORMAT_PS_NV: c_uint = 0x9071;
    pub const PATH_FORMAT_SVG_NV: c_uint = 0x9070;
    pub const PATH_GEN_COEFF_NV: c_uint = 0x90B1;
    pub const PATH_GEN_COMPONENTS_NV: c_uint = 0x90B3;
    pub const PATH_GEN_MODE_NV: c_uint = 0x90B0;
    pub const PATH_INITIAL_DASH_CAP_NV: c_uint = 0x907C;
    pub const PATH_INITIAL_END_CAP_NV: c_uint = 0x9077;
    pub const PATH_JOIN_STYLE_NV: c_uint = 0x9079;
    pub const PATH_MAX_MODELVIEW_STACK_DEPTH_NV: c_uint = 0x0D36;
    pub const PATH_MAX_PROJECTION_STACK_DEPTH_NV: c_uint = 0x0D38;
    pub const PATH_MITER_LIMIT_NV: c_uint = 0x907A;
    pub const PATH_MODELVIEW_MATRIX_NV: c_uint = 0x0BA6;
    pub const PATH_MODELVIEW_NV: c_uint = 0x1700;
    pub const PATH_MODELVIEW_STACK_DEPTH_NV: c_uint = 0x0BA3;
    pub const PATH_OBJECT_BOUNDING_BOX_NV: c_uint = 0x908A;
    pub const PATH_PROJECTION_MATRIX_NV: c_uint = 0x0BA7;
    pub const PATH_PROJECTION_NV: c_uint = 0x1701;
    pub const PATH_PROJECTION_STACK_DEPTH_NV: c_uint = 0x0BA4;
    pub const PATH_STENCIL_DEPTH_OFFSET_FACTOR_NV: c_uint = 0x90BD;
    pub const PATH_STENCIL_DEPTH_OFFSET_UNITS_NV: c_uint = 0x90BE;
    pub const PATH_STENCIL_FUNC_NV: c_uint = 0x90B7;
    pub const PATH_STENCIL_REF_NV: c_uint = 0x90B8;
    pub const PATH_STENCIL_VALUE_MASK_NV: c_uint = 0x90B9;
    pub const PATH_STROKE_BOUNDING_BOX_NV: c_uint = 0x90A2;
    pub const PATH_STROKE_COVER_MODE_NV: c_uint = 0x9083;
    pub const PATH_STROKE_MASK_NV: c_uint = 0x9084;
    pub const PATH_STROKE_WIDTH_NV: c_uint = 0x9075;
    pub const PATH_TERMINAL_DASH_CAP_NV: c_uint = 0x907D;
    pub const PATH_TERMINAL_END_CAP_NV: c_uint = 0x9078;
    pub const PATH_TRANSPOSE_MODELVIEW_MATRIX_NV: c_uint = 0x84E3;
    pub const PATH_TRANSPOSE_PROJECTION_MATRIX_NV: c_uint = 0x84E4;
    pub const PERCENTAGE_AMD: c_uint = 0x8BC3;
    pub const PERFMON_RESULT_AMD: c_uint = 0x8BC6;
    pub const PERFMON_RESULT_AVAILABLE_AMD: c_uint = 0x8BC4;
    pub const PERFMON_RESULT_SIZE_AMD: c_uint = 0x8BC5;
    pub const PERFORMANCE_MONITOR_AMD: c_uint = 0x9152;
    pub const PERFQUERY_COUNTER_DATA_BOOL32_INTEL: c_uint = 0x94FC;
    pub const PERFQUERY_COUNTER_DATA_DOUBLE_INTEL: c_uint = 0x94FB;
    pub const PERFQUERY_COUNTER_DATA_FLOAT_INTEL: c_uint = 0x94FA;
    pub const PERFQUERY_COUNTER_DATA_UINT32_INTEL: c_uint = 0x94F8;
    pub const PERFQUERY_COUNTER_DATA_UINT64_INTEL: c_uint = 0x94F9;
    pub const PERFQUERY_COUNTER_DESC_LENGTH_MAX_INTEL: c_uint = 0x94FF;
    pub const PERFQUERY_COUNTER_DURATION_NORM_INTEL: c_uint = 0x94F1;
    pub const PERFQUERY_COUNTER_DURATION_RAW_INTEL: c_uint = 0x94F2;
    pub const PERFQUERY_COUNTER_EVENT_INTEL: c_uint = 0x94F0;
    pub const PERFQUERY_COUNTER_NAME_LENGTH_MAX_INTEL: c_uint = 0x94FE;
    pub const PERFQUERY_COUNTER_RAW_INTEL: c_uint = 0x94F4;
    pub const PERFQUERY_COUNTER_THROUGHPUT_INTEL: c_uint = 0x94F3;
    pub const PERFQUERY_COUNTER_TIMESTAMP_INTEL: c_uint = 0x94F5;
    pub const PERFQUERY_DONOT_FLUSH_INTEL: c_uint = 0x83F9;
    pub const PERFQUERY_FLUSH_INTEL: c_uint = 0x83FA;
    pub const PERFQUERY_GLOBAL_CONTEXT_INTEL: c_uint = 0x00000001;
    pub const PERFQUERY_GPA_EXTENDED_COUNTERS_INTEL: c_uint = 0x9500;
    pub const PERFQUERY_QUERY_NAME_LENGTH_MAX_INTEL: c_uint = 0x94FD;
    pub const PERFQUERY_SINGLE_CONTEXT_INTEL: c_uint = 0x00000000;
    pub const PERFQUERY_WAIT_INTEL: c_uint = 0x83FB;
    pub const PERTURB_EXT: c_uint = 0x85AE;
    pub const PER_GPU_STORAGE_BIT_NV: c_uint = 0x0800;
    pub const PER_GPU_STORAGE_NV: c_uint = 0x9548;
    pub const PER_STAGE_CONSTANTS_NV: c_uint = 0x8535;
    pub const PHONG_HINT_WIN: c_uint = 0x80EB;
    pub const PHONG_WIN: c_uint = 0x80EA;
    pub const PINLIGHT_NV: c_uint = 0x92A8;
    pub const PIXELS_PER_SAMPLE_PATTERN_X_AMD: c_uint = 0x91AE;
    pub const PIXELS_PER_SAMPLE_PATTERN_Y_AMD: c_uint = 0x91AF;
    pub const PIXEL_BUFFER_BARRIER_BIT: c_uint = 0x00000080;
    pub const PIXEL_BUFFER_BARRIER_BIT_EXT: c_uint = 0x00000080;
    pub const PIXEL_COUNTER_BITS_NV: c_uint = 0x8864;
    pub const PIXEL_COUNT_AVAILABLE_NV: c_uint = 0x8867;
    pub const PIXEL_COUNT_NV: c_uint = 0x8866;
    pub const PIXEL_CUBIC_WEIGHT_EXT: c_uint = 0x8333;
    pub const PIXEL_FRAGMENT_ALPHA_SOURCE_SGIS: c_uint = 0x8355;
    pub const PIXEL_FRAGMENT_RGB_SOURCE_SGIS: c_uint = 0x8354;
    pub const PIXEL_GROUP_COLOR_SGIS: c_uint = 0x8356;
    pub const PIXEL_MAG_FILTER_EXT: c_uint = 0x8331;
    pub const PIXEL_MIN_FILTER_EXT: c_uint = 0x8332;
    pub const PIXEL_PACK_BUFFER: c_uint = 0x88EB;
    pub const PIXEL_PACK_BUFFER_ARB: c_uint = 0x88EB;
    pub const PIXEL_PACK_BUFFER_BINDING: c_uint = 0x88ED;
    pub const PIXEL_PACK_BUFFER_BINDING_ARB: c_uint = 0x88ED;
    pub const PIXEL_PACK_BUFFER_BINDING_EXT: c_uint = 0x88ED;
    pub const PIXEL_PACK_BUFFER_EXT: c_uint = 0x88EB;
    pub const PIXEL_SUBSAMPLE_2424_SGIX: c_uint = 0x85A3;
    pub const PIXEL_SUBSAMPLE_4242_SGIX: c_uint = 0x85A4;
    pub const PIXEL_SUBSAMPLE_4444_SGIX: c_uint = 0x85A2;
    pub const PIXEL_TEXTURE_SGIS: c_uint = 0x8353;
    pub const PIXEL_TEX_GEN_MODE_SGIX: c_uint = 0x832B;
    pub const PIXEL_TEX_GEN_SGIX: c_uint = 0x8139;
    pub const PIXEL_TILE_BEST_ALIGNMENT_SGIX: c_uint = 0x813E;
    pub const PIXEL_TILE_CACHE_INCREMENT_SGIX: c_uint = 0x813F;
    pub const PIXEL_TILE_CACHE_SIZE_SGIX: c_uint = 0x8145;
    pub const PIXEL_TILE_GRID_DEPTH_SGIX: c_uint = 0x8144;
    pub const PIXEL_TILE_GRID_HEIGHT_SGIX: c_uint = 0x8143;
    pub const PIXEL_TILE_GRID_WIDTH_SGIX: c_uint = 0x8142;
    pub const PIXEL_TILE_HEIGHT_SGIX: c_uint = 0x8141;
    pub const PIXEL_TILE_WIDTH_SGIX: c_uint = 0x8140;
    pub const PIXEL_TRANSFORM_2D_EXT: c_uint = 0x8330;
    pub const PIXEL_TRANSFORM_2D_MATRIX_EXT: c_uint = 0x8338;
    pub const PIXEL_TRANSFORM_2D_STACK_DEPTH_EXT: c_uint = 0x8336;
    pub const PIXEL_UNPACK_BUFFER: c_uint = 0x88EC;
    pub const PIXEL_UNPACK_BUFFER_ARB: c_uint = 0x88EC;
    pub const PIXEL_UNPACK_BUFFER_BINDING: c_uint = 0x88EF;
    pub const PIXEL_UNPACK_BUFFER_BINDING_ARB: c_uint = 0x88EF;
    pub const PIXEL_UNPACK_BUFFER_BINDING_EXT: c_uint = 0x88EF;
    pub const PIXEL_UNPACK_BUFFER_EXT: c_uint = 0x88EC;
    pub const PLUS_CLAMPED_ALPHA_NV: c_uint = 0x92B2;
    pub const PLUS_CLAMPED_NV: c_uint = 0x92B1;
    pub const PLUS_DARKER_NV: c_uint = 0x9292;
    pub const PLUS_NV: c_uint = 0x9291;
    pub const PN_TRIANGLES_ATI: c_uint = 0x87F0;
    pub const PN_TRIANGLES_NORMAL_MODE_ATI: c_uint = 0x87F3;
    pub const PN_TRIANGLES_NORMAL_MODE_LINEAR_ATI: c_uint = 0x87F7;
    pub const PN_TRIANGLES_NORMAL_MODE_QUADRATIC_ATI: c_uint = 0x87F8;
    pub const PN_TRIANGLES_POINT_MODE_ATI: c_uint = 0x87F2;
    pub const PN_TRIANGLES_POINT_MODE_CUBIC_ATI: c_uint = 0x87F6;
    pub const PN_TRIANGLES_POINT_MODE_LINEAR_ATI: c_uint = 0x87F5;
    pub const PN_TRIANGLES_TESSELATION_LEVEL_ATI: c_uint = 0x87F4;
    pub const POINT: c_uint = 0x1B00;
    pub const POINTS: c_uint = 0x0000;
    pub const POINT_DISTANCE_ATTENUATION_ARB: c_uint = 0x8129;
    pub const POINT_FADE_THRESHOLD_SIZE: c_uint = 0x8128;
    pub const POINT_FADE_THRESHOLD_SIZE_ARB: c_uint = 0x8128;
    pub const POINT_FADE_THRESHOLD_SIZE_EXT: c_uint = 0x8128;
    pub const POINT_FADE_THRESHOLD_SIZE_SGIS: c_uint = 0x8128;
    pub const POINT_SIZE: c_uint = 0x0B11;
    pub const POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
    pub const POINT_SIZE_MAX_ARB: c_uint = 0x8127;
    pub const POINT_SIZE_MAX_EXT: c_uint = 0x8127;
    pub const POINT_SIZE_MAX_SGIS: c_uint = 0x8127;
    pub const POINT_SIZE_MIN_ARB: c_uint = 0x8126;
    pub const POINT_SIZE_MIN_EXT: c_uint = 0x8126;
    pub const POINT_SIZE_MIN_SGIS: c_uint = 0x8126;
    pub const POINT_SIZE_RANGE: c_uint = 0x0B12;
    pub const POINT_SPRITE_ARB: c_uint = 0x8861;
    pub const POINT_SPRITE_COORD_ORIGIN: c_uint = 0x8CA0;
    pub const POINT_SPRITE_NV: c_uint = 0x8861;
    pub const POINT_SPRITE_R_MODE_NV: c_uint = 0x8863;
    pub const POLYGON_MODE: c_uint = 0x0B40;
    pub const POLYGON_OFFSET_BIAS_EXT: c_uint = 0x8039;
    pub const POLYGON_OFFSET_CLAMP: c_uint = 0x8E1B;
    pub const POLYGON_OFFSET_CLAMP_EXT: c_uint = 0x8E1B;
    pub const POLYGON_OFFSET_COMMAND_NV: c_uint = 0x000E;
    pub const POLYGON_OFFSET_EXT: c_uint = 0x8037;
    pub const POLYGON_OFFSET_FACTOR: c_uint = 0x8038;
    pub const POLYGON_OFFSET_FACTOR_EXT: c_uint = 0x8038;
    pub const POLYGON_OFFSET_FILL: c_uint = 0x8037;
    pub const POLYGON_OFFSET_LINE: c_uint = 0x2A02;
    pub const POLYGON_OFFSET_POINT: c_uint = 0x2A01;
    pub const POLYGON_OFFSET_UNITS: c_uint = 0x2A00;
    pub const POLYGON_SMOOTH: c_uint = 0x0B41;
    pub const POLYGON_SMOOTH_HINT: c_uint = 0x0C53;
    pub const POST_COLOR_MATRIX_ALPHA_BIAS_SGI: c_uint = 0x80BB;
    pub const POST_COLOR_MATRIX_ALPHA_SCALE_SGI: c_uint = 0x80B7;
    pub const POST_COLOR_MATRIX_BLUE_BIAS_SGI: c_uint = 0x80BA;
    pub const POST_COLOR_MATRIX_BLUE_SCALE_SGI: c_uint = 0x80B6;
    pub const POST_COLOR_MATRIX_COLOR_TABLE_SGI: c_uint = 0x80D2;
    pub const POST_COLOR_MATRIX_GREEN_BIAS_SGI: c_uint = 0x80B9;
    pub const POST_COLOR_MATRIX_GREEN_SCALE_SGI: c_uint = 0x80B5;
    pub const POST_COLOR_MATRIX_RED_BIAS_SGI: c_uint = 0x80B8;
    pub const POST_COLOR_MATRIX_RED_SCALE_SGI: c_uint = 0x80B4;
    pub const POST_CONVOLUTION_ALPHA_BIAS_EXT: c_uint = 0x8023;
    pub const POST_CONVOLUTION_ALPHA_SCALE_EXT: c_uint = 0x801F;
    pub const POST_CONVOLUTION_BLUE_BIAS_EXT: c_uint = 0x8022;
    pub const POST_CONVOLUTION_BLUE_SCALE_EXT: c_uint = 0x801E;
    pub const POST_CONVOLUTION_COLOR_TABLE_SGI: c_uint = 0x80D1;
    pub const POST_CONVOLUTION_GREEN_BIAS_EXT: c_uint = 0x8021;
    pub const POST_CONVOLUTION_GREEN_SCALE_EXT: c_uint = 0x801D;
    pub const POST_CONVOLUTION_RED_BIAS_EXT: c_uint = 0x8020;
    pub const POST_CONVOLUTION_RED_SCALE_EXT: c_uint = 0x801C;
    pub const POST_IMAGE_TRANSFORM_COLOR_TABLE_HP: c_uint = 0x8162;
    pub const POST_TEXTURE_FILTER_BIAS_RANGE_SGIX: c_uint = 0x817B;
    pub const POST_TEXTURE_FILTER_BIAS_SGIX: c_uint = 0x8179;
    pub const POST_TEXTURE_FILTER_SCALE_RANGE_SGIX: c_uint = 0x817C;
    pub const POST_TEXTURE_FILTER_SCALE_SGIX: c_uint = 0x817A;
    pub const PREFER_DOUBLEBUFFER_HINT_PGI: c_uint = 0x1A1F8;
    pub const PRESENT_DURATION_NV: c_uint = 0x8E2B;
    pub const PRESENT_TIME_NV: c_uint = 0x8E2A;
    pub const PRESERVE_ATI: c_uint = 0x8762;
    pub const PREVIOUS_ARB: c_uint = 0x8578;
    pub const PREVIOUS_EXT: c_uint = 0x8578;
    pub const PREVIOUS_TEXTURE_INPUT_NV: c_uint = 0x86E4;
    pub const PRIMARY_COLOR_ARB: c_uint = 0x8577;
    pub const PRIMARY_COLOR_EXT: c_uint = 0x8577;
    pub const PRIMARY_COLOR_NV: c_uint = 0x852C;
    pub const PRIMITIVES_GENERATED: c_uint = 0x8C87;
    pub const PRIMITIVES_GENERATED_EXT: c_uint = 0x8C87;
    pub const PRIMITIVES_GENERATED_NV: c_uint = 0x8C87;
    pub const PRIMITIVES_SUBMITTED: c_uint = 0x82EF;
    pub const PRIMITIVES_SUBMITTED_ARB: c_uint = 0x82EF;
    pub const PRIMITIVE_BOUNDING_BOX_ARB: c_uint = 0x92BE;
    pub const PRIMITIVE_ID_NV: c_uint = 0x8C7C;
    pub const PRIMITIVE_RESTART: c_uint = 0x8F9D;
    pub const PRIMITIVE_RESTART_FIXED_INDEX: c_uint = 0x8D69;
    pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: c_uint = 0x8221;
    pub const PRIMITIVE_RESTART_INDEX: c_uint = 0x8F9E;
    pub const PRIMITIVE_RESTART_INDEX_NV: c_uint = 0x8559;
    pub const PRIMITIVE_RESTART_NV: c_uint = 0x8558;
    pub const PROGRAM: c_uint = 0x82E2;
    pub const PROGRAMMABLE_SAMPLE_LOCATION_ARB: c_uint = 0x9341;
    pub const PROGRAMMABLE_SAMPLE_LOCATION_NV: c_uint = 0x9341;
    pub const PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_ARB: c_uint = 0x9340;
    pub const PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_NV: c_uint = 0x9340;
    pub const PROGRAM_ADDRESS_REGISTERS_ARB: c_uint = 0x88B0;
    pub const PROGRAM_ALU_INSTRUCTIONS_ARB: c_uint = 0x8805;
    pub const PROGRAM_ATTRIBS_ARB: c_uint = 0x88AC;
    pub const PROGRAM_ATTRIB_COMPONENTS_NV: c_uint = 0x8906;
    pub const PROGRAM_BINARY_FORMATS: c_uint = 0x87FF;
    pub const PROGRAM_BINARY_FORMAT_MESA: c_uint = 0x875F;
    pub const PROGRAM_BINARY_LENGTH: c_uint = 0x8741;
    pub const PROGRAM_BINARY_RETRIEVABLE_HINT: c_uint = 0x8257;
    pub const PROGRAM_BINDING_ARB: c_uint = 0x8677;
    pub const PROGRAM_ERROR_POSITION_ARB: c_uint = 0x864B;
    pub const PROGRAM_ERROR_POSITION_NV: c_uint = 0x864B;
    pub const PROGRAM_ERROR_STRING_ARB: c_uint = 0x8874;
    pub const PROGRAM_ERROR_STRING_NV: c_uint = 0x8874;
    pub const PROGRAM_FORMAT_ARB: c_uint = 0x8876;
    pub const PROGRAM_FORMAT_ASCII_ARB: c_uint = 0x8875;
    pub const PROGRAM_INPUT: c_uint = 0x92E3;
    pub const PROGRAM_INSTRUCTIONS_ARB: c_uint = 0x88A0;
    pub const PROGRAM_LENGTH_ARB: c_uint = 0x8627;
    pub const PROGRAM_LENGTH_NV: c_uint = 0x8627;
    pub const PROGRAM_MATRIX_EXT: c_uint = 0x8E2D;
    pub const PROGRAM_MATRIX_STACK_DEPTH_EXT: c_uint = 0x8E2F;
    pub const PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: c_uint = 0x88B2;
    pub const PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB: c_uint = 0x8808;
    pub const PROGRAM_NATIVE_ATTRIBS_ARB: c_uint = 0x88AE;
    pub const PROGRAM_NATIVE_INSTRUCTIONS_ARB: c_uint = 0x88A2;
    pub const PROGRAM_NATIVE_PARAMETERS_ARB: c_uint = 0x88AA;
    pub const PROGRAM_NATIVE_TEMPORARIES_ARB: c_uint = 0x88A6;
    pub const PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB: c_uint = 0x880A;
    pub const PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB: c_uint = 0x8809;
    pub const PROGRAM_OBJECT_ARB: c_uint = 0x8B40;
    pub const PROGRAM_OBJECT_EXT: c_uint = 0x8B40;
    pub const PROGRAM_OUTPUT: c_uint = 0x92E4;
    pub const PROGRAM_PARAMETERS_ARB: c_uint = 0x88A8;
    pub const PROGRAM_PARAMETER_NV: c_uint = 0x8644;
    pub const PROGRAM_PIPELINE: c_uint = 0x82E4;
    pub const PROGRAM_PIPELINE_BINDING: c_uint = 0x825A;
    pub const PROGRAM_PIPELINE_OBJECT_EXT: c_uint = 0x8A4F;
    pub const PROGRAM_POINT_SIZE: c_uint = 0x8642;
    pub const PROGRAM_POINT_SIZE_ARB: c_uint = 0x8642;
    pub const PROGRAM_POINT_SIZE_EXT: c_uint = 0x8642;
    pub const PROGRAM_RESIDENT_NV: c_uint = 0x8647;
    pub const PROGRAM_RESULT_COMPONENTS_NV: c_uint = 0x8907;
    pub const PROGRAM_SEPARABLE: c_uint = 0x8258;
    pub const PROGRAM_STRING_ARB: c_uint = 0x8628;
    pub const PROGRAM_STRING_NV: c_uint = 0x8628;
    pub const PROGRAM_TARGET_NV: c_uint = 0x8646;
    pub const PROGRAM_TEMPORARIES_ARB: c_uint = 0x88A4;
    pub const PROGRAM_TEX_INDIRECTIONS_ARB: c_uint = 0x8807;
    pub const PROGRAM_TEX_INSTRUCTIONS_ARB: c_uint = 0x8806;
    pub const PROGRAM_UNDER_NATIVE_LIMITS_ARB: c_uint = 0x88B6;
    pub const PROTECTED_MEMORY_OBJECT_EXT: c_uint = 0x959B;
    pub const PROVOKING_VERTEX: c_uint = 0x8E4F;
    pub const PROVOKING_VERTEX_EXT: c_uint = 0x8E4F;
    pub const PROXY_COLOR_TABLE_SGI: c_uint = 0x80D3;
    pub const PROXY_HISTOGRAM_EXT: c_uint = 0x8025;
    pub const PROXY_POST_COLOR_MATRIX_COLOR_TABLE_SGI: c_uint = 0x80D5;
    pub const PROXY_POST_CONVOLUTION_COLOR_TABLE_SGI: c_uint = 0x80D4;
    pub const PROXY_POST_IMAGE_TRANSFORM_COLOR_TABLE_HP: c_uint = 0x8163;
    pub const PROXY_TEXTURE_1D: c_uint = 0x8063;
    pub const PROXY_TEXTURE_1D_ARRAY: c_uint = 0x8C19;
    pub const PROXY_TEXTURE_1D_ARRAY_EXT: c_uint = 0x8C19;
    pub const PROXY_TEXTURE_1D_EXT: c_uint = 0x8063;
    pub const PROXY_TEXTURE_1D_STACK_MESAX: c_uint = 0x875B;
    pub const PROXY_TEXTURE_2D: c_uint = 0x8064;
    pub const PROXY_TEXTURE_2D_ARRAY: c_uint = 0x8C1B;
    pub const PROXY_TEXTURE_2D_ARRAY_EXT: c_uint = 0x8C1B;
    pub const PROXY_TEXTURE_2D_EXT: c_uint = 0x8064;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE: c_uint = 0x9101;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9103;
    pub const PROXY_TEXTURE_2D_STACK_MESAX: c_uint = 0x875C;
    pub const PROXY_TEXTURE_3D: c_uint = 0x8070;
    pub const PROXY_TEXTURE_3D_EXT: c_uint = 0x8070;
    pub const PROXY_TEXTURE_4D_SGIS: c_uint = 0x8135;
    pub const PROXY_TEXTURE_COLOR_TABLE_SGI: c_uint = 0x80BD;
    pub const PROXY_TEXTURE_CUBE_MAP: c_uint = 0x851B;
    pub const PROXY_TEXTURE_CUBE_MAP_ARB: c_uint = 0x851B;
    pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: c_uint = 0x900B;
    pub const PROXY_TEXTURE_CUBE_MAP_ARRAY_ARB: c_uint = 0x900B;
    pub const PROXY_TEXTURE_CUBE_MAP_EXT: c_uint = 0x851B;
    pub const PROXY_TEXTURE_RECTANGLE: c_uint = 0x84F7;
    pub const PROXY_TEXTURE_RECTANGLE_ARB: c_uint = 0x84F7;
    pub const PROXY_TEXTURE_RECTANGLE_NV: c_uint = 0x84F7;
    pub const PURGEABLE_APPLE: c_uint = 0x8A1D;
    pub const PURGED_CONTEXT_RESET_NV: c_uint = 0x92BB;
    pub const QUADRATIC_CURVE_TO_NV: c_uint = 0x0A;
    pub const QUADS: c_uint = 0x0007;
    pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: c_uint = 0x8E4C;
    pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION_EXT: c_uint = 0x8E4C;
    pub const QUAD_ALPHA4_SGIS: c_uint = 0x811E;
    pub const QUAD_ALPHA8_SGIS: c_uint = 0x811F;
    pub const QUAD_INTENSITY4_SGIS: c_uint = 0x8122;
    pub const QUAD_INTENSITY8_SGIS: c_uint = 0x8123;
    pub const QUAD_LUMINANCE4_SGIS: c_uint = 0x8120;
    pub const QUAD_LUMINANCE8_SGIS: c_uint = 0x8121;
    pub const QUAD_MESH_SUN: c_uint = 0x8614;
    pub const QUAD_TEXTURE_SELECT_SGIS: c_uint = 0x8125;
    pub const QUARTER_BIT_ATI: c_uint = 0x00000010;
    pub const QUERY: c_uint = 0x82E3;
    pub const QUERY_ALL_EVENT_BITS_AMD: c_uint = 0xFFFFFFFF;
    pub const QUERY_BUFFER: c_uint = 0x9192;
    pub const QUERY_BUFFER_AMD: c_uint = 0x9192;
    pub const QUERY_BUFFER_BARRIER_BIT: c_uint = 0x00008000;
    pub const QUERY_BUFFER_BINDING: c_uint = 0x9193;
    pub const QUERY_BUFFER_BINDING_AMD: c_uint = 0x9193;
    pub const QUERY_BY_REGION_NO_WAIT: c_uint = 0x8E16;
    pub const QUERY_BY_REGION_NO_WAIT_INVERTED: c_uint = 0x8E1A;
    pub const QUERY_BY_REGION_NO_WAIT_NV: c_uint = 0x8E16;
    pub const QUERY_BY_REGION_WAIT: c_uint = 0x8E15;
    pub const QUERY_BY_REGION_WAIT_INVERTED: c_uint = 0x8E19;
    pub const QUERY_BY_REGION_WAIT_NV: c_uint = 0x8E15;
    pub const QUERY_COUNTER_BITS: c_uint = 0x8864;
    pub const QUERY_COUNTER_BITS_ARB: c_uint = 0x8864;
    pub const QUERY_DEPTH_BOUNDS_FAIL_EVENT_BIT_AMD: c_uint = 0x00000008;
    pub const QUERY_DEPTH_FAIL_EVENT_BIT_AMD: c_uint = 0x00000002;
    pub const QUERY_DEPTH_PASS_EVENT_BIT_AMD: c_uint = 0x00000001;
    pub const QUERY_NO_WAIT: c_uint = 0x8E14;
    pub const QUERY_NO_WAIT_INVERTED: c_uint = 0x8E18;
    pub const QUERY_NO_WAIT_NV: c_uint = 0x8E14;
    pub const QUERY_OBJECT_AMD: c_uint = 0x9153;
    pub const QUERY_OBJECT_EXT: c_uint = 0x9153;
    pub const QUERY_RESOURCE_BUFFEROBJECT_NV: c_uint = 0x9547;
    pub const QUERY_RESOURCE_MEMTYPE_VIDMEM_NV: c_uint = 0x9542;
    pub const QUERY_RESOURCE_RENDERBUFFER_NV: c_uint = 0x9546;
    pub const QUERY_RESOURCE_SYS_RESERVED_NV: c_uint = 0x9544;
    pub const QUERY_RESOURCE_TEXTURE_NV: c_uint = 0x9545;
    pub const QUERY_RESOURCE_TYPE_VIDMEM_ALLOC_NV: c_uint = 0x9540;
    pub const QUERY_RESULT: c_uint = 0x8866;
    pub const QUERY_RESULT_ARB: c_uint = 0x8866;
    pub const QUERY_RESULT_AVAILABLE: c_uint = 0x8867;
    pub const QUERY_RESULT_AVAILABLE_ARB: c_uint = 0x8867;
    pub const QUERY_RESULT_NO_WAIT: c_uint = 0x9194;
    pub const QUERY_RESULT_NO_WAIT_AMD: c_uint = 0x9194;
    pub const QUERY_STENCIL_FAIL_EVENT_BIT_AMD: c_uint = 0x00000004;
    pub const QUERY_TARGET: c_uint = 0x82EA;
    pub const QUERY_WAIT: c_uint = 0x8E13;
    pub const QUERY_WAIT_INVERTED: c_uint = 0x8E17;
    pub const QUERY_WAIT_NV: c_uint = 0x8E13;
    pub const R11F_G11F_B10F: c_uint = 0x8C3A;
    pub const R11F_G11F_B10F_EXT: c_uint = 0x8C3A;
    pub const R16: c_uint = 0x822A;
    pub const R16F: c_uint = 0x822D;
    pub const R16F_EXT: c_uint = 0x822D;
    pub const R16I: c_uint = 0x8233;
    pub const R16UI: c_uint = 0x8234;
    pub const R16_SNORM: c_uint = 0x8F98;
    pub const R1UI_C3F_V3F_SUN: c_uint = 0x85C6;
    pub const R1UI_C4F_N3F_V3F_SUN: c_uint = 0x85C8;
    pub const R1UI_C4UB_V3F_SUN: c_uint = 0x85C5;
    pub const R1UI_N3F_V3F_SUN: c_uint = 0x85C7;
    pub const R1UI_T2F_C4F_N3F_V3F_SUN: c_uint = 0x85CB;
    pub const R1UI_T2F_N3F_V3F_SUN: c_uint = 0x85CA;
    pub const R1UI_T2F_V3F_SUN: c_uint = 0x85C9;
    pub const R1UI_V3F_SUN: c_uint = 0x85C4;
    pub const R32F: c_uint = 0x822E;
    pub const R32F_EXT: c_uint = 0x822E;
    pub const R32I: c_uint = 0x8235;
    pub const R32UI: c_uint = 0x8236;
    pub const R3_G3_B2: c_uint = 0x2A10;
    pub const R8: c_uint = 0x8229;
    pub const R8I: c_uint = 0x8231;
    pub const R8UI: c_uint = 0x8232;
    pub const R8_EXT: c_uint = 0x8229;
    pub const R8_SNORM: c_uint = 0x8F94;
    pub const RASTERIZER_DISCARD: c_uint = 0x8C89;
    pub const RASTERIZER_DISCARD_EXT: c_uint = 0x8C89;
    pub const RASTERIZER_DISCARD_NV: c_uint = 0x8C89;
    pub const RASTER_FIXED_SAMPLE_LOCATIONS_EXT: c_uint = 0x932A;
    pub const RASTER_MULTISAMPLE_EXT: c_uint = 0x9327;
    pub const RASTER_POSITION_UNCLIPPED_IBM: c_uint = 0x19262;
    pub const RASTER_SAMPLES_EXT: c_uint = 0x9328;
    pub const READ_BUFFER: c_uint = 0x0C02;
    pub const READ_FRAMEBUFFER: c_uint = 0x8CA8;
    pub const READ_FRAMEBUFFER_BINDING: c_uint = 0x8CAA;
    pub const READ_FRAMEBUFFER_BINDING_EXT: c_uint = 0x8CAA;
    pub const READ_FRAMEBUFFER_EXT: c_uint = 0x8CA8;
    pub const READ_ONLY: c_uint = 0x88B8;
    pub const READ_ONLY_ARB: c_uint = 0x88B8;
    pub const READ_PIXELS: c_uint = 0x828C;
    pub const READ_PIXELS_FORMAT: c_uint = 0x828D;
    pub const READ_PIXELS_TYPE: c_uint = 0x828E;
    pub const READ_PIXEL_DATA_RANGE_LENGTH_NV: c_uint = 0x887B;
    pub const READ_PIXEL_DATA_RANGE_NV: c_uint = 0x8879;
    pub const READ_PIXEL_DATA_RANGE_POINTER_NV: c_uint = 0x887D;
    pub const READ_WRITE: c_uint = 0x88BA;
    pub const READ_WRITE_ARB: c_uint = 0x88BA;
    pub const RECLAIM_MEMORY_HINT_PGI: c_uint = 0x1A1FE;
    pub const RECT_NV: c_uint = 0xF6;
    pub const RED: c_uint = 0x1903;
    pub const REDUCE_EXT: c_uint = 0x8016;
    pub const RED_BIT_ATI: c_uint = 0x00000001;
    pub const RED_INTEGER: c_uint = 0x8D94;
    pub const RED_INTEGER_EXT: c_uint = 0x8D94;
    pub const RED_MAX_CLAMP_INGR: c_uint = 0x8564;
    pub const RED_MIN_CLAMP_INGR: c_uint = 0x8560;
    pub const RED_NV: c_uint = 0x1903;
    pub const RED_SNORM: c_uint = 0x8F90;
    pub const REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x930B;
    pub const REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x930A;
    pub const REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x9309;
    pub const REFERENCED_BY_MESH_SHADER_NV: c_uint = 0x95A0;
    pub const REFERENCED_BY_TASK_SHADER_NV: c_uint = 0x95A1;
    pub const REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x9307;
    pub const REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x9308;
    pub const REFERENCED_BY_VERTEX_SHADER: c_uint = 0x9306;
    pub const REFERENCE_PLANE_EQUATION_SGIX: c_uint = 0x817E;
    pub const REFERENCE_PLANE_SGIX: c_uint = 0x817D;
    pub const REFLECTION_MAP_ARB: c_uint = 0x8512;
    pub const REFLECTION_MAP_EXT: c_uint = 0x8512;
    pub const REFLECTION_MAP_NV: c_uint = 0x8512;
    pub const REGISTER_COMBINERS_NV: c_uint = 0x8522;
    pub const REG_0_ATI: c_uint = 0x8921;
    pub const REG_10_ATI: c_uint = 0x892B;
    pub const REG_11_ATI: c_uint = 0x892C;
    pub const REG_12_ATI: c_uint = 0x892D;
    pub const REG_13_ATI: c_uint = 0x892E;
    pub const REG_14_ATI: c_uint = 0x892F;
    pub const REG_15_ATI: c_uint = 0x8930;
    pub const REG_16_ATI: c_uint = 0x8931;
    pub const REG_17_ATI: c_uint = 0x8932;
    pub const REG_18_ATI: c_uint = 0x8933;
    pub const REG_19_ATI: c_uint = 0x8934;
    pub const REG_1_ATI: c_uint = 0x8922;
    pub const REG_20_ATI: c_uint = 0x8935;
    pub const REG_21_ATI: c_uint = 0x8936;
    pub const REG_22_ATI: c_uint = 0x8937;
    pub const REG_23_ATI: c_uint = 0x8938;
    pub const REG_24_ATI: c_uint = 0x8939;
    pub const REG_25_ATI: c_uint = 0x893A;
    pub const REG_26_ATI: c_uint = 0x893B;
    pub const REG_27_ATI: c_uint = 0x893C;
    pub const REG_28_ATI: c_uint = 0x893D;
    pub const REG_29_ATI: c_uint = 0x893E;
    pub const REG_2_ATI: c_uint = 0x8923;
    pub const REG_30_ATI: c_uint = 0x893F;
    pub const REG_31_ATI: c_uint = 0x8940;
    pub const REG_3_ATI: c_uint = 0x8924;
    pub const REG_4_ATI: c_uint = 0x8925;
    pub const REG_5_ATI: c_uint = 0x8926;
    pub const REG_6_ATI: c_uint = 0x8927;
    pub const REG_7_ATI: c_uint = 0x8928;
    pub const REG_8_ATI: c_uint = 0x8929;
    pub const REG_9_ATI: c_uint = 0x892A;
    pub const RELATIVE_ARC_TO_NV: c_uint = 0xFF;
    pub const RELATIVE_CONIC_CURVE_TO_NV: c_uint = 0x1B;
    pub const RELATIVE_CUBIC_CURVE_TO_NV: c_uint = 0x0D;
    pub const RELATIVE_HORIZONTAL_LINE_TO_NV: c_uint = 0x07;
    pub const RELATIVE_LARGE_CCW_ARC_TO_NV: c_uint = 0x17;
    pub const RELATIVE_LARGE_CW_ARC_TO_NV: c_uint = 0x19;
    pub const RELATIVE_LINE_TO_NV: c_uint = 0x05;
    pub const RELATIVE_MOVE_TO_NV: c_uint = 0x03;
    pub const RELATIVE_QUADRATIC_CURVE_TO_NV: c_uint = 0x0B;
    pub const RELATIVE_RECT_NV: c_uint = 0xF7;
    pub const RELATIVE_ROUNDED_RECT2_NV: c_uint = 0xEB;
    pub const RELATIVE_ROUNDED_RECT4_NV: c_uint = 0xED;
    pub const RELATIVE_ROUNDED_RECT8_NV: c_uint = 0xEF;
    pub const RELATIVE_ROUNDED_RECT_NV: c_uint = 0xE9;
    pub const RELATIVE_SMALL_CCW_ARC_TO_NV: c_uint = 0x13;
    pub const RELATIVE_SMALL_CW_ARC_TO_NV: c_uint = 0x15;
    pub const RELATIVE_SMOOTH_CUBIC_CURVE_TO_NV: c_uint = 0x11;
    pub const RELATIVE_SMOOTH_QUADRATIC_CURVE_TO_NV: c_uint = 0x0F;
    pub const RELATIVE_VERTICAL_LINE_TO_NV: c_uint = 0x09;
    pub const RELEASED_APPLE: c_uint = 0x8A19;
    pub const RENDERBUFFER: c_uint = 0x8D41;
    pub const RENDERBUFFER_ALPHA_SIZE: c_uint = 0x8D53;
    pub const RENDERBUFFER_ALPHA_SIZE_EXT: c_uint = 0x8D53;
    pub const RENDERBUFFER_BINDING: c_uint = 0x8CA7;
    pub const RENDERBUFFER_BINDING_EXT: c_uint = 0x8CA7;
    pub const RENDERBUFFER_BLUE_SIZE: c_uint = 0x8D52;
    pub const RENDERBUFFER_BLUE_SIZE_EXT: c_uint = 0x8D52;
    pub const RENDERBUFFER_COLOR_SAMPLES_NV: c_uint = 0x8E10;
    pub const RENDERBUFFER_COVERAGE_SAMPLES_NV: c_uint = 0x8CAB;
    pub const RENDERBUFFER_DEPTH_SIZE: c_uint = 0x8D54;
    pub const RENDERBUFFER_DEPTH_SIZE_EXT: c_uint = 0x8D54;
    pub const RENDERBUFFER_EXT: c_uint = 0x8D41;
    pub const RENDERBUFFER_FREE_MEMORY_ATI: c_uint = 0x87FD;
    pub const RENDERBUFFER_GREEN_SIZE: c_uint = 0x8D51;
    pub const RENDERBUFFER_GREEN_SIZE_EXT: c_uint = 0x8D51;
    pub const RENDERBUFFER_HEIGHT: c_uint = 0x8D43;
    pub const RENDERBUFFER_HEIGHT_EXT: c_uint = 0x8D43;
    pub const RENDERBUFFER_INTERNAL_FORMAT: c_uint = 0x8D44;
    pub const RENDERBUFFER_INTERNAL_FORMAT_EXT: c_uint = 0x8D44;
    pub const RENDERBUFFER_RED_SIZE: c_uint = 0x8D50;
    pub const RENDERBUFFER_RED_SIZE_EXT: c_uint = 0x8D50;
    pub const RENDERBUFFER_SAMPLES: c_uint = 0x8CAB;
    pub const RENDERBUFFER_SAMPLES_EXT: c_uint = 0x8CAB;
    pub const RENDERBUFFER_STENCIL_SIZE: c_uint = 0x8D55;
    pub const RENDERBUFFER_STENCIL_SIZE_EXT: c_uint = 0x8D55;
    pub const RENDERBUFFER_STORAGE_SAMPLES_AMD: c_uint = 0x91B2;
    pub const RENDERBUFFER_WIDTH: c_uint = 0x8D42;
    pub const RENDERBUFFER_WIDTH_EXT: c_uint = 0x8D42;
    pub const RENDERER: c_uint = 0x1F01;
    pub const RENDER_GPU_MASK_NV: c_uint = 0x9558;
    pub const REPEAT: c_uint = 0x2901;
    pub const REPLACE: c_uint = 0x1E01;
    pub const REPLACEMENT_CODE_ARRAY_POINTER_SUN: c_uint = 0x85C3;
    pub const REPLACEMENT_CODE_ARRAY_STRIDE_SUN: c_uint = 0x85C2;
    pub const REPLACEMENT_CODE_ARRAY_SUN: c_uint = 0x85C0;
    pub const REPLACEMENT_CODE_ARRAY_TYPE_SUN: c_uint = 0x85C1;
    pub const REPLACEMENT_CODE_SUN: c_uint = 0x81D8;
    pub const REPLACE_EXT: c_uint = 0x8062;
    pub const REPLACE_MIDDLE_SUN: c_uint = 0x0002;
    pub const REPLACE_OLDEST_SUN: c_uint = 0x0003;
    pub const REPLACE_VALUE_AMD: c_uint = 0x874B;
    pub const REPLICATE_BORDER_HP: c_uint = 0x8153;
    pub const REPRESENTATIVE_FRAGMENT_TEST_NV: c_uint = 0x937F;
    pub const RESAMPLE_AVERAGE_OML: c_uint = 0x8988;
    pub const RESAMPLE_DECIMATE_OML: c_uint = 0x8989;
    pub const RESAMPLE_DECIMATE_SGIX: c_uint = 0x8430;
    pub const RESAMPLE_REPLICATE_OML: c_uint = 0x8986;
    pub const RESAMPLE_REPLICATE_SGIX: c_uint = 0x8433;
    pub const RESAMPLE_ZERO_FILL_OML: c_uint = 0x8987;
    pub const RESAMPLE_ZERO_FILL_SGIX: c_uint = 0x8434;
    pub const RESCALE_NORMAL_EXT: c_uint = 0x803A;
    pub const RESET_NOTIFICATION_STRATEGY: c_uint = 0x8256;
    pub const RESET_NOTIFICATION_STRATEGY_ARB: c_uint = 0x8256;
    pub const RESTART_PATH_NV: c_uint = 0xF0;
    pub const RESTART_SUN: c_uint = 0x0001;
    pub const RETAINED_APPLE: c_uint = 0x8A1B;
    pub const RG: c_uint = 0x8227;
    pub const RG16: c_uint = 0x822C;
    pub const RG16F: c_uint = 0x822F;
    pub const RG16F_EXT: c_uint = 0x822F;
    pub const RG16I: c_uint = 0x8239;
    pub const RG16UI: c_uint = 0x823A;
    pub const RG16_SNORM: c_uint = 0x8F99;
    pub const RG32F: c_uint = 0x8230;
    pub const RG32F_EXT: c_uint = 0x8230;
    pub const RG32I: c_uint = 0x823B;
    pub const RG32UI: c_uint = 0x823C;
    pub const RG8: c_uint = 0x822B;
    pub const RG8I: c_uint = 0x8237;
    pub const RG8UI: c_uint = 0x8238;
    pub const RG8_EXT: c_uint = 0x822B;
    pub const RG8_SNORM: c_uint = 0x8F95;
    pub const RGB: c_uint = 0x1907;
    pub const RGB10: c_uint = 0x8052;
    pub const RGB10_A2: c_uint = 0x8059;
    pub const RGB10_A2UI: c_uint = 0x906F;
    pub const RGB10_A2_EXT: c_uint = 0x8059;
    pub const RGB10_EXT: c_uint = 0x8052;
    pub const RGB12: c_uint = 0x8053;
    pub const RGB12_EXT: c_uint = 0x8053;
    pub const RGB16: c_uint = 0x8054;
    pub const RGB16F: c_uint = 0x881B;
    pub const RGB16F_ARB: c_uint = 0x881B;
    pub const RGB16F_EXT: c_uint = 0x881B;
    pub const RGB16I: c_uint = 0x8D89;
    pub const RGB16I_EXT: c_uint = 0x8D89;
    pub const RGB16UI: c_uint = 0x8D77;
    pub const RGB16UI_EXT: c_uint = 0x8D77;
    pub const RGB16_EXT: c_uint = 0x8054;
    pub const RGB16_SNORM: c_uint = 0x8F9A;
    pub const RGB2_EXT: c_uint = 0x804E;
    pub const RGB32F: c_uint = 0x8815;
    pub const RGB32F_ARB: c_uint = 0x8815;
    pub const RGB32F_EXT: c_uint = 0x8815;
    pub const RGB32I: c_uint = 0x8D83;
    pub const RGB32I_EXT: c_uint = 0x8D83;
    pub const RGB32UI: c_uint = 0x8D71;
    pub const RGB32UI_EXT: c_uint = 0x8D71;
    pub const RGB4: c_uint = 0x804F;
    pub const RGB4_EXT: c_uint = 0x804F;
    pub const RGB4_S3TC: c_uint = 0x83A1;
    pub const RGB5: c_uint = 0x8050;
    pub const RGB565: c_uint = 0x8D62;
    pub const RGB5_A1: c_uint = 0x8057;
    pub const RGB5_A1_EXT: c_uint = 0x8057;
    pub const RGB5_EXT: c_uint = 0x8050;
    pub const RGB8: c_uint = 0x8051;
    pub const RGB8I: c_uint = 0x8D8F;
    pub const RGB8I_EXT: c_uint = 0x8D8F;
    pub const RGB8UI: c_uint = 0x8D7D;
    pub const RGB8UI_EXT: c_uint = 0x8D7D;
    pub const RGB8_EXT: c_uint = 0x8051;
    pub const RGB8_SNORM: c_uint = 0x8F96;
    pub const RGB9_E5: c_uint = 0x8C3D;
    pub const RGB9_E5_EXT: c_uint = 0x8C3D;
    pub const RGBA: c_uint = 0x1908;
    pub const RGBA12: c_uint = 0x805A;
    pub const RGBA12_EXT: c_uint = 0x805A;
    pub const RGBA16: c_uint = 0x805B;
    pub const RGBA16F: c_uint = 0x881A;
    pub const RGBA16F_ARB: c_uint = 0x881A;
    pub const RGBA16F_EXT: c_uint = 0x881A;
    pub const RGBA16I: c_uint = 0x8D88;
    pub const RGBA16I_EXT: c_uint = 0x8D88;
    pub const RGBA16UI: c_uint = 0x8D76;
    pub const RGBA16UI_EXT: c_uint = 0x8D76;
    pub const RGBA16_EXT: c_uint = 0x805B;
    pub const RGBA16_SNORM: c_uint = 0x8F9B;
    pub const RGBA2: c_uint = 0x8055;
    pub const RGBA2_EXT: c_uint = 0x8055;
    pub const RGBA32F: c_uint = 0x8814;
    pub const RGBA32F_ARB: c_uint = 0x8814;
    pub const RGBA32F_EXT: c_uint = 0x8814;
    pub const RGBA32I: c_uint = 0x8D82;
    pub const RGBA32I_EXT: c_uint = 0x8D82;
    pub const RGBA32UI: c_uint = 0x8D70;
    pub const RGBA32UI_EXT: c_uint = 0x8D70;
    pub const RGBA4: c_uint = 0x8056;
    pub const RGBA4_DXT5_S3TC: c_uint = 0x83A5;
    pub const RGBA4_EXT: c_uint = 0x8056;
    pub const RGBA4_S3TC: c_uint = 0x83A3;
    pub const RGBA8: c_uint = 0x8058;
    pub const RGBA8I: c_uint = 0x8D8E;
    pub const RGBA8I_EXT: c_uint = 0x8D8E;
    pub const RGBA8UI: c_uint = 0x8D7C;
    pub const RGBA8UI_EXT: c_uint = 0x8D7C;
    pub const RGBA8_EXT: c_uint = 0x8058;
    pub const RGBA8_SNORM: c_uint = 0x8F97;
    pub const RGBA_DXT5_S3TC: c_uint = 0x83A4;
    pub const RGBA_FLOAT16_APPLE: c_uint = 0x881A;
    pub const RGBA_FLOAT16_ATI: c_uint = 0x881A;
    pub const RGBA_FLOAT32_APPLE: c_uint = 0x8814;
    pub const RGBA_FLOAT32_ATI: c_uint = 0x8814;
    pub const RGBA_FLOAT_MODE_ARB: c_uint = 0x8820;
    pub const RGBA_FLOAT_MODE_ATI: c_uint = 0x8820;
    pub const RGBA_INTEGER: c_uint = 0x8D99;
    pub const RGBA_INTEGER_EXT: c_uint = 0x8D99;
    pub const RGBA_INTEGER_MODE_EXT: c_uint = 0x8D9E;
    pub const RGBA_S3TC: c_uint = 0x83A2;
    pub const RGBA_SIGNED_COMPONENTS_EXT: c_uint = 0x8C3C;
    pub const RGBA_SNORM: c_uint = 0x8F93;
    pub const RGBA_UNSIGNED_DOT_PRODUCT_MAPPING_NV: c_uint = 0x86D9;
    pub const RGB_422_APPLE: c_uint = 0x8A1F;
    pub const RGB_FLOAT16_APPLE: c_uint = 0x881B;
    pub const RGB_FLOAT16_ATI: c_uint = 0x881B;
    pub const RGB_FLOAT32_APPLE: c_uint = 0x8815;
    pub const RGB_FLOAT32_ATI: c_uint = 0x8815;
    pub const RGB_INTEGER: c_uint = 0x8D98;
    pub const RGB_INTEGER_EXT: c_uint = 0x8D98;
    pub const RGB_RAW_422_APPLE: c_uint = 0x8A51;
    pub const RGB_S3TC: c_uint = 0x83A0;
    pub const RGB_SCALE_ARB: c_uint = 0x8573;
    pub const RGB_SCALE_EXT: c_uint = 0x8573;
    pub const RGB_SNORM: c_uint = 0x8F92;
    pub const RG_INTEGER: c_uint = 0x8228;
    pub const RG_SNORM: c_uint = 0x8F91;
    pub const RIGHT: c_uint = 0x0407;
    pub const ROUNDED_RECT2_NV: c_uint = 0xEA;
    pub const ROUNDED_RECT4_NV: c_uint = 0xEC;
    pub const ROUNDED_RECT8_NV: c_uint = 0xEE;
    pub const ROUNDED_RECT_NV: c_uint = 0xE8;
    pub const ROUND_NV: c_uint = 0x90A4;
    pub const SAMPLER: c_uint = 0x82E6;
    pub const SAMPLER_1D: c_uint = 0x8B5D;
    pub const SAMPLER_1D_ARB: c_uint = 0x8B5D;
    pub const SAMPLER_1D_ARRAY: c_uint = 0x8DC0;
    pub const SAMPLER_1D_ARRAY_EXT: c_uint = 0x8DC0;
    pub const SAMPLER_1D_ARRAY_SHADOW: c_uint = 0x8DC3;
    pub const SAMPLER_1D_ARRAY_SHADOW_EXT: c_uint = 0x8DC3;
    pub const SAMPLER_1D_SHADOW: c_uint = 0x8B61;
    pub const SAMPLER_1D_SHADOW_ARB: c_uint = 0x8B61;
    pub const SAMPLER_2D: c_uint = 0x8B5E;
    pub const SAMPLER_2D_ARB: c_uint = 0x8B5E;
    pub const SAMPLER_2D_ARRAY: c_uint = 0x8DC1;
    pub const SAMPLER_2D_ARRAY_EXT: c_uint = 0x8DC1;
    pub const SAMPLER_2D_ARRAY_SHADOW: c_uint = 0x8DC4;
    pub const SAMPLER_2D_ARRAY_SHADOW_EXT: c_uint = 0x8DC4;
    pub const SAMPLER_2D_MULTISAMPLE: c_uint = 0x9108;
    pub const SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910B;
    pub const SAMPLER_2D_RECT: c_uint = 0x8B63;
    pub const SAMPLER_2D_RECT_ARB: c_uint = 0x8B63;
    pub const SAMPLER_2D_RECT_SHADOW: c_uint = 0x8B64;
    pub const SAMPLER_2D_RECT_SHADOW_ARB: c_uint = 0x8B64;
    pub const SAMPLER_2D_SHADOW: c_uint = 0x8B62;
    pub const SAMPLER_2D_SHADOW_ARB: c_uint = 0x8B62;
    pub const SAMPLER_3D: c_uint = 0x8B5F;
    pub const SAMPLER_3D_ARB: c_uint = 0x8B5F;
    pub const SAMPLER_BINDING: c_uint = 0x8919;
    pub const SAMPLER_BUFFER: c_uint = 0x8DC2;
    pub const SAMPLER_BUFFER_AMD: c_uint = 0x9001;
    pub const SAMPLER_BUFFER_EXT: c_uint = 0x8DC2;
    pub const SAMPLER_CUBE: c_uint = 0x8B60;
    pub const SAMPLER_CUBE_ARB: c_uint = 0x8B60;
    pub const SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900C;
    pub const SAMPLER_CUBE_MAP_ARRAY_ARB: c_uint = 0x900C;
    pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: c_uint = 0x900D;
    pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW_ARB: c_uint = 0x900D;
    pub const SAMPLER_CUBE_SHADOW: c_uint = 0x8DC5;
    pub const SAMPLER_CUBE_SHADOW_EXT: c_uint = 0x8DC5;
    pub const SAMPLER_OBJECT_AMD: c_uint = 0x9155;
    pub const SAMPLER_RENDERBUFFER_NV: c_uint = 0x8E56;
    pub const SAMPLES: c_uint = 0x80A9;
    pub const SAMPLES_3DFX: c_uint = 0x86B4;
    pub const SAMPLES_ARB: c_uint = 0x80A9;
    pub const SAMPLES_EXT: c_uint = 0x80A9;
    pub const SAMPLES_PASSED: c_uint = 0x8914;
    pub const SAMPLES_PASSED_ARB: c_uint = 0x8914;
    pub const SAMPLES_SGIS: c_uint = 0x80A9;
    pub const SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E;
    pub const SAMPLE_ALPHA_TO_COVERAGE_ARB: c_uint = 0x809E;
    pub const SAMPLE_ALPHA_TO_MASK_EXT: c_uint = 0x809E;
    pub const SAMPLE_ALPHA_TO_MASK_SGIS: c_uint = 0x809E;
    pub const SAMPLE_ALPHA_TO_ONE: c_uint = 0x809F;
    pub const SAMPLE_ALPHA_TO_ONE_ARB: c_uint = 0x809F;
    pub const SAMPLE_ALPHA_TO_ONE_EXT: c_uint = 0x809F;
    pub const SAMPLE_ALPHA_TO_ONE_SGIS: c_uint = 0x809F;
    pub const SAMPLE_BUFFERS: c_uint = 0x80A8;
    pub const SAMPLE_BUFFERS_3DFX: c_uint = 0x86B3;
    pub const SAMPLE_BUFFERS_ARB: c_uint = 0x80A8;
    pub const SAMPLE_BUFFERS_EXT: c_uint = 0x80A8;
    pub const SAMPLE_BUFFERS_SGIS: c_uint = 0x80A8;
    pub const SAMPLE_COVERAGE: c_uint = 0x80A0;
    pub const SAMPLE_COVERAGE_ARB: c_uint = 0x80A0;
    pub const SAMPLE_COVERAGE_INVERT: c_uint = 0x80AB;
    pub const SAMPLE_COVERAGE_INVERT_ARB: c_uint = 0x80AB;
    pub const SAMPLE_COVERAGE_VALUE: c_uint = 0x80AA;
    pub const SAMPLE_COVERAGE_VALUE_ARB: c_uint = 0x80AA;
    pub const SAMPLE_LOCATION_ARB: c_uint = 0x8E50;
    pub const SAMPLE_LOCATION_NV: c_uint = 0x8E50;
    pub const SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_ARB: c_uint = 0x933F;
    pub const SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_NV: c_uint = 0x933F;
    pub const SAMPLE_LOCATION_PIXEL_GRID_WIDTH_ARB: c_uint = 0x933E;
    pub const SAMPLE_LOCATION_PIXEL_GRID_WIDTH_NV: c_uint = 0x933E;
    pub const SAMPLE_LOCATION_SUBPIXEL_BITS_ARB: c_uint = 0x933D;
    pub const SAMPLE_LOCATION_SUBPIXEL_BITS_NV: c_uint = 0x933D;
    pub const SAMPLE_MASK: c_uint = 0x8E51;
    pub const SAMPLE_MASK_EXT: c_uint = 0x80A0;
    pub const SAMPLE_MASK_INVERT_EXT: c_uint = 0x80AB;
    pub const SAMPLE_MASK_INVERT_SGIS: c_uint = 0x80AB;
    pub const SAMPLE_MASK_NV: c_uint = 0x8E51;
    pub const SAMPLE_MASK_SGIS: c_uint = 0x80A0;
    pub const SAMPLE_MASK_VALUE: c_uint = 0x8E52;
    pub const SAMPLE_MASK_VALUE_EXT: c_uint = 0x80AA;
    pub const SAMPLE_MASK_VALUE_NV: c_uint = 0x8E52;
    pub const SAMPLE_MASK_VALUE_SGIS: c_uint = 0x80AA;
    pub const SAMPLE_PATTERN_EXT: c_uint = 0x80AC;
    pub const SAMPLE_PATTERN_SGIS: c_uint = 0x80AC;
    pub const SAMPLE_POSITION: c_uint = 0x8E50;
    pub const SAMPLE_POSITION_NV: c_uint = 0x8E50;
    pub const SAMPLE_SHADING: c_uint = 0x8C36;
    pub const SAMPLE_SHADING_ARB: c_uint = 0x8C36;
    pub const SATURATE_BIT_ATI: c_uint = 0x00000040;
    pub const SCALAR_EXT: c_uint = 0x87BE;
    pub const SCALEBIAS_HINT_SGIX: c_uint = 0x8322;
    pub const SCALED_RESOLVE_FASTEST_EXT: c_uint = 0x90BA;
    pub const SCALED_RESOLVE_NICEST_EXT: c_uint = 0x90BB;
    pub const SCALE_BY_FOUR_NV: c_uint = 0x853F;
    pub const SCALE_BY_ONE_HALF_NV: c_uint = 0x8540;
    pub const SCALE_BY_TWO_NV: c_uint = 0x853E;
    pub const SCISSOR_BOX: c_uint = 0x0C10;
    pub const SCISSOR_BOX_EXCLUSIVE_NV: c_uint = 0x9556;
    pub const SCISSOR_COMMAND_NV: c_uint = 0x0011;
    pub const SCISSOR_TEST: c_uint = 0x0C11;
    pub const SCISSOR_TEST_EXCLUSIVE_NV: c_uint = 0x9555;
    pub const SCREEN_COORDINATES_REND: c_uint = 0x8490;
    pub const SCREEN_KHR: c_uint = 0x9295;
    pub const SCREEN_NV: c_uint = 0x9295;
    pub const SECONDARY_COLOR_ARRAY_ADDRESS_NV: c_uint = 0x8F27;
    pub const SECONDARY_COLOR_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889C;
    pub const SECONDARY_COLOR_ARRAY_EXT: c_uint = 0x845E;
    pub const SECONDARY_COLOR_ARRAY_LENGTH_NV: c_uint = 0x8F31;
    pub const SECONDARY_COLOR_ARRAY_LIST_IBM: c_uint = 103077;
    pub const SECONDARY_COLOR_ARRAY_LIST_STRIDE_IBM: c_uint = 103087;
    pub const SECONDARY_COLOR_ARRAY_POINTER_EXT: c_uint = 0x845D;
    pub const SECONDARY_COLOR_ARRAY_SIZE_EXT: c_uint = 0x845A;
    pub const SECONDARY_COLOR_ARRAY_STRIDE_EXT: c_uint = 0x845C;
    pub const SECONDARY_COLOR_ARRAY_TYPE_EXT: c_uint = 0x845B;
    pub const SECONDARY_COLOR_NV: c_uint = 0x852D;
    pub const SECONDARY_INTERPOLATOR_ATI: c_uint = 0x896D;
    pub const SEMAPHORE_TYPE_BINARY_NV: c_uint = 0x95B4;
    pub const SEMAPHORE_TYPE_NV: c_uint = 0x95B3;
    pub const SEMAPHORE_TYPE_TIMELINE_NV: c_uint = 0x95B5;
    pub const SEPARABLE_2D_EXT: c_uint = 0x8012;
    pub const SEPARATE_ATTRIBS: c_uint = 0x8C8D;
    pub const SEPARATE_ATTRIBS_EXT: c_uint = 0x8C8D;
    pub const SEPARATE_ATTRIBS_NV: c_uint = 0x8C8D;
    pub const SEPARATE_SPECULAR_COLOR_EXT: c_uint = 0x81FA;
    pub const SET: c_uint = 0x150F;
    pub const SET_AMD: c_uint = 0x874A;
    pub const SHADER: c_uint = 0x82E1;
    pub const SHADER_BINARY_FORMATS: c_uint = 0x8DF8;
    pub const SHADER_BINARY_FORMAT_SPIR_V: c_uint = 0x9551;
    pub const SHADER_BINARY_FORMAT_SPIR_V_ARB: c_uint = 0x9551;
    pub const SHADER_COMPILER: c_uint = 0x8DFA;
    pub const SHADER_CONSISTENT_NV: c_uint = 0x86DD;
    pub const SHADER_GLOBAL_ACCESS_BARRIER_BIT_NV: c_uint = 0x00000010;
    pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: c_uint = 0x00000020;
    pub const SHADER_IMAGE_ACCESS_BARRIER_BIT_EXT: c_uint = 0x00000020;
    pub const SHADER_IMAGE_ATOMIC: c_uint = 0x82A6;
    pub const SHADER_IMAGE_LOAD: c_uint = 0x82A4;
    pub const SHADER_IMAGE_STORE: c_uint = 0x82A5;
    pub const SHADER_INCLUDE_ARB: c_uint = 0x8DAE;
    pub const SHADER_OBJECT_ARB: c_uint = 0x8B48;
    pub const SHADER_OBJECT_EXT: c_uint = 0x8B48;
    pub const SHADER_OPERATION_NV: c_uint = 0x86DF;
    pub const SHADER_SOURCE_LENGTH: c_uint = 0x8B88;
    pub const SHADER_STORAGE_BARRIER_BIT: c_uint = 0x00002000;
    pub const SHADER_STORAGE_BLOCK: c_uint = 0x92E6;
    pub const SHADER_STORAGE_BUFFER: c_uint = 0x90D2;
    pub const SHADER_STORAGE_BUFFER_BINDING: c_uint = 0x90D3;
    pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x90DF;
    pub const SHADER_STORAGE_BUFFER_SIZE: c_uint = 0x90D5;
    pub const SHADER_STORAGE_BUFFER_START: c_uint = 0x90D4;
    pub const SHADER_TYPE: c_uint = 0x8B4F;
    pub const SHADING_LANGUAGE_VERSION: c_uint = 0x8B8C;
    pub const SHADING_LANGUAGE_VERSION_ARB: c_uint = 0x8B8C;
    pub const SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: c_uint = 0x956F;
    pub const SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: c_uint = 0x9566;
    pub const SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: c_uint = 0x9567;
    pub const SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: c_uint = 0x9568;
    pub const SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: c_uint = 0x9569;
    pub const SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: c_uint = 0x956A;
    pub const SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: c_uint = 0x956B;
    pub const SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: c_uint = 0x9565;
    pub const SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: c_uint = 0x956C;
    pub const SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: c_uint = 0x956D;
    pub const SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: c_uint = 0x956E;
    pub const SHADING_RATE_IMAGE_BINDING_NV: c_uint = 0x955B;
    pub const SHADING_RATE_IMAGE_NV: c_uint = 0x9563;
    pub const SHADING_RATE_IMAGE_PALETTE_COUNT_NV: c_uint = 0x95B2;
    pub const SHADING_RATE_IMAGE_PALETTE_SIZE_NV: c_uint = 0x955E;
    pub const SHADING_RATE_IMAGE_PER_PRIMITIVE_NV: c_uint = 0x95B1;
    pub const SHADING_RATE_IMAGE_TEXEL_HEIGHT_NV: c_uint = 0x955D;
    pub const SHADING_RATE_IMAGE_TEXEL_WIDTH_NV: c_uint = 0x955C;
    pub const SHADING_RATE_NO_INVOCATIONS_NV: c_uint = 0x9564;
    pub const SHADING_RATE_SAMPLE_ORDER_DEFAULT_NV: c_uint = 0x95AE;
    pub const SHADING_RATE_SAMPLE_ORDER_PIXEL_MAJOR_NV: c_uint = 0x95AF;
    pub const SHADING_RATE_SAMPLE_ORDER_SAMPLE_MAJOR_NV: c_uint = 0x95B0;
    pub const SHADOW_AMBIENT_SGIX: c_uint = 0x80BF;
    pub const SHADOW_ATTENUATION_EXT: c_uint = 0x834E;
    pub const SHARED_EDGE_NV: c_uint = 0xC0;
    pub const SHARED_TEXTURE_PALETTE_EXT: c_uint = 0x81FB;
    pub const SHARPEN_TEXTURE_FUNC_POINTS_SGIS: c_uint = 0x80B0;
    pub const SHORT: c_uint = 0x1402;
    pub const SIGNALED: c_uint = 0x9119;
    pub const SIGNED_ALPHA8_NV: c_uint = 0x8706;
    pub const SIGNED_ALPHA_NV: c_uint = 0x8705;
    pub const SIGNED_HILO16_NV: c_uint = 0x86FA;
    pub const SIGNED_HILO8_NV: c_uint = 0x885F;
    pub const SIGNED_HILO_NV: c_uint = 0x86F9;
    pub const SIGNED_IDENTITY_NV: c_uint = 0x853C;
    pub const SIGNED_INTENSITY8_NV: c_uint = 0x8708;
    pub const SIGNED_INTENSITY_NV: c_uint = 0x8707;
    pub const SIGNED_LUMINANCE8_ALPHA8_NV: c_uint = 0x8704;
    pub const SIGNED_LUMINANCE8_NV: c_uint = 0x8702;
    pub const SIGNED_LUMINANCE_ALPHA_NV: c_uint = 0x8703;
    pub const SIGNED_LUMINANCE_NV: c_uint = 0x8701;
    pub const SIGNED_NEGATE_NV: c_uint = 0x853D;
    pub const SIGNED_NORMALIZED: c_uint = 0x8F9C;
    pub const SIGNED_RGB8_NV: c_uint = 0x86FF;
    pub const SIGNED_RGB8_UNSIGNED_ALPHA8_NV: c_uint = 0x870D;
    pub const SIGNED_RGBA8_NV: c_uint = 0x86FC;
    pub const SIGNED_RGBA_NV: c_uint = 0x86FB;
    pub const SIGNED_RGB_NV: c_uint = 0x86FE;
    pub const SIGNED_RGB_UNSIGNED_ALPHA_NV: c_uint = 0x870C;
    pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: c_uint = 0x82AC;
    pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: c_uint = 0x82AE;
    pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: c_uint = 0x82AD;
    pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: c_uint = 0x82AF;
    pub const SINGLE_COLOR_EXT: c_uint = 0x81F9;
    pub const SKIP_COMPONENTS1_NV: c_int = -6;
    pub const SKIP_COMPONENTS2_NV: c_int = -5;
    pub const SKIP_COMPONENTS3_NV: c_int = -4;
    pub const SKIP_COMPONENTS4_NV: c_int = -3;
    pub const SKIP_DECODE_EXT: c_uint = 0x8A4A;
    pub const SKIP_MISSING_GLYPH_NV: c_uint = 0x90A9;
    pub const SLICE_ACCUM_SUN: c_uint = 0x85CC;
    pub const SLUMINANCE8_ALPHA8_EXT: c_uint = 0x8C45;
    pub const SLUMINANCE8_EXT: c_uint = 0x8C47;
    pub const SLUMINANCE_ALPHA_EXT: c_uint = 0x8C44;
    pub const SLUMINANCE_EXT: c_uint = 0x8C46;
    pub const SMALL_CCW_ARC_TO_NV: c_uint = 0x12;
    pub const SMALL_CW_ARC_TO_NV: c_uint = 0x14;
    pub const SMOOTH_CUBIC_CURVE_TO_NV: c_uint = 0x10;
    pub const SMOOTH_LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
    pub const SMOOTH_LINE_WIDTH_RANGE: c_uint = 0x0B22;
    pub const SMOOTH_POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
    pub const SMOOTH_POINT_SIZE_RANGE: c_uint = 0x0B12;
    pub const SMOOTH_QUADRATIC_CURVE_TO_NV: c_uint = 0x0E;
    pub const SM_COUNT_NV: c_uint = 0x933B;
    pub const SOFTLIGHT_KHR: c_uint = 0x929C;
    pub const SOFTLIGHT_NV: c_uint = 0x929C;
    pub const SOURCE0_ALPHA_ARB: c_uint = 0x8588;
    pub const SOURCE0_ALPHA_EXT: c_uint = 0x8588;
    pub const SOURCE0_RGB_ARB: c_uint = 0x8580;
    pub const SOURCE0_RGB_EXT: c_uint = 0x8580;
    pub const SOURCE1_ALPHA: c_uint = 0x8589;
    pub const SOURCE1_ALPHA_ARB: c_uint = 0x8589;
    pub const SOURCE1_ALPHA_EXT: c_uint = 0x8589;
    pub const SOURCE1_RGB_ARB: c_uint = 0x8581;
    pub const SOURCE1_RGB_EXT: c_uint = 0x8581;
    pub const SOURCE2_ALPHA_ARB: c_uint = 0x858A;
    pub const SOURCE2_ALPHA_EXT: c_uint = 0x858A;
    pub const SOURCE2_RGB_ARB: c_uint = 0x8582;
    pub const SOURCE2_RGB_EXT: c_uint = 0x8582;
    pub const SOURCE3_ALPHA_NV: c_uint = 0x858B;
    pub const SOURCE3_RGB_NV: c_uint = 0x8583;
    pub const SPARE0_NV: c_uint = 0x852E;
    pub const SPARE0_PLUS_SECONDARY_COLOR_NV: c_uint = 0x8532;
    pub const SPARE1_NV: c_uint = 0x852F;
    pub const SPARSE_BUFFER_PAGE_SIZE_ARB: c_uint = 0x82F8;
    pub const SPARSE_STORAGE_BIT_ARB: c_uint = 0x0400;
    pub const SPARSE_TEXTURE_FULL_ARRAY_CUBE_MIPMAPS_ARB: c_uint = 0x91A9;
    pub const SPIR_V_BINARY: c_uint = 0x9552;
    pub const SPIR_V_BINARY_ARB: c_uint = 0x9552;
    pub const SPIR_V_EXTENSIONS: c_uint = 0x9553;
    pub const SPRITE_AXIAL_SGIX: c_uint = 0x814C;
    pub const SPRITE_AXIS_SGIX: c_uint = 0x814A;
    pub const SPRITE_EYE_ALIGNED_SGIX: c_uint = 0x814E;
    pub const SPRITE_MODE_SGIX: c_uint = 0x8149;
    pub const SPRITE_OBJECT_ALIGNED_SGIX: c_uint = 0x814D;
    pub const SPRITE_SGIX: c_uint = 0x8148;
    pub const SPRITE_TRANSLATION_SGIX: c_uint = 0x814B;
    pub const SQUARE_NV: c_uint = 0x90A3;
    pub const SR8_EXT: c_uint = 0x8FBD;
    pub const SRC1_ALPHA: c_uint = 0x8589;
    pub const SRC1_COLOR: c_uint = 0x88F9;
    pub const SRC_ALPHA: c_uint = 0x0302;
    pub const SRC_ALPHA_SATURATE: c_uint = 0x0308;
    pub const SRC_ATOP_NV: c_uint = 0x928E;
    pub const SRC_COLOR: c_uint = 0x0300;
    pub const SRC_IN_NV: c_uint = 0x928A;
    pub const SRC_NV: c_uint = 0x9286;
    pub const SRC_OUT_NV: c_uint = 0x928C;
    pub const SRC_OVER_NV: c_uint = 0x9288;
    pub const SRG8_EXT: c_uint = 0x8FBE;
    pub const SRGB: c_uint = 0x8C40;
    pub const SRGB8: c_uint = 0x8C41;
    pub const SRGB8_ALPHA8: c_uint = 0x8C43;
    pub const SRGB8_ALPHA8_EXT: c_uint = 0x8C43;
    pub const SRGB8_EXT: c_uint = 0x8C41;
    pub const SRGB_ALPHA: c_uint = 0x8C42;
    pub const SRGB_ALPHA_EXT: c_uint = 0x8C42;
    pub const SRGB_DECODE_ARB: c_uint = 0x8299;
    pub const SRGB_EXT: c_uint = 0x8C40;
    pub const SRGB_READ: c_uint = 0x8297;
    pub const SRGB_WRITE: c_uint = 0x8298;
    pub const STACK_OVERFLOW: c_uint = 0x0503;
    pub const STACK_UNDERFLOW: c_uint = 0x0504;
    pub const STANDARD_FONT_FORMAT_NV: c_uint = 0x936C;
    pub const STANDARD_FONT_NAME_NV: c_uint = 0x9072;
    pub const STATIC_ATI: c_uint = 0x8760;
    pub const STATIC_COPY: c_uint = 0x88E6;
    pub const STATIC_COPY_ARB: c_uint = 0x88E6;
    pub const STATIC_DRAW: c_uint = 0x88E4;
    pub const STATIC_DRAW_ARB: c_uint = 0x88E4;
    pub const STATIC_READ: c_uint = 0x88E5;
    pub const STATIC_READ_ARB: c_uint = 0x88E5;
    pub const STATIC_VERTEX_ARRAY_IBM: c_uint = 103061;
    pub const STENCIL: c_uint = 0x1802;
    pub const STENCIL_ATTACHMENT: c_uint = 0x8D20;
    pub const STENCIL_ATTACHMENT_EXT: c_uint = 0x8D20;
    pub const STENCIL_BACK_FAIL: c_uint = 0x8801;
    pub const STENCIL_BACK_FAIL_ATI: c_uint = 0x8801;
    pub const STENCIL_BACK_FUNC: c_uint = 0x8800;
    pub const STENCIL_BACK_FUNC_ATI: c_uint = 0x8800;
    pub const STENCIL_BACK_OP_VALUE_AMD: c_uint = 0x874D;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: c_uint = 0x8802;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL_ATI: c_uint = 0x8802;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: c_uint = 0x8803;
    pub const STENCIL_BACK_PASS_DEPTH_PASS_ATI: c_uint = 0x8803;
    pub const STENCIL_BACK_REF: c_uint = 0x8CA3;
    pub const STENCIL_BACK_VALUE_MASK: c_uint = 0x8CA4;
    pub const STENCIL_BACK_WRITEMASK: c_uint = 0x8CA5;
    pub const STENCIL_BUFFER_BIT: c_uint = 0x00000400;
    pub const STENCIL_CLEAR_TAG_VALUE_EXT: c_uint = 0x88F3;
    pub const STENCIL_CLEAR_VALUE: c_uint = 0x0B91;
    pub const STENCIL_COMPONENTS: c_uint = 0x8285;
    pub const STENCIL_FAIL: c_uint = 0x0B94;
    pub const STENCIL_FUNC: c_uint = 0x0B92;
    pub const STENCIL_INDEX: c_uint = 0x1901;
    pub const STENCIL_INDEX1: c_uint = 0x8D46;
    pub const STENCIL_INDEX16: c_uint = 0x8D49;
    pub const STENCIL_INDEX16_EXT: c_uint = 0x8D49;
    pub const STENCIL_INDEX1_EXT: c_uint = 0x8D46;
    pub const STENCIL_INDEX4: c_uint = 0x8D47;
    pub const STENCIL_INDEX4_EXT: c_uint = 0x8D47;
    pub const STENCIL_INDEX8: c_uint = 0x8D48;
    pub const STENCIL_INDEX8_EXT: c_uint = 0x8D48;
    pub const STENCIL_OP_VALUE_AMD: c_uint = 0x874C;
    pub const STENCIL_PASS_DEPTH_FAIL: c_uint = 0x0B95;
    pub const STENCIL_PASS_DEPTH_PASS: c_uint = 0x0B96;
    pub const STENCIL_REF: c_uint = 0x0B97;
    pub const STENCIL_REF_COMMAND_NV: c_uint = 0x000C;
    pub const STENCIL_RENDERABLE: c_uint = 0x8288;
    pub const STENCIL_SAMPLES_NV: c_uint = 0x932E;
    pub const STENCIL_TAG_BITS_EXT: c_uint = 0x88F2;
    pub const STENCIL_TEST: c_uint = 0x0B90;
    pub const STENCIL_TEST_TWO_SIDE_EXT: c_uint = 0x8910;
    pub const STENCIL_VALUE_MASK: c_uint = 0x0B93;
    pub const STENCIL_WRITEMASK: c_uint = 0x0B98;
    pub const STEREO: c_uint = 0x0C33;
    pub const STORAGE_CACHED_APPLE: c_uint = 0x85BE;
    pub const STORAGE_CLIENT_APPLE: c_uint = 0x85B4;
    pub const STORAGE_PRIVATE_APPLE: c_uint = 0x85BD;
    pub const STORAGE_SHARED_APPLE: c_uint = 0x85BF;
    pub const STREAM_COPY: c_uint = 0x88E2;
    pub const STREAM_COPY_ARB: c_uint = 0x88E2;
    pub const STREAM_DRAW: c_uint = 0x88E0;
    pub const STREAM_DRAW_ARB: c_uint = 0x88E0;
    pub const STREAM_RASTERIZATION_AMD: c_uint = 0x91A0;
    pub const STREAM_READ: c_uint = 0x88E1;
    pub const STREAM_READ_ARB: c_uint = 0x88E1;
    pub const STRICT_DEPTHFUNC_HINT_PGI: c_uint = 0x1A216;
    pub const STRICT_LIGHTING_HINT_PGI: c_uint = 0x1A217;
    pub const STRICT_SCISSOR_HINT_PGI: c_uint = 0x1A218;
    pub const SUBGROUP_FEATURE_ARITHMETIC_BIT_KHR: c_uint = 0x00000004;
    pub const SUBGROUP_FEATURE_BALLOT_BIT_KHR: c_uint = 0x00000008;
    pub const SUBGROUP_FEATURE_BASIC_BIT_KHR: c_uint = 0x00000001;
    pub const SUBGROUP_FEATURE_CLUSTERED_BIT_KHR: c_uint = 0x00000040;
    pub const SUBGROUP_FEATURE_PARTITIONED_BIT_NV: c_uint = 0x00000100;
    pub const SUBGROUP_FEATURE_QUAD_BIT_KHR: c_uint = 0x00000080;
    pub const SUBGROUP_FEATURE_SHUFFLE_BIT_KHR: c_uint = 0x00000010;
    pub const SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT_KHR: c_uint = 0x00000020;
    pub const SUBGROUP_FEATURE_VOTE_BIT_KHR: c_uint = 0x00000002;
    pub const SUBGROUP_QUAD_ALL_STAGES_KHR: c_uint = 0x9535;
    pub const SUBGROUP_SIZE_KHR: c_uint = 0x9532;
    pub const SUBGROUP_SUPPORTED_FEATURES_KHR: c_uint = 0x9534;
    pub const SUBGROUP_SUPPORTED_STAGES_KHR: c_uint = 0x9533;
    pub const SUBPIXEL_BITS: c_uint = 0x0D50;
    pub const SUBPIXEL_PRECISION_BIAS_X_BITS_NV: c_uint = 0x9347;
    pub const SUBPIXEL_PRECISION_BIAS_Y_BITS_NV: c_uint = 0x9348;
    pub const SUBSAMPLE_DISTANCE_AMD: c_uint = 0x883F;
    pub const SUBTRACT_ARB: c_uint = 0x84E7;
    pub const SUB_ATI: c_uint = 0x8965;
    pub const SUCCESS_NV: c_uint = 0x902F;
    pub const SUPERSAMPLE_SCALE_X_NV: c_uint = 0x9372;
    pub const SUPERSAMPLE_SCALE_Y_NV: c_uint = 0x9373;
    pub const SUPPORTED_MULTISAMPLE_MODES_AMD: c_uint = 0x91B7;
    pub const SURFACE_MAPPED_NV: c_uint = 0x8700;
    pub const SURFACE_REGISTERED_NV: c_uint = 0x86FD;
    pub const SURFACE_STATE_NV: c_uint = 0x86EB;
    pub const SWIZZLE_STQ_ATI: c_uint = 0x8977;
    pub const SWIZZLE_STQ_DQ_ATI: c_uint = 0x8979;
    pub const SWIZZLE_STRQ_ATI: c_uint = 0x897A;
    pub const SWIZZLE_STRQ_DQ_ATI: c_uint = 0x897B;
    pub const SWIZZLE_STR_ATI: c_uint = 0x8976;
    pub const SWIZZLE_STR_DR_ATI: c_uint = 0x8978;
    pub const SYNC_CL_EVENT_ARB: c_uint = 0x8240;
    pub const SYNC_CL_EVENT_COMPLETE_ARB: c_uint = 0x8241;
    pub const SYNC_CONDITION: c_uint = 0x9113;
    pub const SYNC_FENCE: c_uint = 0x9116;
    pub const SYNC_FLAGS: c_uint = 0x9115;
    pub const SYNC_FLUSH_COMMANDS_BIT: c_uint = 0x00000001;
    pub const SYNC_GPU_COMMANDS_COMPLETE: c_uint = 0x9117;
    pub const SYNC_STATUS: c_uint = 0x9114;
    pub const SYNC_X11_FENCE_EXT: c_uint = 0x90E1;
    pub const SYSTEM_FONT_NAME_NV: c_uint = 0x9073;
    pub const T2F_IUI_N3F_V2F_EXT: c_uint = 0x81B3;
    pub const T2F_IUI_N3F_V3F_EXT: c_uint = 0x81B4;
    pub const T2F_IUI_V2F_EXT: c_uint = 0x81B1;
    pub const T2F_IUI_V3F_EXT: c_uint = 0x81B2;
    pub const TABLE_TOO_LARGE_EXT: c_uint = 0x8031;
    pub const TANGENT_ARRAY_EXT: c_uint = 0x8439;
    pub const TANGENT_ARRAY_POINTER_EXT: c_uint = 0x8442;
    pub const TANGENT_ARRAY_STRIDE_EXT: c_uint = 0x843F;
    pub const TANGENT_ARRAY_TYPE_EXT: c_uint = 0x843E;
    pub const TASK_SHADER_BIT_NV: c_uint = 0x00000080;
    pub const TASK_SHADER_NV: c_uint = 0x955A;
    pub const TASK_SUBROUTINE_NV: c_uint = 0x957D;
    pub const TASK_SUBROUTINE_UNIFORM_NV: c_uint = 0x957F;
    pub const TASK_WORK_GROUP_SIZE_NV: c_uint = 0x953F;
    pub const TERMINATE_SEQUENCE_COMMAND_NV: c_uint = 0x0000;
    pub const TESSELLATION_FACTOR_AMD: c_uint = 0x9005;
    pub const TESSELLATION_MODE_AMD: c_uint = 0x9004;
    pub const TESS_CONTROL_OUTPUT_VERTICES: c_uint = 0x8E75;
    pub const TESS_CONTROL_PROGRAM_NV: c_uint = 0x891E;
    pub const TESS_CONTROL_PROGRAM_PARAMETER_BUFFER_NV: c_uint = 0x8C74;
    pub const TESS_CONTROL_SHADER: c_uint = 0x8E88;
    pub const TESS_CONTROL_SHADER_BIT: c_uint = 0x00000008;
    pub const TESS_CONTROL_SHADER_PATCHES: c_uint = 0x82F1;
    pub const TESS_CONTROL_SHADER_PATCHES_ARB: c_uint = 0x82F1;
    pub const TESS_CONTROL_SUBROUTINE: c_uint = 0x92E9;
    pub const TESS_CONTROL_SUBROUTINE_UNIFORM: c_uint = 0x92EF;
    pub const TESS_CONTROL_TEXTURE: c_uint = 0x829C;
    pub const TESS_EVALUATION_PROGRAM_NV: c_uint = 0x891F;
    pub const TESS_EVALUATION_PROGRAM_PARAMETER_BUFFER_NV: c_uint = 0x8C75;
    pub const TESS_EVALUATION_SHADER: c_uint = 0x8E87;
    pub const TESS_EVALUATION_SHADER_BIT: c_uint = 0x00000010;
    pub const TESS_EVALUATION_SHADER_INVOCATIONS: c_uint = 0x82F2;
    pub const TESS_EVALUATION_SHADER_INVOCATIONS_ARB: c_uint = 0x82F2;
    pub const TESS_EVALUATION_SUBROUTINE: c_uint = 0x92EA;
    pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: c_uint = 0x92F0;
    pub const TESS_EVALUATION_TEXTURE: c_uint = 0x829D;
    pub const TESS_GEN_MODE: c_uint = 0x8E76;
    pub const TESS_GEN_POINT_MODE: c_uint = 0x8E79;
    pub const TESS_GEN_SPACING: c_uint = 0x8E77;
    pub const TESS_GEN_VERTEX_ORDER: c_uint = 0x8E78;
    pub const TEXCOORD1_BIT_PGI: c_uint = 0x10000000;
    pub const TEXCOORD2_BIT_PGI: c_uint = 0x20000000;
    pub const TEXCOORD3_BIT_PGI: c_uint = 0x40000000;
    pub const TEXCOORD4_BIT_PGI: c_uint = 0x80000000;
    pub const TEXTURE: c_uint = 0x1702;
    pub const TEXTURE0: c_uint = 0x84C0;
    pub const TEXTURE0_ARB: c_uint = 0x84C0;
    pub const TEXTURE1: c_uint = 0x84C1;
    pub const TEXTURE10: c_uint = 0x84CA;
    pub const TEXTURE10_ARB: c_uint = 0x84CA;
    pub const TEXTURE11: c_uint = 0x84CB;
    pub const TEXTURE11_ARB: c_uint = 0x84CB;
    pub const TEXTURE12: c_uint = 0x84CC;
    pub const TEXTURE12_ARB: c_uint = 0x84CC;
    pub const TEXTURE13: c_uint = 0x84CD;
    pub const TEXTURE13_ARB: c_uint = 0x84CD;
    pub const TEXTURE14: c_uint = 0x84CE;
    pub const TEXTURE14_ARB: c_uint = 0x84CE;
    pub const TEXTURE15: c_uint = 0x84CF;
    pub const TEXTURE15_ARB: c_uint = 0x84CF;
    pub const TEXTURE16: c_uint = 0x84D0;
    pub const TEXTURE16_ARB: c_uint = 0x84D0;
    pub const TEXTURE17: c_uint = 0x84D1;
    pub const TEXTURE17_ARB: c_uint = 0x84D1;
    pub const TEXTURE18: c_uint = 0x84D2;
    pub const TEXTURE18_ARB: c_uint = 0x84D2;
    pub const TEXTURE19: c_uint = 0x84D3;
    pub const TEXTURE19_ARB: c_uint = 0x84D3;
    pub const TEXTURE1_ARB: c_uint = 0x84C1;
    pub const TEXTURE2: c_uint = 0x84C2;
    pub const TEXTURE20: c_uint = 0x84D4;
    pub const TEXTURE20_ARB: c_uint = 0x84D4;
    pub const TEXTURE21: c_uint = 0x84D5;
    pub const TEXTURE21_ARB: c_uint = 0x84D5;
    pub const TEXTURE22: c_uint = 0x84D6;
    pub const TEXTURE22_ARB: c_uint = 0x84D6;
    pub const TEXTURE23: c_uint = 0x84D7;
    pub const TEXTURE23_ARB: c_uint = 0x84D7;
    pub const TEXTURE24: c_uint = 0x84D8;
    pub const TEXTURE24_ARB: c_uint = 0x84D8;
    pub const TEXTURE25: c_uint = 0x84D9;
    pub const TEXTURE25_ARB: c_uint = 0x84D9;
    pub const TEXTURE26: c_uint = 0x84DA;
    pub const TEXTURE26_ARB: c_uint = 0x84DA;
    pub const TEXTURE27: c_uint = 0x84DB;
    pub const TEXTURE27_ARB: c_uint = 0x84DB;
    pub const TEXTURE28: c_uint = 0x84DC;
    pub const TEXTURE28_ARB: c_uint = 0x84DC;
    pub const TEXTURE29: c_uint = 0x84DD;
    pub const TEXTURE29_ARB: c_uint = 0x84DD;
    pub const TEXTURE2_ARB: c_uint = 0x84C2;
    pub const TEXTURE3: c_uint = 0x84C3;
    pub const TEXTURE30: c_uint = 0x84DE;
    pub const TEXTURE30_ARB: c_uint = 0x84DE;
    pub const TEXTURE31: c_uint = 0x84DF;
    pub const TEXTURE31_ARB: c_uint = 0x84DF;
    pub const TEXTURE3_ARB: c_uint = 0x84C3;
    pub const TEXTURE4: c_uint = 0x84C4;
    pub const TEXTURE4_ARB: c_uint = 0x84C4;
    pub const TEXTURE5: c_uint = 0x84C5;
    pub const TEXTURE5_ARB: c_uint = 0x84C5;
    pub const TEXTURE6: c_uint = 0x84C6;
    pub const TEXTURE6_ARB: c_uint = 0x84C6;
    pub const TEXTURE7: c_uint = 0x84C7;
    pub const TEXTURE7_ARB: c_uint = 0x84C7;
    pub const TEXTURE8: c_uint = 0x84C8;
    pub const TEXTURE8_ARB: c_uint = 0x84C8;
    pub const TEXTURE9: c_uint = 0x84C9;
    pub const TEXTURE9_ARB: c_uint = 0x84C9;
    pub const TEXTURE_1D: c_uint = 0x0DE0;
    pub const TEXTURE_1D_ARRAY: c_uint = 0x8C18;
    pub const TEXTURE_1D_ARRAY_EXT: c_uint = 0x8C18;
    pub const TEXTURE_1D_BINDING_EXT: c_uint = 0x8068;
    pub const TEXTURE_1D_STACK_BINDING_MESAX: c_uint = 0x875D;
    pub const TEXTURE_1D_STACK_MESAX: c_uint = 0x8759;
    pub const TEXTURE_2D: c_uint = 0x0DE1;
    pub const TEXTURE_2D_ARRAY: c_uint = 0x8C1A;
    pub const TEXTURE_2D_ARRAY_EXT: c_uint = 0x8C1A;
    pub const TEXTURE_2D_BINDING_EXT: c_uint = 0x8069;
    pub const TEXTURE_2D_MULTISAMPLE: c_uint = 0x9100;
    pub const TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9102;
    pub const TEXTURE_2D_STACK_BINDING_MESAX: c_uint = 0x875E;
    pub const TEXTURE_2D_STACK_MESAX: c_uint = 0x875A;
    pub const TEXTURE_3D: c_uint = 0x806F;
    pub const TEXTURE_3D_BINDING_EXT: c_uint = 0x806A;
    pub const TEXTURE_3D_EXT: c_uint = 0x806F;
    pub const TEXTURE_4DSIZE_SGIS: c_uint = 0x8136;
    pub const TEXTURE_4D_BINDING_SGIS: c_uint = 0x814F;
    pub const TEXTURE_4D_SGIS: c_uint = 0x8134;
    pub const TEXTURE_ALPHA_SIZE: c_uint = 0x805F;
    pub const TEXTURE_ALPHA_SIZE_EXT: c_uint = 0x805F;
    pub const TEXTURE_ALPHA_TYPE: c_uint = 0x8C13;
    pub const TEXTURE_ALPHA_TYPE_ARB: c_uint = 0x8C13;
    pub const TEXTURE_APPLICATION_MODE_EXT: c_uint = 0x834F;
    pub const TEXTURE_BASE_LEVEL: c_uint = 0x813C;
    pub const TEXTURE_BASE_LEVEL_SGIS: c_uint = 0x813C;
    pub const TEXTURE_BINDING_1D: c_uint = 0x8068;
    pub const TEXTURE_BINDING_1D_ARRAY: c_uint = 0x8C1C;
    pub const TEXTURE_BINDING_1D_ARRAY_EXT: c_uint = 0x8C1C;
    pub const TEXTURE_BINDING_2D: c_uint = 0x8069;
    pub const TEXTURE_BINDING_2D_ARRAY: c_uint = 0x8C1D;
    pub const TEXTURE_BINDING_2D_ARRAY_EXT: c_uint = 0x8C1D;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE: c_uint = 0x9104;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: c_uint = 0x9105;
    pub const TEXTURE_BINDING_3D: c_uint = 0x806A;
    pub const TEXTURE_BINDING_BUFFER: c_uint = 0x8C2C;
    pub const TEXTURE_BINDING_BUFFER_ARB: c_uint = 0x8C2C;
    pub const TEXTURE_BINDING_BUFFER_EXT: c_uint = 0x8C2C;
    pub const TEXTURE_BINDING_CUBE_MAP: c_uint = 0x8514;
    pub const TEXTURE_BINDING_CUBE_MAP_ARB: c_uint = 0x8514;
    pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: c_uint = 0x900A;
    pub const TEXTURE_BINDING_CUBE_MAP_ARRAY_ARB: c_uint = 0x900A;
    pub const TEXTURE_BINDING_CUBE_MAP_EXT: c_uint = 0x8514;
    pub const TEXTURE_BINDING_RECTANGLE: c_uint = 0x84F6;
    pub const TEXTURE_BINDING_RECTANGLE_ARB: c_uint = 0x84F6;
    pub const TEXTURE_BINDING_RECTANGLE_NV: c_uint = 0x84F6;
    pub const TEXTURE_BINDING_RENDERBUFFER_NV: c_uint = 0x8E53;
    pub const TEXTURE_BLUE_SIZE: c_uint = 0x805E;
    pub const TEXTURE_BLUE_SIZE_EXT: c_uint = 0x805E;
    pub const TEXTURE_BLUE_TYPE: c_uint = 0x8C12;
    pub const TEXTURE_BLUE_TYPE_ARB: c_uint = 0x8C12;
    pub const TEXTURE_BORDER_COLOR: c_uint = 0x1004;
    pub const TEXTURE_BORDER_VALUES_NV: c_uint = 0x871A;
    pub const TEXTURE_BUFFER: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_ARB: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_BINDING: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_DATA_STORE_BINDING: c_uint = 0x8C2D;
    pub const TEXTURE_BUFFER_DATA_STORE_BINDING_ARB: c_uint = 0x8C2D;
    pub const TEXTURE_BUFFER_DATA_STORE_BINDING_EXT: c_uint = 0x8C2D;
    pub const TEXTURE_BUFFER_EXT: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_FORMAT_ARB: c_uint = 0x8C2E;
    pub const TEXTURE_BUFFER_FORMAT_EXT: c_uint = 0x8C2E;
    pub const TEXTURE_BUFFER_OFFSET: c_uint = 0x919D;
    pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x919F;
    pub const TEXTURE_BUFFER_SIZE: c_uint = 0x919E;
    pub const TEXTURE_CLIPMAP_CENTER_SGIX: c_uint = 0x8171;
    pub const TEXTURE_CLIPMAP_DEPTH_SGIX: c_uint = 0x8176;
    pub const TEXTURE_CLIPMAP_FRAME_SGIX: c_uint = 0x8172;
    pub const TEXTURE_CLIPMAP_LOD_OFFSET_SGIX: c_uint = 0x8175;
    pub const TEXTURE_CLIPMAP_OFFSET_SGIX: c_uint = 0x8173;
    pub const TEXTURE_CLIPMAP_VIRTUAL_DEPTH_SGIX: c_uint = 0x8174;
    pub const TEXTURE_COLOR_SAMPLES_NV: c_uint = 0x9046;
    pub const TEXTURE_COLOR_TABLE_SGI: c_uint = 0x80BC;
    pub const TEXTURE_COLOR_WRITEMASK_SGIS: c_uint = 0x81EF;
    pub const TEXTURE_COMPARE_FAIL_VALUE_ARB: c_uint = 0x80BF;
    pub const TEXTURE_COMPARE_FUNC: c_uint = 0x884D;
    pub const TEXTURE_COMPARE_FUNC_ARB: c_uint = 0x884D;
    pub const TEXTURE_COMPARE_MODE: c_uint = 0x884C;
    pub const TEXTURE_COMPARE_MODE_ARB: c_uint = 0x884C;
    pub const TEXTURE_COMPARE_OPERATOR_SGIX: c_uint = 0x819B;
    pub const TEXTURE_COMPARE_SGIX: c_uint = 0x819A;
    pub const TEXTURE_COMPRESSED: c_uint = 0x86A1;
    pub const TEXTURE_COMPRESSED_ARB: c_uint = 0x86A1;
    pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x82B2;
    pub const TEXTURE_COMPRESSED_BLOCK_SIZE: c_uint = 0x82B3;
    pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: c_uint = 0x82B1;
    pub const TEXTURE_COMPRESSED_IMAGE_SIZE: c_uint = 0x86A0;
    pub const TEXTURE_COMPRESSED_IMAGE_SIZE_ARB: c_uint = 0x86A0;
    pub const TEXTURE_COMPRESSION_HINT: c_uint = 0x84EF;
    pub const TEXTURE_COMPRESSION_HINT_ARB: c_uint = 0x84EF;
    pub const TEXTURE_CONSTANT_DATA_SUNX: c_uint = 0x81D6;
    pub const TEXTURE_COORD_ARRAY_ADDRESS_NV: c_uint = 0x8F25;
    pub const TEXTURE_COORD_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889A;
    pub const TEXTURE_COORD_ARRAY_COUNT_EXT: c_uint = 0x808B;
    pub const TEXTURE_COORD_ARRAY_EXT: c_uint = 0x8078;
    pub const TEXTURE_COORD_ARRAY_LENGTH_NV: c_uint = 0x8F2F;
    pub const TEXTURE_COORD_ARRAY_LIST_IBM: c_uint = 103074;
    pub const TEXTURE_COORD_ARRAY_LIST_STRIDE_IBM: c_uint = 103084;
    pub const TEXTURE_COORD_ARRAY_PARALLEL_POINTERS_INTEL: c_uint = 0x83F8;
    pub const TEXTURE_COORD_ARRAY_POINTER_EXT: c_uint = 0x8092;
    pub const TEXTURE_COORD_ARRAY_SIZE_EXT: c_uint = 0x8088;
    pub const TEXTURE_COORD_ARRAY_STRIDE_EXT: c_uint = 0x808A;
    pub const TEXTURE_COORD_ARRAY_TYPE_EXT: c_uint = 0x8089;
    pub const TEXTURE_COORD_NV: c_uint = 0x8C79;
    pub const TEXTURE_COVERAGE_SAMPLES_NV: c_uint = 0x9045;
    pub const TEXTURE_CUBE_MAP: c_uint = 0x8513;
    pub const TEXTURE_CUBE_MAP_ARB: c_uint = 0x8513;
    pub const TEXTURE_CUBE_MAP_ARRAY: c_uint = 0x9009;
    pub const TEXTURE_CUBE_MAP_ARRAY_ARB: c_uint = 0x9009;
    pub const TEXTURE_CUBE_MAP_EXT: c_uint = 0x8513;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: c_uint = 0x8516;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X_ARB: c_uint = 0x8516;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X_EXT: c_uint = 0x8516;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: c_uint = 0x8518;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y_ARB: c_uint = 0x8518;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y_EXT: c_uint = 0x8518;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: c_uint = 0x851A;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z_ARB: c_uint = 0x851A;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z_EXT: c_uint = 0x851A;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: c_uint = 0x8515;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X_ARB: c_uint = 0x8515;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X_EXT: c_uint = 0x8515;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: c_uint = 0x8517;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y_ARB: c_uint = 0x8517;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y_EXT: c_uint = 0x8517;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: c_uint = 0x8519;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z_ARB: c_uint = 0x8519;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z_EXT: c_uint = 0x8519;
    pub const TEXTURE_CUBE_MAP_SEAMLESS: c_uint = 0x884F;
    pub const TEXTURE_DEFORMATION_BIT_SGIX: c_uint = 0x00000001;
    pub const TEXTURE_DEFORMATION_SGIX: c_uint = 0x8195;
    pub const TEXTURE_DEPTH: c_uint = 0x8071;
    pub const TEXTURE_DEPTH_EXT: c_uint = 0x8071;
    pub const TEXTURE_DEPTH_SIZE: c_uint = 0x884A;
    pub const TEXTURE_DEPTH_SIZE_ARB: c_uint = 0x884A;
    pub const TEXTURE_DEPTH_TYPE: c_uint = 0x8C16;
    pub const TEXTURE_DEPTH_TYPE_ARB: c_uint = 0x8C16;
    pub const TEXTURE_DS_SIZE_NV: c_uint = 0x871D;
    pub const TEXTURE_DT_SIZE_NV: c_uint = 0x871E;
    pub const TEXTURE_ENV_BIAS_SGIX: c_uint = 0x80BE;
    pub const TEXTURE_FETCH_BARRIER_BIT: c_uint = 0x00000008;
    pub const TEXTURE_FETCH_BARRIER_BIT_EXT: c_uint = 0x00000008;
    pub const TEXTURE_FILTER4_SIZE_SGIS: c_uint = 0x8147;
    pub const TEXTURE_FILTER_CONTROL_EXT: c_uint = 0x8500;
    pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: c_uint = 0x9107;
    pub const TEXTURE_FLOAT_COMPONENTS_NV: c_uint = 0x888C;
    pub const TEXTURE_FREE_MEMORY_ATI: c_uint = 0x87FC;
    pub const TEXTURE_GATHER: c_uint = 0x82A2;
    pub const TEXTURE_GATHER_SHADOW: c_uint = 0x82A3;
    pub const TEXTURE_GEQUAL_R_SGIX: c_uint = 0x819D;
    pub const TEXTURE_GREEN_SIZE: c_uint = 0x805D;
    pub const TEXTURE_GREEN_SIZE_EXT: c_uint = 0x805D;
    pub const TEXTURE_GREEN_TYPE: c_uint = 0x8C11;
    pub const TEXTURE_GREEN_TYPE_ARB: c_uint = 0x8C11;
    pub const TEXTURE_HEIGHT: c_uint = 0x1001;
    pub const TEXTURE_HI_SIZE_NV: c_uint = 0x871B;
    pub const TEXTURE_IMAGE_FORMAT: c_uint = 0x828F;
    pub const TEXTURE_IMAGE_TYPE: c_uint = 0x8290;
    pub const TEXTURE_IMMUTABLE_FORMAT: c_uint = 0x912F;
    pub const TEXTURE_IMMUTABLE_FORMAT_EXT: c_uint = 0x912F;
    pub const TEXTURE_IMMUTABLE_LEVELS: c_uint = 0x82DF;
    pub const TEXTURE_INDEX_SIZE_EXT: c_uint = 0x80ED;
    pub const TEXTURE_INTENSITY_SIZE_EXT: c_uint = 0x8061;
    pub const TEXTURE_INTENSITY_TYPE_ARB: c_uint = 0x8C15;
    pub const TEXTURE_INTERNAL_FORMAT: c_uint = 0x1003;
    pub const TEXTURE_LEQUAL_R_SGIX: c_uint = 0x819C;
    pub const TEXTURE_LIGHTING_MODE_HP: c_uint = 0x8167;
    pub const TEXTURE_LIGHT_EXT: c_uint = 0x8350;
    pub const TEXTURE_LOD_BIAS: c_uint = 0x8501;
    pub const TEXTURE_LOD_BIAS_EXT: c_uint = 0x8501;
    pub const TEXTURE_LOD_BIAS_R_SGIX: c_uint = 0x8190;
    pub const TEXTURE_LOD_BIAS_S_SGIX: c_uint = 0x818E;
    pub const TEXTURE_LOD_BIAS_T_SGIX: c_uint = 0x818F;
    pub const TEXTURE_LO_SIZE_NV: c_uint = 0x871C;
    pub const TEXTURE_LUMINANCE_SIZE_EXT: c_uint = 0x8060;
    pub const TEXTURE_LUMINANCE_TYPE_ARB: c_uint = 0x8C14;
    pub const TEXTURE_MAG_FILTER: c_uint = 0x2800;
    pub const TEXTURE_MAG_SIZE_NV: c_uint = 0x871F;
    pub const TEXTURE_MATERIAL_FACE_EXT: c_uint = 0x8351;
    pub const TEXTURE_MATERIAL_PARAMETER_EXT: c_uint = 0x8352;
    pub const TEXTURE_MAX_ANISOTROPY: c_uint = 0x84FE;
    pub const TEXTURE_MAX_ANISOTROPY_EXT: c_uint = 0x84FE;
    pub const TEXTURE_MAX_CLAMP_R_SGIX: c_uint = 0x836B;
    pub const TEXTURE_MAX_CLAMP_S_SGIX: c_uint = 0x8369;
    pub const TEXTURE_MAX_CLAMP_T_SGIX: c_uint = 0x836A;
    pub const TEXTURE_MAX_LEVEL: c_uint = 0x813D;
    pub const TEXTURE_MAX_LEVEL_SGIS: c_uint = 0x813D;
    pub const TEXTURE_MAX_LOD: c_uint = 0x813B;
    pub const TEXTURE_MAX_LOD_SGIS: c_uint = 0x813B;
    pub const TEXTURE_MEMORY_LAYOUT_INTEL: c_uint = 0x83FF;
    pub const TEXTURE_MIN_FILTER: c_uint = 0x2801;
    pub const TEXTURE_MIN_LOD: c_uint = 0x813A;
    pub const TEXTURE_MIN_LOD_SGIS: c_uint = 0x813A;
    pub const TEXTURE_MULTI_BUFFER_HINT_SGIX: c_uint = 0x812E;
    pub const TEXTURE_NORMAL_EXT: c_uint = 0x85AF;
    pub const TEXTURE_POST_SPECULAR_HP: c_uint = 0x8168;
    pub const TEXTURE_PRE_SPECULAR_HP: c_uint = 0x8169;
    pub const TEXTURE_PRIORITY_EXT: c_uint = 0x8066;
    pub const TEXTURE_RANGE_LENGTH_APPLE: c_uint = 0x85B7;
    pub const TEXTURE_RANGE_POINTER_APPLE: c_uint = 0x85B8;
    pub const TEXTURE_RECTANGLE: c_uint = 0x84F5;
    pub const TEXTURE_RECTANGLE_ARB: c_uint = 0x84F5;
    pub const TEXTURE_RECTANGLE_NV: c_uint = 0x84F5;
    pub const TEXTURE_REDUCTION_MODE_ARB: c_uint = 0x9366;
    pub const TEXTURE_REDUCTION_MODE_EXT: c_uint = 0x9366;
    pub const TEXTURE_RED_SIZE: c_uint = 0x805C;
    pub const TEXTURE_RED_SIZE_EXT: c_uint = 0x805C;
    pub const TEXTURE_RED_TYPE: c_uint = 0x8C10;
    pub const TEXTURE_RED_TYPE_ARB: c_uint = 0x8C10;
    pub const TEXTURE_RENDERBUFFER_DATA_STORE_BINDING_NV: c_uint = 0x8E54;
    pub const TEXTURE_RENDERBUFFER_NV: c_uint = 0x8E55;
    pub const TEXTURE_RESIDENT_EXT: c_uint = 0x8067;
    pub const TEXTURE_SAMPLES: c_uint = 0x9106;
    pub const TEXTURE_SHADER_NV: c_uint = 0x86DE;
    pub const TEXTURE_SHADOW: c_uint = 0x82A1;
    pub const TEXTURE_SHARED_SIZE: c_uint = 0x8C3F;
    pub const TEXTURE_SHARED_SIZE_EXT: c_uint = 0x8C3F;
    pub const TEXTURE_SPARSE_ARB: c_uint = 0x91A6;
    pub const TEXTURE_SRGB_DECODE_EXT: c_uint = 0x8A48;
    pub const TEXTURE_STENCIL_SIZE: c_uint = 0x88F1;
    pub const TEXTURE_STENCIL_SIZE_EXT: c_uint = 0x88F1;
    pub const TEXTURE_STORAGE_HINT_APPLE: c_uint = 0x85BC;
    pub const TEXTURE_STORAGE_SPARSE_BIT_AMD: c_uint = 0x00000001;
    pub const TEXTURE_SWIZZLE_A: c_uint = 0x8E45;
    pub const TEXTURE_SWIZZLE_A_EXT: c_uint = 0x8E45;
    pub const TEXTURE_SWIZZLE_B: c_uint = 0x8E44;
    pub const TEXTURE_SWIZZLE_B_EXT: c_uint = 0x8E44;
    pub const TEXTURE_SWIZZLE_G: c_uint = 0x8E43;
    pub const TEXTURE_SWIZZLE_G_EXT: c_uint = 0x8E43;
    pub const TEXTURE_SWIZZLE_R: c_uint = 0x8E42;
    pub const TEXTURE_SWIZZLE_RGBA: c_uint = 0x8E46;
    pub const TEXTURE_SWIZZLE_RGBA_EXT: c_uint = 0x8E46;
    pub const TEXTURE_SWIZZLE_R_EXT: c_uint = 0x8E42;
    pub const TEXTURE_TARGET: c_uint = 0x1006;
    pub const TEXTURE_TILING_EXT: c_uint = 0x9580;
    pub const TEXTURE_TOO_LARGE_EXT: c_uint = 0x8065;
    pub const TEXTURE_UNSIGNED_REMAP_MODE_NV: c_uint = 0x888F;
    pub const TEXTURE_UPDATE_BARRIER_BIT: c_uint = 0x00000100;
    pub const TEXTURE_UPDATE_BARRIER_BIT_EXT: c_uint = 0x00000100;
    pub const TEXTURE_VIEW: c_uint = 0x82B5;
    pub const TEXTURE_VIEW_MIN_LAYER: c_uint = 0x82DD;
    pub const TEXTURE_VIEW_MIN_LEVEL: c_uint = 0x82DB;
    pub const TEXTURE_VIEW_NUM_LAYERS: c_uint = 0x82DE;
    pub const TEXTURE_VIEW_NUM_LEVELS: c_uint = 0x82DC;
    pub const TEXTURE_WIDTH: c_uint = 0x1000;
    pub const TEXTURE_WRAP_Q_SGIS: c_uint = 0x8137;
    pub const TEXTURE_WRAP_R: c_uint = 0x8072;
    pub const TEXTURE_WRAP_R_EXT: c_uint = 0x8072;
    pub const TEXTURE_WRAP_S: c_uint = 0x2802;
    pub const TEXTURE_WRAP_T: c_uint = 0x2803;
    pub const TEXT_FRAGMENT_SHADER_ATI: c_uint = 0x8200;
    pub const TILE_RASTER_ORDER_FIXED_MESA: c_uint = 0x8BB8;
    pub const TILE_RASTER_ORDER_INCREASING_X_MESA: c_uint = 0x8BB9;
    pub const TILE_RASTER_ORDER_INCREASING_Y_MESA: c_uint = 0x8BBA;
    pub const TILING_TYPES_EXT: c_uint = 0x9583;
    pub const TIMELINE_SEMAPHORE_VALUE_NV: c_uint = 0x9595;
    pub const TIMEOUT_EXPIRED: c_uint = 0x911B;
    pub const TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const TIMESTAMP: c_uint = 0x8E28;
    pub const TIME_ELAPSED: c_uint = 0x88BF;
    pub const TIME_ELAPSED_EXT: c_uint = 0x88BF;
    pub const TOP_LEVEL_ARRAY_SIZE: c_uint = 0x930C;
    pub const TOP_LEVEL_ARRAY_STRIDE: c_uint = 0x930D;
    pub const TRACK_MATRIX_NV: c_uint = 0x8648;
    pub const TRACK_MATRIX_TRANSFORM_NV: c_uint = 0x8649;
    pub const TRANSFORM_FEEDBACK: c_uint = 0x8E22;
    pub const TRANSFORM_FEEDBACK_ACTIVE: c_uint = 0x8E24;
    pub const TRANSFORM_FEEDBACK_ATTRIBS_NV: c_uint = 0x8C7E;
    pub const TRANSFORM_FEEDBACK_BARRIER_BIT: c_uint = 0x00000800;
    pub const TRANSFORM_FEEDBACK_BARRIER_BIT_EXT: c_uint = 0x00000800;
    pub const TRANSFORM_FEEDBACK_BINDING: c_uint = 0x8E25;
    pub const TRANSFORM_FEEDBACK_BINDING_NV: c_uint = 0x8E25;
    pub const TRANSFORM_FEEDBACK_BUFFER: c_uint = 0x8C8E;
    pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: c_uint = 0x8E24;
    pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE_NV: c_uint = 0x8E24;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: c_uint = 0x8C8F;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING_EXT: c_uint = 0x8C8F;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING_NV: c_uint = 0x8C8F;
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: c_uint = 0x8C8E;
    pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: c_uint = 0x934B;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: c_uint = 0x8C7F;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE_EXT: c_uint = 0x8C7F;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE_NV: c_uint = 0x8C7F;
    pub const TRANSFORM_FEEDBACK_BUFFER_NV: c_uint = 0x8C8E;
    pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: c_uint = 0x8E23;
    pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED_NV: c_uint = 0x8E23;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: c_uint = 0x8C85;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE_EXT: c_uint = 0x8C85;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE_NV: c_uint = 0x8C85;
    pub const TRANSFORM_FEEDBACK_BUFFER_START: c_uint = 0x8C84;
    pub const TRANSFORM_FEEDBACK_BUFFER_START_EXT: c_uint = 0x8C84;
    pub const TRANSFORM_FEEDBACK_BUFFER_START_NV: c_uint = 0x8C84;
    pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: c_uint = 0x934C;
    pub const TRANSFORM_FEEDBACK_NV: c_uint = 0x8E22;
    pub const TRANSFORM_FEEDBACK_OVERFLOW: c_uint = 0x82EC;
    pub const TRANSFORM_FEEDBACK_OVERFLOW_ARB: c_uint = 0x82EC;
    pub const TRANSFORM_FEEDBACK_PAUSED: c_uint = 0x8E23;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: c_uint = 0x8C88;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN_EXT: c_uint = 0x8C88;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN_NV: c_uint = 0x8C88;
    pub const TRANSFORM_FEEDBACK_RECORD_NV: c_uint = 0x8C86;
    pub const TRANSFORM_FEEDBACK_STREAM_OVERFLOW: c_uint = 0x82ED;
    pub const TRANSFORM_FEEDBACK_STREAM_OVERFLOW_ARB: c_uint = 0x82ED;
    pub const TRANSFORM_FEEDBACK_VARYING: c_uint = 0x92F4;
    pub const TRANSFORM_FEEDBACK_VARYINGS: c_uint = 0x8C83;
    pub const TRANSFORM_FEEDBACK_VARYINGS_EXT: c_uint = 0x8C83;
    pub const TRANSFORM_FEEDBACK_VARYINGS_NV: c_uint = 0x8C83;
    pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: c_uint = 0x8C76;
    pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH_EXT: c_uint = 0x8C76;
    pub const TRANSFORM_HINT_APPLE: c_uint = 0x85B1;
    pub const TRANSLATE_2D_NV: c_uint = 0x9090;
    pub const TRANSLATE_3D_NV: c_uint = 0x9091;
    pub const TRANSLATE_X_NV: c_uint = 0x908E;
    pub const TRANSLATE_Y_NV: c_uint = 0x908F;
    pub const TRANSPOSE_AFFINE_2D_NV: c_uint = 0x9096;
    pub const TRANSPOSE_AFFINE_3D_NV: c_uint = 0x9098;
    pub const TRANSPOSE_COLOR_MATRIX_ARB: c_uint = 0x84E6;
    pub const TRANSPOSE_CURRENT_MATRIX_ARB: c_uint = 0x88B7;
    pub const TRANSPOSE_MODELVIEW_MATRIX_ARB: c_uint = 0x84E3;
    pub const TRANSPOSE_NV: c_uint = 0x862C;
    pub const TRANSPOSE_PROGRAM_MATRIX_EXT: c_uint = 0x8E2E;
    pub const TRANSPOSE_PROJECTION_MATRIX_ARB: c_uint = 0x84E4;
    pub const TRANSPOSE_TEXTURE_MATRIX_ARB: c_uint = 0x84E5;
    pub const TRIANGLES: c_uint = 0x0004;
    pub const TRIANGLES_ADJACENCY: c_uint = 0x000C;
    pub const TRIANGLES_ADJACENCY_ARB: c_uint = 0x000C;
    pub const TRIANGLES_ADJACENCY_EXT: c_uint = 0x000C;
    pub const TRIANGLE_FAN: c_uint = 0x0006;
    pub const TRIANGLE_LIST_SUN: c_uint = 0x81D7;
    pub const TRIANGLE_MESH_SUN: c_uint = 0x8615;
    pub const TRIANGLE_STRIP: c_uint = 0x0005;
    pub const TRIANGLE_STRIP_ADJACENCY: c_uint = 0x000D;
    pub const TRIANGLE_STRIP_ADJACENCY_ARB: c_uint = 0x000D;
    pub const TRIANGLE_STRIP_ADJACENCY_EXT: c_uint = 0x000D;
    pub const TRIANGULAR_NV: c_uint = 0x90A5;
    pub const TRUE: c_uchar = 1;
    pub const TYPE: c_uint = 0x92FA;
    pub const UNCORRELATED_NV: c_uint = 0x9282;
    pub const UNDEFINED_APPLE: c_uint = 0x8A1C;
    pub const UNDEFINED_VERTEX: c_uint = 0x8260;
    pub const UNIFORM: c_uint = 0x92E1;
    pub const UNIFORM_ADDRESS_COMMAND_NV: c_uint = 0x000A;
    pub const UNIFORM_ARRAY_STRIDE: c_uint = 0x8A3C;
    pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: c_uint = 0x92DA;
    pub const UNIFORM_BARRIER_BIT: c_uint = 0x00000004;
    pub const UNIFORM_BARRIER_BIT_EXT: c_uint = 0x00000004;
    pub const UNIFORM_BLOCK: c_uint = 0x92E2;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: c_uint = 0x8A42;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: c_uint = 0x8A43;
    pub const UNIFORM_BLOCK_BINDING: c_uint = 0x8A3F;
    pub const UNIFORM_BLOCK_DATA_SIZE: c_uint = 0x8A40;
    pub const UNIFORM_BLOCK_INDEX: c_uint = 0x8A3A;
    pub const UNIFORM_BLOCK_NAME_LENGTH: c_uint = 0x8A41;
    pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x90EC;
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x8A46;
    pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x8A45;
    pub const UNIFORM_BLOCK_REFERENCED_BY_MESH_SHADER_NV: c_uint = 0x959C;
    pub const UNIFORM_BLOCK_REFERENCED_BY_TASK_SHADER_NV: c_uint = 0x959D;
    pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x84F0;
    pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x84F1;
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: c_uint = 0x8A44;
    pub const UNIFORM_BUFFER: c_uint = 0x8A11;
    pub const UNIFORM_BUFFER_ADDRESS_NV: c_uint = 0x936F;
    pub const UNIFORM_BUFFER_BINDING: c_uint = 0x8A28;
    pub const UNIFORM_BUFFER_BINDING_EXT: c_uint = 0x8DEF;
    pub const UNIFORM_BUFFER_EXT: c_uint = 0x8DEE;
    pub const UNIFORM_BUFFER_LENGTH_NV: c_uint = 0x9370;
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x8A34;
    pub const UNIFORM_BUFFER_SIZE: c_uint = 0x8A2A;
    pub const UNIFORM_BUFFER_START: c_uint = 0x8A29;
    pub const UNIFORM_BUFFER_UNIFIED_NV: c_uint = 0x936E;
    pub const UNIFORM_IS_ROW_MAJOR: c_uint = 0x8A3E;
    pub const UNIFORM_MATRIX_STRIDE: c_uint = 0x8A3D;
    pub const UNIFORM_NAME_LENGTH: c_uint = 0x8A39;
    pub const UNIFORM_OFFSET: c_uint = 0x8A3B;
    pub const UNIFORM_SIZE: c_uint = 0x8A38;
    pub const UNIFORM_TYPE: c_uint = 0x8A37;
    pub const UNKNOWN_CONTEXT_RESET: c_uint = 0x8255;
    pub const UNKNOWN_CONTEXT_RESET_ARB: c_uint = 0x8255;
    pub const UNPACK_ALIGNMENT: c_uint = 0x0CF5;
    pub const UNPACK_CLIENT_STORAGE_APPLE: c_uint = 0x85B2;
    pub const UNPACK_CMYK_HINT_EXT: c_uint = 0x800F;
    pub const UNPACK_COMPRESSED_BLOCK_DEPTH: c_uint = 0x9129;
    pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x9128;
    pub const UNPACK_COMPRESSED_BLOCK_SIZE: c_uint = 0x912A;
    pub const UNPACK_COMPRESSED_BLOCK_WIDTH: c_uint = 0x9127;
    pub const UNPACK_CONSTANT_DATA_SUNX: c_uint = 0x81D5;
    pub const UNPACK_IMAGE_DEPTH_SGIS: c_uint = 0x8133;
    pub const UNPACK_IMAGE_HEIGHT: c_uint = 0x806E;
    pub const UNPACK_IMAGE_HEIGHT_EXT: c_uint = 0x806E;
    pub const UNPACK_LSB_FIRST: c_uint = 0x0CF1;
    pub const UNPACK_RESAMPLE_OML: c_uint = 0x8985;
    pub const UNPACK_RESAMPLE_SGIX: c_uint = 0x842F;
    pub const UNPACK_ROW_BYTES_APPLE: c_uint = 0x8A16;
    pub const UNPACK_ROW_LENGTH: c_uint = 0x0CF2;
    pub const UNPACK_SKIP_IMAGES: c_uint = 0x806D;
    pub const UNPACK_SKIP_IMAGES_EXT: c_uint = 0x806D;
    pub const UNPACK_SKIP_PIXELS: c_uint = 0x0CF4;
    pub const UNPACK_SKIP_ROWS: c_uint = 0x0CF3;
    pub const UNPACK_SKIP_VOLUMES_SGIS: c_uint = 0x8132;
    pub const UNPACK_SUBSAMPLE_RATE_SGIX: c_uint = 0x85A1;
    pub const UNPACK_SWAP_BYTES: c_uint = 0x0CF0;
    pub const UNSIGNALED: c_uint = 0x9118;
    pub const UNSIGNED_BYTE: c_uint = 0x1401;
    pub const UNSIGNED_BYTE_2_3_3_REV: c_uint = 0x8362;
    pub const UNSIGNED_BYTE_3_3_2: c_uint = 0x8032;
    pub const UNSIGNED_BYTE_3_3_2_EXT: c_uint = 0x8032;
    pub const UNSIGNED_IDENTITY_NV: c_uint = 0x8536;
    pub const UNSIGNED_INT: c_uint = 0x1405;
    pub const UNSIGNED_INT16_NV: c_uint = 0x8FF0;
    pub const UNSIGNED_INT16_VEC2_NV: c_uint = 0x8FF1;
    pub const UNSIGNED_INT16_VEC3_NV: c_uint = 0x8FF2;
    pub const UNSIGNED_INT16_VEC4_NV: c_uint = 0x8FF3;
    pub const UNSIGNED_INT64_AMD: c_uint = 0x8BC2;
    pub const UNSIGNED_INT64_ARB: c_uint = 0x140F;
    pub const UNSIGNED_INT64_NV: c_uint = 0x140F;
    pub const UNSIGNED_INT64_VEC2_ARB: c_uint = 0x8FF5;
    pub const UNSIGNED_INT64_VEC2_NV: c_uint = 0x8FF5;
    pub const UNSIGNED_INT64_VEC3_ARB: c_uint = 0x8FF6;
    pub const UNSIGNED_INT64_VEC3_NV: c_uint = 0x8FF6;
    pub const UNSIGNED_INT64_VEC4_ARB: c_uint = 0x8FF7;
    pub const UNSIGNED_INT64_VEC4_NV: c_uint = 0x8FF7;
    pub const UNSIGNED_INT8_NV: c_uint = 0x8FEC;
    pub const UNSIGNED_INT8_VEC2_NV: c_uint = 0x8FED;
    pub const UNSIGNED_INT8_VEC3_NV: c_uint = 0x8FEE;
    pub const UNSIGNED_INT8_VEC4_NV: c_uint = 0x8FEF;
    pub const UNSIGNED_INT_10F_11F_11F_REV: c_uint = 0x8C3B;
    pub const UNSIGNED_INT_10F_11F_11F_REV_EXT: c_uint = 0x8C3B;
    pub const UNSIGNED_INT_10_10_10_2: c_uint = 0x8036;
    pub const UNSIGNED_INT_10_10_10_2_EXT: c_uint = 0x8036;
    pub const UNSIGNED_INT_24_8: c_uint = 0x84FA;
    pub const UNSIGNED_INT_24_8_EXT: c_uint = 0x84FA;
    pub const UNSIGNED_INT_24_8_NV: c_uint = 0x84FA;
    pub const UNSIGNED_INT_2_10_10_10_REV: c_uint = 0x8368;
    pub const UNSIGNED_INT_5_9_9_9_REV: c_uint = 0x8C3E;
    pub const UNSIGNED_INT_5_9_9_9_REV_EXT: c_uint = 0x8C3E;
    pub const UNSIGNED_INT_8_8_8_8: c_uint = 0x8035;
    pub const UNSIGNED_INT_8_8_8_8_EXT: c_uint = 0x8035;
    pub const UNSIGNED_INT_8_8_8_8_REV: c_uint = 0x8367;
    pub const UNSIGNED_INT_8_8_S8_S8_REV_NV: c_uint = 0x86DB;
    pub const UNSIGNED_INT_ATOMIC_COUNTER: c_uint = 0x92DB;
    pub const UNSIGNED_INT_IMAGE_1D: c_uint = 0x9062;
    pub const UNSIGNED_INT_IMAGE_1D_ARRAY: c_uint = 0x9068;
    pub const UNSIGNED_INT_IMAGE_1D_ARRAY_EXT: c_uint = 0x9068;
    pub const UNSIGNED_INT_IMAGE_1D_EXT: c_uint = 0x9062;
    pub const UNSIGNED_INT_IMAGE_2D: c_uint = 0x9063;
    pub const UNSIGNED_INT_IMAGE_2D_ARRAY: c_uint = 0x9069;
    pub const UNSIGNED_INT_IMAGE_2D_ARRAY_EXT: c_uint = 0x9069;
    pub const UNSIGNED_INT_IMAGE_2D_EXT: c_uint = 0x9063;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: c_uint = 0x906B;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x906C;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: c_uint = 0x906C;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_EXT: c_uint = 0x906B;
    pub const UNSIGNED_INT_IMAGE_2D_RECT: c_uint = 0x9065;
    pub const UNSIGNED_INT_IMAGE_2D_RECT_EXT: c_uint = 0x9065;
    pub const UNSIGNED_INT_IMAGE_3D: c_uint = 0x9064;
    pub const UNSIGNED_INT_IMAGE_3D_EXT: c_uint = 0x9064;
    pub const UNSIGNED_INT_IMAGE_BUFFER: c_uint = 0x9067;
    pub const UNSIGNED_INT_IMAGE_BUFFER_EXT: c_uint = 0x9067;
    pub const UNSIGNED_INT_IMAGE_CUBE: c_uint = 0x9066;
    pub const UNSIGNED_INT_IMAGE_CUBE_EXT: c_uint = 0x9066;
    pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: c_uint = 0x906A;
    pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_EXT: c_uint = 0x906A;
    pub const UNSIGNED_INT_S8_S8_8_8_NV: c_uint = 0x86DA;
    pub const UNSIGNED_INT_SAMPLER_1D: c_uint = 0x8DD1;
    pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: c_uint = 0x8DD6;
    pub const UNSIGNED_INT_SAMPLER_1D_ARRAY_EXT: c_uint = 0x8DD6;
    pub const UNSIGNED_INT_SAMPLER_1D_EXT: c_uint = 0x8DD1;
    pub const UNSIGNED_INT_SAMPLER_2D: c_uint = 0x8DD2;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: c_uint = 0x8DD7;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY_EXT: c_uint = 0x8DD7;
    pub const UNSIGNED_INT_SAMPLER_2D_EXT: c_uint = 0x8DD2;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x910A;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910D;
    pub const UNSIGNED_INT_SAMPLER_2D_RECT: c_uint = 0x8DD5;
    pub const UNSIGNED_INT_SAMPLER_2D_RECT_EXT: c_uint = 0x8DD5;
    pub const UNSIGNED_INT_SAMPLER_3D: c_uint = 0x8DD3;
    pub const UNSIGNED_INT_SAMPLER_3D_EXT: c_uint = 0x8DD3;
    pub const UNSIGNED_INT_SAMPLER_BUFFER: c_uint = 0x8DD8;
    pub const UNSIGNED_INT_SAMPLER_BUFFER_AMD: c_uint = 0x9003;
    pub const UNSIGNED_INT_SAMPLER_BUFFER_EXT: c_uint = 0x8DD8;
    pub const UNSIGNED_INT_SAMPLER_CUBE: c_uint = 0x8DD4;
    pub const UNSIGNED_INT_SAMPLER_CUBE_EXT: c_uint = 0x8DD4;
    pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900F;
    pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_ARB: c_uint = 0x900F;
    pub const UNSIGNED_INT_SAMPLER_RENDERBUFFER_NV: c_uint = 0x8E58;
    pub const UNSIGNED_INT_VEC2: c_uint = 0x8DC6;
    pub const UNSIGNED_INT_VEC2_EXT: c_uint = 0x8DC6;
    pub const UNSIGNED_INT_VEC3: c_uint = 0x8DC7;
    pub const UNSIGNED_INT_VEC3_EXT: c_uint = 0x8DC7;
    pub const UNSIGNED_INT_VEC4: c_uint = 0x8DC8;
    pub const UNSIGNED_INT_VEC4_EXT: c_uint = 0x8DC8;
    pub const UNSIGNED_INVERT_NV: c_uint = 0x8537;
    pub const UNSIGNED_NORMALIZED: c_uint = 0x8C17;
    pub const UNSIGNED_NORMALIZED_ARB: c_uint = 0x8C17;
    pub const UNSIGNED_SHORT: c_uint = 0x1403;
    pub const UNSIGNED_SHORT_1_5_5_5_REV: c_uint = 0x8366;
    pub const UNSIGNED_SHORT_4_4_4_4: c_uint = 0x8033;
    pub const UNSIGNED_SHORT_4_4_4_4_EXT: c_uint = 0x8033;
    pub const UNSIGNED_SHORT_4_4_4_4_REV: c_uint = 0x8365;
    pub const UNSIGNED_SHORT_5_5_5_1: c_uint = 0x8034;
    pub const UNSIGNED_SHORT_5_5_5_1_EXT: c_uint = 0x8034;
    pub const UNSIGNED_SHORT_5_6_5: c_uint = 0x8363;
    pub const UNSIGNED_SHORT_5_6_5_REV: c_uint = 0x8364;
    pub const UNSIGNED_SHORT_8_8_APPLE: c_uint = 0x85BA;
    pub const UNSIGNED_SHORT_8_8_MESA: c_uint = 0x85BA;
    pub const UNSIGNED_SHORT_8_8_REV_APPLE: c_uint = 0x85BB;
    pub const UNSIGNED_SHORT_8_8_REV_MESA: c_uint = 0x85BB;
    pub const UPLOAD_GPU_MASK_NVX: c_uint = 0x954A;
    pub const UPPER_LEFT: c_uint = 0x8CA2;
    pub const USE_MISSING_GLYPH_NV: c_uint = 0x90AA;
    pub const UTF16_NV: c_uint = 0x909B;
    pub const UTF8_NV: c_uint = 0x909A;
    pub const UUID_SIZE_EXT: c_uint = 16;
    pub const VALIDATE_STATUS: c_uint = 0x8B83;
    pub const VARIABLE_A_NV: c_uint = 0x8523;
    pub const VARIABLE_B_NV: c_uint = 0x8524;
    pub const VARIABLE_C_NV: c_uint = 0x8525;
    pub const VARIABLE_D_NV: c_uint = 0x8526;
    pub const VARIABLE_E_NV: c_uint = 0x8527;
    pub const VARIABLE_F_NV: c_uint = 0x8528;
    pub const VARIABLE_G_NV: c_uint = 0x8529;
    pub const VARIANT_ARRAY_EXT: c_uint = 0x87E8;
    pub const VARIANT_ARRAY_POINTER_EXT: c_uint = 0x87E9;
    pub const VARIANT_ARRAY_STRIDE_EXT: c_uint = 0x87E6;
    pub const VARIANT_ARRAY_TYPE_EXT: c_uint = 0x87E7;
    pub const VARIANT_DATATYPE_EXT: c_uint = 0x87E5;
    pub const VARIANT_EXT: c_uint = 0x87C1;
    pub const VARIANT_VALUE_EXT: c_uint = 0x87E4;
    pub const VBO_FREE_MEMORY_ATI: c_uint = 0x87FB;
    pub const VECTOR_EXT: c_uint = 0x87BF;
    pub const VENDOR: c_uint = 0x1F00;
    pub const VERSION: c_uint = 0x1F02;
    pub const VERTEX23_BIT_PGI: c_uint = 0x00000004;
    pub const VERTEX4_BIT_PGI: c_uint = 0x00000008;
    pub const VERTEX_ARRAY: c_uint = 0x8074;
    pub const VERTEX_ARRAY_ADDRESS_NV: c_uint = 0x8F21;
    pub const VERTEX_ARRAY_BINDING: c_uint = 0x85B5;
    pub const VERTEX_ARRAY_BINDING_APPLE: c_uint = 0x85B5;
    pub const VERTEX_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8896;
    pub const VERTEX_ARRAY_COUNT_EXT: c_uint = 0x807D;
    pub const VERTEX_ARRAY_EXT: c_uint = 0x8074;
    pub const VERTEX_ARRAY_LENGTH_NV: c_uint = 0x8F2B;
    pub const VERTEX_ARRAY_LIST_IBM: c_uint = 103070;
    pub const VERTEX_ARRAY_LIST_STRIDE_IBM: c_uint = 103080;
    pub const VERTEX_ARRAY_OBJECT_AMD: c_uint = 0x9154;
    pub const VERTEX_ARRAY_OBJECT_EXT: c_uint = 0x9154;
    pub const VERTEX_ARRAY_PARALLEL_POINTERS_INTEL: c_uint = 0x83F5;
    pub const VERTEX_ARRAY_POINTER_EXT: c_uint = 0x808E;
    pub const VERTEX_ARRAY_RANGE_APPLE: c_uint = 0x851D;
    pub const VERTEX_ARRAY_RANGE_LENGTH_APPLE: c_uint = 0x851E;
    pub const VERTEX_ARRAY_RANGE_LENGTH_NV: c_uint = 0x851E;
    pub const VERTEX_ARRAY_RANGE_NV: c_uint = 0x851D;
    pub const VERTEX_ARRAY_RANGE_POINTER_APPLE: c_uint = 0x8521;
    pub const VERTEX_ARRAY_RANGE_POINTER_NV: c_uint = 0x8521;
    pub const VERTEX_ARRAY_RANGE_VALID_NV: c_uint = 0x851F;
    pub const VERTEX_ARRAY_RANGE_WITHOUT_FLUSH_NV: c_uint = 0x8533;
    pub const VERTEX_ARRAY_SIZE_EXT: c_uint = 0x807A;
    pub const VERTEX_ARRAY_STORAGE_HINT_APPLE: c_uint = 0x851F;
    pub const VERTEX_ARRAY_STRIDE_EXT: c_uint = 0x807C;
    pub const VERTEX_ARRAY_TYPE_EXT: c_uint = 0x807B;
    pub const VERTEX_ATTRIB_ARRAY0_NV: c_uint = 0x8650;
    pub const VERTEX_ATTRIB_ARRAY10_NV: c_uint = 0x865A;
    pub const VERTEX_ATTRIB_ARRAY11_NV: c_uint = 0x865B;
    pub const VERTEX_ATTRIB_ARRAY12_NV: c_uint = 0x865C;
    pub const VERTEX_ATTRIB_ARRAY13_NV: c_uint = 0x865D;
    pub const VERTEX_ATTRIB_ARRAY14_NV: c_uint = 0x865E;
    pub const VERTEX_ATTRIB_ARRAY15_NV: c_uint = 0x865F;
    pub const VERTEX_ATTRIB_ARRAY1_NV: c_uint = 0x8651;
    pub const VERTEX_ATTRIB_ARRAY2_NV: c_uint = 0x8652;
    pub const VERTEX_ATTRIB_ARRAY3_NV: c_uint = 0x8653;
    pub const VERTEX_ATTRIB_ARRAY4_NV: c_uint = 0x8654;
    pub const VERTEX_ATTRIB_ARRAY5_NV: c_uint = 0x8655;
    pub const VERTEX_ATTRIB_ARRAY6_NV: c_uint = 0x8656;
    pub const VERTEX_ATTRIB_ARRAY7_NV: c_uint = 0x8657;
    pub const VERTEX_ATTRIB_ARRAY8_NV: c_uint = 0x8658;
    pub const VERTEX_ATTRIB_ARRAY9_NV: c_uint = 0x8659;
    pub const VERTEX_ATTRIB_ARRAY_ADDRESS_NV: c_uint = 0x8F20;
    pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: c_uint = 0x00000001;
    pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT_EXT: c_uint = 0x00000001;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: c_uint = 0x889F;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889F;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: c_uint = 0x88FE;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR_ARB: c_uint = 0x88FE;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: c_uint = 0x8622;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED_ARB: c_uint = 0x8622;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: c_uint = 0x88FD;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER_EXT: c_uint = 0x88FD;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER_NV: c_uint = 0x88FD;
    pub const VERTEX_ATTRIB_ARRAY_LENGTH_NV: c_uint = 0x8F2A;
    pub const VERTEX_ATTRIB_ARRAY_LONG: c_uint = 0x874E;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: c_uint = 0x886A;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED_ARB: c_uint = 0x886A;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: c_uint = 0x8645;
    pub const VERTEX_ATTRIB_ARRAY_POINTER_ARB: c_uint = 0x8645;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: c_uint = 0x8623;
    pub const VERTEX_ATTRIB_ARRAY_SIZE_ARB: c_uint = 0x8623;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: c_uint = 0x8624;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE_ARB: c_uint = 0x8624;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: c_uint = 0x8625;
    pub const VERTEX_ATTRIB_ARRAY_TYPE_ARB: c_uint = 0x8625;
    pub const VERTEX_ATTRIB_ARRAY_UNIFIED_NV: c_uint = 0x8F1E;
    pub const VERTEX_ATTRIB_BINDING: c_uint = 0x82D4;
    pub const VERTEX_ATTRIB_MAP1_APPLE: c_uint = 0x8A00;
    pub const VERTEX_ATTRIB_MAP1_COEFF_APPLE: c_uint = 0x8A03;
    pub const VERTEX_ATTRIB_MAP1_DOMAIN_APPLE: c_uint = 0x8A05;
    pub const VERTEX_ATTRIB_MAP1_ORDER_APPLE: c_uint = 0x8A04;
    pub const VERTEX_ATTRIB_MAP1_SIZE_APPLE: c_uint = 0x8A02;
    pub const VERTEX_ATTRIB_MAP2_APPLE: c_uint = 0x8A01;
    pub const VERTEX_ATTRIB_MAP2_COEFF_APPLE: c_uint = 0x8A07;
    pub const VERTEX_ATTRIB_MAP2_DOMAIN_APPLE: c_uint = 0x8A09;
    pub const VERTEX_ATTRIB_MAP2_ORDER_APPLE: c_uint = 0x8A08;
    pub const VERTEX_ATTRIB_MAP2_SIZE_APPLE: c_uint = 0x8A06;
    pub const VERTEX_ATTRIB_RELATIVE_OFFSET: c_uint = 0x82D5;
    pub const VERTEX_BINDING_BUFFER: c_uint = 0x8F4F;
    pub const VERTEX_BINDING_DIVISOR: c_uint = 0x82D6;
    pub const VERTEX_BINDING_OFFSET: c_uint = 0x82D7;
    pub const VERTEX_BINDING_STRIDE: c_uint = 0x82D8;
    pub const VERTEX_BLEND_ARB: c_uint = 0x86A7;
    pub const VERTEX_CONSISTENT_HINT_PGI: c_uint = 0x1A22B;
    pub const VERTEX_DATA_HINT_PGI: c_uint = 0x1A22A;
    pub const VERTEX_ELEMENT_SWIZZLE_AMD: c_uint = 0x91A4;
    pub const VERTEX_ID_NV: c_uint = 0x8C7B;
    pub const VERTEX_ID_SWIZZLE_AMD: c_uint = 0x91A5;
    pub const VERTEX_PRECLIP_HINT_SGIX: c_uint = 0x83EF;
    pub const VERTEX_PRECLIP_SGIX: c_uint = 0x83EE;
    pub const VERTEX_PROGRAM_ARB: c_uint = 0x8620;
    pub const VERTEX_PROGRAM_BINDING_NV: c_uint = 0x864A;
    pub const VERTEX_PROGRAM_NV: c_uint = 0x8620;
    pub const VERTEX_PROGRAM_PARAMETER_BUFFER_NV: c_uint = 0x8DA2;
    pub const VERTEX_PROGRAM_POINT_SIZE: c_uint = 0x8642;
    pub const VERTEX_PROGRAM_POINT_SIZE_ARB: c_uint = 0x8642;
    pub const VERTEX_PROGRAM_POINT_SIZE_NV: c_uint = 0x8642;
    pub const VERTEX_PROGRAM_TWO_SIDE_ARB: c_uint = 0x8643;
    pub const VERTEX_PROGRAM_TWO_SIDE_NV: c_uint = 0x8643;
    pub const VERTEX_SHADER: c_uint = 0x8B31;
    pub const VERTEX_SHADER_ARB: c_uint = 0x8B31;
    pub const VERTEX_SHADER_BINDING_EXT: c_uint = 0x8781;
    pub const VERTEX_SHADER_BIT: c_uint = 0x00000001;
    pub const VERTEX_SHADER_EXT: c_uint = 0x8780;
    pub const VERTEX_SHADER_INSTRUCTIONS_EXT: c_uint = 0x87CF;
    pub const VERTEX_SHADER_INVARIANTS_EXT: c_uint = 0x87D1;
    pub const VERTEX_SHADER_INVOCATIONS: c_uint = 0x82F0;
    pub const VERTEX_SHADER_INVOCATIONS_ARB: c_uint = 0x82F0;
    pub const VERTEX_SHADER_LOCALS_EXT: c_uint = 0x87D3;
    pub const VERTEX_SHADER_LOCAL_CONSTANTS_EXT: c_uint = 0x87D2;
    pub const VERTEX_SHADER_OPTIMIZED_EXT: c_uint = 0x87D4;
    pub const VERTEX_SHADER_VARIANTS_EXT: c_uint = 0x87D0;
    pub const VERTEX_SOURCE_ATI: c_uint = 0x8774;
    pub const VERTEX_STATE_PROGRAM_NV: c_uint = 0x8621;
    pub const VERTEX_STREAM0_ATI: c_uint = 0x876C;
    pub const VERTEX_STREAM1_ATI: c_uint = 0x876D;
    pub const VERTEX_STREAM2_ATI: c_uint = 0x876E;
    pub const VERTEX_STREAM3_ATI: c_uint = 0x876F;
    pub const VERTEX_STREAM4_ATI: c_uint = 0x8770;
    pub const VERTEX_STREAM5_ATI: c_uint = 0x8771;
    pub const VERTEX_STREAM6_ATI: c_uint = 0x8772;
    pub const VERTEX_STREAM7_ATI: c_uint = 0x8773;
    pub const VERTEX_SUBROUTINE: c_uint = 0x92E8;
    pub const VERTEX_SUBROUTINE_UNIFORM: c_uint = 0x92EE;
    pub const VERTEX_TEXTURE: c_uint = 0x829B;
    pub const VERTEX_WEIGHTING_EXT: c_uint = 0x8509;
    pub const VERTEX_WEIGHT_ARRAY_EXT: c_uint = 0x850C;
    pub const VERTEX_WEIGHT_ARRAY_POINTER_EXT: c_uint = 0x8510;
    pub const VERTEX_WEIGHT_ARRAY_SIZE_EXT: c_uint = 0x850D;
    pub const VERTEX_WEIGHT_ARRAY_STRIDE_EXT: c_uint = 0x850F;
    pub const VERTEX_WEIGHT_ARRAY_TYPE_EXT: c_uint = 0x850E;
    pub const VERTICAL_LINE_TO_NV: c_uint = 0x08;
    pub const VERTICES_SUBMITTED: c_uint = 0x82EE;
    pub const VERTICES_SUBMITTED_ARB: c_uint = 0x82EE;
    pub const VIBRANCE_BIAS_NV: c_uint = 0x8719;
    pub const VIBRANCE_SCALE_NV: c_uint = 0x8713;
    pub const VIDEO_BUFFER_BINDING_NV: c_uint = 0x9021;
    pub const VIDEO_BUFFER_INTERNAL_FORMAT_NV: c_uint = 0x902D;
    pub const VIDEO_BUFFER_NV: c_uint = 0x9020;
    pub const VIDEO_BUFFER_PITCH_NV: c_uint = 0x9028;
    pub const VIDEO_CAPTURE_FIELD_LOWER_HEIGHT_NV: c_uint = 0x903B;
    pub const VIDEO_CAPTURE_FIELD_UPPER_HEIGHT_NV: c_uint = 0x903A;
    pub const VIDEO_CAPTURE_FRAME_HEIGHT_NV: c_uint = 0x9039;
    pub const VIDEO_CAPTURE_FRAME_WIDTH_NV: c_uint = 0x9038;
    pub const VIDEO_CAPTURE_SURFACE_ORIGIN_NV: c_uint = 0x903C;
    pub const VIDEO_CAPTURE_TO_422_SUPPORTED_NV: c_uint = 0x9026;
    pub const VIDEO_COLOR_CONVERSION_MATRIX_NV: c_uint = 0x9029;
    pub const VIDEO_COLOR_CONVERSION_MAX_NV: c_uint = 0x902A;
    pub const VIDEO_COLOR_CONVERSION_MIN_NV: c_uint = 0x902B;
    pub const VIDEO_COLOR_CONVERSION_OFFSET_NV: c_uint = 0x902C;
    pub const VIEWPORT: c_uint = 0x0BA2;
    pub const VIEWPORT_BOUNDS_RANGE: c_uint = 0x825D;
    pub const VIEWPORT_COMMAND_NV: c_uint = 0x0010;
    pub const VIEWPORT_INDEX_PROVOKING_VERTEX: c_uint = 0x825F;
    pub const VIEWPORT_POSITION_W_SCALE_NV: c_uint = 0x937C;
    pub const VIEWPORT_POSITION_W_SCALE_X_COEFF_NV: c_uint = 0x937D;
    pub const VIEWPORT_POSITION_W_SCALE_Y_COEFF_NV: c_uint = 0x937E;
    pub const VIEWPORT_SUBPIXEL_BITS: c_uint = 0x825C;
    pub const VIEWPORT_SWIZZLE_NEGATIVE_W_NV: c_uint = 0x9357;
    pub const VIEWPORT_SWIZZLE_NEGATIVE_X_NV: c_uint = 0x9351;
    pub const VIEWPORT_SWIZZLE_NEGATIVE_Y_NV: c_uint = 0x9353;
    pub const VIEWPORT_SWIZZLE_NEGATIVE_Z_NV: c_uint = 0x9355;
    pub const VIEWPORT_SWIZZLE_POSITIVE_W_NV: c_uint = 0x9356;
    pub const VIEWPORT_SWIZZLE_POSITIVE_X_NV: c_uint = 0x9350;
    pub const VIEWPORT_SWIZZLE_POSITIVE_Y_NV: c_uint = 0x9352;
    pub const VIEWPORT_SWIZZLE_POSITIVE_Z_NV: c_uint = 0x9354;
    pub const VIEWPORT_SWIZZLE_W_NV: c_uint = 0x935B;
    pub const VIEWPORT_SWIZZLE_X_NV: c_uint = 0x9358;
    pub const VIEWPORT_SWIZZLE_Y_NV: c_uint = 0x9359;
    pub const VIEWPORT_SWIZZLE_Z_NV: c_uint = 0x935A;
    pub const VIEW_CLASS_128_BITS: c_uint = 0x82C4;
    pub const VIEW_CLASS_16_BITS: c_uint = 0x82CA;
    pub const VIEW_CLASS_24_BITS: c_uint = 0x82C9;
    pub const VIEW_CLASS_32_BITS: c_uint = 0x82C8;
    pub const VIEW_CLASS_48_BITS: c_uint = 0x82C7;
    pub const VIEW_CLASS_64_BITS: c_uint = 0x82C6;
    pub const VIEW_CLASS_8_BITS: c_uint = 0x82CB;
    pub const VIEW_CLASS_96_BITS: c_uint = 0x82C5;
    pub const VIEW_CLASS_ASTC_10x10_RGBA: c_uint = 0x9393;
    pub const VIEW_CLASS_ASTC_10x5_RGBA: c_uint = 0x9390;
    pub const VIEW_CLASS_ASTC_10x6_RGBA: c_uint = 0x9391;
    pub const VIEW_CLASS_ASTC_10x8_RGBA: c_uint = 0x9392;
    pub const VIEW_CLASS_ASTC_12x10_RGBA: c_uint = 0x9394;
    pub const VIEW_CLASS_ASTC_12x12_RGBA: c_uint = 0x9395;
    pub const VIEW_CLASS_ASTC_4x4_RGBA: c_uint = 0x9388;
    pub const VIEW_CLASS_ASTC_5x4_RGBA: c_uint = 0x9389;
    pub const VIEW_CLASS_ASTC_5x5_RGBA: c_uint = 0x938A;
    pub const VIEW_CLASS_ASTC_6x5_RGBA: c_uint = 0x938B;
    pub const VIEW_CLASS_ASTC_6x6_RGBA: c_uint = 0x938C;
    pub const VIEW_CLASS_ASTC_8x5_RGBA: c_uint = 0x938D;
    pub const VIEW_CLASS_ASTC_8x6_RGBA: c_uint = 0x938E;
    pub const VIEW_CLASS_ASTC_8x8_RGBA: c_uint = 0x938F;
    pub const VIEW_CLASS_BPTC_FLOAT: c_uint = 0x82D3;
    pub const VIEW_CLASS_BPTC_UNORM: c_uint = 0x82D2;
    pub const VIEW_CLASS_EAC_R11: c_uint = 0x9383;
    pub const VIEW_CLASS_EAC_RG11: c_uint = 0x9384;
    pub const VIEW_CLASS_ETC2_EAC_RGBA: c_uint = 0x9387;
    pub const VIEW_CLASS_ETC2_RGB: c_uint = 0x9385;
    pub const VIEW_CLASS_ETC2_RGBA: c_uint = 0x9386;
    pub const VIEW_CLASS_RGTC1_RED: c_uint = 0x82D0;
    pub const VIEW_CLASS_RGTC2_RG: c_uint = 0x82D1;
    pub const VIEW_CLASS_S3TC_DXT1_RGB: c_uint = 0x82CC;
    pub const VIEW_CLASS_S3TC_DXT1_RGBA: c_uint = 0x82CD;
    pub const VIEW_CLASS_S3TC_DXT3_RGBA: c_uint = 0x82CE;
    pub const VIEW_CLASS_S3TC_DXT5_RGBA: c_uint = 0x82CF;
    pub const VIEW_COMPATIBILITY_CLASS: c_uint = 0x82B6;
    pub const VIRTUAL_PAGE_SIZE_INDEX_ARB: c_uint = 0x91A7;
    pub const VIRTUAL_PAGE_SIZE_X_AMD: c_uint = 0x9195;
    pub const VIRTUAL_PAGE_SIZE_X_ARB: c_uint = 0x9195;
    pub const VIRTUAL_PAGE_SIZE_Y_AMD: c_uint = 0x9196;
    pub const VIRTUAL_PAGE_SIZE_Y_ARB: c_uint = 0x9196;
    pub const VIRTUAL_PAGE_SIZE_Z_AMD: c_uint = 0x9197;
    pub const VIRTUAL_PAGE_SIZE_Z_ARB: c_uint = 0x9197;
    pub const VIVIDLIGHT_NV: c_uint = 0x92A6;
    pub const VOLATILE_APPLE: c_uint = 0x8A1A;
    pub const WAIT_FAILED: c_uint = 0x911D;
    pub const WARPS_PER_SM_NV: c_uint = 0x933A;
    pub const WARP_SIZE_NV: c_uint = 0x9339;
    pub const WEIGHTED_AVERAGE_ARB: c_uint = 0x9367;
    pub const WEIGHTED_AVERAGE_EXT: c_uint = 0x9367;
    pub const WEIGHT_ARRAY_ARB: c_uint = 0x86AD;
    pub const WEIGHT_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889E;
    pub const WEIGHT_ARRAY_POINTER_ARB: c_uint = 0x86AC;
    pub const WEIGHT_ARRAY_SIZE_ARB: c_uint = 0x86AB;
    pub const WEIGHT_ARRAY_STRIDE_ARB: c_uint = 0x86AA;
    pub const WEIGHT_ARRAY_TYPE_ARB: c_uint = 0x86A9;
    pub const WEIGHT_SUM_UNITY_ARB: c_uint = 0x86A6;
    pub const WIDE_LINE_HINT_PGI: c_uint = 0x1A222;
    pub const WINDOW_RECTANGLE_EXT: c_uint = 0x8F12;
    pub const WINDOW_RECTANGLE_MODE_EXT: c_uint = 0x8F13;
    pub const WRAP_BORDER_SUN: c_uint = 0x81D4;
    pub const WRITE_DISCARD_NV: c_uint = 0x88BE;
    pub const WRITE_ONLY: c_uint = 0x88B9;
    pub const WRITE_ONLY_ARB: c_uint = 0x88B9;
    pub const WRITE_PIXEL_DATA_RANGE_LENGTH_NV: c_uint = 0x887A;
    pub const WRITE_PIXEL_DATA_RANGE_NV: c_uint = 0x8878;
    pub const WRITE_PIXEL_DATA_RANGE_POINTER_NV: c_uint = 0x887C;
    pub const W_EXT: c_uint = 0x87D8;
    pub const XOR: c_uint = 0x1506;
    pub const XOR_NV: c_uint = 0x1506;
    pub const X_EXT: c_uint = 0x87D5;
    pub const YCBAYCR8A_4224_NV: c_uint = 0x9032;
    pub const YCBCR_422_APPLE: c_uint = 0x85B9;
    pub const YCBCR_MESA: c_uint = 0x8757;
    pub const YCBYCR8_422_NV: c_uint = 0x9031;
    pub const YCRCBA_SGIX: c_uint = 0x8319;
    pub const YCRCB_422_SGIX: c_uint = 0x81BB;
    pub const YCRCB_444_SGIX: c_uint = 0x81BC;
    pub const YCRCB_SGIX: c_uint = 0x8318;
    pub const Y_EXT: c_uint = 0x87D6;
    pub const Z4Y12Z4CB12Z4A12Z4Y12Z4CR12Z4A12_4224_NV: c_uint = 0x9036;
    pub const Z4Y12Z4CB12Z4CR12_444_NV: c_uint = 0x9037;
    pub const Z4Y12Z4CB12Z4Y12Z4CR12_422_NV: c_uint = 0x9035;
    pub const Z6Y10Z6CB10Z6A10Z6Y10Z6CR10Z6A10_4224_NV: c_uint = 0x9034;
    pub const Z6Y10Z6CB10Z6Y10Z6CR10_422_NV: c_uint = 0x9033;
    pub const ZERO: c_uint = 0;
    pub const ZERO_EXT: c_uint = 0x87DD;
    pub const ZERO_TO_ONE: c_uint = 0x935F;
    pub const Z_EXT: c_uint = 0x87D7;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

    use std::mem::transmute;
    use std::os::raw::*;
    use super::*;
    use super::types::*;

    macro_rules! func {
        ($fun:ident, $ret:ty, $($name:ident: $typ:ty),*) => {
            #[inline] pub unsafe fn $fun(&self, $($name: $typ),*) -> $ret {
                transmute::<_, extern "system" fn($($typ),*) -> $ret>(self.$fun.ptr)($($name),*)
            }
        }
    }

    pub struct Gl {
         pub(super) AccumxOES: FnPtr,
         pub(super) AcquireKeyedMutexWin32EXT: FnPtr,
         pub(super) ActiveProgramEXT: FnPtr,
         pub(super) ActiveShaderProgram: FnPtr,
         pub(super) ActiveStencilFaceEXT: FnPtr,
         pub(super) ActiveTexture: FnPtr,
         pub(super) ActiveTextureARB: FnPtr,
         pub(super) ActiveVaryingNV: FnPtr,
         pub(super) AlphaFragmentOp1ATI: FnPtr,
         pub(super) AlphaFragmentOp2ATI: FnPtr,
         pub(super) AlphaFragmentOp3ATI: FnPtr,
         pub(super) AlphaFuncxOES: FnPtr,
         pub(super) AlphaToCoverageDitherControlNV: FnPtr,
         pub(super) ApplyFramebufferAttachmentCMAAINTEL: FnPtr,
         pub(super) ApplyTextureEXT: FnPtr,
         pub(super) AreProgramsResidentNV: FnPtr,
         pub(super) AreTexturesResidentEXT: FnPtr,
         pub(super) ArrayElementEXT: FnPtr,
         pub(super) ArrayObjectATI: FnPtr,
         pub(super) AsyncCopyBufferSubDataNVX: FnPtr,
         pub(super) AsyncCopyImageSubDataNVX: FnPtr,
         pub(super) AsyncMarkerSGIX: FnPtr,
         pub(super) AttachObjectARB: FnPtr,
         pub(super) AttachShader: FnPtr,
         pub(super) BeginConditionalRender: FnPtr,
         pub(super) BeginConditionalRenderNV: FnPtr,
         pub(super) BeginConditionalRenderNVX: FnPtr,
         pub(super) BeginFragmentShaderATI: FnPtr,
         pub(super) BeginOcclusionQueryNV: FnPtr,
         pub(super) BeginPerfMonitorAMD: FnPtr,
         pub(super) BeginPerfQueryINTEL: FnPtr,
         pub(super) BeginQuery: FnPtr,
         pub(super) BeginQueryARB: FnPtr,
         pub(super) BeginQueryIndexed: FnPtr,
         pub(super) BeginTransformFeedback: FnPtr,
         pub(super) BeginTransformFeedbackEXT: FnPtr,
         pub(super) BeginTransformFeedbackNV: FnPtr,
         pub(super) BeginVertexShaderEXT: FnPtr,
         pub(super) BeginVideoCaptureNV: FnPtr,
         pub(super) BindAttribLocation: FnPtr,
         pub(super) BindAttribLocationARB: FnPtr,
         pub(super) BindBuffer: FnPtr,
         pub(super) BindBufferARB: FnPtr,
         pub(super) BindBufferBase: FnPtr,
         pub(super) BindBufferBaseEXT: FnPtr,
         pub(super) BindBufferBaseNV: FnPtr,
         pub(super) BindBufferOffsetEXT: FnPtr,
         pub(super) BindBufferOffsetNV: FnPtr,
         pub(super) BindBufferRange: FnPtr,
         pub(super) BindBufferRangeEXT: FnPtr,
         pub(super) BindBufferRangeNV: FnPtr,
         pub(super) BindBuffersBase: FnPtr,
         pub(super) BindBuffersRange: FnPtr,
         pub(super) BindFragDataLocation: FnPtr,
         pub(super) BindFragDataLocationEXT: FnPtr,
         pub(super) BindFragDataLocationIndexed: FnPtr,
         pub(super) BindFragmentShaderATI: FnPtr,
         pub(super) BindFramebuffer: FnPtr,
         pub(super) BindFramebufferEXT: FnPtr,
         pub(super) BindImageTexture: FnPtr,
         pub(super) BindImageTextureEXT: FnPtr,
         pub(super) BindImageTextures: FnPtr,
         pub(super) BindLightParameterEXT: FnPtr,
         pub(super) BindMaterialParameterEXT: FnPtr,
         pub(super) BindMultiTextureEXT: FnPtr,
         pub(super) BindParameterEXT: FnPtr,
         pub(super) BindProgramARB: FnPtr,
         pub(super) BindProgramNV: FnPtr,
         pub(super) BindProgramPipeline: FnPtr,
         pub(super) BindRenderbuffer: FnPtr,
         pub(super) BindRenderbufferEXT: FnPtr,
         pub(super) BindSampler: FnPtr,
         pub(super) BindSamplers: FnPtr,
         pub(super) BindShadingRateImageNV: FnPtr,
         pub(super) BindTexGenParameterEXT: FnPtr,
         pub(super) BindTexture: FnPtr,
         pub(super) BindTextureEXT: FnPtr,
         pub(super) BindTextureUnit: FnPtr,
         pub(super) BindTextureUnitParameterEXT: FnPtr,
         pub(super) BindTextures: FnPtr,
         pub(super) BindTransformFeedback: FnPtr,
         pub(super) BindTransformFeedbackNV: FnPtr,
         pub(super) BindVertexArray: FnPtr,
         pub(super) BindVertexArrayAPPLE: FnPtr,
         pub(super) BindVertexBuffer: FnPtr,
         pub(super) BindVertexBuffers: FnPtr,
         pub(super) BindVertexShaderEXT: FnPtr,
         pub(super) BindVideoCaptureStreamBufferNV: FnPtr,
         pub(super) BindVideoCaptureStreamTextureNV: FnPtr,
         pub(super) Binormal3bEXT: FnPtr,
         pub(super) Binormal3bvEXT: FnPtr,
         pub(super) Binormal3dEXT: FnPtr,
         pub(super) Binormal3dvEXT: FnPtr,
         pub(super) Binormal3fEXT: FnPtr,
         pub(super) Binormal3fvEXT: FnPtr,
         pub(super) Binormal3iEXT: FnPtr,
         pub(super) Binormal3ivEXT: FnPtr,
         pub(super) Binormal3sEXT: FnPtr,
         pub(super) Binormal3svEXT: FnPtr,
         pub(super) BinormalPointerEXT: FnPtr,
         pub(super) BitmapxOES: FnPtr,
         pub(super) BlendBarrierKHR: FnPtr,
         pub(super) BlendBarrierNV: FnPtr,
         pub(super) BlendColor: FnPtr,
         pub(super) BlendColorEXT: FnPtr,
         pub(super) BlendColorxOES: FnPtr,
         pub(super) BlendEquation: FnPtr,
         pub(super) BlendEquationEXT: FnPtr,
         pub(super) BlendEquationIndexedAMD: FnPtr,
         pub(super) BlendEquationSeparate: FnPtr,
         pub(super) BlendEquationSeparateEXT: FnPtr,
         pub(super) BlendEquationSeparateIndexedAMD: FnPtr,
         pub(super) BlendEquationSeparatei: FnPtr,
         pub(super) BlendEquationSeparateiARB: FnPtr,
         pub(super) BlendEquationi: FnPtr,
         pub(super) BlendEquationiARB: FnPtr,
         pub(super) BlendFunc: FnPtr,
         pub(super) BlendFuncIndexedAMD: FnPtr,
         pub(super) BlendFuncSeparate: FnPtr,
         pub(super) BlendFuncSeparateEXT: FnPtr,
         pub(super) BlendFuncSeparateINGR: FnPtr,
         pub(super) BlendFuncSeparateIndexedAMD: FnPtr,
         pub(super) BlendFuncSeparatei: FnPtr,
         pub(super) BlendFuncSeparateiARB: FnPtr,
         pub(super) BlendFunci: FnPtr,
         pub(super) BlendFunciARB: FnPtr,
         pub(super) BlendParameteriNV: FnPtr,
         pub(super) BlitFramebuffer: FnPtr,
         pub(super) BlitFramebufferEXT: FnPtr,
         pub(super) BlitFramebufferLayerEXT: FnPtr,
         pub(super) BlitFramebufferLayersEXT: FnPtr,
         pub(super) BlitNamedFramebuffer: FnPtr,
         pub(super) BufferAddressRangeNV: FnPtr,
         pub(super) BufferAttachMemoryNV: FnPtr,
         pub(super) BufferData: FnPtr,
         pub(super) BufferDataARB: FnPtr,
         pub(super) BufferPageCommitmentARB: FnPtr,
         pub(super) BufferPageCommitmentMemNV: FnPtr,
         pub(super) BufferParameteriAPPLE: FnPtr,
         pub(super) BufferStorage: FnPtr,
         pub(super) BufferStorageExternalEXT: FnPtr,
         pub(super) BufferStorageMemEXT: FnPtr,
         pub(super) BufferSubData: FnPtr,
         pub(super) BufferSubDataARB: FnPtr,
         pub(super) CallCommandListNV: FnPtr,
         pub(super) CheckFramebufferStatus: FnPtr,
         pub(super) CheckFramebufferStatusEXT: FnPtr,
         pub(super) CheckNamedFramebufferStatus: FnPtr,
         pub(super) CheckNamedFramebufferStatusEXT: FnPtr,
         pub(super) ClampColor: FnPtr,
         pub(super) ClampColorARB: FnPtr,
         pub(super) Clear: FnPtr,
         pub(super) ClearAccumxOES: FnPtr,
         pub(super) ClearBufferData: FnPtr,
         pub(super) ClearBufferSubData: FnPtr,
         pub(super) ClearBufferfi: FnPtr,
         pub(super) ClearBufferfv: FnPtr,
         pub(super) ClearBufferiv: FnPtr,
         pub(super) ClearBufferuiv: FnPtr,
         pub(super) ClearColor: FnPtr,
         pub(super) ClearColorIiEXT: FnPtr,
         pub(super) ClearColorIuiEXT: FnPtr,
         pub(super) ClearColorxOES: FnPtr,
         pub(super) ClearDepth: FnPtr,
         pub(super) ClearDepthdNV: FnPtr,
         pub(super) ClearDepthf: FnPtr,
         pub(super) ClearDepthfOES: FnPtr,
         pub(super) ClearDepthxOES: FnPtr,
         pub(super) ClearNamedBufferData: FnPtr,
         pub(super) ClearNamedBufferDataEXT: FnPtr,
         pub(super) ClearNamedBufferSubData: FnPtr,
         pub(super) ClearNamedBufferSubDataEXT: FnPtr,
         pub(super) ClearNamedFramebufferfi: FnPtr,
         pub(super) ClearNamedFramebufferfv: FnPtr,
         pub(super) ClearNamedFramebufferiv: FnPtr,
         pub(super) ClearNamedFramebufferuiv: FnPtr,
         pub(super) ClearStencil: FnPtr,
         pub(super) ClearTexImage: FnPtr,
         pub(super) ClearTexSubImage: FnPtr,
         pub(super) ClientActiveTextureARB: FnPtr,
         pub(super) ClientActiveVertexStreamATI: FnPtr,
         pub(super) ClientAttribDefaultEXT: FnPtr,
         pub(super) ClientWaitSemaphoreui64NVX: FnPtr,
         pub(super) ClientWaitSync: FnPtr,
         pub(super) ClipControl: FnPtr,
         pub(super) ClipPlanefOES: FnPtr,
         pub(super) ClipPlanexOES: FnPtr,
         pub(super) Color3fVertex3fSUN: FnPtr,
         pub(super) Color3fVertex3fvSUN: FnPtr,
         pub(super) Color3hNV: FnPtr,
         pub(super) Color3hvNV: FnPtr,
         pub(super) Color3xOES: FnPtr,
         pub(super) Color3xvOES: FnPtr,
         pub(super) Color4fNormal3fVertex3fSUN: FnPtr,
         pub(super) Color4fNormal3fVertex3fvSUN: FnPtr,
         pub(super) Color4hNV: FnPtr,
         pub(super) Color4hvNV: FnPtr,
         pub(super) Color4ubVertex2fSUN: FnPtr,
         pub(super) Color4ubVertex2fvSUN: FnPtr,
         pub(super) Color4ubVertex3fSUN: FnPtr,
         pub(super) Color4ubVertex3fvSUN: FnPtr,
         pub(super) Color4xOES: FnPtr,
         pub(super) Color4xvOES: FnPtr,
         pub(super) ColorFormatNV: FnPtr,
         pub(super) ColorFragmentOp1ATI: FnPtr,
         pub(super) ColorFragmentOp2ATI: FnPtr,
         pub(super) ColorFragmentOp3ATI: FnPtr,
         pub(super) ColorMask: FnPtr,
         pub(super) ColorMaskIndexedEXT: FnPtr,
         pub(super) ColorMaski: FnPtr,
         pub(super) ColorPointerEXT: FnPtr,
         pub(super) ColorPointerListIBM: FnPtr,
         pub(super) ColorPointervINTEL: FnPtr,
         pub(super) ColorSubTableEXT: FnPtr,
         pub(super) ColorTableEXT: FnPtr,
         pub(super) ColorTableParameterfvSGI: FnPtr,
         pub(super) ColorTableParameterivSGI: FnPtr,
         pub(super) ColorTableSGI: FnPtr,
         pub(super) CombinerInputNV: FnPtr,
         pub(super) CombinerOutputNV: FnPtr,
         pub(super) CombinerParameterfNV: FnPtr,
         pub(super) CombinerParameterfvNV: FnPtr,
         pub(super) CombinerParameteriNV: FnPtr,
         pub(super) CombinerParameterivNV: FnPtr,
         pub(super) CombinerStageParameterfvNV: FnPtr,
         pub(super) CommandListSegmentsNV: FnPtr,
         pub(super) CompileCommandListNV: FnPtr,
         pub(super) CompileShader: FnPtr,
         pub(super) CompileShaderARB: FnPtr,
         pub(super) CompileShaderIncludeARB: FnPtr,
         pub(super) CompressedMultiTexImage1DEXT: FnPtr,
         pub(super) CompressedMultiTexImage2DEXT: FnPtr,
         pub(super) CompressedMultiTexImage3DEXT: FnPtr,
         pub(super) CompressedMultiTexSubImage1DEXT: FnPtr,
         pub(super) CompressedMultiTexSubImage2DEXT: FnPtr,
         pub(super) CompressedMultiTexSubImage3DEXT: FnPtr,
         pub(super) CompressedTexImage1D: FnPtr,
         pub(super) CompressedTexImage1DARB: FnPtr,
         pub(super) CompressedTexImage2D: FnPtr,
         pub(super) CompressedTexImage2DARB: FnPtr,
         pub(super) CompressedTexImage3D: FnPtr,
         pub(super) CompressedTexImage3DARB: FnPtr,
         pub(super) CompressedTexSubImage1D: FnPtr,
         pub(super) CompressedTexSubImage1DARB: FnPtr,
         pub(super) CompressedTexSubImage2D: FnPtr,
         pub(super) CompressedTexSubImage2DARB: FnPtr,
         pub(super) CompressedTexSubImage3D: FnPtr,
         pub(super) CompressedTexSubImage3DARB: FnPtr,
         pub(super) CompressedTextureImage1DEXT: FnPtr,
         pub(super) CompressedTextureImage2DEXT: FnPtr,
         pub(super) CompressedTextureImage3DEXT: FnPtr,
         pub(super) CompressedTextureSubImage1D: FnPtr,
         pub(super) CompressedTextureSubImage1DEXT: FnPtr,
         pub(super) CompressedTextureSubImage2D: FnPtr,
         pub(super) CompressedTextureSubImage2DEXT: FnPtr,
         pub(super) CompressedTextureSubImage3D: FnPtr,
         pub(super) CompressedTextureSubImage3DEXT: FnPtr,
         pub(super) ConservativeRasterParameterfNV: FnPtr,
         pub(super) ConservativeRasterParameteriNV: FnPtr,
         pub(super) ConvolutionFilter1DEXT: FnPtr,
         pub(super) ConvolutionFilter2DEXT: FnPtr,
         pub(super) ConvolutionParameterfEXT: FnPtr,
         pub(super) ConvolutionParameterfvEXT: FnPtr,
         pub(super) ConvolutionParameteriEXT: FnPtr,
         pub(super) ConvolutionParameterivEXT: FnPtr,
         pub(super) ConvolutionParameterxOES: FnPtr,
         pub(super) ConvolutionParameterxvOES: FnPtr,
         pub(super) CopyBufferSubData: FnPtr,
         pub(super) CopyColorSubTableEXT: FnPtr,
         pub(super) CopyColorTableSGI: FnPtr,
         pub(super) CopyConvolutionFilter1DEXT: FnPtr,
         pub(super) CopyConvolutionFilter2DEXT: FnPtr,
         pub(super) CopyImageSubData: FnPtr,
         pub(super) CopyImageSubDataNV: FnPtr,
         pub(super) CopyMultiTexImage1DEXT: FnPtr,
         pub(super) CopyMultiTexImage2DEXT: FnPtr,
         pub(super) CopyMultiTexSubImage1DEXT: FnPtr,
         pub(super) CopyMultiTexSubImage2DEXT: FnPtr,
         pub(super) CopyMultiTexSubImage3DEXT: FnPtr,
         pub(super) CopyNamedBufferSubData: FnPtr,
         pub(super) CopyPathNV: FnPtr,
         pub(super) CopyTexImage1D: FnPtr,
         pub(super) CopyTexImage1DEXT: FnPtr,
         pub(super) CopyTexImage2D: FnPtr,
         pub(super) CopyTexImage2DEXT: FnPtr,
         pub(super) CopyTexSubImage1D: FnPtr,
         pub(super) CopyTexSubImage1DEXT: FnPtr,
         pub(super) CopyTexSubImage2D: FnPtr,
         pub(super) CopyTexSubImage2DEXT: FnPtr,
         pub(super) CopyTexSubImage3D: FnPtr,
         pub(super) CopyTexSubImage3DEXT: FnPtr,
         pub(super) CopyTextureImage1DEXT: FnPtr,
         pub(super) CopyTextureImage2DEXT: FnPtr,
         pub(super) CopyTextureSubImage1D: FnPtr,
         pub(super) CopyTextureSubImage1DEXT: FnPtr,
         pub(super) CopyTextureSubImage2D: FnPtr,
         pub(super) CopyTextureSubImage2DEXT: FnPtr,
         pub(super) CopyTextureSubImage3D: FnPtr,
         pub(super) CopyTextureSubImage3DEXT: FnPtr,
         pub(super) CoverFillPathInstancedNV: FnPtr,
         pub(super) CoverFillPathNV: FnPtr,
         pub(super) CoverStrokePathInstancedNV: FnPtr,
         pub(super) CoverStrokePathNV: FnPtr,
         pub(super) CoverageModulationNV: FnPtr,
         pub(super) CoverageModulationTableNV: FnPtr,
         pub(super) CreateBuffers: FnPtr,
         pub(super) CreateCommandListsNV: FnPtr,
         pub(super) CreateFramebuffers: FnPtr,
         pub(super) CreateMemoryObjectsEXT: FnPtr,
         pub(super) CreatePerfQueryINTEL: FnPtr,
         pub(super) CreateProgram: FnPtr,
         pub(super) CreateProgramObjectARB: FnPtr,
         pub(super) CreateProgramPipelines: FnPtr,
         pub(super) CreateProgressFenceNVX: FnPtr,
         pub(super) CreateQueries: FnPtr,
         pub(super) CreateRenderbuffers: FnPtr,
         pub(super) CreateSamplers: FnPtr,
         pub(super) CreateSemaphoresNV: FnPtr,
         pub(super) CreateShader: FnPtr,
         pub(super) CreateShaderObjectARB: FnPtr,
         pub(super) CreateShaderProgramEXT: FnPtr,
         pub(super) CreateShaderProgramv: FnPtr,
         pub(super) CreateStatesNV: FnPtr,
         pub(super) CreateSyncFromCLeventARB: FnPtr,
         pub(super) CreateTextures: FnPtr,
         pub(super) CreateTransformFeedbacks: FnPtr,
         pub(super) CreateVertexArrays: FnPtr,
         pub(super) CullFace: FnPtr,
         pub(super) CullParameterdvEXT: FnPtr,
         pub(super) CullParameterfvEXT: FnPtr,
         pub(super) CurrentPaletteMatrixARB: FnPtr,
         pub(super) DebugMessageCallback: FnPtr,
         pub(super) DebugMessageCallbackAMD: FnPtr,
         pub(super) DebugMessageCallbackARB: FnPtr,
         pub(super) DebugMessageControl: FnPtr,
         pub(super) DebugMessageControlARB: FnPtr,
         pub(super) DebugMessageEnableAMD: FnPtr,
         pub(super) DebugMessageInsert: FnPtr,
         pub(super) DebugMessageInsertAMD: FnPtr,
         pub(super) DebugMessageInsertARB: FnPtr,
         pub(super) DeformSGIX: FnPtr,
         pub(super) DeformationMap3dSGIX: FnPtr,
         pub(super) DeformationMap3fSGIX: FnPtr,
         pub(super) DeleteAsyncMarkersSGIX: FnPtr,
         pub(super) DeleteBuffers: FnPtr,
         pub(super) DeleteBuffersARB: FnPtr,
         pub(super) DeleteCommandListsNV: FnPtr,
         pub(super) DeleteFencesAPPLE: FnPtr,
         pub(super) DeleteFencesNV: FnPtr,
         pub(super) DeleteFragmentShaderATI: FnPtr,
         pub(super) DeleteFramebuffers: FnPtr,
         pub(super) DeleteFramebuffersEXT: FnPtr,
         pub(super) DeleteMemoryObjectsEXT: FnPtr,
         pub(super) DeleteNamedStringARB: FnPtr,
         pub(super) DeleteNamesAMD: FnPtr,
         pub(super) DeleteObjectARB: FnPtr,
         pub(super) DeleteOcclusionQueriesNV: FnPtr,
         pub(super) DeletePathsNV: FnPtr,
         pub(super) DeletePerfMonitorsAMD: FnPtr,
         pub(super) DeletePerfQueryINTEL: FnPtr,
         pub(super) DeleteProgram: FnPtr,
         pub(super) DeleteProgramPipelines: FnPtr,
         pub(super) DeleteProgramsARB: FnPtr,
         pub(super) DeleteProgramsNV: FnPtr,
         pub(super) DeleteQueries: FnPtr,
         pub(super) DeleteQueriesARB: FnPtr,
         pub(super) DeleteQueryResourceTagNV: FnPtr,
         pub(super) DeleteRenderbuffers: FnPtr,
         pub(super) DeleteRenderbuffersEXT: FnPtr,
         pub(super) DeleteSamplers: FnPtr,
         pub(super) DeleteSemaphoresEXT: FnPtr,
         pub(super) DeleteShader: FnPtr,
         pub(super) DeleteStatesNV: FnPtr,
         pub(super) DeleteSync: FnPtr,
         pub(super) DeleteTextures: FnPtr,
         pub(super) DeleteTexturesEXT: FnPtr,
         pub(super) DeleteTransformFeedbacks: FnPtr,
         pub(super) DeleteTransformFeedbacksNV: FnPtr,
         pub(super) DeleteVertexArrays: FnPtr,
         pub(super) DeleteVertexArraysAPPLE: FnPtr,
         pub(super) DeleteVertexShaderEXT: FnPtr,
         pub(super) DepthBoundsEXT: FnPtr,
         pub(super) DepthBoundsdNV: FnPtr,
         pub(super) DepthFunc: FnPtr,
         pub(super) DepthMask: FnPtr,
         pub(super) DepthRange: FnPtr,
         pub(super) DepthRangeArraydvNV: FnPtr,
         pub(super) DepthRangeArrayv: FnPtr,
         pub(super) DepthRangeIndexed: FnPtr,
         pub(super) DepthRangeIndexeddNV: FnPtr,
         pub(super) DepthRangedNV: FnPtr,
         pub(super) DepthRangef: FnPtr,
         pub(super) DepthRangefOES: FnPtr,
         pub(super) DepthRangexOES: FnPtr,
         pub(super) DetachObjectARB: FnPtr,
         pub(super) DetachShader: FnPtr,
         pub(super) DetailTexFuncSGIS: FnPtr,
         pub(super) Disable: FnPtr,
         pub(super) DisableClientStateIndexedEXT: FnPtr,
         pub(super) DisableClientStateiEXT: FnPtr,
         pub(super) DisableIndexedEXT: FnPtr,
         pub(super) DisableVariantClientStateEXT: FnPtr,
         pub(super) DisableVertexArrayAttrib: FnPtr,
         pub(super) DisableVertexArrayAttribEXT: FnPtr,
         pub(super) DisableVertexArrayEXT: FnPtr,
         pub(super) DisableVertexAttribAPPLE: FnPtr,
         pub(super) DisableVertexAttribArray: FnPtr,
         pub(super) DisableVertexAttribArrayARB: FnPtr,
         pub(super) Disablei: FnPtr,
         pub(super) DispatchCompute: FnPtr,
         pub(super) DispatchComputeGroupSizeARB: FnPtr,
         pub(super) DispatchComputeIndirect: FnPtr,
         pub(super) DrawArrays: FnPtr,
         pub(super) DrawArraysEXT: FnPtr,
         pub(super) DrawArraysIndirect: FnPtr,
         pub(super) DrawArraysInstanced: FnPtr,
         pub(super) DrawArraysInstancedARB: FnPtr,
         pub(super) DrawArraysInstancedBaseInstance: FnPtr,
         pub(super) DrawArraysInstancedEXT: FnPtr,
         pub(super) DrawBuffer: FnPtr,
         pub(super) DrawBuffers: FnPtr,
         pub(super) DrawBuffersARB: FnPtr,
         pub(super) DrawBuffersATI: FnPtr,
         pub(super) DrawCommandsAddressNV: FnPtr,
         pub(super) DrawCommandsNV: FnPtr,
         pub(super) DrawCommandsStatesAddressNV: FnPtr,
         pub(super) DrawCommandsStatesNV: FnPtr,
         pub(super) DrawElementArrayAPPLE: FnPtr,
         pub(super) DrawElementArrayATI: FnPtr,
         pub(super) DrawElements: FnPtr,
         pub(super) DrawElementsBaseVertex: FnPtr,
         pub(super) DrawElementsIndirect: FnPtr,
         pub(super) DrawElementsInstanced: FnPtr,
         pub(super) DrawElementsInstancedARB: FnPtr,
         pub(super) DrawElementsInstancedBaseInstance: FnPtr,
         pub(super) DrawElementsInstancedBaseVertex: FnPtr,
         pub(super) DrawElementsInstancedBaseVertexBaseInstance: FnPtr,
         pub(super) DrawElementsInstancedEXT: FnPtr,
         pub(super) DrawMeshArraysSUN: FnPtr,
         pub(super) DrawMeshTasksIndirectNV: FnPtr,
         pub(super) DrawMeshTasksNV: FnPtr,
         pub(super) DrawRangeElementArrayAPPLE: FnPtr,
         pub(super) DrawRangeElementArrayATI: FnPtr,
         pub(super) DrawRangeElements: FnPtr,
         pub(super) DrawRangeElementsBaseVertex: FnPtr,
         pub(super) DrawRangeElementsEXT: FnPtr,
         pub(super) DrawTextureNV: FnPtr,
         pub(super) DrawTransformFeedback: FnPtr,
         pub(super) DrawTransformFeedbackInstanced: FnPtr,
         pub(super) DrawTransformFeedbackNV: FnPtr,
         pub(super) DrawTransformFeedbackStream: FnPtr,
         pub(super) DrawTransformFeedbackStreamInstanced: FnPtr,
         pub(super) DrawVkImageNV: FnPtr,
         pub(super) EGLImageTargetTexStorageEXT: FnPtr,
         pub(super) EGLImageTargetTextureStorageEXT: FnPtr,
         pub(super) EdgeFlagFormatNV: FnPtr,
         pub(super) EdgeFlagPointerEXT: FnPtr,
         pub(super) EdgeFlagPointerListIBM: FnPtr,
         pub(super) ElementPointerAPPLE: FnPtr,
         pub(super) ElementPointerATI: FnPtr,
         pub(super) Enable: FnPtr,
         pub(super) EnableClientStateIndexedEXT: FnPtr,
         pub(super) EnableClientStateiEXT: FnPtr,
         pub(super) EnableIndexedEXT: FnPtr,
         pub(super) EnableVariantClientStateEXT: FnPtr,
         pub(super) EnableVertexArrayAttrib: FnPtr,
         pub(super) EnableVertexArrayAttribEXT: FnPtr,
         pub(super) EnableVertexArrayEXT: FnPtr,
         pub(super) EnableVertexAttribAPPLE: FnPtr,
         pub(super) EnableVertexAttribArray: FnPtr,
         pub(super) EnableVertexAttribArrayARB: FnPtr,
         pub(super) Enablei: FnPtr,
         pub(super) EndConditionalRender: FnPtr,
         pub(super) EndConditionalRenderNV: FnPtr,
         pub(super) EndConditionalRenderNVX: FnPtr,
         pub(super) EndFragmentShaderATI: FnPtr,
         pub(super) EndOcclusionQueryNV: FnPtr,
         pub(super) EndPerfMonitorAMD: FnPtr,
         pub(super) EndPerfQueryINTEL: FnPtr,
         pub(super) EndQuery: FnPtr,
         pub(super) EndQueryARB: FnPtr,
         pub(super) EndQueryIndexed: FnPtr,
         pub(super) EndTransformFeedback: FnPtr,
         pub(super) EndTransformFeedbackEXT: FnPtr,
         pub(super) EndTransformFeedbackNV: FnPtr,
         pub(super) EndVertexShaderEXT: FnPtr,
         pub(super) EndVideoCaptureNV: FnPtr,
         pub(super) EvalCoord1xOES: FnPtr,
         pub(super) EvalCoord1xvOES: FnPtr,
         pub(super) EvalCoord2xOES: FnPtr,
         pub(super) EvalCoord2xvOES: FnPtr,
         pub(super) EvalMapsNV: FnPtr,
         pub(super) EvaluateDepthValuesARB: FnPtr,
         pub(super) ExecuteProgramNV: FnPtr,
         pub(super) ExtractComponentEXT: FnPtr,
         pub(super) FeedbackBufferxOES: FnPtr,
         pub(super) FenceSync: FnPtr,
         pub(super) FinalCombinerInputNV: FnPtr,
         pub(super) Finish: FnPtr,
         pub(super) FinishAsyncSGIX: FnPtr,
         pub(super) FinishFenceAPPLE: FnPtr,
         pub(super) FinishFenceNV: FnPtr,
         pub(super) FinishObjectAPPLE: FnPtr,
         pub(super) FinishTextureSUNX: FnPtr,
         pub(super) Flush: FnPtr,
         pub(super) FlushMappedBufferRange: FnPtr,
         pub(super) FlushMappedBufferRangeAPPLE: FnPtr,
         pub(super) FlushMappedNamedBufferRange: FnPtr,
         pub(super) FlushMappedNamedBufferRangeEXT: FnPtr,
         pub(super) FlushPixelDataRangeNV: FnPtr,
         pub(super) FlushRasterSGIX: FnPtr,
         pub(super) FlushStaticDataIBM: FnPtr,
         pub(super) FlushVertexArrayRangeAPPLE: FnPtr,
         pub(super) FlushVertexArrayRangeNV: FnPtr,
         pub(super) FogCoordFormatNV: FnPtr,
         pub(super) FogCoordPointerEXT: FnPtr,
         pub(super) FogCoordPointerListIBM: FnPtr,
         pub(super) FogCoorddEXT: FnPtr,
         pub(super) FogCoorddvEXT: FnPtr,
         pub(super) FogCoordfEXT: FnPtr,
         pub(super) FogCoordfvEXT: FnPtr,
         pub(super) FogCoordhNV: FnPtr,
         pub(super) FogCoordhvNV: FnPtr,
         pub(super) FogFuncSGIS: FnPtr,
         pub(super) FogxOES: FnPtr,
         pub(super) FogxvOES: FnPtr,
         pub(super) FragmentColorMaterialSGIX: FnPtr,
         pub(super) FragmentCoverageColorNV: FnPtr,
         pub(super) FragmentLightModelfSGIX: FnPtr,
         pub(super) FragmentLightModelfvSGIX: FnPtr,
         pub(super) FragmentLightModeliSGIX: FnPtr,
         pub(super) FragmentLightModelivSGIX: FnPtr,
         pub(super) FragmentLightfSGIX: FnPtr,
         pub(super) FragmentLightfvSGIX: FnPtr,
         pub(super) FragmentLightiSGIX: FnPtr,
         pub(super) FragmentLightivSGIX: FnPtr,
         pub(super) FragmentMaterialfSGIX: FnPtr,
         pub(super) FragmentMaterialfvSGIX: FnPtr,
         pub(super) FragmentMaterialiSGIX: FnPtr,
         pub(super) FragmentMaterialivSGIX: FnPtr,
         pub(super) FrameTerminatorGREMEDY: FnPtr,
         pub(super) FrameZoomSGIX: FnPtr,
         pub(super) FramebufferDrawBufferEXT: FnPtr,
         pub(super) FramebufferDrawBuffersEXT: FnPtr,
         pub(super) FramebufferFetchBarrierEXT: FnPtr,
         pub(super) FramebufferParameteri: FnPtr,
         pub(super) FramebufferParameteriMESA: FnPtr,
         pub(super) FramebufferReadBufferEXT: FnPtr,
         pub(super) FramebufferRenderbuffer: FnPtr,
         pub(super) FramebufferRenderbufferEXT: FnPtr,
         pub(super) FramebufferSampleLocationsfvARB: FnPtr,
         pub(super) FramebufferSampleLocationsfvNV: FnPtr,
         pub(super) FramebufferSamplePositionsfvAMD: FnPtr,
         pub(super) FramebufferTexture: FnPtr,
         pub(super) FramebufferTexture1D: FnPtr,
         pub(super) FramebufferTexture1DEXT: FnPtr,
         pub(super) FramebufferTexture2D: FnPtr,
         pub(super) FramebufferTexture2DEXT: FnPtr,
         pub(super) FramebufferTexture3D: FnPtr,
         pub(super) FramebufferTexture3DEXT: FnPtr,
         pub(super) FramebufferTextureARB: FnPtr,
         pub(super) FramebufferTextureEXT: FnPtr,
         pub(super) FramebufferTextureFaceARB: FnPtr,
         pub(super) FramebufferTextureFaceEXT: FnPtr,
         pub(super) FramebufferTextureLayer: FnPtr,
         pub(super) FramebufferTextureLayerARB: FnPtr,
         pub(super) FramebufferTextureLayerEXT: FnPtr,
         pub(super) FramebufferTextureMultiviewOVR: FnPtr,
         pub(super) FreeObjectBufferATI: FnPtr,
         pub(super) FrontFace: FnPtr,
         pub(super) FrustumfOES: FnPtr,
         pub(super) FrustumxOES: FnPtr,
         pub(super) GenAsyncMarkersSGIX: FnPtr,
         pub(super) GenBuffers: FnPtr,
         pub(super) GenBuffersARB: FnPtr,
         pub(super) GenFencesAPPLE: FnPtr,
         pub(super) GenFencesNV: FnPtr,
         pub(super) GenFragmentShadersATI: FnPtr,
         pub(super) GenFramebuffers: FnPtr,
         pub(super) GenFramebuffersEXT: FnPtr,
         pub(super) GenNamesAMD: FnPtr,
         pub(super) GenOcclusionQueriesNV: FnPtr,
         pub(super) GenPathsNV: FnPtr,
         pub(super) GenPerfMonitorsAMD: FnPtr,
         pub(super) GenProgramPipelines: FnPtr,
         pub(super) GenProgramsARB: FnPtr,
         pub(super) GenProgramsNV: FnPtr,
         pub(super) GenQueries: FnPtr,
         pub(super) GenQueriesARB: FnPtr,
         pub(super) GenQueryResourceTagNV: FnPtr,
         pub(super) GenRenderbuffers: FnPtr,
         pub(super) GenRenderbuffersEXT: FnPtr,
         pub(super) GenSamplers: FnPtr,
         pub(super) GenSemaphoresEXT: FnPtr,
         pub(super) GenSymbolsEXT: FnPtr,
         pub(super) GenTextures: FnPtr,
         pub(super) GenTexturesEXT: FnPtr,
         pub(super) GenTransformFeedbacks: FnPtr,
         pub(super) GenTransformFeedbacksNV: FnPtr,
         pub(super) GenVertexArrays: FnPtr,
         pub(super) GenVertexArraysAPPLE: FnPtr,
         pub(super) GenVertexShadersEXT: FnPtr,
         pub(super) GenerateMipmap: FnPtr,
         pub(super) GenerateMipmapEXT: FnPtr,
         pub(super) GenerateMultiTexMipmapEXT: FnPtr,
         pub(super) GenerateTextureMipmap: FnPtr,
         pub(super) GenerateTextureMipmapEXT: FnPtr,
         pub(super) GetActiveAtomicCounterBufferiv: FnPtr,
         pub(super) GetActiveAttrib: FnPtr,
         pub(super) GetActiveAttribARB: FnPtr,
         pub(super) GetActiveSubroutineName: FnPtr,
         pub(super) GetActiveSubroutineUniformName: FnPtr,
         pub(super) GetActiveSubroutineUniformiv: FnPtr,
         pub(super) GetActiveUniform: FnPtr,
         pub(super) GetActiveUniformARB: FnPtr,
         pub(super) GetActiveUniformBlockName: FnPtr,
         pub(super) GetActiveUniformBlockiv: FnPtr,
         pub(super) GetActiveUniformName: FnPtr,
         pub(super) GetActiveUniformsiv: FnPtr,
         pub(super) GetActiveVaryingNV: FnPtr,
         pub(super) GetArrayObjectfvATI: FnPtr,
         pub(super) GetArrayObjectivATI: FnPtr,
         pub(super) GetAttachedObjectsARB: FnPtr,
         pub(super) GetAttachedShaders: FnPtr,
         pub(super) GetAttribLocation: FnPtr,
         pub(super) GetAttribLocationARB: FnPtr,
         pub(super) GetBooleanIndexedvEXT: FnPtr,
         pub(super) GetBooleani_v: FnPtr,
         pub(super) GetBooleanv: FnPtr,
         pub(super) GetBufferParameteri64v: FnPtr,
         pub(super) GetBufferParameteriv: FnPtr,
         pub(super) GetBufferParameterivARB: FnPtr,
         pub(super) GetBufferParameterui64vNV: FnPtr,
         pub(super) GetBufferPointerv: FnPtr,
         pub(super) GetBufferPointervARB: FnPtr,
         pub(super) GetBufferSubData: FnPtr,
         pub(super) GetBufferSubDataARB: FnPtr,
         pub(super) GetClipPlanefOES: FnPtr,
         pub(super) GetClipPlanexOES: FnPtr,
         pub(super) GetColorTableEXT: FnPtr,
         pub(super) GetColorTableParameterfvEXT: FnPtr,
         pub(super) GetColorTableParameterfvSGI: FnPtr,
         pub(super) GetColorTableParameterivEXT: FnPtr,
         pub(super) GetColorTableParameterivSGI: FnPtr,
         pub(super) GetColorTableSGI: FnPtr,
         pub(super) GetCombinerInputParameterfvNV: FnPtr,
         pub(super) GetCombinerInputParameterivNV: FnPtr,
         pub(super) GetCombinerOutputParameterfvNV: FnPtr,
         pub(super) GetCombinerOutputParameterivNV: FnPtr,
         pub(super) GetCombinerStageParameterfvNV: FnPtr,
         pub(super) GetCommandHeaderNV: FnPtr,
         pub(super) GetCompressedMultiTexImageEXT: FnPtr,
         pub(super) GetCompressedTexImage: FnPtr,
         pub(super) GetCompressedTexImageARB: FnPtr,
         pub(super) GetCompressedTextureImage: FnPtr,
         pub(super) GetCompressedTextureImageEXT: FnPtr,
         pub(super) GetCompressedTextureSubImage: FnPtr,
         pub(super) GetConvolutionFilterEXT: FnPtr,
         pub(super) GetConvolutionParameterfvEXT: FnPtr,
         pub(super) GetConvolutionParameterivEXT: FnPtr,
         pub(super) GetConvolutionParameterxvOES: FnPtr,
         pub(super) GetCoverageModulationTableNV: FnPtr,
         pub(super) GetDebugMessageLog: FnPtr,
         pub(super) GetDebugMessageLogAMD: FnPtr,
         pub(super) GetDebugMessageLogARB: FnPtr,
         pub(super) GetDetailTexFuncSGIS: FnPtr,
         pub(super) GetDoubleIndexedvEXT: FnPtr,
         pub(super) GetDoublei_v: FnPtr,
         pub(super) GetDoublei_vEXT: FnPtr,
         pub(super) GetDoublev: FnPtr,
         pub(super) GetError: FnPtr,
         pub(super) GetFenceivNV: FnPtr,
         pub(super) GetFinalCombinerInputParameterfvNV: FnPtr,
         pub(super) GetFinalCombinerInputParameterivNV: FnPtr,
         pub(super) GetFirstPerfQueryIdINTEL: FnPtr,
         pub(super) GetFixedvOES: FnPtr,
         pub(super) GetFloatIndexedvEXT: FnPtr,
         pub(super) GetFloati_v: FnPtr,
         pub(super) GetFloati_vEXT: FnPtr,
         pub(super) GetFloatv: FnPtr,
         pub(super) GetFogFuncSGIS: FnPtr,
         pub(super) GetFragDataIndex: FnPtr,
         pub(super) GetFragDataLocation: FnPtr,
         pub(super) GetFragDataLocationEXT: FnPtr,
         pub(super) GetFragmentLightfvSGIX: FnPtr,
         pub(super) GetFragmentLightivSGIX: FnPtr,
         pub(super) GetFragmentMaterialfvSGIX: FnPtr,
         pub(super) GetFragmentMaterialivSGIX: FnPtr,
         pub(super) GetFramebufferAttachmentParameteriv: FnPtr,
         pub(super) GetFramebufferAttachmentParameterivEXT: FnPtr,
         pub(super) GetFramebufferParameterfvAMD: FnPtr,
         pub(super) GetFramebufferParameteriv: FnPtr,
         pub(super) GetFramebufferParameterivEXT: FnPtr,
         pub(super) GetFramebufferParameterivMESA: FnPtr,
         pub(super) GetGraphicsResetStatus: FnPtr,
         pub(super) GetGraphicsResetStatusARB: FnPtr,
         pub(super) GetHandleARB: FnPtr,
         pub(super) GetHistogramEXT: FnPtr,
         pub(super) GetHistogramParameterfvEXT: FnPtr,
         pub(super) GetHistogramParameterivEXT: FnPtr,
         pub(super) GetHistogramParameterxvOES: FnPtr,
         pub(super) GetImageHandleARB: FnPtr,
         pub(super) GetImageHandleNV: FnPtr,
         pub(super) GetImageTransformParameterfvHP: FnPtr,
         pub(super) GetImageTransformParameterivHP: FnPtr,
         pub(super) GetInfoLogARB: FnPtr,
         pub(super) GetInstrumentsSGIX: FnPtr,
         pub(super) GetInteger64i_v: FnPtr,
         pub(super) GetInteger64v: FnPtr,
         pub(super) GetIntegerIndexedvEXT: FnPtr,
         pub(super) GetIntegeri_v: FnPtr,
         pub(super) GetIntegerui64i_vNV: FnPtr,
         pub(super) GetIntegerui64vNV: FnPtr,
         pub(super) GetIntegerv: FnPtr,
         pub(super) GetInternalformatSampleivNV: FnPtr,
         pub(super) GetInternalformati64v: FnPtr,
         pub(super) GetInternalformativ: FnPtr,
         pub(super) GetInvariantBooleanvEXT: FnPtr,
         pub(super) GetInvariantFloatvEXT: FnPtr,
         pub(super) GetInvariantIntegervEXT: FnPtr,
         pub(super) GetLightxOES: FnPtr,
         pub(super) GetListParameterfvSGIX: FnPtr,
         pub(super) GetListParameterivSGIX: FnPtr,
         pub(super) GetLocalConstantBooleanvEXT: FnPtr,
         pub(super) GetLocalConstantFloatvEXT: FnPtr,
         pub(super) GetLocalConstantIntegervEXT: FnPtr,
         pub(super) GetMapAttribParameterfvNV: FnPtr,
         pub(super) GetMapAttribParameterivNV: FnPtr,
         pub(super) GetMapControlPointsNV: FnPtr,
         pub(super) GetMapParameterfvNV: FnPtr,
         pub(super) GetMapParameterivNV: FnPtr,
         pub(super) GetMapxvOES: FnPtr,
         pub(super) GetMaterialxOES: FnPtr,
         pub(super) GetMemoryObjectDetachedResourcesuivNV: FnPtr,
         pub(super) GetMemoryObjectParameterivEXT: FnPtr,
         pub(super) GetMinmaxEXT: FnPtr,
         pub(super) GetMinmaxParameterfvEXT: FnPtr,
         pub(super) GetMinmaxParameterivEXT: FnPtr,
         pub(super) GetMultiTexEnvfvEXT: FnPtr,
         pub(super) GetMultiTexEnvivEXT: FnPtr,
         pub(super) GetMultiTexGendvEXT: FnPtr,
         pub(super) GetMultiTexGenfvEXT: FnPtr,
         pub(super) GetMultiTexGenivEXT: FnPtr,
         pub(super) GetMultiTexImageEXT: FnPtr,
         pub(super) GetMultiTexLevelParameterfvEXT: FnPtr,
         pub(super) GetMultiTexLevelParameterivEXT: FnPtr,
         pub(super) GetMultiTexParameterIivEXT: FnPtr,
         pub(super) GetMultiTexParameterIuivEXT: FnPtr,
         pub(super) GetMultiTexParameterfvEXT: FnPtr,
         pub(super) GetMultiTexParameterivEXT: FnPtr,
         pub(super) GetMultisamplefv: FnPtr,
         pub(super) GetMultisamplefvNV: FnPtr,
         pub(super) GetNamedBufferParameteri64v: FnPtr,
         pub(super) GetNamedBufferParameteriv: FnPtr,
         pub(super) GetNamedBufferParameterivEXT: FnPtr,
         pub(super) GetNamedBufferParameterui64vNV: FnPtr,
         pub(super) GetNamedBufferPointerv: FnPtr,
         pub(super) GetNamedBufferPointervEXT: FnPtr,
         pub(super) GetNamedBufferSubData: FnPtr,
         pub(super) GetNamedBufferSubDataEXT: FnPtr,
         pub(super) GetNamedFramebufferAttachmentParameteriv: FnPtr,
         pub(super) GetNamedFramebufferAttachmentParameterivEXT: FnPtr,
         pub(super) GetNamedFramebufferParameterfvAMD: FnPtr,
         pub(super) GetNamedFramebufferParameteriv: FnPtr,
         pub(super) GetNamedFramebufferParameterivEXT: FnPtr,
         pub(super) GetNamedProgramLocalParameterIivEXT: FnPtr,
         pub(super) GetNamedProgramLocalParameterIuivEXT: FnPtr,
         pub(super) GetNamedProgramLocalParameterdvEXT: FnPtr,
         pub(super) GetNamedProgramLocalParameterfvEXT: FnPtr,
         pub(super) GetNamedProgramStringEXT: FnPtr,
         pub(super) GetNamedProgramivEXT: FnPtr,
         pub(super) GetNamedRenderbufferParameteriv: FnPtr,
         pub(super) GetNamedRenderbufferParameterivEXT: FnPtr,
         pub(super) GetNamedStringARB: FnPtr,
         pub(super) GetNamedStringivARB: FnPtr,
         pub(super) GetNextPerfQueryIdINTEL: FnPtr,
         pub(super) GetObjectBufferfvATI: FnPtr,
         pub(super) GetObjectBufferivATI: FnPtr,
         pub(super) GetObjectLabel: FnPtr,
         pub(super) GetObjectLabelEXT: FnPtr,
         pub(super) GetObjectParameterfvARB: FnPtr,
         pub(super) GetObjectParameterivAPPLE: FnPtr,
         pub(super) GetObjectParameterivARB: FnPtr,
         pub(super) GetObjectPtrLabel: FnPtr,
         pub(super) GetOcclusionQueryivNV: FnPtr,
         pub(super) GetOcclusionQueryuivNV: FnPtr,
         pub(super) GetPathCommandsNV: FnPtr,
         pub(super) GetPathCoordsNV: FnPtr,
         pub(super) GetPathDashArrayNV: FnPtr,
         pub(super) GetPathLengthNV: FnPtr,
         pub(super) GetPathMetricRangeNV: FnPtr,
         pub(super) GetPathMetricsNV: FnPtr,
         pub(super) GetPathParameterfvNV: FnPtr,
         pub(super) GetPathParameterivNV: FnPtr,
         pub(super) GetPathSpacingNV: FnPtr,
         pub(super) GetPerfCounterInfoINTEL: FnPtr,
         pub(super) GetPerfMonitorCounterDataAMD: FnPtr,
         pub(super) GetPerfMonitorCounterInfoAMD: FnPtr,
         pub(super) GetPerfMonitorCounterStringAMD: FnPtr,
         pub(super) GetPerfMonitorCountersAMD: FnPtr,
         pub(super) GetPerfMonitorGroupStringAMD: FnPtr,
         pub(super) GetPerfMonitorGroupsAMD: FnPtr,
         pub(super) GetPerfQueryDataINTEL: FnPtr,
         pub(super) GetPerfQueryIdByNameINTEL: FnPtr,
         pub(super) GetPerfQueryInfoINTEL: FnPtr,
         pub(super) GetPixelMapxv: FnPtr,
         pub(super) GetPixelTexGenParameterfvSGIS: FnPtr,
         pub(super) GetPixelTexGenParameterivSGIS: FnPtr,
         pub(super) GetPixelTransformParameterfvEXT: FnPtr,
         pub(super) GetPixelTransformParameterivEXT: FnPtr,
         pub(super) GetPointerIndexedvEXT: FnPtr,
         pub(super) GetPointeri_vEXT: FnPtr,
         pub(super) GetPointerv: FnPtr,
         pub(super) GetPointervEXT: FnPtr,
         pub(super) GetProgramBinary: FnPtr,
         pub(super) GetProgramEnvParameterIivNV: FnPtr,
         pub(super) GetProgramEnvParameterIuivNV: FnPtr,
         pub(super) GetProgramEnvParameterdvARB: FnPtr,
         pub(super) GetProgramEnvParameterfvARB: FnPtr,
         pub(super) GetProgramInfoLog: FnPtr,
         pub(super) GetProgramInterfaceiv: FnPtr,
         pub(super) GetProgramLocalParameterIivNV: FnPtr,
         pub(super) GetProgramLocalParameterIuivNV: FnPtr,
         pub(super) GetProgramLocalParameterdvARB: FnPtr,
         pub(super) GetProgramLocalParameterfvARB: FnPtr,
         pub(super) GetProgramNamedParameterdvNV: FnPtr,
         pub(super) GetProgramNamedParameterfvNV: FnPtr,
         pub(super) GetProgramParameterdvNV: FnPtr,
         pub(super) GetProgramParameterfvNV: FnPtr,
         pub(super) GetProgramPipelineInfoLog: FnPtr,
         pub(super) GetProgramPipelineiv: FnPtr,
         pub(super) GetProgramResourceIndex: FnPtr,
         pub(super) GetProgramResourceLocation: FnPtr,
         pub(super) GetProgramResourceLocationIndex: FnPtr,
         pub(super) GetProgramResourceName: FnPtr,
         pub(super) GetProgramResourcefvNV: FnPtr,
         pub(super) GetProgramResourceiv: FnPtr,
         pub(super) GetProgramStageiv: FnPtr,
         pub(super) GetProgramStringARB: FnPtr,
         pub(super) GetProgramStringNV: FnPtr,
         pub(super) GetProgramSubroutineParameteruivNV: FnPtr,
         pub(super) GetProgramiv: FnPtr,
         pub(super) GetProgramivARB: FnPtr,
         pub(super) GetProgramivNV: FnPtr,
         pub(super) GetQueryBufferObjecti64v: FnPtr,
         pub(super) GetQueryBufferObjectiv: FnPtr,
         pub(super) GetQueryBufferObjectui64v: FnPtr,
         pub(super) GetQueryBufferObjectuiv: FnPtr,
         pub(super) GetQueryIndexediv: FnPtr,
         pub(super) GetQueryObjecti64v: FnPtr,
         pub(super) GetQueryObjecti64vEXT: FnPtr,
         pub(super) GetQueryObjectiv: FnPtr,
         pub(super) GetQueryObjectivARB: FnPtr,
         pub(super) GetQueryObjectui64v: FnPtr,
         pub(super) GetQueryObjectui64vEXT: FnPtr,
         pub(super) GetQueryObjectuiv: FnPtr,
         pub(super) GetQueryObjectuivARB: FnPtr,
         pub(super) GetQueryiv: FnPtr,
         pub(super) GetQueryivARB: FnPtr,
         pub(super) GetRenderbufferParameteriv: FnPtr,
         pub(super) GetRenderbufferParameterivEXT: FnPtr,
         pub(super) GetSamplerParameterIiv: FnPtr,
         pub(super) GetSamplerParameterIuiv: FnPtr,
         pub(super) GetSamplerParameterfv: FnPtr,
         pub(super) GetSamplerParameteriv: FnPtr,
         pub(super) GetSemaphoreParameterivNV: FnPtr,
         pub(super) GetSemaphoreParameterui64vEXT: FnPtr,
         pub(super) GetSeparableFilterEXT: FnPtr,
         pub(super) GetShaderInfoLog: FnPtr,
         pub(super) GetShaderPrecisionFormat: FnPtr,
         pub(super) GetShaderSource: FnPtr,
         pub(super) GetShaderSourceARB: FnPtr,
         pub(super) GetShaderiv: FnPtr,
         pub(super) GetShadingRateImagePaletteNV: FnPtr,
         pub(super) GetShadingRateSampleLocationivNV: FnPtr,
         pub(super) GetSharpenTexFuncSGIS: FnPtr,
         pub(super) GetStageIndexNV: FnPtr,
         pub(super) GetString: FnPtr,
         pub(super) GetStringi: FnPtr,
         pub(super) GetSubroutineIndex: FnPtr,
         pub(super) GetSubroutineUniformLocation: FnPtr,
         pub(super) GetSynciv: FnPtr,
         pub(super) GetTexBumpParameterfvATI: FnPtr,
         pub(super) GetTexBumpParameterivATI: FnPtr,
         pub(super) GetTexEnvxvOES: FnPtr,
         pub(super) GetTexFilterFuncSGIS: FnPtr,
         pub(super) GetTexGenxvOES: FnPtr,
         pub(super) GetTexImage: FnPtr,
         pub(super) GetTexLevelParameterfv: FnPtr,
         pub(super) GetTexLevelParameteriv: FnPtr,
         pub(super) GetTexLevelParameterxvOES: FnPtr,
         pub(super) GetTexParameterIiv: FnPtr,
         pub(super) GetTexParameterIivEXT: FnPtr,
         pub(super) GetTexParameterIuiv: FnPtr,
         pub(super) GetTexParameterIuivEXT: FnPtr,
         pub(super) GetTexParameterPointervAPPLE: FnPtr,
         pub(super) GetTexParameterfv: FnPtr,
         pub(super) GetTexParameteriv: FnPtr,
         pub(super) GetTexParameterxvOES: FnPtr,
         pub(super) GetTextureHandleARB: FnPtr,
         pub(super) GetTextureHandleNV: FnPtr,
         pub(super) GetTextureImage: FnPtr,
         pub(super) GetTextureImageEXT: FnPtr,
         pub(super) GetTextureLevelParameterfv: FnPtr,
         pub(super) GetTextureLevelParameterfvEXT: FnPtr,
         pub(super) GetTextureLevelParameteriv: FnPtr,
         pub(super) GetTextureLevelParameterivEXT: FnPtr,
         pub(super) GetTextureParameterIiv: FnPtr,
         pub(super) GetTextureParameterIivEXT: FnPtr,
         pub(super) GetTextureParameterIuiv: FnPtr,
         pub(super) GetTextureParameterIuivEXT: FnPtr,
         pub(super) GetTextureParameterfv: FnPtr,
         pub(super) GetTextureParameterfvEXT: FnPtr,
         pub(super) GetTextureParameteriv: FnPtr,
         pub(super) GetTextureParameterivEXT: FnPtr,
         pub(super) GetTextureSamplerHandleARB: FnPtr,
         pub(super) GetTextureSamplerHandleNV: FnPtr,
         pub(super) GetTextureSubImage: FnPtr,
         pub(super) GetTrackMatrixivNV: FnPtr,
         pub(super) GetTransformFeedbackVarying: FnPtr,
         pub(super) GetTransformFeedbackVaryingEXT: FnPtr,
         pub(super) GetTransformFeedbackVaryingNV: FnPtr,
         pub(super) GetTransformFeedbacki64_v: FnPtr,
         pub(super) GetTransformFeedbacki_v: FnPtr,
         pub(super) GetTransformFeedbackiv: FnPtr,
         pub(super) GetUniformBlockIndex: FnPtr,
         pub(super) GetUniformBufferSizeEXT: FnPtr,
         pub(super) GetUniformIndices: FnPtr,
         pub(super) GetUniformLocation: FnPtr,
         pub(super) GetUniformLocationARB: FnPtr,
         pub(super) GetUniformOffsetEXT: FnPtr,
         pub(super) GetUniformSubroutineuiv: FnPtr,
         pub(super) GetUniformdv: FnPtr,
         pub(super) GetUniformfv: FnPtr,
         pub(super) GetUniformfvARB: FnPtr,
         pub(super) GetUniformi64vARB: FnPtr,
         pub(super) GetUniformi64vNV: FnPtr,
         pub(super) GetUniformiv: FnPtr,
         pub(super) GetUniformivARB: FnPtr,
         pub(super) GetUniformui64vARB: FnPtr,
         pub(super) GetUniformui64vNV: FnPtr,
         pub(super) GetUniformuiv: FnPtr,
         pub(super) GetUniformuivEXT: FnPtr,
         pub(super) GetUnsignedBytei_vEXT: FnPtr,
         pub(super) GetUnsignedBytevEXT: FnPtr,
         pub(super) GetVariantArrayObjectfvATI: FnPtr,
         pub(super) GetVariantArrayObjectivATI: FnPtr,
         pub(super) GetVariantBooleanvEXT: FnPtr,
         pub(super) GetVariantFloatvEXT: FnPtr,
         pub(super) GetVariantIntegervEXT: FnPtr,
         pub(super) GetVariantPointervEXT: FnPtr,
         pub(super) GetVaryingLocationNV: FnPtr,
         pub(super) GetVertexArrayIndexed64iv: FnPtr,
         pub(super) GetVertexArrayIndexediv: FnPtr,
         pub(super) GetVertexArrayIntegeri_vEXT: FnPtr,
         pub(super) GetVertexArrayIntegervEXT: FnPtr,
         pub(super) GetVertexArrayPointeri_vEXT: FnPtr,
         pub(super) GetVertexArrayPointervEXT: FnPtr,
         pub(super) GetVertexArrayiv: FnPtr,
         pub(super) GetVertexAttribArrayObjectfvATI: FnPtr,
         pub(super) GetVertexAttribArrayObjectivATI: FnPtr,
         pub(super) GetVertexAttribIiv: FnPtr,
         pub(super) GetVertexAttribIivEXT: FnPtr,
         pub(super) GetVertexAttribIuiv: FnPtr,
         pub(super) GetVertexAttribIuivEXT: FnPtr,
         pub(super) GetVertexAttribLdv: FnPtr,
         pub(super) GetVertexAttribLdvEXT: FnPtr,
         pub(super) GetVertexAttribLi64vNV: FnPtr,
         pub(super) GetVertexAttribLui64vARB: FnPtr,
         pub(super) GetVertexAttribLui64vNV: FnPtr,
         pub(super) GetVertexAttribPointerv: FnPtr,
         pub(super) GetVertexAttribPointervARB: FnPtr,
         pub(super) GetVertexAttribPointervNV: FnPtr,
         pub(super) GetVertexAttribdv: FnPtr,
         pub(super) GetVertexAttribdvARB: FnPtr,
         pub(super) GetVertexAttribdvNV: FnPtr,
         pub(super) GetVertexAttribfv: FnPtr,
         pub(super) GetVertexAttribfvARB: FnPtr,
         pub(super) GetVertexAttribfvNV: FnPtr,
         pub(super) GetVertexAttribiv: FnPtr,
         pub(super) GetVertexAttribivARB: FnPtr,
         pub(super) GetVertexAttribivNV: FnPtr,
         pub(super) GetVideoCaptureStreamdvNV: FnPtr,
         pub(super) GetVideoCaptureStreamfvNV: FnPtr,
         pub(super) GetVideoCaptureStreamivNV: FnPtr,
         pub(super) GetVideoCaptureivNV: FnPtr,
         pub(super) GetVideoi64vNV: FnPtr,
         pub(super) GetVideoivNV: FnPtr,
         pub(super) GetVideoui64vNV: FnPtr,
         pub(super) GetVideouivNV: FnPtr,
         pub(super) GetVkProcAddrNV: FnPtr,
         pub(super) GetnCompressedTexImage: FnPtr,
         pub(super) GetnCompressedTexImageARB: FnPtr,
         pub(super) GetnTexImage: FnPtr,
         pub(super) GetnTexImageARB: FnPtr,
         pub(super) GetnUniformdv: FnPtr,
         pub(super) GetnUniformdvARB: FnPtr,
         pub(super) GetnUniformfv: FnPtr,
         pub(super) GetnUniformfvARB: FnPtr,
         pub(super) GetnUniformi64vARB: FnPtr,
         pub(super) GetnUniformiv: FnPtr,
         pub(super) GetnUniformivARB: FnPtr,
         pub(super) GetnUniformui64vARB: FnPtr,
         pub(super) GetnUniformuiv: FnPtr,
         pub(super) GetnUniformuivARB: FnPtr,
         pub(super) GlobalAlphaFactorbSUN: FnPtr,
         pub(super) GlobalAlphaFactordSUN: FnPtr,
         pub(super) GlobalAlphaFactorfSUN: FnPtr,
         pub(super) GlobalAlphaFactoriSUN: FnPtr,
         pub(super) GlobalAlphaFactorsSUN: FnPtr,
         pub(super) GlobalAlphaFactorubSUN: FnPtr,
         pub(super) GlobalAlphaFactoruiSUN: FnPtr,
         pub(super) GlobalAlphaFactorusSUN: FnPtr,
         pub(super) Hint: FnPtr,
         pub(super) HintPGI: FnPtr,
         pub(super) HistogramEXT: FnPtr,
         pub(super) IglooInterfaceSGIX: FnPtr,
         pub(super) ImageTransformParameterfHP: FnPtr,
         pub(super) ImageTransformParameterfvHP: FnPtr,
         pub(super) ImageTransformParameteriHP: FnPtr,
         pub(super) ImageTransformParameterivHP: FnPtr,
         pub(super) ImportMemoryFdEXT: FnPtr,
         pub(super) ImportMemoryWin32HandleEXT: FnPtr,
         pub(super) ImportMemoryWin32NameEXT: FnPtr,
         pub(super) ImportSemaphoreFdEXT: FnPtr,
         pub(super) ImportSemaphoreWin32HandleEXT: FnPtr,
         pub(super) ImportSemaphoreWin32NameEXT: FnPtr,
         pub(super) ImportSyncEXT: FnPtr,
         pub(super) IndexFormatNV: FnPtr,
         pub(super) IndexFuncEXT: FnPtr,
         pub(super) IndexMaterialEXT: FnPtr,
         pub(super) IndexPointerEXT: FnPtr,
         pub(super) IndexPointerListIBM: FnPtr,
         pub(super) IndexxOES: FnPtr,
         pub(super) IndexxvOES: FnPtr,
         pub(super) InsertComponentEXT: FnPtr,
         pub(super) InsertEventMarkerEXT: FnPtr,
         pub(super) InstrumentsBufferSGIX: FnPtr,
         pub(super) InterpolatePathsNV: FnPtr,
         pub(super) InvalidateBufferData: FnPtr,
         pub(super) InvalidateBufferSubData: FnPtr,
         pub(super) InvalidateFramebuffer: FnPtr,
         pub(super) InvalidateNamedFramebufferData: FnPtr,
         pub(super) InvalidateNamedFramebufferSubData: FnPtr,
         pub(super) InvalidateSubFramebuffer: FnPtr,
         pub(super) InvalidateTexImage: FnPtr,
         pub(super) InvalidateTexSubImage: FnPtr,
         pub(super) IsAsyncMarkerSGIX: FnPtr,
         pub(super) IsBuffer: FnPtr,
         pub(super) IsBufferARB: FnPtr,
         pub(super) IsBufferResidentNV: FnPtr,
         pub(super) IsCommandListNV: FnPtr,
         pub(super) IsEnabled: FnPtr,
         pub(super) IsEnabledIndexedEXT: FnPtr,
         pub(super) IsEnabledi: FnPtr,
         pub(super) IsFenceAPPLE: FnPtr,
         pub(super) IsFenceNV: FnPtr,
         pub(super) IsFramebuffer: FnPtr,
         pub(super) IsFramebufferEXT: FnPtr,
         pub(super) IsImageHandleResidentARB: FnPtr,
         pub(super) IsImageHandleResidentNV: FnPtr,
         pub(super) IsMemoryObjectEXT: FnPtr,
         pub(super) IsNameAMD: FnPtr,
         pub(super) IsNamedBufferResidentNV: FnPtr,
         pub(super) IsNamedStringARB: FnPtr,
         pub(super) IsObjectBufferATI: FnPtr,
         pub(super) IsOcclusionQueryNV: FnPtr,
         pub(super) IsPathNV: FnPtr,
         pub(super) IsPointInFillPathNV: FnPtr,
         pub(super) IsPointInStrokePathNV: FnPtr,
         pub(super) IsProgram: FnPtr,
         pub(super) IsProgramARB: FnPtr,
         pub(super) IsProgramNV: FnPtr,
         pub(super) IsProgramPipeline: FnPtr,
         pub(super) IsQuery: FnPtr,
         pub(super) IsQueryARB: FnPtr,
         pub(super) IsRenderbuffer: FnPtr,
         pub(super) IsRenderbufferEXT: FnPtr,
         pub(super) IsSampler: FnPtr,
         pub(super) IsSemaphoreEXT: FnPtr,
         pub(super) IsShader: FnPtr,
         pub(super) IsStateNV: FnPtr,
         pub(super) IsSync: FnPtr,
         pub(super) IsTexture: FnPtr,
         pub(super) IsTextureEXT: FnPtr,
         pub(super) IsTextureHandleResidentARB: FnPtr,
         pub(super) IsTextureHandleResidentNV: FnPtr,
         pub(super) IsTransformFeedback: FnPtr,
         pub(super) IsTransformFeedbackNV: FnPtr,
         pub(super) IsVariantEnabledEXT: FnPtr,
         pub(super) IsVertexArray: FnPtr,
         pub(super) IsVertexArrayAPPLE: FnPtr,
         pub(super) IsVertexAttribEnabledAPPLE: FnPtr,
         pub(super) LGPUCopyImageSubDataNVX: FnPtr,
         pub(super) LGPUInterlockNVX: FnPtr,
         pub(super) LGPUNamedBufferSubDataNVX: FnPtr,
         pub(super) LabelObjectEXT: FnPtr,
         pub(super) LightEnviSGIX: FnPtr,
         pub(super) LightModelxOES: FnPtr,
         pub(super) LightModelxvOES: FnPtr,
         pub(super) LightxOES: FnPtr,
         pub(super) LightxvOES: FnPtr,
         pub(super) LineWidth: FnPtr,
         pub(super) LineWidthxOES: FnPtr,
         pub(super) LinkProgram: FnPtr,
         pub(super) LinkProgramARB: FnPtr,
         pub(super) ListDrawCommandsStatesClientNV: FnPtr,
         pub(super) ListParameterfSGIX: FnPtr,
         pub(super) ListParameterfvSGIX: FnPtr,
         pub(super) ListParameteriSGIX: FnPtr,
         pub(super) ListParameterivSGIX: FnPtr,
         pub(super) LoadIdentityDeformationMapSGIX: FnPtr,
         pub(super) LoadMatrixxOES: FnPtr,
         pub(super) LoadProgramNV: FnPtr,
         pub(super) LoadTransposeMatrixdARB: FnPtr,
         pub(super) LoadTransposeMatrixfARB: FnPtr,
         pub(super) LoadTransposeMatrixxOES: FnPtr,
         pub(super) LockArraysEXT: FnPtr,
         pub(super) LogicOp: FnPtr,
         pub(super) MakeBufferNonResidentNV: FnPtr,
         pub(super) MakeBufferResidentNV: FnPtr,
         pub(super) MakeImageHandleNonResidentARB: FnPtr,
         pub(super) MakeImageHandleNonResidentNV: FnPtr,
         pub(super) MakeImageHandleResidentARB: FnPtr,
         pub(super) MakeImageHandleResidentNV: FnPtr,
         pub(super) MakeNamedBufferNonResidentNV: FnPtr,
         pub(super) MakeNamedBufferResidentNV: FnPtr,
         pub(super) MakeTextureHandleNonResidentARB: FnPtr,
         pub(super) MakeTextureHandleNonResidentNV: FnPtr,
         pub(super) MakeTextureHandleResidentARB: FnPtr,
         pub(super) MakeTextureHandleResidentNV: FnPtr,
         pub(super) Map1xOES: FnPtr,
         pub(super) Map2xOES: FnPtr,
         pub(super) MapBuffer: FnPtr,
         pub(super) MapBufferARB: FnPtr,
         pub(super) MapBufferRange: FnPtr,
         pub(super) MapControlPointsNV: FnPtr,
         pub(super) MapGrid1xOES: FnPtr,
         pub(super) MapGrid2xOES: FnPtr,
         pub(super) MapNamedBuffer: FnPtr,
         pub(super) MapNamedBufferEXT: FnPtr,
         pub(super) MapNamedBufferRange: FnPtr,
         pub(super) MapNamedBufferRangeEXT: FnPtr,
         pub(super) MapObjectBufferATI: FnPtr,
         pub(super) MapParameterfvNV: FnPtr,
         pub(super) MapParameterivNV: FnPtr,
         pub(super) MapTexture2DINTEL: FnPtr,
         pub(super) MapVertexAttrib1dAPPLE: FnPtr,
         pub(super) MapVertexAttrib1fAPPLE: FnPtr,
         pub(super) MapVertexAttrib2dAPPLE: FnPtr,
         pub(super) MapVertexAttrib2fAPPLE: FnPtr,
         pub(super) MaterialxOES: FnPtr,
         pub(super) MaterialxvOES: FnPtr,
         pub(super) MatrixFrustumEXT: FnPtr,
         pub(super) MatrixIndexPointerARB: FnPtr,
         pub(super) MatrixIndexubvARB: FnPtr,
         pub(super) MatrixIndexuivARB: FnPtr,
         pub(super) MatrixIndexusvARB: FnPtr,
         pub(super) MatrixLoad3x2fNV: FnPtr,
         pub(super) MatrixLoad3x3fNV: FnPtr,
         pub(super) MatrixLoadIdentityEXT: FnPtr,
         pub(super) MatrixLoadTranspose3x3fNV: FnPtr,
         pub(super) MatrixLoadTransposedEXT: FnPtr,
         pub(super) MatrixLoadTransposefEXT: FnPtr,
         pub(super) MatrixLoaddEXT: FnPtr,
         pub(super) MatrixLoadfEXT: FnPtr,
         pub(super) MatrixMult3x2fNV: FnPtr,
         pub(super) MatrixMult3x3fNV: FnPtr,
         pub(super) MatrixMultTranspose3x3fNV: FnPtr,
         pub(super) MatrixMultTransposedEXT: FnPtr,
         pub(super) MatrixMultTransposefEXT: FnPtr,
         pub(super) MatrixMultdEXT: FnPtr,
         pub(super) MatrixMultfEXT: FnPtr,
         pub(super) MatrixOrthoEXT: FnPtr,
         pub(super) MatrixPopEXT: FnPtr,
         pub(super) MatrixPushEXT: FnPtr,
         pub(super) MatrixRotatedEXT: FnPtr,
         pub(super) MatrixRotatefEXT: FnPtr,
         pub(super) MatrixScaledEXT: FnPtr,
         pub(super) MatrixScalefEXT: FnPtr,
         pub(super) MatrixTranslatedEXT: FnPtr,
         pub(super) MatrixTranslatefEXT: FnPtr,
         pub(super) MaxShaderCompilerThreadsARB: FnPtr,
         pub(super) MaxShaderCompilerThreadsKHR: FnPtr,
         pub(super) MemoryBarrier: FnPtr,
         pub(super) MemoryBarrierByRegion: FnPtr,
         pub(super) MemoryBarrierEXT: FnPtr,
         pub(super) MemoryObjectParameterivEXT: FnPtr,
         pub(super) MinSampleShading: FnPtr,
         pub(super) MinSampleShadingARB: FnPtr,
         pub(super) MinmaxEXT: FnPtr,
         pub(super) MultMatrixxOES: FnPtr,
         pub(super) MultTransposeMatrixdARB: FnPtr,
         pub(super) MultTransposeMatrixfARB: FnPtr,
         pub(super) MultTransposeMatrixxOES: FnPtr,
         pub(super) MultiDrawArrays: FnPtr,
         pub(super) MultiDrawArraysEXT: FnPtr,
         pub(super) MultiDrawArraysIndirect: FnPtr,
         pub(super) MultiDrawArraysIndirectAMD: FnPtr,
         pub(super) MultiDrawArraysIndirectBindlessCountNV: FnPtr,
         pub(super) MultiDrawArraysIndirectBindlessNV: FnPtr,
         pub(super) MultiDrawArraysIndirectCount: FnPtr,
         pub(super) MultiDrawArraysIndirectCountARB: FnPtr,
         pub(super) MultiDrawElementArrayAPPLE: FnPtr,
         pub(super) MultiDrawElements: FnPtr,
         pub(super) MultiDrawElementsBaseVertex: FnPtr,
         pub(super) MultiDrawElementsEXT: FnPtr,
         pub(super) MultiDrawElementsIndirect: FnPtr,
         pub(super) MultiDrawElementsIndirectAMD: FnPtr,
         pub(super) MultiDrawElementsIndirectBindlessCountNV: FnPtr,
         pub(super) MultiDrawElementsIndirectBindlessNV: FnPtr,
         pub(super) MultiDrawElementsIndirectCount: FnPtr,
         pub(super) MultiDrawElementsIndirectCountARB: FnPtr,
         pub(super) MultiDrawMeshTasksIndirectCountNV: FnPtr,
         pub(super) MultiDrawMeshTasksIndirectNV: FnPtr,
         pub(super) MultiDrawRangeElementArrayAPPLE: FnPtr,
         pub(super) MultiModeDrawArraysIBM: FnPtr,
         pub(super) MultiModeDrawElementsIBM: FnPtr,
         pub(super) MultiTexBufferEXT: FnPtr,
         pub(super) MultiTexCoord1bOES: FnPtr,
         pub(super) MultiTexCoord1bvOES: FnPtr,
         pub(super) MultiTexCoord1dARB: FnPtr,
         pub(super) MultiTexCoord1dvARB: FnPtr,
         pub(super) MultiTexCoord1fARB: FnPtr,
         pub(super) MultiTexCoord1fvARB: FnPtr,
         pub(super) MultiTexCoord1hNV: FnPtr,
         pub(super) MultiTexCoord1hvNV: FnPtr,
         pub(super) MultiTexCoord1iARB: FnPtr,
         pub(super) MultiTexCoord1ivARB: FnPtr,
         pub(super) MultiTexCoord1sARB: FnPtr,
         pub(super) MultiTexCoord1svARB: FnPtr,
         pub(super) MultiTexCoord1xOES: FnPtr,
         pub(super) MultiTexCoord1xvOES: FnPtr,
         pub(super) MultiTexCoord2bOES: FnPtr,
         pub(super) MultiTexCoord2bvOES: FnPtr,
         pub(super) MultiTexCoord2dARB: FnPtr,
         pub(super) MultiTexCoord2dvARB: FnPtr,
         pub(super) MultiTexCoord2fARB: FnPtr,
         pub(super) MultiTexCoord2fvARB: FnPtr,
         pub(super) MultiTexCoord2hNV: FnPtr,
         pub(super) MultiTexCoord2hvNV: FnPtr,
         pub(super) MultiTexCoord2iARB: FnPtr,
         pub(super) MultiTexCoord2ivARB: FnPtr,
         pub(super) MultiTexCoord2sARB: FnPtr,
         pub(super) MultiTexCoord2svARB: FnPtr,
         pub(super) MultiTexCoord2xOES: FnPtr,
         pub(super) MultiTexCoord2xvOES: FnPtr,
         pub(super) MultiTexCoord3bOES: FnPtr,
         pub(super) MultiTexCoord3bvOES: FnPtr,
         pub(super) MultiTexCoord3dARB: FnPtr,
         pub(super) MultiTexCoord3dvARB: FnPtr,
         pub(super) MultiTexCoord3fARB: FnPtr,
         pub(super) MultiTexCoord3fvARB: FnPtr,
         pub(super) MultiTexCoord3hNV: FnPtr,
         pub(super) MultiTexCoord3hvNV: FnPtr,
         pub(super) MultiTexCoord3iARB: FnPtr,
         pub(super) MultiTexCoord3ivARB: FnPtr,
         pub(super) MultiTexCoord3sARB: FnPtr,
         pub(super) MultiTexCoord3svARB: FnPtr,
         pub(super) MultiTexCoord3xOES: FnPtr,
         pub(super) MultiTexCoord3xvOES: FnPtr,
         pub(super) MultiTexCoord4bOES: FnPtr,
         pub(super) MultiTexCoord4bvOES: FnPtr,
         pub(super) MultiTexCoord4dARB: FnPtr,
         pub(super) MultiTexCoord4dvARB: FnPtr,
         pub(super) MultiTexCoord4fARB: FnPtr,
         pub(super) MultiTexCoord4fvARB: FnPtr,
         pub(super) MultiTexCoord4hNV: FnPtr,
         pub(super) MultiTexCoord4hvNV: FnPtr,
         pub(super) MultiTexCoord4iARB: FnPtr,
         pub(super) MultiTexCoord4ivARB: FnPtr,
         pub(super) MultiTexCoord4sARB: FnPtr,
         pub(super) MultiTexCoord4svARB: FnPtr,
         pub(super) MultiTexCoord4xOES: FnPtr,
         pub(super) MultiTexCoord4xvOES: FnPtr,
         pub(super) MultiTexCoordPointerEXT: FnPtr,
         pub(super) MultiTexEnvfEXT: FnPtr,
         pub(super) MultiTexEnvfvEXT: FnPtr,
         pub(super) MultiTexEnviEXT: FnPtr,
         pub(super) MultiTexEnvivEXT: FnPtr,
         pub(super) MultiTexGendEXT: FnPtr,
         pub(super) MultiTexGendvEXT: FnPtr,
         pub(super) MultiTexGenfEXT: FnPtr,
         pub(super) MultiTexGenfvEXT: FnPtr,
         pub(super) MultiTexGeniEXT: FnPtr,
         pub(super) MultiTexGenivEXT: FnPtr,
         pub(super) MultiTexImage1DEXT: FnPtr,
         pub(super) MultiTexImage2DEXT: FnPtr,
         pub(super) MultiTexImage3DEXT: FnPtr,
         pub(super) MultiTexParameterIivEXT: FnPtr,
         pub(super) MultiTexParameterIuivEXT: FnPtr,
         pub(super) MultiTexParameterfEXT: FnPtr,
         pub(super) MultiTexParameterfvEXT: FnPtr,
         pub(super) MultiTexParameteriEXT: FnPtr,
         pub(super) MultiTexParameterivEXT: FnPtr,
         pub(super) MultiTexRenderbufferEXT: FnPtr,
         pub(super) MultiTexSubImage1DEXT: FnPtr,
         pub(super) MultiTexSubImage2DEXT: FnPtr,
         pub(super) MultiTexSubImage3DEXT: FnPtr,
         pub(super) MulticastBarrierNV: FnPtr,
         pub(super) MulticastBlitFramebufferNV: FnPtr,
         pub(super) MulticastBufferSubDataNV: FnPtr,
         pub(super) MulticastCopyBufferSubDataNV: FnPtr,
         pub(super) MulticastCopyImageSubDataNV: FnPtr,
         pub(super) MulticastFramebufferSampleLocationsfvNV: FnPtr,
         pub(super) MulticastGetQueryObjecti64vNV: FnPtr,
         pub(super) MulticastGetQueryObjectivNV: FnPtr,
         pub(super) MulticastGetQueryObjectui64vNV: FnPtr,
         pub(super) MulticastGetQueryObjectuivNV: FnPtr,
         pub(super) MulticastScissorArrayvNVX: FnPtr,
         pub(super) MulticastViewportArrayvNVX: FnPtr,
         pub(super) MulticastViewportPositionWScaleNVX: FnPtr,
         pub(super) MulticastWaitSyncNV: FnPtr,
         pub(super) NamedBufferAttachMemoryNV: FnPtr,
         pub(super) NamedBufferData: FnPtr,
         pub(super) NamedBufferDataEXT: FnPtr,
         pub(super) NamedBufferPageCommitmentARB: FnPtr,
         pub(super) NamedBufferPageCommitmentEXT: FnPtr,
         pub(super) NamedBufferPageCommitmentMemNV: FnPtr,
         pub(super) NamedBufferStorage: FnPtr,
         pub(super) NamedBufferStorageEXT: FnPtr,
         pub(super) NamedBufferStorageExternalEXT: FnPtr,
         pub(super) NamedBufferStorageMemEXT: FnPtr,
         pub(super) NamedBufferSubData: FnPtr,
         pub(super) NamedBufferSubDataEXT: FnPtr,
         pub(super) NamedCopyBufferSubDataEXT: FnPtr,
         pub(super) NamedFramebufferDrawBuffer: FnPtr,
         pub(super) NamedFramebufferDrawBuffers: FnPtr,
         pub(super) NamedFramebufferParameteri: FnPtr,
         pub(super) NamedFramebufferParameteriEXT: FnPtr,
         pub(super) NamedFramebufferReadBuffer: FnPtr,
         pub(super) NamedFramebufferRenderbuffer: FnPtr,
         pub(super) NamedFramebufferRenderbufferEXT: FnPtr,
         pub(super) NamedFramebufferSampleLocationsfvARB: FnPtr,
         pub(super) NamedFramebufferSampleLocationsfvNV: FnPtr,
         pub(super) NamedFramebufferSamplePositionsfvAMD: FnPtr,
         pub(super) NamedFramebufferTexture: FnPtr,
         pub(super) NamedFramebufferTexture1DEXT: FnPtr,
         pub(super) NamedFramebufferTexture2DEXT: FnPtr,
         pub(super) NamedFramebufferTexture3DEXT: FnPtr,
         pub(super) NamedFramebufferTextureEXT: FnPtr,
         pub(super) NamedFramebufferTextureFaceEXT: FnPtr,
         pub(super) NamedFramebufferTextureLayer: FnPtr,
         pub(super) NamedFramebufferTextureLayerEXT: FnPtr,
         pub(super) NamedProgramLocalParameter4dEXT: FnPtr,
         pub(super) NamedProgramLocalParameter4dvEXT: FnPtr,
         pub(super) NamedProgramLocalParameter4fEXT: FnPtr,
         pub(super) NamedProgramLocalParameter4fvEXT: FnPtr,
         pub(super) NamedProgramLocalParameterI4iEXT: FnPtr,
         pub(super) NamedProgramLocalParameterI4ivEXT: FnPtr,
         pub(super) NamedProgramLocalParameterI4uiEXT: FnPtr,
         pub(super) NamedProgramLocalParameterI4uivEXT: FnPtr,
         pub(super) NamedProgramLocalParameters4fvEXT: FnPtr,
         pub(super) NamedProgramLocalParametersI4ivEXT: FnPtr,
         pub(super) NamedProgramLocalParametersI4uivEXT: FnPtr,
         pub(super) NamedProgramStringEXT: FnPtr,
         pub(super) NamedRenderbufferStorage: FnPtr,
         pub(super) NamedRenderbufferStorageEXT: FnPtr,
         pub(super) NamedRenderbufferStorageMultisample: FnPtr,
         pub(super) NamedRenderbufferStorageMultisampleAdvancedAMD: FnPtr,
         pub(super) NamedRenderbufferStorageMultisampleCoverageEXT: FnPtr,
         pub(super) NamedRenderbufferStorageMultisampleEXT: FnPtr,
         pub(super) NamedStringARB: FnPtr,
         pub(super) NewObjectBufferATI: FnPtr,
         pub(super) Normal3fVertex3fSUN: FnPtr,
         pub(super) Normal3fVertex3fvSUN: FnPtr,
         pub(super) Normal3hNV: FnPtr,
         pub(super) Normal3hvNV: FnPtr,
         pub(super) Normal3xOES: FnPtr,
         pub(super) Normal3xvOES: FnPtr,
         pub(super) NormalFormatNV: FnPtr,
         pub(super) NormalPointerEXT: FnPtr,
         pub(super) NormalPointerListIBM: FnPtr,
         pub(super) NormalPointervINTEL: FnPtr,
         pub(super) NormalStream3bATI: FnPtr,
         pub(super) NormalStream3bvATI: FnPtr,
         pub(super) NormalStream3dATI: FnPtr,
         pub(super) NormalStream3dvATI: FnPtr,
         pub(super) NormalStream3fATI: FnPtr,
         pub(super) NormalStream3fvATI: FnPtr,
         pub(super) NormalStream3iATI: FnPtr,
         pub(super) NormalStream3ivATI: FnPtr,
         pub(super) NormalStream3sATI: FnPtr,
         pub(super) NormalStream3svATI: FnPtr,
         pub(super) ObjectLabel: FnPtr,
         pub(super) ObjectPtrLabel: FnPtr,
         pub(super) ObjectPurgeableAPPLE: FnPtr,
         pub(super) ObjectUnpurgeableAPPLE: FnPtr,
         pub(super) OrthofOES: FnPtr,
         pub(super) OrthoxOES: FnPtr,
         pub(super) PNTrianglesfATI: FnPtr,
         pub(super) PNTrianglesiATI: FnPtr,
         pub(super) PassTexCoordATI: FnPtr,
         pub(super) PassThroughxOES: FnPtr,
         pub(super) PatchParameterfv: FnPtr,
         pub(super) PatchParameteri: FnPtr,
         pub(super) PathCommandsNV: FnPtr,
         pub(super) PathCoordsNV: FnPtr,
         pub(super) PathCoverDepthFuncNV: FnPtr,
         pub(super) PathDashArrayNV: FnPtr,
         pub(super) PathGlyphIndexArrayNV: FnPtr,
         pub(super) PathGlyphIndexRangeNV: FnPtr,
         pub(super) PathGlyphRangeNV: FnPtr,
         pub(super) PathGlyphsNV: FnPtr,
         pub(super) PathMemoryGlyphIndexArrayNV: FnPtr,
         pub(super) PathParameterfNV: FnPtr,
         pub(super) PathParameterfvNV: FnPtr,
         pub(super) PathParameteriNV: FnPtr,
         pub(super) PathParameterivNV: FnPtr,
         pub(super) PathStencilDepthOffsetNV: FnPtr,
         pub(super) PathStencilFuncNV: FnPtr,
         pub(super) PathStringNV: FnPtr,
         pub(super) PathSubCommandsNV: FnPtr,
         pub(super) PathSubCoordsNV: FnPtr,
         pub(super) PauseTransformFeedback: FnPtr,
         pub(super) PauseTransformFeedbackNV: FnPtr,
         pub(super) PixelDataRangeNV: FnPtr,
         pub(super) PixelMapx: FnPtr,
         pub(super) PixelStoref: FnPtr,
         pub(super) PixelStorei: FnPtr,
         pub(super) PixelStorex: FnPtr,
         pub(super) PixelTexGenParameterfSGIS: FnPtr,
         pub(super) PixelTexGenParameterfvSGIS: FnPtr,
         pub(super) PixelTexGenParameteriSGIS: FnPtr,
         pub(super) PixelTexGenParameterivSGIS: FnPtr,
         pub(super) PixelTexGenSGIX: FnPtr,
         pub(super) PixelTransferxOES: FnPtr,
         pub(super) PixelTransformParameterfEXT: FnPtr,
         pub(super) PixelTransformParameterfvEXT: FnPtr,
         pub(super) PixelTransformParameteriEXT: FnPtr,
         pub(super) PixelTransformParameterivEXT: FnPtr,
         pub(super) PixelZoomxOES: FnPtr,
         pub(super) PointAlongPathNV: FnPtr,
         pub(super) PointParameterf: FnPtr,
         pub(super) PointParameterfARB: FnPtr,
         pub(super) PointParameterfEXT: FnPtr,
         pub(super) PointParameterfSGIS: FnPtr,
         pub(super) PointParameterfv: FnPtr,
         pub(super) PointParameterfvARB: FnPtr,
         pub(super) PointParameterfvEXT: FnPtr,
         pub(super) PointParameterfvSGIS: FnPtr,
         pub(super) PointParameteri: FnPtr,
         pub(super) PointParameteriNV: FnPtr,
         pub(super) PointParameteriv: FnPtr,
         pub(super) PointParameterivNV: FnPtr,
         pub(super) PointParameterxvOES: FnPtr,
         pub(super) PointSize: FnPtr,
         pub(super) PointSizexOES: FnPtr,
         pub(super) PollAsyncSGIX: FnPtr,
         pub(super) PollInstrumentsSGIX: FnPtr,
         pub(super) PolygonMode: FnPtr,
         pub(super) PolygonOffset: FnPtr,
         pub(super) PolygonOffsetClamp: FnPtr,
         pub(super) PolygonOffsetClampEXT: FnPtr,
         pub(super) PolygonOffsetEXT: FnPtr,
         pub(super) PolygonOffsetxOES: FnPtr,
         pub(super) PopDebugGroup: FnPtr,
         pub(super) PopGroupMarkerEXT: FnPtr,
         pub(super) PresentFrameDualFillNV: FnPtr,
         pub(super) PresentFrameKeyedNV: FnPtr,
         pub(super) PrimitiveBoundingBoxARB: FnPtr,
         pub(super) PrimitiveRestartIndex: FnPtr,
         pub(super) PrimitiveRestartIndexNV: FnPtr,
         pub(super) PrimitiveRestartNV: FnPtr,
         pub(super) PrioritizeTexturesEXT: FnPtr,
         pub(super) PrioritizeTexturesxOES: FnPtr,
         pub(super) ProgramBinary: FnPtr,
         pub(super) ProgramBufferParametersIivNV: FnPtr,
         pub(super) ProgramBufferParametersIuivNV: FnPtr,
         pub(super) ProgramBufferParametersfvNV: FnPtr,
         pub(super) ProgramEnvParameter4dARB: FnPtr,
         pub(super) ProgramEnvParameter4dvARB: FnPtr,
         pub(super) ProgramEnvParameter4fARB: FnPtr,
         pub(super) ProgramEnvParameter4fvARB: FnPtr,
         pub(super) ProgramEnvParameterI4iNV: FnPtr,
         pub(super) ProgramEnvParameterI4ivNV: FnPtr,
         pub(super) ProgramEnvParameterI4uiNV: FnPtr,
         pub(super) ProgramEnvParameterI4uivNV: FnPtr,
         pub(super) ProgramEnvParameters4fvEXT: FnPtr,
         pub(super) ProgramEnvParametersI4ivNV: FnPtr,
         pub(super) ProgramEnvParametersI4uivNV: FnPtr,
         pub(super) ProgramLocalParameter4dARB: FnPtr,
         pub(super) ProgramLocalParameter4dvARB: FnPtr,
         pub(super) ProgramLocalParameter4fARB: FnPtr,
         pub(super) ProgramLocalParameter4fvARB: FnPtr,
         pub(super) ProgramLocalParameterI4iNV: FnPtr,
         pub(super) ProgramLocalParameterI4ivNV: FnPtr,
         pub(super) ProgramLocalParameterI4uiNV: FnPtr,
         pub(super) ProgramLocalParameterI4uivNV: FnPtr,
         pub(super) ProgramLocalParameters4fvEXT: FnPtr,
         pub(super) ProgramLocalParametersI4ivNV: FnPtr,
         pub(super) ProgramLocalParametersI4uivNV: FnPtr,
         pub(super) ProgramNamedParameter4dNV: FnPtr,
         pub(super) ProgramNamedParameter4dvNV: FnPtr,
         pub(super) ProgramNamedParameter4fNV: FnPtr,
         pub(super) ProgramNamedParameter4fvNV: FnPtr,
         pub(super) ProgramParameter4dNV: FnPtr,
         pub(super) ProgramParameter4dvNV: FnPtr,
         pub(super) ProgramParameter4fNV: FnPtr,
         pub(super) ProgramParameter4fvNV: FnPtr,
         pub(super) ProgramParameteri: FnPtr,
         pub(super) ProgramParameteriARB: FnPtr,
         pub(super) ProgramParameteriEXT: FnPtr,
         pub(super) ProgramParameters4dvNV: FnPtr,
         pub(super) ProgramParameters4fvNV: FnPtr,
         pub(super) ProgramPathFragmentInputGenNV: FnPtr,
         pub(super) ProgramStringARB: FnPtr,
         pub(super) ProgramSubroutineParametersuivNV: FnPtr,
         pub(super) ProgramUniform1d: FnPtr,
         pub(super) ProgramUniform1dEXT: FnPtr,
         pub(super) ProgramUniform1dv: FnPtr,
         pub(super) ProgramUniform1dvEXT: FnPtr,
         pub(super) ProgramUniform1f: FnPtr,
         pub(super) ProgramUniform1fEXT: FnPtr,
         pub(super) ProgramUniform1fv: FnPtr,
         pub(super) ProgramUniform1fvEXT: FnPtr,
         pub(super) ProgramUniform1i: FnPtr,
         pub(super) ProgramUniform1i64ARB: FnPtr,
         pub(super) ProgramUniform1i64NV: FnPtr,
         pub(super) ProgramUniform1i64vARB: FnPtr,
         pub(super) ProgramUniform1i64vNV: FnPtr,
         pub(super) ProgramUniform1iEXT: FnPtr,
         pub(super) ProgramUniform1iv: FnPtr,
         pub(super) ProgramUniform1ivEXT: FnPtr,
         pub(super) ProgramUniform1ui: FnPtr,
         pub(super) ProgramUniform1ui64ARB: FnPtr,
         pub(super) ProgramUniform1ui64NV: FnPtr,
         pub(super) ProgramUniform1ui64vARB: FnPtr,
         pub(super) ProgramUniform1ui64vNV: FnPtr,
         pub(super) ProgramUniform1uiEXT: FnPtr,
         pub(super) ProgramUniform1uiv: FnPtr,
         pub(super) ProgramUniform1uivEXT: FnPtr,
         pub(super) ProgramUniform2d: FnPtr,
         pub(super) ProgramUniform2dEXT: FnPtr,
         pub(super) ProgramUniform2dv: FnPtr,
         pub(super) ProgramUniform2dvEXT: FnPtr,
         pub(super) ProgramUniform2f: FnPtr,
         pub(super) ProgramUniform2fEXT: FnPtr,
         pub(super) ProgramUniform2fv: FnPtr,
         pub(super) ProgramUniform2fvEXT: FnPtr,
         pub(super) ProgramUniform2i: FnPtr,
         pub(super) ProgramUniform2i64ARB: FnPtr,
         pub(super) ProgramUniform2i64NV: FnPtr,
         pub(super) ProgramUniform2i64vARB: FnPtr,
         pub(super) ProgramUniform2i64vNV: FnPtr,
         pub(super) ProgramUniform2iEXT: FnPtr,
         pub(super) ProgramUniform2iv: FnPtr,
         pub(super) ProgramUniform2ivEXT: FnPtr,
         pub(super) ProgramUniform2ui: FnPtr,
         pub(super) ProgramUniform2ui64ARB: FnPtr,
         pub(super) ProgramUniform2ui64NV: FnPtr,
         pub(super) ProgramUniform2ui64vARB: FnPtr,
         pub(super) ProgramUniform2ui64vNV: FnPtr,
         pub(super) ProgramUniform2uiEXT: FnPtr,
         pub(super) ProgramUniform2uiv: FnPtr,
         pub(super) ProgramUniform2uivEXT: FnPtr,
         pub(super) ProgramUniform3d: FnPtr,
         pub(super) ProgramUniform3dEXT: FnPtr,
         pub(super) ProgramUniform3dv: FnPtr,
         pub(super) ProgramUniform3dvEXT: FnPtr,
         pub(super) ProgramUniform3f: FnPtr,
         pub(super) ProgramUniform3fEXT: FnPtr,
         pub(super) ProgramUniform3fv: FnPtr,
         pub(super) ProgramUniform3fvEXT: FnPtr,
         pub(super) ProgramUniform3i: FnPtr,
         pub(super) ProgramUniform3i64ARB: FnPtr,
         pub(super) ProgramUniform3i64NV: FnPtr,
         pub(super) ProgramUniform3i64vARB: FnPtr,
         pub(super) ProgramUniform3i64vNV: FnPtr,
         pub(super) ProgramUniform3iEXT: FnPtr,
         pub(super) ProgramUniform3iv: FnPtr,
         pub(super) ProgramUniform3ivEXT: FnPtr,
         pub(super) ProgramUniform3ui: FnPtr,
         pub(super) ProgramUniform3ui64ARB: FnPtr,
         pub(super) ProgramUniform3ui64NV: FnPtr,
         pub(super) ProgramUniform3ui64vARB: FnPtr,
         pub(super) ProgramUniform3ui64vNV: FnPtr,
         pub(super) ProgramUniform3uiEXT: FnPtr,
         pub(super) ProgramUniform3uiv: FnPtr,
         pub(super) ProgramUniform3uivEXT: FnPtr,
         pub(super) ProgramUniform4d: FnPtr,
         pub(super) ProgramUniform4dEXT: FnPtr,
         pub(super) ProgramUniform4dv: FnPtr,
         pub(super) ProgramUniform4dvEXT: FnPtr,
         pub(super) ProgramUniform4f: FnPtr,
         pub(super) ProgramUniform4fEXT: FnPtr,
         pub(super) ProgramUniform4fv: FnPtr,
         pub(super) ProgramUniform4fvEXT: FnPtr,
         pub(super) ProgramUniform4i: FnPtr,
         pub(super) ProgramUniform4i64ARB: FnPtr,
         pub(super) ProgramUniform4i64NV: FnPtr,
         pub(super) ProgramUniform4i64vARB: FnPtr,
         pub(super) ProgramUniform4i64vNV: FnPtr,
         pub(super) ProgramUniform4iEXT: FnPtr,
         pub(super) ProgramUniform4iv: FnPtr,
         pub(super) ProgramUniform4ivEXT: FnPtr,
         pub(super) ProgramUniform4ui: FnPtr,
         pub(super) ProgramUniform4ui64ARB: FnPtr,
         pub(super) ProgramUniform4ui64NV: FnPtr,
         pub(super) ProgramUniform4ui64vARB: FnPtr,
         pub(super) ProgramUniform4ui64vNV: FnPtr,
         pub(super) ProgramUniform4uiEXT: FnPtr,
         pub(super) ProgramUniform4uiv: FnPtr,
         pub(super) ProgramUniform4uivEXT: FnPtr,
         pub(super) ProgramUniformHandleui64ARB: FnPtr,
         pub(super) ProgramUniformHandleui64NV: FnPtr,
         pub(super) ProgramUniformHandleui64vARB: FnPtr,
         pub(super) ProgramUniformHandleui64vNV: FnPtr,
         pub(super) ProgramUniformMatrix2dv: FnPtr,
         pub(super) ProgramUniformMatrix2dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2fv: FnPtr,
         pub(super) ProgramUniformMatrix2fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2x3dv: FnPtr,
         pub(super) ProgramUniformMatrix2x3dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2x3fv: FnPtr,
         pub(super) ProgramUniformMatrix2x3fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2x4dv: FnPtr,
         pub(super) ProgramUniformMatrix2x4dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2x4fv: FnPtr,
         pub(super) ProgramUniformMatrix2x4fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3dv: FnPtr,
         pub(super) ProgramUniformMatrix3dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3fv: FnPtr,
         pub(super) ProgramUniformMatrix3fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3x2dv: FnPtr,
         pub(super) ProgramUniformMatrix3x2dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3x2fv: FnPtr,
         pub(super) ProgramUniformMatrix3x2fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3x4dv: FnPtr,
         pub(super) ProgramUniformMatrix3x4dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3x4fv: FnPtr,
         pub(super) ProgramUniformMatrix3x4fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4dv: FnPtr,
         pub(super) ProgramUniformMatrix4dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4fv: FnPtr,
         pub(super) ProgramUniformMatrix4fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4x2dv: FnPtr,
         pub(super) ProgramUniformMatrix4x2dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4x2fv: FnPtr,
         pub(super) ProgramUniformMatrix4x2fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4x3dv: FnPtr,
         pub(super) ProgramUniformMatrix4x3dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4x3fv: FnPtr,
         pub(super) ProgramUniformMatrix4x3fvEXT: FnPtr,
         pub(super) ProgramUniformui64NV: FnPtr,
         pub(super) ProgramUniformui64vNV: FnPtr,
         pub(super) ProgramVertexLimitNV: FnPtr,
         pub(super) ProvokingVertex: FnPtr,
         pub(super) ProvokingVertexEXT: FnPtr,
         pub(super) PushClientAttribDefaultEXT: FnPtr,
         pub(super) PushDebugGroup: FnPtr,
         pub(super) PushGroupMarkerEXT: FnPtr,
         pub(super) QueryCounter: FnPtr,
         pub(super) QueryMatrixxOES: FnPtr,
         pub(super) QueryObjectParameteruiAMD: FnPtr,
         pub(super) QueryResourceNV: FnPtr,
         pub(super) QueryResourceTagNV: FnPtr,
         pub(super) RasterPos2xOES: FnPtr,
         pub(super) RasterPos2xvOES: FnPtr,
         pub(super) RasterPos3xOES: FnPtr,
         pub(super) RasterPos3xvOES: FnPtr,
         pub(super) RasterPos4xOES: FnPtr,
         pub(super) RasterPos4xvOES: FnPtr,
         pub(super) RasterSamplesEXT: FnPtr,
         pub(super) ReadBuffer: FnPtr,
         pub(super) ReadInstrumentsSGIX: FnPtr,
         pub(super) ReadPixels: FnPtr,
         pub(super) ReadnPixels: FnPtr,
         pub(super) ReadnPixelsARB: FnPtr,
         pub(super) RectxOES: FnPtr,
         pub(super) RectxvOES: FnPtr,
         pub(super) ReferencePlaneSGIX: FnPtr,
         pub(super) ReleaseKeyedMutexWin32EXT: FnPtr,
         pub(super) ReleaseShaderCompiler: FnPtr,
         pub(super) RenderGpuMaskNV: FnPtr,
         pub(super) RenderbufferStorage: FnPtr,
         pub(super) RenderbufferStorageEXT: FnPtr,
         pub(super) RenderbufferStorageMultisample: FnPtr,
         pub(super) RenderbufferStorageMultisampleAdvancedAMD: FnPtr,
         pub(super) RenderbufferStorageMultisampleCoverageNV: FnPtr,
         pub(super) RenderbufferStorageMultisampleEXT: FnPtr,
         pub(super) ReplacementCodePointerSUN: FnPtr,
         pub(super) ReplacementCodeubSUN: FnPtr,
         pub(super) ReplacementCodeubvSUN: FnPtr,
         pub(super) ReplacementCodeuiColor3fVertex3fSUN: FnPtr,
         pub(super) ReplacementCodeuiColor3fVertex3fvSUN: FnPtr,
         pub(super) ReplacementCodeuiColor4fNormal3fVertex3fSUN: FnPtr,
         pub(super) ReplacementCodeuiColor4fNormal3fVertex3fvSUN: FnPtr,
         pub(super) ReplacementCodeuiColor4ubVertex3fSUN: FnPtr,
         pub(super) ReplacementCodeuiColor4ubVertex3fvSUN: FnPtr,
         pub(super) ReplacementCodeuiNormal3fVertex3fSUN: FnPtr,
         pub(super) ReplacementCodeuiNormal3fVertex3fvSUN: FnPtr,
         pub(super) ReplacementCodeuiSUN: FnPtr,
         pub(super) ReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN: FnPtr,
         pub(super) ReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN: FnPtr,
         pub(super) ReplacementCodeuiTexCoord2fNormal3fVertex3fSUN: FnPtr,
         pub(super) ReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN: FnPtr,
         pub(super) ReplacementCodeuiTexCoord2fVertex3fSUN: FnPtr,
         pub(super) ReplacementCodeuiTexCoord2fVertex3fvSUN: FnPtr,
         pub(super) ReplacementCodeuiVertex3fSUN: FnPtr,
         pub(super) ReplacementCodeuiVertex3fvSUN: FnPtr,
         pub(super) ReplacementCodeuivSUN: FnPtr,
         pub(super) ReplacementCodeusSUN: FnPtr,
         pub(super) ReplacementCodeusvSUN: FnPtr,
         pub(super) RequestResidentProgramsNV: FnPtr,
         pub(super) ResetHistogramEXT: FnPtr,
         pub(super) ResetMemoryObjectParameterNV: FnPtr,
         pub(super) ResetMinmaxEXT: FnPtr,
         pub(super) ResizeBuffersMESA: FnPtr,
         pub(super) ResolveDepthValuesNV: FnPtr,
         pub(super) ResumeTransformFeedback: FnPtr,
         pub(super) ResumeTransformFeedbackNV: FnPtr,
         pub(super) RotatexOES: FnPtr,
         pub(super) SampleCoverage: FnPtr,
         pub(super) SampleCoverageARB: FnPtr,
         pub(super) SampleMapATI: FnPtr,
         pub(super) SampleMaskEXT: FnPtr,
         pub(super) SampleMaskIndexedNV: FnPtr,
         pub(super) SampleMaskSGIS: FnPtr,
         pub(super) SampleMaski: FnPtr,
         pub(super) SamplePatternEXT: FnPtr,
         pub(super) SamplePatternSGIS: FnPtr,
         pub(super) SamplerParameterIiv: FnPtr,
         pub(super) SamplerParameterIuiv: FnPtr,
         pub(super) SamplerParameterf: FnPtr,
         pub(super) SamplerParameterfv: FnPtr,
         pub(super) SamplerParameteri: FnPtr,
         pub(super) SamplerParameteriv: FnPtr,
         pub(super) ScalexOES: FnPtr,
         pub(super) Scissor: FnPtr,
         pub(super) ScissorArrayv: FnPtr,
         pub(super) ScissorExclusiveArrayvNV: FnPtr,
         pub(super) ScissorExclusiveNV: FnPtr,
         pub(super) ScissorIndexed: FnPtr,
         pub(super) ScissorIndexedv: FnPtr,
         pub(super) SecondaryColor3bEXT: FnPtr,
         pub(super) SecondaryColor3bvEXT: FnPtr,
         pub(super) SecondaryColor3dEXT: FnPtr,
         pub(super) SecondaryColor3dvEXT: FnPtr,
         pub(super) SecondaryColor3fEXT: FnPtr,
         pub(super) SecondaryColor3fvEXT: FnPtr,
         pub(super) SecondaryColor3hNV: FnPtr,
         pub(super) SecondaryColor3hvNV: FnPtr,
         pub(super) SecondaryColor3iEXT: FnPtr,
         pub(super) SecondaryColor3ivEXT: FnPtr,
         pub(super) SecondaryColor3sEXT: FnPtr,
         pub(super) SecondaryColor3svEXT: FnPtr,
         pub(super) SecondaryColor3ubEXT: FnPtr,
         pub(super) SecondaryColor3ubvEXT: FnPtr,
         pub(super) SecondaryColor3uiEXT: FnPtr,
         pub(super) SecondaryColor3uivEXT: FnPtr,
         pub(super) SecondaryColor3usEXT: FnPtr,
         pub(super) SecondaryColor3usvEXT: FnPtr,
         pub(super) SecondaryColorFormatNV: FnPtr,
         pub(super) SecondaryColorPointerEXT: FnPtr,
         pub(super) SecondaryColorPointerListIBM: FnPtr,
         pub(super) SelectPerfMonitorCountersAMD: FnPtr,
         pub(super) SemaphoreParameterivNV: FnPtr,
         pub(super) SemaphoreParameterui64vEXT: FnPtr,
         pub(super) SeparableFilter2DEXT: FnPtr,
         pub(super) SetFenceAPPLE: FnPtr,
         pub(super) SetFenceNV: FnPtr,
         pub(super) SetFragmentShaderConstantATI: FnPtr,
         pub(super) SetInvariantEXT: FnPtr,
         pub(super) SetLocalConstantEXT: FnPtr,
         pub(super) SetMultisamplefvAMD: FnPtr,
         pub(super) ShaderBinary: FnPtr,
         pub(super) ShaderOp1EXT: FnPtr,
         pub(super) ShaderOp2EXT: FnPtr,
         pub(super) ShaderOp3EXT: FnPtr,
         pub(super) ShaderSource: FnPtr,
         pub(super) ShaderSourceARB: FnPtr,
         pub(super) ShaderStorageBlockBinding: FnPtr,
         pub(super) ShadingRateImageBarrierNV: FnPtr,
         pub(super) ShadingRateImagePaletteNV: FnPtr,
         pub(super) ShadingRateSampleOrderCustomNV: FnPtr,
         pub(super) ShadingRateSampleOrderNV: FnPtr,
         pub(super) SharpenTexFuncSGIS: FnPtr,
         pub(super) SignalSemaphoreEXT: FnPtr,
         pub(super) SignalSemaphoreui64NVX: FnPtr,
         pub(super) SignalVkFenceNV: FnPtr,
         pub(super) SignalVkSemaphoreNV: FnPtr,
         pub(super) SpecializeShader: FnPtr,
         pub(super) SpecializeShaderARB: FnPtr,
         pub(super) SpriteParameterfSGIX: FnPtr,
         pub(super) SpriteParameterfvSGIX: FnPtr,
         pub(super) SpriteParameteriSGIX: FnPtr,
         pub(super) SpriteParameterivSGIX: FnPtr,
         pub(super) StartInstrumentsSGIX: FnPtr,
         pub(super) StateCaptureNV: FnPtr,
         pub(super) StencilClearTagEXT: FnPtr,
         pub(super) StencilFillPathInstancedNV: FnPtr,
         pub(super) StencilFillPathNV: FnPtr,
         pub(super) StencilFunc: FnPtr,
         pub(super) StencilFuncSeparate: FnPtr,
         pub(super) StencilFuncSeparateATI: FnPtr,
         pub(super) StencilMask: FnPtr,
         pub(super) StencilMaskSeparate: FnPtr,
         pub(super) StencilOp: FnPtr,
         pub(super) StencilOpSeparate: FnPtr,
         pub(super) StencilOpSeparateATI: FnPtr,
         pub(super) StencilOpValueAMD: FnPtr,
         pub(super) StencilStrokePathInstancedNV: FnPtr,
         pub(super) StencilStrokePathNV: FnPtr,
         pub(super) StencilThenCoverFillPathInstancedNV: FnPtr,
         pub(super) StencilThenCoverFillPathNV: FnPtr,
         pub(super) StencilThenCoverStrokePathInstancedNV: FnPtr,
         pub(super) StencilThenCoverStrokePathNV: FnPtr,
         pub(super) StopInstrumentsSGIX: FnPtr,
         pub(super) StringMarkerGREMEDY: FnPtr,
         pub(super) SubpixelPrecisionBiasNV: FnPtr,
         pub(super) SwizzleEXT: FnPtr,
         pub(super) SyncTextureINTEL: FnPtr,
         pub(super) TagSampleBufferSGIX: FnPtr,
         pub(super) Tangent3bEXT: FnPtr,
         pub(super) Tangent3bvEXT: FnPtr,
         pub(super) Tangent3dEXT: FnPtr,
         pub(super) Tangent3dvEXT: FnPtr,
         pub(super) Tangent3fEXT: FnPtr,
         pub(super) Tangent3fvEXT: FnPtr,
         pub(super) Tangent3iEXT: FnPtr,
         pub(super) Tangent3ivEXT: FnPtr,
         pub(super) Tangent3sEXT: FnPtr,
         pub(super) Tangent3svEXT: FnPtr,
         pub(super) TangentPointerEXT: FnPtr,
         pub(super) TbufferMask3DFX: FnPtr,
         pub(super) TessellationFactorAMD: FnPtr,
         pub(super) TessellationModeAMD: FnPtr,
         pub(super) TestFenceAPPLE: FnPtr,
         pub(super) TestFenceNV: FnPtr,
         pub(super) TestObjectAPPLE: FnPtr,
         pub(super) TexAttachMemoryNV: FnPtr,
         pub(super) TexBuffer: FnPtr,
         pub(super) TexBufferARB: FnPtr,
         pub(super) TexBufferEXT: FnPtr,
         pub(super) TexBufferRange: FnPtr,
         pub(super) TexBumpParameterfvATI: FnPtr,
         pub(super) TexBumpParameterivATI: FnPtr,
         pub(super) TexCoord1bOES: FnPtr,
         pub(super) TexCoord1bvOES: FnPtr,
         pub(super) TexCoord1hNV: FnPtr,
         pub(super) TexCoord1hvNV: FnPtr,
         pub(super) TexCoord1xOES: FnPtr,
         pub(super) TexCoord1xvOES: FnPtr,
         pub(super) TexCoord2bOES: FnPtr,
         pub(super) TexCoord2bvOES: FnPtr,
         pub(super) TexCoord2fColor3fVertex3fSUN: FnPtr,
         pub(super) TexCoord2fColor3fVertex3fvSUN: FnPtr,
         pub(super) TexCoord2fColor4fNormal3fVertex3fSUN: FnPtr,
         pub(super) TexCoord2fColor4fNormal3fVertex3fvSUN: FnPtr,
         pub(super) TexCoord2fColor4ubVertex3fSUN: FnPtr,
         pub(super) TexCoord2fColor4ubVertex3fvSUN: FnPtr,
         pub(super) TexCoord2fNormal3fVertex3fSUN: FnPtr,
         pub(super) TexCoord2fNormal3fVertex3fvSUN: FnPtr,
         pub(super) TexCoord2fVertex3fSUN: FnPtr,
         pub(super) TexCoord2fVertex3fvSUN: FnPtr,
         pub(super) TexCoord2hNV: FnPtr,
         pub(super) TexCoord2hvNV: FnPtr,
         pub(super) TexCoord2xOES: FnPtr,
         pub(super) TexCoord2xvOES: FnPtr,
         pub(super) TexCoord3bOES: FnPtr,
         pub(super) TexCoord3bvOES: FnPtr,
         pub(super) TexCoord3hNV: FnPtr,
         pub(super) TexCoord3hvNV: FnPtr,
         pub(super) TexCoord3xOES: FnPtr,
         pub(super) TexCoord3xvOES: FnPtr,
         pub(super) TexCoord4bOES: FnPtr,
         pub(super) TexCoord4bvOES: FnPtr,
         pub(super) TexCoord4fColor4fNormal3fVertex4fSUN: FnPtr,
         pub(super) TexCoord4fColor4fNormal3fVertex4fvSUN: FnPtr,
         pub(super) TexCoord4fVertex4fSUN: FnPtr,
         pub(super) TexCoord4fVertex4fvSUN: FnPtr,
         pub(super) TexCoord4hNV: FnPtr,
         pub(super) TexCoord4hvNV: FnPtr,
         pub(super) TexCoord4xOES: FnPtr,
         pub(super) TexCoord4xvOES: FnPtr,
         pub(super) TexCoordFormatNV: FnPtr,
         pub(super) TexCoordPointerEXT: FnPtr,
         pub(super) TexCoordPointerListIBM: FnPtr,
         pub(super) TexCoordPointervINTEL: FnPtr,
         pub(super) TexEnvxOES: FnPtr,
         pub(super) TexEnvxvOES: FnPtr,
         pub(super) TexFilterFuncSGIS: FnPtr,
         pub(super) TexGenxOES: FnPtr,
         pub(super) TexGenxvOES: FnPtr,
         pub(super) TexImage1D: FnPtr,
         pub(super) TexImage2D: FnPtr,
         pub(super) TexImage2DMultisample: FnPtr,
         pub(super) TexImage2DMultisampleCoverageNV: FnPtr,
         pub(super) TexImage3D: FnPtr,
         pub(super) TexImage3DEXT: FnPtr,
         pub(super) TexImage3DMultisample: FnPtr,
         pub(super) TexImage3DMultisampleCoverageNV: FnPtr,
         pub(super) TexImage4DSGIS: FnPtr,
         pub(super) TexPageCommitmentARB: FnPtr,
         pub(super) TexPageCommitmentMemNV: FnPtr,
         pub(super) TexParameterIiv: FnPtr,
         pub(super) TexParameterIivEXT: FnPtr,
         pub(super) TexParameterIuiv: FnPtr,
         pub(super) TexParameterIuivEXT: FnPtr,
         pub(super) TexParameterf: FnPtr,
         pub(super) TexParameterfv: FnPtr,
         pub(super) TexParameteri: FnPtr,
         pub(super) TexParameteriv: FnPtr,
         pub(super) TexParameterxOES: FnPtr,
         pub(super) TexParameterxvOES: FnPtr,
         pub(super) TexRenderbufferNV: FnPtr,
         pub(super) TexStorage1D: FnPtr,
         pub(super) TexStorage1DEXT: FnPtr,
         pub(super) TexStorage2D: FnPtr,
         pub(super) TexStorage2DEXT: FnPtr,
         pub(super) TexStorage2DMultisample: FnPtr,
         pub(super) TexStorage3D: FnPtr,
         pub(super) TexStorage3DEXT: FnPtr,
         pub(super) TexStorage3DMultisample: FnPtr,
         pub(super) TexStorageMem1DEXT: FnPtr,
         pub(super) TexStorageMem2DEXT: FnPtr,
         pub(super) TexStorageMem2DMultisampleEXT: FnPtr,
         pub(super) TexStorageMem3DEXT: FnPtr,
         pub(super) TexStorageMem3DMultisampleEXT: FnPtr,
         pub(super) TexStorageSparseAMD: FnPtr,
         pub(super) TexSubImage1D: FnPtr,
         pub(super) TexSubImage1DEXT: FnPtr,
         pub(super) TexSubImage2D: FnPtr,
         pub(super) TexSubImage2DEXT: FnPtr,
         pub(super) TexSubImage3D: FnPtr,
         pub(super) TexSubImage3DEXT: FnPtr,
         pub(super) TexSubImage4DSGIS: FnPtr,
         pub(super) TextureAttachMemoryNV: FnPtr,
         pub(super) TextureBarrier: FnPtr,
         pub(super) TextureBarrierNV: FnPtr,
         pub(super) TextureBuffer: FnPtr,
         pub(super) TextureBufferEXT: FnPtr,
         pub(super) TextureBufferRange: FnPtr,
         pub(super) TextureBufferRangeEXT: FnPtr,
         pub(super) TextureColorMaskSGIS: FnPtr,
         pub(super) TextureImage1DEXT: FnPtr,
         pub(super) TextureImage2DEXT: FnPtr,
         pub(super) TextureImage2DMultisampleCoverageNV: FnPtr,
         pub(super) TextureImage2DMultisampleNV: FnPtr,
         pub(super) TextureImage3DEXT: FnPtr,
         pub(super) TextureImage3DMultisampleCoverageNV: FnPtr,
         pub(super) TextureImage3DMultisampleNV: FnPtr,
         pub(super) TextureLightEXT: FnPtr,
         pub(super) TextureMaterialEXT: FnPtr,
         pub(super) TextureNormalEXT: FnPtr,
         pub(super) TexturePageCommitmentEXT: FnPtr,
         pub(super) TexturePageCommitmentMemNV: FnPtr,
         pub(super) TextureParameterIiv: FnPtr,
         pub(super) TextureParameterIivEXT: FnPtr,
         pub(super) TextureParameterIuiv: FnPtr,
         pub(super) TextureParameterIuivEXT: FnPtr,
         pub(super) TextureParameterf: FnPtr,
         pub(super) TextureParameterfEXT: FnPtr,
         pub(super) TextureParameterfv: FnPtr,
         pub(super) TextureParameterfvEXT: FnPtr,
         pub(super) TextureParameteri: FnPtr,
         pub(super) TextureParameteriEXT: FnPtr,
         pub(super) TextureParameteriv: FnPtr,
         pub(super) TextureParameterivEXT: FnPtr,
         pub(super) TextureRangeAPPLE: FnPtr,
         pub(super) TextureRenderbufferEXT: FnPtr,
         pub(super) TextureStorage1D: FnPtr,
         pub(super) TextureStorage1DEXT: FnPtr,
         pub(super) TextureStorage2D: FnPtr,
         pub(super) TextureStorage2DEXT: FnPtr,
         pub(super) TextureStorage2DMultisample: FnPtr,
         pub(super) TextureStorage2DMultisampleEXT: FnPtr,
         pub(super) TextureStorage3D: FnPtr,
         pub(super) TextureStorage3DEXT: FnPtr,
         pub(super) TextureStorage3DMultisample: FnPtr,
         pub(super) TextureStorage3DMultisampleEXT: FnPtr,
         pub(super) TextureStorageMem1DEXT: FnPtr,
         pub(super) TextureStorageMem2DEXT: FnPtr,
         pub(super) TextureStorageMem2DMultisampleEXT: FnPtr,
         pub(super) TextureStorageMem3DEXT: FnPtr,
         pub(super) TextureStorageMem3DMultisampleEXT: FnPtr,
         pub(super) TextureStorageSparseAMD: FnPtr,
         pub(super) TextureSubImage1D: FnPtr,
         pub(super) TextureSubImage1DEXT: FnPtr,
         pub(super) TextureSubImage2D: FnPtr,
         pub(super) TextureSubImage2DEXT: FnPtr,
         pub(super) TextureSubImage3D: FnPtr,
         pub(super) TextureSubImage3DEXT: FnPtr,
         pub(super) TextureView: FnPtr,
         pub(super) TrackMatrixNV: FnPtr,
         pub(super) TransformFeedbackAttribsNV: FnPtr,
         pub(super) TransformFeedbackBufferBase: FnPtr,
         pub(super) TransformFeedbackBufferRange: FnPtr,
         pub(super) TransformFeedbackStreamAttribsNV: FnPtr,
         pub(super) TransformFeedbackVaryings: FnPtr,
         pub(super) TransformFeedbackVaryingsEXT: FnPtr,
         pub(super) TransformFeedbackVaryingsNV: FnPtr,
         pub(super) TransformPathNV: FnPtr,
         pub(super) TranslatexOES: FnPtr,
         pub(super) Uniform1d: FnPtr,
         pub(super) Uniform1dv: FnPtr,
         pub(super) Uniform1f: FnPtr,
         pub(super) Uniform1fARB: FnPtr,
         pub(super) Uniform1fv: FnPtr,
         pub(super) Uniform1fvARB: FnPtr,
         pub(super) Uniform1i: FnPtr,
         pub(super) Uniform1i64ARB: FnPtr,
         pub(super) Uniform1i64NV: FnPtr,
         pub(super) Uniform1i64vARB: FnPtr,
         pub(super) Uniform1i64vNV: FnPtr,
         pub(super) Uniform1iARB: FnPtr,
         pub(super) Uniform1iv: FnPtr,
         pub(super) Uniform1ivARB: FnPtr,
         pub(super) Uniform1ui: FnPtr,
         pub(super) Uniform1ui64ARB: FnPtr,
         pub(super) Uniform1ui64NV: FnPtr,
         pub(super) Uniform1ui64vARB: FnPtr,
         pub(super) Uniform1ui64vNV: FnPtr,
         pub(super) Uniform1uiEXT: FnPtr,
         pub(super) Uniform1uiv: FnPtr,
         pub(super) Uniform1uivEXT: FnPtr,
         pub(super) Uniform2d: FnPtr,
         pub(super) Uniform2dv: FnPtr,
         pub(super) Uniform2f: FnPtr,
         pub(super) Uniform2fARB: FnPtr,
         pub(super) Uniform2fv: FnPtr,
         pub(super) Uniform2fvARB: FnPtr,
         pub(super) Uniform2i: FnPtr,
         pub(super) Uniform2i64ARB: FnPtr,
         pub(super) Uniform2i64NV: FnPtr,
         pub(super) Uniform2i64vARB: FnPtr,
         pub(super) Uniform2i64vNV: FnPtr,
         pub(super) Uniform2iARB: FnPtr,
         pub(super) Uniform2iv: FnPtr,
         pub(super) Uniform2ivARB: FnPtr,
         pub(super) Uniform2ui: FnPtr,
         pub(super) Uniform2ui64ARB: FnPtr,
         pub(super) Uniform2ui64NV: FnPtr,
         pub(super) Uniform2ui64vARB: FnPtr,
         pub(super) Uniform2ui64vNV: FnPtr,
         pub(super) Uniform2uiEXT: FnPtr,
         pub(super) Uniform2uiv: FnPtr,
         pub(super) Uniform2uivEXT: FnPtr,
         pub(super) Uniform3d: FnPtr,
         pub(super) Uniform3dv: FnPtr,
         pub(super) Uniform3f: FnPtr,
         pub(super) Uniform3fARB: FnPtr,
         pub(super) Uniform3fv: FnPtr,
         pub(super) Uniform3fvARB: FnPtr,
         pub(super) Uniform3i: FnPtr,
         pub(super) Uniform3i64ARB: FnPtr,
         pub(super) Uniform3i64NV: FnPtr,
         pub(super) Uniform3i64vARB: FnPtr,
         pub(super) Uniform3i64vNV: FnPtr,
         pub(super) Uniform3iARB: FnPtr,
         pub(super) Uniform3iv: FnPtr,
         pub(super) Uniform3ivARB: FnPtr,
         pub(super) Uniform3ui: FnPtr,
         pub(super) Uniform3ui64ARB: FnPtr,
         pub(super) Uniform3ui64NV: FnPtr,
         pub(super) Uniform3ui64vARB: FnPtr,
         pub(super) Uniform3ui64vNV: FnPtr,
         pub(super) Uniform3uiEXT: FnPtr,
         pub(super) Uniform3uiv: FnPtr,
         pub(super) Uniform3uivEXT: FnPtr,
         pub(super) Uniform4d: FnPtr,
         pub(super) Uniform4dv: FnPtr,
         pub(super) Uniform4f: FnPtr,
         pub(super) Uniform4fARB: FnPtr,
         pub(super) Uniform4fv: FnPtr,
         pub(super) Uniform4fvARB: FnPtr,
         pub(super) Uniform4i: FnPtr,
         pub(super) Uniform4i64ARB: FnPtr,
         pub(super) Uniform4i64NV: FnPtr,
         pub(super) Uniform4i64vARB: FnPtr,
         pub(super) Uniform4i64vNV: FnPtr,
         pub(super) Uniform4iARB: FnPtr,
         pub(super) Uniform4iv: FnPtr,
         pub(super) Uniform4ivARB: FnPtr,
         pub(super) Uniform4ui: FnPtr,
         pub(super) Uniform4ui64ARB: FnPtr,
         pub(super) Uniform4ui64NV: FnPtr,
         pub(super) Uniform4ui64vARB: FnPtr,
         pub(super) Uniform4ui64vNV: FnPtr,
         pub(super) Uniform4uiEXT: FnPtr,
         pub(super) Uniform4uiv: FnPtr,
         pub(super) Uniform4uivEXT: FnPtr,
         pub(super) UniformBlockBinding: FnPtr,
         pub(super) UniformBufferEXT: FnPtr,
         pub(super) UniformHandleui64ARB: FnPtr,
         pub(super) UniformHandleui64NV: FnPtr,
         pub(super) UniformHandleui64vARB: FnPtr,
         pub(super) UniformHandleui64vNV: FnPtr,
         pub(super) UniformMatrix2dv: FnPtr,
         pub(super) UniformMatrix2fv: FnPtr,
         pub(super) UniformMatrix2fvARB: FnPtr,
         pub(super) UniformMatrix2x3dv: FnPtr,
         pub(super) UniformMatrix2x3fv: FnPtr,
         pub(super) UniformMatrix2x4dv: FnPtr,
         pub(super) UniformMatrix2x4fv: FnPtr,
         pub(super) UniformMatrix3dv: FnPtr,
         pub(super) UniformMatrix3fv: FnPtr,
         pub(super) UniformMatrix3fvARB: FnPtr,
         pub(super) UniformMatrix3x2dv: FnPtr,
         pub(super) UniformMatrix3x2fv: FnPtr,
         pub(super) UniformMatrix3x4dv: FnPtr,
         pub(super) UniformMatrix3x4fv: FnPtr,
         pub(super) UniformMatrix4dv: FnPtr,
         pub(super) UniformMatrix4fv: FnPtr,
         pub(super) UniformMatrix4fvARB: FnPtr,
         pub(super) UniformMatrix4x2dv: FnPtr,
         pub(super) UniformMatrix4x2fv: FnPtr,
         pub(super) UniformMatrix4x3dv: FnPtr,
         pub(super) UniformMatrix4x3fv: FnPtr,
         pub(super) UniformSubroutinesuiv: FnPtr,
         pub(super) Uniformui64NV: FnPtr,
         pub(super) Uniformui64vNV: FnPtr,
         pub(super) UnlockArraysEXT: FnPtr,
         pub(super) UnmapBuffer: FnPtr,
         pub(super) UnmapBufferARB: FnPtr,
         pub(super) UnmapNamedBuffer: FnPtr,
         pub(super) UnmapNamedBufferEXT: FnPtr,
         pub(super) UnmapObjectBufferATI: FnPtr,
         pub(super) UnmapTexture2DINTEL: FnPtr,
         pub(super) UpdateObjectBufferATI: FnPtr,
         pub(super) UploadGpuMaskNVX: FnPtr,
         pub(super) UseProgram: FnPtr,
         pub(super) UseProgramObjectARB: FnPtr,
         pub(super) UseProgramStages: FnPtr,
         pub(super) UseShaderProgramEXT: FnPtr,
         pub(super) VDPAUFiniNV: FnPtr,
         pub(super) VDPAUGetSurfaceivNV: FnPtr,
         pub(super) VDPAUInitNV: FnPtr,
         pub(super) VDPAUIsSurfaceNV: FnPtr,
         pub(super) VDPAUMapSurfacesNV: FnPtr,
         pub(super) VDPAURegisterOutputSurfaceNV: FnPtr,
         pub(super) VDPAURegisterVideoSurfaceNV: FnPtr,
         pub(super) VDPAURegisterVideoSurfaceWithPictureStructureNV: FnPtr,
         pub(super) VDPAUSurfaceAccessNV: FnPtr,
         pub(super) VDPAUUnmapSurfacesNV: FnPtr,
         pub(super) VDPAUUnregisterSurfaceNV: FnPtr,
         pub(super) ValidateProgram: FnPtr,
         pub(super) ValidateProgramARB: FnPtr,
         pub(super) ValidateProgramPipeline: FnPtr,
         pub(super) VariantArrayObjectATI: FnPtr,
         pub(super) VariantPointerEXT: FnPtr,
         pub(super) VariantbvEXT: FnPtr,
         pub(super) VariantdvEXT: FnPtr,
         pub(super) VariantfvEXT: FnPtr,
         pub(super) VariantivEXT: FnPtr,
         pub(super) VariantsvEXT: FnPtr,
         pub(super) VariantubvEXT: FnPtr,
         pub(super) VariantuivEXT: FnPtr,
         pub(super) VariantusvEXT: FnPtr,
         pub(super) Vertex2bOES: FnPtr,
         pub(super) Vertex2bvOES: FnPtr,
         pub(super) Vertex2hNV: FnPtr,
         pub(super) Vertex2hvNV: FnPtr,
         pub(super) Vertex2xOES: FnPtr,
         pub(super) Vertex2xvOES: FnPtr,
         pub(super) Vertex3bOES: FnPtr,
         pub(super) Vertex3bvOES: FnPtr,
         pub(super) Vertex3hNV: FnPtr,
         pub(super) Vertex3hvNV: FnPtr,
         pub(super) Vertex3xOES: FnPtr,
         pub(super) Vertex3xvOES: FnPtr,
         pub(super) Vertex4bOES: FnPtr,
         pub(super) Vertex4bvOES: FnPtr,
         pub(super) Vertex4hNV: FnPtr,
         pub(super) Vertex4hvNV: FnPtr,
         pub(super) Vertex4xOES: FnPtr,
         pub(super) Vertex4xvOES: FnPtr,
         pub(super) VertexArrayAttribBinding: FnPtr,
         pub(super) VertexArrayAttribFormat: FnPtr,
         pub(super) VertexArrayAttribIFormat: FnPtr,
         pub(super) VertexArrayAttribLFormat: FnPtr,
         pub(super) VertexArrayBindVertexBufferEXT: FnPtr,
         pub(super) VertexArrayBindingDivisor: FnPtr,
         pub(super) VertexArrayColorOffsetEXT: FnPtr,
         pub(super) VertexArrayEdgeFlagOffsetEXT: FnPtr,
         pub(super) VertexArrayElementBuffer: FnPtr,
         pub(super) VertexArrayFogCoordOffsetEXT: FnPtr,
         pub(super) VertexArrayIndexOffsetEXT: FnPtr,
         pub(super) VertexArrayMultiTexCoordOffsetEXT: FnPtr,
         pub(super) VertexArrayNormalOffsetEXT: FnPtr,
         pub(super) VertexArrayParameteriAPPLE: FnPtr,
         pub(super) VertexArrayRangeAPPLE: FnPtr,
         pub(super) VertexArrayRangeNV: FnPtr,
         pub(super) VertexArraySecondaryColorOffsetEXT: FnPtr,
         pub(super) VertexArrayTexCoordOffsetEXT: FnPtr,
         pub(super) VertexArrayVertexAttribBindingEXT: FnPtr,
         pub(super) VertexArrayVertexAttribDivisorEXT: FnPtr,
         pub(super) VertexArrayVertexAttribFormatEXT: FnPtr,
         pub(super) VertexArrayVertexAttribIFormatEXT: FnPtr,
         pub(super) VertexArrayVertexAttribIOffsetEXT: FnPtr,
         pub(super) VertexArrayVertexAttribLFormatEXT: FnPtr,
         pub(super) VertexArrayVertexAttribLOffsetEXT: FnPtr,
         pub(super) VertexArrayVertexAttribOffsetEXT: FnPtr,
         pub(super) VertexArrayVertexBindingDivisorEXT: FnPtr,
         pub(super) VertexArrayVertexBuffer: FnPtr,
         pub(super) VertexArrayVertexBuffers: FnPtr,
         pub(super) VertexArrayVertexOffsetEXT: FnPtr,
         pub(super) VertexAttrib1d: FnPtr,
         pub(super) VertexAttrib1dARB: FnPtr,
         pub(super) VertexAttrib1dNV: FnPtr,
         pub(super) VertexAttrib1dv: FnPtr,
         pub(super) VertexAttrib1dvARB: FnPtr,
         pub(super) VertexAttrib1dvNV: FnPtr,
         pub(super) VertexAttrib1f: FnPtr,
         pub(super) VertexAttrib1fARB: FnPtr,
         pub(super) VertexAttrib1fNV: FnPtr,
         pub(super) VertexAttrib1fv: FnPtr,
         pub(super) VertexAttrib1fvARB: FnPtr,
         pub(super) VertexAttrib1fvNV: FnPtr,
         pub(super) VertexAttrib1hNV: FnPtr,
         pub(super) VertexAttrib1hvNV: FnPtr,
         pub(super) VertexAttrib1s: FnPtr,
         pub(super) VertexAttrib1sARB: FnPtr,
         pub(super) VertexAttrib1sNV: FnPtr,
         pub(super) VertexAttrib1sv: FnPtr,
         pub(super) VertexAttrib1svARB: FnPtr,
         pub(super) VertexAttrib1svNV: FnPtr,
         pub(super) VertexAttrib2d: FnPtr,
         pub(super) VertexAttrib2dARB: FnPtr,
         pub(super) VertexAttrib2dNV: FnPtr,
         pub(super) VertexAttrib2dv: FnPtr,
         pub(super) VertexAttrib2dvARB: FnPtr,
         pub(super) VertexAttrib2dvNV: FnPtr,
         pub(super) VertexAttrib2f: FnPtr,
         pub(super) VertexAttrib2fARB: FnPtr,
         pub(super) VertexAttrib2fNV: FnPtr,
         pub(super) VertexAttrib2fv: FnPtr,
         pub(super) VertexAttrib2fvARB: FnPtr,
         pub(super) VertexAttrib2fvNV: FnPtr,
         pub(super) VertexAttrib2hNV: FnPtr,
         pub(super) VertexAttrib2hvNV: FnPtr,
         pub(super) VertexAttrib2s: FnPtr,
         pub(super) VertexAttrib2sARB: FnPtr,
         pub(super) VertexAttrib2sNV: FnPtr,
         pub(super) VertexAttrib2sv: FnPtr,
         pub(super) VertexAttrib2svARB: FnPtr,
         pub(super) VertexAttrib2svNV: FnPtr,
         pub(super) VertexAttrib3d: FnPtr,
         pub(super) VertexAttrib3dARB: FnPtr,
         pub(super) VertexAttrib3dNV: FnPtr,
         pub(super) VertexAttrib3dv: FnPtr,
         pub(super) VertexAttrib3dvARB: FnPtr,
         pub(super) VertexAttrib3dvNV: FnPtr,
         pub(super) VertexAttrib3f: FnPtr,
         pub(super) VertexAttrib3fARB: FnPtr,
         pub(super) VertexAttrib3fNV: FnPtr,
         pub(super) VertexAttrib3fv: FnPtr,
         pub(super) VertexAttrib3fvARB: FnPtr,
         pub(super) VertexAttrib3fvNV: FnPtr,
         pub(super) VertexAttrib3hNV: FnPtr,
         pub(super) VertexAttrib3hvNV: FnPtr,
         pub(super) VertexAttrib3s: FnPtr,
         pub(super) VertexAttrib3sARB: FnPtr,
         pub(super) VertexAttrib3sNV: FnPtr,
         pub(super) VertexAttrib3sv: FnPtr,
         pub(super) VertexAttrib3svARB: FnPtr,
         pub(super) VertexAttrib3svNV: FnPtr,
         pub(super) VertexAttrib4Nbv: FnPtr,
         pub(super) VertexAttrib4NbvARB: FnPtr,
         pub(super) VertexAttrib4Niv: FnPtr,
         pub(super) VertexAttrib4NivARB: FnPtr,
         pub(super) VertexAttrib4Nsv: FnPtr,
         pub(super) VertexAttrib4NsvARB: FnPtr,
         pub(super) VertexAttrib4Nub: FnPtr,
         pub(super) VertexAttrib4NubARB: FnPtr,
         pub(super) VertexAttrib4Nubv: FnPtr,
         pub(super) VertexAttrib4NubvARB: FnPtr,
         pub(super) VertexAttrib4Nuiv: FnPtr,
         pub(super) VertexAttrib4NuivARB: FnPtr,
         pub(super) VertexAttrib4Nusv: FnPtr,
         pub(super) VertexAttrib4NusvARB: FnPtr,
         pub(super) VertexAttrib4bv: FnPtr,
         pub(super) VertexAttrib4bvARB: FnPtr,
         pub(super) VertexAttrib4d: FnPtr,
         pub(super) VertexAttrib4dARB: FnPtr,
         pub(super) VertexAttrib4dNV: FnPtr,
         pub(super) VertexAttrib4dv: FnPtr,
         pub(super) VertexAttrib4dvARB: FnPtr,
         pub(super) VertexAttrib4dvNV: FnPtr,
         pub(super) VertexAttrib4f: FnPtr,
         pub(super) VertexAttrib4fARB: FnPtr,
         pub(super) VertexAttrib4fNV: FnPtr,
         pub(super) VertexAttrib4fv: FnPtr,
         pub(super) VertexAttrib4fvARB: FnPtr,
         pub(super) VertexAttrib4fvNV: FnPtr,
         pub(super) VertexAttrib4hNV: FnPtr,
         pub(super) VertexAttrib4hvNV: FnPtr,
         pub(super) VertexAttrib4iv: FnPtr,
         pub(super) VertexAttrib4ivARB: FnPtr,
         pub(super) VertexAttrib4s: FnPtr,
         pub(super) VertexAttrib4sARB: FnPtr,
         pub(super) VertexAttrib4sNV: FnPtr,
         pub(super) VertexAttrib4sv: FnPtr,
         pub(super) VertexAttrib4svARB: FnPtr,
         pub(super) VertexAttrib4svNV: FnPtr,
         pub(super) VertexAttrib4ubNV: FnPtr,
         pub(super) VertexAttrib4ubv: FnPtr,
         pub(super) VertexAttrib4ubvARB: FnPtr,
         pub(super) VertexAttrib4ubvNV: FnPtr,
         pub(super) VertexAttrib4uiv: FnPtr,
         pub(super) VertexAttrib4uivARB: FnPtr,
         pub(super) VertexAttrib4usv: FnPtr,
         pub(super) VertexAttrib4usvARB: FnPtr,
         pub(super) VertexAttribArrayObjectATI: FnPtr,
         pub(super) VertexAttribBinding: FnPtr,
         pub(super) VertexAttribDivisor: FnPtr,
         pub(super) VertexAttribDivisorARB: FnPtr,
         pub(super) VertexAttribFormat: FnPtr,
         pub(super) VertexAttribFormatNV: FnPtr,
         pub(super) VertexAttribI1i: FnPtr,
         pub(super) VertexAttribI1iEXT: FnPtr,
         pub(super) VertexAttribI1iv: FnPtr,
         pub(super) VertexAttribI1ivEXT: FnPtr,
         pub(super) VertexAttribI1ui: FnPtr,
         pub(super) VertexAttribI1uiEXT: FnPtr,
         pub(super) VertexAttribI1uiv: FnPtr,
         pub(super) VertexAttribI1uivEXT: FnPtr,
         pub(super) VertexAttribI2i: FnPtr,
         pub(super) VertexAttribI2iEXT: FnPtr,
         pub(super) VertexAttribI2iv: FnPtr,
         pub(super) VertexAttribI2ivEXT: FnPtr,
         pub(super) VertexAttribI2ui: FnPtr,
         pub(super) VertexAttribI2uiEXT: FnPtr,
         pub(super) VertexAttribI2uiv: FnPtr,
         pub(super) VertexAttribI2uivEXT: FnPtr,
         pub(super) VertexAttribI3i: FnPtr,
         pub(super) VertexAttribI3iEXT: FnPtr,
         pub(super) VertexAttribI3iv: FnPtr,
         pub(super) VertexAttribI3ivEXT: FnPtr,
         pub(super) VertexAttribI3ui: FnPtr,
         pub(super) VertexAttribI3uiEXT: FnPtr,
         pub(super) VertexAttribI3uiv: FnPtr,
         pub(super) VertexAttribI3uivEXT: FnPtr,
         pub(super) VertexAttribI4bv: FnPtr,
         pub(super) VertexAttribI4bvEXT: FnPtr,
         pub(super) VertexAttribI4i: FnPtr,
         pub(super) VertexAttribI4iEXT: FnPtr,
         pub(super) VertexAttribI4iv: FnPtr,
         pub(super) VertexAttribI4ivEXT: FnPtr,
         pub(super) VertexAttribI4sv: FnPtr,
         pub(super) VertexAttribI4svEXT: FnPtr,
         pub(super) VertexAttribI4ubv: FnPtr,
         pub(super) VertexAttribI4ubvEXT: FnPtr,
         pub(super) VertexAttribI4ui: FnPtr,
         pub(super) VertexAttribI4uiEXT: FnPtr,
         pub(super) VertexAttribI4uiv: FnPtr,
         pub(super) VertexAttribI4uivEXT: FnPtr,
         pub(super) VertexAttribI4usv: FnPtr,
         pub(super) VertexAttribI4usvEXT: FnPtr,
         pub(super) VertexAttribIFormat: FnPtr,
         pub(super) VertexAttribIFormatNV: FnPtr,
         pub(super) VertexAttribIPointer: FnPtr,
         pub(super) VertexAttribIPointerEXT: FnPtr,
         pub(super) VertexAttribL1d: FnPtr,
         pub(super) VertexAttribL1dEXT: FnPtr,
         pub(super) VertexAttribL1dv: FnPtr,
         pub(super) VertexAttribL1dvEXT: FnPtr,
         pub(super) VertexAttribL1i64NV: FnPtr,
         pub(super) VertexAttribL1i64vNV: FnPtr,
         pub(super) VertexAttribL1ui64ARB: FnPtr,
         pub(super) VertexAttribL1ui64NV: FnPtr,
         pub(super) VertexAttribL1ui64vARB: FnPtr,
         pub(super) VertexAttribL1ui64vNV: FnPtr,
         pub(super) VertexAttribL2d: FnPtr,
         pub(super) VertexAttribL2dEXT: FnPtr,
         pub(super) VertexAttribL2dv: FnPtr,
         pub(super) VertexAttribL2dvEXT: FnPtr,
         pub(super) VertexAttribL2i64NV: FnPtr,
         pub(super) VertexAttribL2i64vNV: FnPtr,
         pub(super) VertexAttribL2ui64NV: FnPtr,
         pub(super) VertexAttribL2ui64vNV: FnPtr,
         pub(super) VertexAttribL3d: FnPtr,
         pub(super) VertexAttribL3dEXT: FnPtr,
         pub(super) VertexAttribL3dv: FnPtr,
         pub(super) VertexAttribL3dvEXT: FnPtr,
         pub(super) VertexAttribL3i64NV: FnPtr,
         pub(super) VertexAttribL3i64vNV: FnPtr,
         pub(super) VertexAttribL3ui64NV: FnPtr,
         pub(super) VertexAttribL3ui64vNV: FnPtr,
         pub(super) VertexAttribL4d: FnPtr,
         pub(super) VertexAttribL4dEXT: FnPtr,
         pub(super) VertexAttribL4dv: FnPtr,
         pub(super) VertexAttribL4dvEXT: FnPtr,
         pub(super) VertexAttribL4i64NV: FnPtr,
         pub(super) VertexAttribL4i64vNV: FnPtr,
         pub(super) VertexAttribL4ui64NV: FnPtr,
         pub(super) VertexAttribL4ui64vNV: FnPtr,
         pub(super) VertexAttribLFormat: FnPtr,
         pub(super) VertexAttribLFormatNV: FnPtr,
         pub(super) VertexAttribLPointer: FnPtr,
         pub(super) VertexAttribLPointerEXT: FnPtr,
         pub(super) VertexAttribP1ui: FnPtr,
         pub(super) VertexAttribP1uiv: FnPtr,
         pub(super) VertexAttribP2ui: FnPtr,
         pub(super) VertexAttribP2uiv: FnPtr,
         pub(super) VertexAttribP3ui: FnPtr,
         pub(super) VertexAttribP3uiv: FnPtr,
         pub(super) VertexAttribP4ui: FnPtr,
         pub(super) VertexAttribP4uiv: FnPtr,
         pub(super) VertexAttribParameteriAMD: FnPtr,
         pub(super) VertexAttribPointer: FnPtr,
         pub(super) VertexAttribPointerARB: FnPtr,
         pub(super) VertexAttribPointerNV: FnPtr,
         pub(super) VertexAttribs1dvNV: FnPtr,
         pub(super) VertexAttribs1fvNV: FnPtr,
         pub(super) VertexAttribs1hvNV: FnPtr,
         pub(super) VertexAttribs1svNV: FnPtr,
         pub(super) VertexAttribs2dvNV: FnPtr,
         pub(super) VertexAttribs2fvNV: FnPtr,
         pub(super) VertexAttribs2hvNV: FnPtr,
         pub(super) VertexAttribs2svNV: FnPtr,
         pub(super) VertexAttribs3dvNV: FnPtr,
         pub(super) VertexAttribs3fvNV: FnPtr,
         pub(super) VertexAttribs3hvNV: FnPtr,
         pub(super) VertexAttribs3svNV: FnPtr,
         pub(super) VertexAttribs4dvNV: FnPtr,
         pub(super) VertexAttribs4fvNV: FnPtr,
         pub(super) VertexAttribs4hvNV: FnPtr,
         pub(super) VertexAttribs4svNV: FnPtr,
         pub(super) VertexAttribs4ubvNV: FnPtr,
         pub(super) VertexBindingDivisor: FnPtr,
         pub(super) VertexBlendARB: FnPtr,
         pub(super) VertexBlendEnvfATI: FnPtr,
         pub(super) VertexBlendEnviATI: FnPtr,
         pub(super) VertexFormatNV: FnPtr,
         pub(super) VertexPointerEXT: FnPtr,
         pub(super) VertexPointerListIBM: FnPtr,
         pub(super) VertexPointervINTEL: FnPtr,
         pub(super) VertexStream1dATI: FnPtr,
         pub(super) VertexStream1dvATI: FnPtr,
         pub(super) VertexStream1fATI: FnPtr,
         pub(super) VertexStream1fvATI: FnPtr,
         pub(super) VertexStream1iATI: FnPtr,
         pub(super) VertexStream1ivATI: FnPtr,
         pub(super) VertexStream1sATI: FnPtr,
         pub(super) VertexStream1svATI: FnPtr,
         pub(super) VertexStream2dATI: FnPtr,
         pub(super) VertexStream2dvATI: FnPtr,
         pub(super) VertexStream2fATI: FnPtr,
         pub(super) VertexStream2fvATI: FnPtr,
         pub(super) VertexStream2iATI: FnPtr,
         pub(super) VertexStream2ivATI: FnPtr,
         pub(super) VertexStream2sATI: FnPtr,
         pub(super) VertexStream2svATI: FnPtr,
         pub(super) VertexStream3dATI: FnPtr,
         pub(super) VertexStream3dvATI: FnPtr,
         pub(super) VertexStream3fATI: FnPtr,
         pub(super) VertexStream3fvATI: FnPtr,
         pub(super) VertexStream3iATI: FnPtr,
         pub(super) VertexStream3ivATI: FnPtr,
         pub(super) VertexStream3sATI: FnPtr,
         pub(super) VertexStream3svATI: FnPtr,
         pub(super) VertexStream4dATI: FnPtr,
         pub(super) VertexStream4dvATI: FnPtr,
         pub(super) VertexStream4fATI: FnPtr,
         pub(super) VertexStream4fvATI: FnPtr,
         pub(super) VertexStream4iATI: FnPtr,
         pub(super) VertexStream4ivATI: FnPtr,
         pub(super) VertexStream4sATI: FnPtr,
         pub(super) VertexStream4svATI: FnPtr,
         pub(super) VertexWeightPointerEXT: FnPtr,
         pub(super) VertexWeightfEXT: FnPtr,
         pub(super) VertexWeightfvEXT: FnPtr,
         pub(super) VertexWeighthNV: FnPtr,
         pub(super) VertexWeighthvNV: FnPtr,
         pub(super) VideoCaptureNV: FnPtr,
         pub(super) VideoCaptureStreamParameterdvNV: FnPtr,
         pub(super) VideoCaptureStreamParameterfvNV: FnPtr,
         pub(super) VideoCaptureStreamParameterivNV: FnPtr,
         pub(super) Viewport: FnPtr,
         pub(super) ViewportArrayv: FnPtr,
         pub(super) ViewportIndexedf: FnPtr,
         pub(super) ViewportIndexedfv: FnPtr,
         pub(super) ViewportPositionWScaleNV: FnPtr,
         pub(super) ViewportSwizzleNV: FnPtr,
         pub(super) WaitSemaphoreEXT: FnPtr,
         pub(super) WaitSemaphoreui64NVX: FnPtr,
         pub(super) WaitSync: FnPtr,
         pub(super) WaitVkSemaphoreNV: FnPtr,
         pub(super) WeightPathsNV: FnPtr,
         pub(super) WeightPointerARB: FnPtr,
         pub(super) WeightbvARB: FnPtr,
         pub(super) WeightdvARB: FnPtr,
         pub(super) WeightfvARB: FnPtr,
         pub(super) WeightivARB: FnPtr,
         pub(super) WeightsvARB: FnPtr,
         pub(super) WeightubvARB: FnPtr,
         pub(super) WeightuivARB: FnPtr,
         pub(super) WeightusvARB: FnPtr,
         pub(super) WindowPos2dARB: FnPtr,
         pub(super) WindowPos2dMESA: FnPtr,
         pub(super) WindowPos2dvARB: FnPtr,
         pub(super) WindowPos2dvMESA: FnPtr,
         pub(super) WindowPos2fARB: FnPtr,
         pub(super) WindowPos2fMESA: FnPtr,
         pub(super) WindowPos2fvARB: FnPtr,
         pub(super) WindowPos2fvMESA: FnPtr,
         pub(super) WindowPos2iARB: FnPtr,
         pub(super) WindowPos2iMESA: FnPtr,
         pub(super) WindowPos2ivARB: FnPtr,
         pub(super) WindowPos2ivMESA: FnPtr,
         pub(super) WindowPos2sARB: FnPtr,
         pub(super) WindowPos2sMESA: FnPtr,
         pub(super) WindowPos2svARB: FnPtr,
         pub(super) WindowPos2svMESA: FnPtr,
         pub(super) WindowPos3dARB: FnPtr,
         pub(super) WindowPos3dMESA: FnPtr,
         pub(super) WindowPos3dvARB: FnPtr,
         pub(super) WindowPos3dvMESA: FnPtr,
         pub(super) WindowPos3fARB: FnPtr,
         pub(super) WindowPos3fMESA: FnPtr,
         pub(super) WindowPos3fvARB: FnPtr,
         pub(super) WindowPos3fvMESA: FnPtr,
         pub(super) WindowPos3iARB: FnPtr,
         pub(super) WindowPos3iMESA: FnPtr,
         pub(super) WindowPos3ivARB: FnPtr,
         pub(super) WindowPos3ivMESA: FnPtr,
         pub(super) WindowPos3sARB: FnPtr,
         pub(super) WindowPos3sMESA: FnPtr,
         pub(super) WindowPos3svARB: FnPtr,
         pub(super) WindowPos3svMESA: FnPtr,
         pub(super) WindowPos4dMESA: FnPtr,
         pub(super) WindowPos4dvMESA: FnPtr,
         pub(super) WindowPos4fMESA: FnPtr,
         pub(super) WindowPos4fvMESA: FnPtr,
         pub(super) WindowPos4iMESA: FnPtr,
         pub(super) WindowPos4ivMESA: FnPtr,
         pub(super) WindowPos4sMESA: FnPtr,
         pub(super) WindowPos4svMESA: FnPtr,
         pub(super) WindowRectanglesEXT: FnPtr,
         pub(super) WriteMaskEXT: FnPtr,
    }


    impl Gl {

     func!(AccumxOES, (), op: GLenum, value: GLfixed);
     func!(AcquireKeyedMutexWin32EXT, GLboolean, memory: GLuint, key: GLuint64, timeout: GLuint);
     func!(ActiveProgramEXT, (), program: GLuint);
     func!(ActiveShaderProgram, (), pipeline: GLuint, program: GLuint);
     func!(ActiveStencilFaceEXT, (), face: GLenum);
     func!(ActiveTexture, (), texture: GLenum);
     func!(ActiveTextureARB, (), texture: GLenum);
     func!(ActiveVaryingNV, (), program: GLuint, name: *const GLchar);
     func!(AlphaFragmentOp1ATI, (), op: GLenum, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint);
     func!(AlphaFragmentOp2ATI, (), op: GLenum, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint);
     func!(AlphaFragmentOp3ATI, (), op: GLenum, dst: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint, arg3: GLuint, arg3Rep: GLuint, arg3Mod: GLuint);
     func!(AlphaFuncxOES, (), func: GLenum, ref_: GLfixed);
     func!(AlphaToCoverageDitherControlNV, (), mode: GLenum);
     func!(ApplyFramebufferAttachmentCMAAINTEL, (), );
     func!(ApplyTextureEXT, (), mode: GLenum);
     func!(AreProgramsResidentNV, GLboolean, n: GLsizei, programs: *const GLuint, residences: *mut GLboolean);
     func!(AreTexturesResidentEXT, GLboolean, n: GLsizei, textures: *const GLuint, residences: *mut GLboolean);
     func!(ArrayElementEXT, (), i: GLint);
     func!(ArrayObjectATI, (), array: GLenum, size: GLint, type_: GLenum, stride: GLsizei, buffer: GLuint, offset: GLuint);
     func!(AsyncCopyBufferSubDataNVX, GLuint, waitSemaphoreCount: GLsizei, waitSemaphoreArray: *const GLuint, fenceValueArray: *const GLuint64, readGpu: GLuint, writeGpuMask: GLbitfield, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr, signalSemaphoreCount: GLsizei, signalSemaphoreArray: *const GLuint, signalValueArray: *const GLuint64);
     func!(AsyncCopyImageSubDataNVX, GLuint, waitSemaphoreCount: GLsizei, waitSemaphoreArray: *const GLuint, waitValueArray: *const GLuint64, srcGpu: GLuint, dstGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei, signalSemaphoreCount: GLsizei, signalSemaphoreArray: *const GLuint, signalValueArray: *const GLuint64);
     func!(AsyncMarkerSGIX, (), marker: GLuint);
     func!(AttachObjectARB, (), containerObj: GLhandleARB, obj: GLhandleARB);
     func!(AttachShader, (), program: GLuint, shader: GLuint);
     func!(BeginConditionalRender, (), id: GLuint, mode: GLenum);
     func!(BeginConditionalRenderNV, (), id: GLuint, mode: GLenum);
     func!(BeginConditionalRenderNVX, (), id: GLuint);
     func!(BeginFragmentShaderATI, (), );
     func!(BeginOcclusionQueryNV, (), id: GLuint);
     func!(BeginPerfMonitorAMD, (), monitor: GLuint);
     func!(BeginPerfQueryINTEL, (), queryHandle: GLuint);
     func!(BeginQuery, (), target: GLenum, id: GLuint);
     func!(BeginQueryARB, (), target: GLenum, id: GLuint);
     func!(BeginQueryIndexed, (), target: GLenum, index: GLuint, id: GLuint);
     func!(BeginTransformFeedback, (), primitiveMode: GLenum);
     func!(BeginTransformFeedbackEXT, (), primitiveMode: GLenum);
     func!(BeginTransformFeedbackNV, (), primitiveMode: GLenum);
     func!(BeginVertexShaderEXT, (), );
     func!(BeginVideoCaptureNV, (), video_capture_slot: GLuint);
     func!(BindAttribLocation, (), program: GLuint, index: GLuint, name: *const GLchar);
     func!(BindAttribLocationARB, (), programObj: GLhandleARB, index: GLuint, name: *const GLcharARB);
     func!(BindBuffer, (), target: GLenum, buffer: GLuint);
     func!(BindBufferARB, (), target: GLenum, buffer: GLuint);
     func!(BindBufferBase, (), target: GLenum, index: GLuint, buffer: GLuint);
     func!(BindBufferBaseEXT, (), target: GLenum, index: GLuint, buffer: GLuint);
     func!(BindBufferBaseNV, (), target: GLenum, index: GLuint, buffer: GLuint);
     func!(BindBufferOffsetEXT, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr);
     func!(BindBufferOffsetNV, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr);
     func!(BindBufferRange, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(BindBufferRangeEXT, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(BindBufferRangeNV, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(BindBuffersBase, (), target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint);
     func!(BindBuffersRange, (), target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr);
     func!(BindFragDataLocation, (), program: GLuint, color: GLuint, name: *const GLchar);
     func!(BindFragDataLocationEXT, (), program: GLuint, color: GLuint, name: *const GLchar);
     func!(BindFragDataLocationIndexed, (), program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);
     func!(BindFragmentShaderATI, (), id: GLuint);
     func!(BindFramebuffer, (), target: GLenum, framebuffer: GLuint);
     func!(BindFramebufferEXT, (), target: GLenum, framebuffer: GLuint);
     func!(BindImageTexture, (), unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum);
     func!(BindImageTextureEXT, (), index: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLint);
     func!(BindImageTextures, (), first: GLuint, count: GLsizei, textures: *const GLuint);
     func!(BindLightParameterEXT, GLuint, light: GLenum, value: GLenum);
     func!(BindMaterialParameterEXT, GLuint, face: GLenum, value: GLenum);
     func!(BindMultiTextureEXT, (), texunit: GLenum, target: GLenum, texture: GLuint);
     func!(BindParameterEXT, GLuint, value: GLenum);
     func!(BindProgramARB, (), target: GLenum, program: GLuint);
     func!(BindProgramNV, (), target: GLenum, id: GLuint);
     func!(BindProgramPipeline, (), pipeline: GLuint);
     func!(BindRenderbuffer, (), target: GLenum, renderbuffer: GLuint);
     func!(BindRenderbufferEXT, (), target: GLenum, renderbuffer: GLuint);
     func!(BindSampler, (), unit: GLuint, sampler: GLuint);
     func!(BindSamplers, (), first: GLuint, count: GLsizei, samplers: *const GLuint);
     func!(BindShadingRateImageNV, (), texture: GLuint);
     func!(BindTexGenParameterEXT, GLuint, unit: GLenum, coord: GLenum, value: GLenum);
     func!(BindTexture, (), target: GLenum, texture: GLuint);
     func!(BindTextureEXT, (), target: GLenum, texture: GLuint);
     func!(BindTextureUnit, (), unit: GLuint, texture: GLuint);
     func!(BindTextureUnitParameterEXT, GLuint, unit: GLenum, value: GLenum);
     func!(BindTextures, (), first: GLuint, count: GLsizei, textures: *const GLuint);
     func!(BindTransformFeedback, (), target: GLenum, id: GLuint);
     func!(BindTransformFeedbackNV, (), target: GLenum, id: GLuint);
     func!(BindVertexArray, (), array: GLuint);
     func!(BindVertexArrayAPPLE, (), array: GLuint);
     func!(BindVertexBuffer, (), bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
     func!(BindVertexBuffers, (), first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
     func!(BindVertexShaderEXT, (), id: GLuint);
     func!(BindVideoCaptureStreamBufferNV, (), video_capture_slot: GLuint, stream: GLuint, frame_region: GLenum, offset: GLintptrARB);
     func!(BindVideoCaptureStreamTextureNV, (), video_capture_slot: GLuint, stream: GLuint, frame_region: GLenum, target: GLenum, texture: GLuint);
     func!(Binormal3bEXT, (), bx: GLbyte, by: GLbyte, bz: GLbyte);
     func!(Binormal3bvEXT, (), v: *const GLbyte);
     func!(Binormal3dEXT, (), bx: GLdouble, by: GLdouble, bz: GLdouble);
     func!(Binormal3dvEXT, (), v: *const GLdouble);
     func!(Binormal3fEXT, (), bx: GLfloat, by: GLfloat, bz: GLfloat);
     func!(Binormal3fvEXT, (), v: *const GLfloat);
     func!(Binormal3iEXT, (), bx: GLint, by: GLint, bz: GLint);
     func!(Binormal3ivEXT, (), v: *const GLint);
     func!(Binormal3sEXT, (), bx: GLshort, by: GLshort, bz: GLshort);
     func!(Binormal3svEXT, (), v: *const GLshort);
     func!(BinormalPointerEXT, (), type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(BitmapxOES, (), width: GLsizei, height: GLsizei, xorig: GLfixed, yorig: GLfixed, xmove: GLfixed, ymove: GLfixed, bitmap: *const GLubyte);
     func!(BlendBarrierKHR, (), );
     func!(BlendBarrierNV, (), );
     func!(BlendColor, (), red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
     func!(BlendColorEXT, (), red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
     func!(BlendColorxOES, (), red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
     func!(BlendEquation, (), mode: GLenum);
     func!(BlendEquationEXT, (), mode: GLenum);
     func!(BlendEquationIndexedAMD, (), buf: GLuint, mode: GLenum);
     func!(BlendEquationSeparate, (), modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationSeparateEXT, (), modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationSeparateIndexedAMD, (), buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationSeparatei, (), buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationSeparateiARB, (), buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationi, (), buf: GLuint, mode: GLenum);
     func!(BlendEquationiARB, (), buf: GLuint, mode: GLenum);
     func!(BlendFunc, (), sfactor: GLenum, dfactor: GLenum);
     func!(BlendFuncIndexedAMD, (), buf: GLuint, src: GLenum, dst: GLenum);
     func!(BlendFuncSeparate, (), sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);
     func!(BlendFuncSeparateEXT, (), sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);
     func!(BlendFuncSeparateINGR, (), sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);
     func!(BlendFuncSeparateIndexedAMD, (), buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
     func!(BlendFuncSeparatei, (), buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
     func!(BlendFuncSeparateiARB, (), buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
     func!(BlendFunci, (), buf: GLuint, src: GLenum, dst: GLenum);
     func!(BlendFunciARB, (), buf: GLuint, src: GLenum, dst: GLenum);
     func!(BlendParameteriNV, (), pname: GLenum, value: GLint);
     func!(BlitFramebuffer, (), srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(BlitFramebufferEXT, (), srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(BlitFramebufferLayerEXT, (), srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, srcLayer: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, dstLayer: GLint, mask: GLbitfield, filter: GLenum);
     func!(BlitFramebufferLayersEXT, (), srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(BlitNamedFramebuffer, (), readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(BufferAddressRangeNV, (), pname: GLenum, index: GLuint, address: GLuint64EXT, length: GLsizeiptr);
     func!(BufferAttachMemoryNV, (), target: GLenum, memory: GLuint, offset: GLuint64);
     func!(BufferData, (), target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
     func!(BufferDataARB, (), target: GLenum, size: GLsizeiptrARB, data: *const c_void, usage: GLenum);
     func!(BufferPageCommitmentARB, (), target: GLenum, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);
     func!(BufferPageCommitmentMemNV, (), target: GLenum, offset: GLintptr, size: GLsizeiptr, memory: GLuint, memOffset: GLuint64, commit: GLboolean);
     func!(BufferParameteriAPPLE, (), target: GLenum, pname: GLenum, param: GLint);
     func!(BufferStorage, (), target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
     func!(BufferStorageExternalEXT, (), target: GLenum, offset: GLintptr, size: GLsizeiptr, clientBuffer: GLeglClientBufferEXT, flags: GLbitfield);
     func!(BufferStorageMemEXT, (), target: GLenum, size: GLsizeiptr, memory: GLuint, offset: GLuint64);
     func!(BufferSubData, (), target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
     func!(BufferSubDataARB, (), target: GLenum, offset: GLintptrARB, size: GLsizeiptrARB, data: *const c_void);
     func!(CallCommandListNV, (), list: GLuint);
     func!(CheckFramebufferStatus, GLenum, target: GLenum);
     func!(CheckFramebufferStatusEXT, GLenum, target: GLenum);
     func!(CheckNamedFramebufferStatus, GLenum, framebuffer: GLuint, target: GLenum);
     func!(CheckNamedFramebufferStatusEXT, GLenum, framebuffer: GLuint, target: GLenum);
     func!(ClampColor, (), target: GLenum, clamp: GLenum);
     func!(ClampColorARB, (), target: GLenum, clamp: GLenum);
     func!(Clear, (), mask: GLbitfield);
     func!(ClearAccumxOES, (), red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
     func!(ClearBufferData, (), target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearBufferSubData, (), target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearBufferfi, (), buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
     func!(ClearBufferfv, (), buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
     func!(ClearBufferiv, (), buffer: GLenum, drawbuffer: GLint, value: *const GLint);
     func!(ClearBufferuiv, (), buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
     func!(ClearColor, (), red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
     func!(ClearColorIiEXT, (), red: GLint, green: GLint, blue: GLint, alpha: GLint);
     func!(ClearColorIuiEXT, (), red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);
     func!(ClearColorxOES, (), red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
     func!(ClearDepth, (), depth: GLdouble);
     func!(ClearDepthdNV, (), depth: GLdouble);
     func!(ClearDepthf, (), d: GLfloat);
     func!(ClearDepthfOES, (), depth: GLclampf);
     func!(ClearDepthxOES, (), depth: GLfixed);
     func!(ClearNamedBufferData, (), buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearNamedBufferDataEXT, (), buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearNamedBufferSubData, (), buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearNamedBufferSubDataEXT, (), buffer: GLuint, internalformat: GLenum, offset: GLsizeiptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearNamedFramebufferfi, (), framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
     func!(ClearNamedFramebufferfv, (), framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
     func!(ClearNamedFramebufferiv, (), framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint);
     func!(ClearNamedFramebufferuiv, (), framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
     func!(ClearStencil, (), s: GLint);
     func!(ClearTexImage, (), texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearTexSubImage, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClientActiveTextureARB, (), texture: GLenum);
     func!(ClientActiveVertexStreamATI, (), stream: GLenum);
     func!(ClientAttribDefaultEXT, (), mask: GLbitfield);
     func!(ClientWaitSemaphoreui64NVX, (), fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);
     func!(ClientWaitSync, GLenum, sync: GLsync, flags: GLbitfield, timeout: GLuint64);
     func!(ClipControl, (), origin: GLenum, depth: GLenum);
     func!(ClipPlanefOES, (), plane: GLenum, equation: *const GLfloat);
     func!(ClipPlanexOES, (), plane: GLenum, equation: *const GLfixed);
     func!(Color3fVertex3fSUN, (), r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(Color3fVertex3fvSUN, (), c: *const GLfloat, v: *const GLfloat);
     func!(Color3hNV, (), red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV);
     func!(Color3hvNV, (), v: *const GLhalfNV);
     func!(Color3xOES, (), red: GLfixed, green: GLfixed, blue: GLfixed);
     func!(Color3xvOES, (), components: *const GLfixed);
     func!(Color4fNormal3fVertex3fSUN, (), r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(Color4fNormal3fVertex3fvSUN, (), c: *const GLfloat, n: *const GLfloat, v: *const GLfloat);
     func!(Color4hNV, (), red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV, alpha: GLhalfNV);
     func!(Color4hvNV, (), v: *const GLhalfNV);
     func!(Color4ubVertex2fSUN, (), r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat);
     func!(Color4ubVertex2fvSUN, (), c: *const GLubyte, v: *const GLfloat);
     func!(Color4ubVertex3fSUN, (), r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(Color4ubVertex3fvSUN, (), c: *const GLubyte, v: *const GLfloat);
     func!(Color4xOES, (), red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
     func!(Color4xvOES, (), components: *const GLfixed);
     func!(ColorFormatNV, (), size: GLint, type_: GLenum, stride: GLsizei);
     func!(ColorFragmentOp1ATI, (), op: GLenum, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint);
     func!(ColorFragmentOp2ATI, (), op: GLenum, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint);
     func!(ColorFragmentOp3ATI, (), op: GLenum, dst: GLuint, dstMask: GLuint, dstMod: GLuint, arg1: GLuint, arg1Rep: GLuint, arg1Mod: GLuint, arg2: GLuint, arg2Rep: GLuint, arg2Mod: GLuint, arg3: GLuint, arg3Rep: GLuint, arg3Mod: GLuint);
     func!(ColorMask, (), red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
     func!(ColorMaskIndexedEXT, (), index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
     func!(ColorMaski, (), index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
     func!(ColorPointerEXT, (), size: GLint, type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(ColorPointerListIBM, (), size: GLint, type_: GLenum, stride: GLint, pointer: *const *const c_void, ptrstride: GLint);
     func!(ColorPointervINTEL, (), size: GLint, type_: GLenum, pointer: *const *const c_void);
     func!(ColorSubTableEXT, (), target: GLenum, start: GLsizei, count: GLsizei, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ColorTableEXT, (), target: GLenum, internalFormat: GLenum, width: GLsizei, format: GLenum, type_: GLenum, table: *const c_void);
     func!(ColorTableParameterfvSGI, (), target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(ColorTableParameterivSGI, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(ColorTableSGI, (), target: GLenum, internalformat: GLenum, width: GLsizei, format: GLenum, type_: GLenum, table: *const c_void);
     func!(CombinerInputNV, (), stage: GLenum, portion: GLenum, variable: GLenum, input: GLenum, mapping: GLenum, componentUsage: GLenum);
     func!(CombinerOutputNV, (), stage: GLenum, portion: GLenum, abOutput: GLenum, cdOutput: GLenum, sumOutput: GLenum, scale: GLenum, bias: GLenum, abDotProduct: GLboolean, cdDotProduct: GLboolean, muxSum: GLboolean);
     func!(CombinerParameterfNV, (), pname: GLenum, param: GLfloat);
     func!(CombinerParameterfvNV, (), pname: GLenum, params: *const GLfloat);
     func!(CombinerParameteriNV, (), pname: GLenum, param: GLint);
     func!(CombinerParameterivNV, (), pname: GLenum, params: *const GLint);
     func!(CombinerStageParameterfvNV, (), stage: GLenum, pname: GLenum, params: *const GLfloat);
     func!(CommandListSegmentsNV, (), list: GLuint, segments: GLuint);
     func!(CompileCommandListNV, (), list: GLuint);
     func!(CompileShader, (), shader: GLuint);
     func!(CompileShaderARB, (), shaderObj: GLhandleARB);
     func!(CompileShaderIncludeARB, (), shader: GLuint, count: GLsizei, path: *const *const GLchar, length: *const GLint);
     func!(CompressedMultiTexImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexSubImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexSubImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexSubImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTexImage1D, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage1DARB, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage2D, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage2DARB, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage3D, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage3DARB, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage1D, (), target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage1DARB, (), target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage2D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage2DARB, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage3D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage3DARB, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTextureImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureSubImage1D, (), texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTextureSubImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureSubImage2D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTextureSubImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureSubImage3D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTextureSubImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(ConservativeRasterParameterfNV, (), pname: GLenum, value: GLfloat);
     func!(ConservativeRasterParameteriNV, (), pname: GLenum, param: GLint);
     func!(ConvolutionFilter1DEXT, (), target: GLenum, internalformat: GLenum, width: GLsizei, format: GLenum, type_: GLenum, image: *const c_void);
     func!(ConvolutionFilter2DEXT, (), target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, image: *const c_void);
     func!(ConvolutionParameterfEXT, (), target: GLenum, pname: GLenum, params: GLfloat);
     func!(ConvolutionParameterfvEXT, (), target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(ConvolutionParameteriEXT, (), target: GLenum, pname: GLenum, params: GLint);
     func!(ConvolutionParameterivEXT, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(ConvolutionParameterxOES, (), target: GLenum, pname: GLenum, param: GLfixed);
     func!(ConvolutionParameterxvOES, (), target: GLenum, pname: GLenum, params: *const GLfixed);
     func!(CopyBufferSubData, (), readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
     func!(CopyColorSubTableEXT, (), target: GLenum, start: GLsizei, x: GLint, y: GLint, width: GLsizei);
     func!(CopyColorTableSGI, (), target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei);
     func!(CopyConvolutionFilter1DEXT, (), target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei);
     func!(CopyConvolutionFilter2DEXT, (), target: GLenum, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyImageSubData, (), srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);
     func!(CopyImageSubDataNV, (), srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(CopyMultiTexImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
     func!(CopyMultiTexImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
     func!(CopyMultiTexSubImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyMultiTexSubImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyMultiTexSubImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyNamedBufferSubData, (), readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
     func!(CopyPathNV, (), resultPath: GLuint, srcPath: GLuint);
     func!(CopyTexImage1D, (), target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
     func!(CopyTexImage1DEXT, (), target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
     func!(CopyTexImage2D, (), target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
     func!(CopyTexImage2DEXT, (), target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
     func!(CopyTexSubImage1D, (), target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyTexSubImage1DEXT, (), target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyTexSubImage2D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTexSubImage2DEXT, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTexSubImage3D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTexSubImage3DEXT, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTextureImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
     func!(CopyTextureImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
     func!(CopyTextureSubImage1D, (), texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyTextureSubImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyTextureSubImage2D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTextureSubImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTextureSubImage3D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTextureSubImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CoverFillPathInstancedNV, (), numPaths: GLsizei, pathNameType: GLenum, paths: *const c_void, pathBase: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);
     func!(CoverFillPathNV, (), path: GLuint, coverMode: GLenum);
     func!(CoverStrokePathInstancedNV, (), numPaths: GLsizei, pathNameType: GLenum, paths: *const c_void, pathBase: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);
     func!(CoverStrokePathNV, (), path: GLuint, coverMode: GLenum);
     func!(CoverageModulationNV, (), components: GLenum);
     func!(CoverageModulationTableNV, (), n: GLsizei, v: *const GLfloat);
     func!(CreateBuffers, (), n: GLsizei, buffers: *mut GLuint);
     func!(CreateCommandListsNV, (), n: GLsizei, lists: *mut GLuint);
     func!(CreateFramebuffers, (), n: GLsizei, framebuffers: *mut GLuint);
     func!(CreateMemoryObjectsEXT, (), n: GLsizei, memoryObjects: *mut GLuint);
     func!(CreatePerfQueryINTEL, (), queryId: GLuint, queryHandle: *mut GLuint);
     func!(CreateProgram, GLuint, );
     func!(CreateProgramObjectARB, GLhandleARB, );
     func!(CreateProgramPipelines, (), n: GLsizei, pipelines: *mut GLuint);
     func!(CreateProgressFenceNVX, GLuint, );
     func!(CreateQueries, (), target: GLenum, n: GLsizei, ids: *mut GLuint);
     func!(CreateRenderbuffers, (), n: GLsizei, renderbuffers: *mut GLuint);
     func!(CreateSamplers, (), n: GLsizei, samplers: *mut GLuint);
     func!(CreateSemaphoresNV, (), n: GLsizei, semaphores: *mut GLuint);
     func!(CreateShader, GLuint, type_: GLenum);
     func!(CreateShaderObjectARB, GLhandleARB, shaderType: GLenum);
     func!(CreateShaderProgramEXT, GLuint, type_: GLenum, string: *const GLchar);
     func!(CreateShaderProgramv, GLuint, type_: GLenum, count: GLsizei, strings: *const *const GLchar);
     func!(CreateStatesNV, (), n: GLsizei, states: *mut GLuint);
     func!(CreateSyncFromCLeventARB, GLsync, context: *mut _cl_context, event: *mut _cl_event, flags: GLbitfield);
     func!(CreateTextures, (), target: GLenum, n: GLsizei, textures: *mut GLuint);
     func!(CreateTransformFeedbacks, (), n: GLsizei, ids: *mut GLuint);
     func!(CreateVertexArrays, (), n: GLsizei, arrays: *mut GLuint);
     func!(CullFace, (), mode: GLenum);
     func!(CullParameterdvEXT, (), pname: GLenum, params: *mut GLdouble);
     func!(CullParameterfvEXT, (), pname: GLenum, params: *mut GLfloat);
     func!(CurrentPaletteMatrixARB, (), index: GLint);
     func!(DebugMessageCallback, (), callback: GLDEBUGPROC, userParam: *const c_void);
     func!(DebugMessageCallbackAMD, (), callback: GLDEBUGPROCAMD, userParam: *mut c_void);
     func!(DebugMessageCallbackARB, (), callback: GLDEBUGPROCARB, userParam: *const c_void);
     func!(DebugMessageControl, (), source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);
     func!(DebugMessageControlARB, (), source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);
     func!(DebugMessageEnableAMD, (), category: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);
     func!(DebugMessageInsert, (), source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);
     func!(DebugMessageInsertAMD, (), category: GLenum, severity: GLenum, id: GLuint, length: GLsizei, buf: *const GLchar);
     func!(DebugMessageInsertARB, (), source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);
     func!(DeformSGIX, (), mask: GLbitfield);
     func!(DeformationMap3dSGIX, (), target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, w1: GLdouble, w2: GLdouble, wstride: GLint, worder: GLint, points: *const GLdouble);
     func!(DeformationMap3fSGIX, (), target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, w1: GLfloat, w2: GLfloat, wstride: GLint, worder: GLint, points: *const GLfloat);
     func!(DeleteAsyncMarkersSGIX, (), marker: GLuint, range: GLsizei);
     func!(DeleteBuffers, (), n: GLsizei, buffers: *const GLuint);
     func!(DeleteBuffersARB, (), n: GLsizei, buffers: *const GLuint);
     func!(DeleteCommandListsNV, (), n: GLsizei, lists: *const GLuint);
     func!(DeleteFencesAPPLE, (), n: GLsizei, fences: *const GLuint);
     func!(DeleteFencesNV, (), n: GLsizei, fences: *const GLuint);
     func!(DeleteFragmentShaderATI, (), id: GLuint);
     func!(DeleteFramebuffers, (), n: GLsizei, framebuffers: *const GLuint);
     func!(DeleteFramebuffersEXT, (), n: GLsizei, framebuffers: *const GLuint);
     func!(DeleteMemoryObjectsEXT, (), n: GLsizei, memoryObjects: *const GLuint);
     func!(DeleteNamedStringARB, (), namelen: GLint, name: *const GLchar);
     func!(DeleteNamesAMD, (), identifier: GLenum, num: GLuint, names: *const GLuint);
     func!(DeleteObjectARB, (), obj: GLhandleARB);
     func!(DeleteOcclusionQueriesNV, (), n: GLsizei, ids: *const GLuint);
     func!(DeletePathsNV, (), path: GLuint, range: GLsizei);
     func!(DeletePerfMonitorsAMD, (), n: GLsizei, monitors: *mut GLuint);
     func!(DeletePerfQueryINTEL, (), queryHandle: GLuint);
     func!(DeleteProgram, (), program: GLuint);
     func!(DeleteProgramPipelines, (), n: GLsizei, pipelines: *const GLuint);
     func!(DeleteProgramsARB, (), n: GLsizei, programs: *const GLuint);
     func!(DeleteProgramsNV, (), n: GLsizei, programs: *const GLuint);
     func!(DeleteQueries, (), n: GLsizei, ids: *const GLuint);
     func!(DeleteQueriesARB, (), n: GLsizei, ids: *const GLuint);
     func!(DeleteQueryResourceTagNV, (), n: GLsizei, tagIds: *const GLint);
     func!(DeleteRenderbuffers, (), n: GLsizei, renderbuffers: *const GLuint);
     func!(DeleteRenderbuffersEXT, (), n: GLsizei, renderbuffers: *const GLuint);
     func!(DeleteSamplers, (), count: GLsizei, samplers: *const GLuint);
     func!(DeleteSemaphoresEXT, (), n: GLsizei, semaphores: *const GLuint);
     func!(DeleteShader, (), shader: GLuint);
     func!(DeleteStatesNV, (), n: GLsizei, states: *const GLuint);
     func!(DeleteSync, (), sync: GLsync);
     func!(DeleteTextures, (), n: GLsizei, textures: *const GLuint);
     func!(DeleteTexturesEXT, (), n: GLsizei, textures: *const GLuint);
     func!(DeleteTransformFeedbacks, (), n: GLsizei, ids: *const GLuint);
     func!(DeleteTransformFeedbacksNV, (), n: GLsizei, ids: *const GLuint);
     func!(DeleteVertexArrays, (), n: GLsizei, arrays: *const GLuint);
     func!(DeleteVertexArraysAPPLE, (), n: GLsizei, arrays: *const GLuint);
     func!(DeleteVertexShaderEXT, (), id: GLuint);
     func!(DepthBoundsEXT, (), zmin: GLclampd, zmax: GLclampd);
     func!(DepthBoundsdNV, (), zmin: GLdouble, zmax: GLdouble);
     func!(DepthFunc, (), func: GLenum);
     func!(DepthMask, (), flag: GLboolean);
     func!(DepthRange, (), n: GLdouble, f: GLdouble);
     func!(DepthRangeArraydvNV, (), first: GLuint, count: GLsizei, v: *const GLdouble);
     func!(DepthRangeArrayv, (), first: GLuint, count: GLsizei, v: *const GLdouble);
     func!(DepthRangeIndexed, (), index: GLuint, n: GLdouble, f: GLdouble);
     func!(DepthRangeIndexeddNV, (), index: GLuint, n: GLdouble, f: GLdouble);
     func!(DepthRangedNV, (), zNear: GLdouble, zFar: GLdouble);
     func!(DepthRangef, (), n: GLfloat, f: GLfloat);
     func!(DepthRangefOES, (), n: GLclampf, f: GLclampf);
     func!(DepthRangexOES, (), n: GLfixed, f: GLfixed);
     func!(DetachObjectARB, (), containerObj: GLhandleARB, attachedObj: GLhandleARB);
     func!(DetachShader, (), program: GLuint, shader: GLuint);
     func!(DetailTexFuncSGIS, (), target: GLenum, n: GLsizei, points: *const GLfloat);
     func!(Disable, (), cap: GLenum);
     func!(DisableClientStateIndexedEXT, (), array: GLenum, index: GLuint);
     func!(DisableClientStateiEXT, (), array: GLenum, index: GLuint);
     func!(DisableIndexedEXT, (), target: GLenum, index: GLuint);
     func!(DisableVariantClientStateEXT, (), id: GLuint);
     func!(DisableVertexArrayAttrib, (), vaobj: GLuint, index: GLuint);
     func!(DisableVertexArrayAttribEXT, (), vaobj: GLuint, index: GLuint);
     func!(DisableVertexArrayEXT, (), vaobj: GLuint, array: GLenum);
     func!(DisableVertexAttribAPPLE, (), index: GLuint, pname: GLenum);
     func!(DisableVertexAttribArray, (), index: GLuint);
     func!(DisableVertexAttribArrayARB, (), index: GLuint);
     func!(Disablei, (), target: GLenum, index: GLuint);
     func!(DispatchCompute, (), num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);
     func!(DispatchComputeGroupSizeARB, (), num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint, group_size_x: GLuint, group_size_y: GLuint, group_size_z: GLuint);
     func!(DispatchComputeIndirect, (), indirect: GLintptr);
     func!(DrawArrays, (), mode: GLenum, first: GLint, count: GLsizei);
     func!(DrawArraysEXT, (), mode: GLenum, first: GLint, count: GLsizei);
     func!(DrawArraysIndirect, (), mode: GLenum, indirect: *const c_void);
     func!(DrawArraysInstanced, (), mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
     func!(DrawArraysInstancedARB, (), mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei);
     func!(DrawArraysInstancedBaseInstance, (), mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);
     func!(DrawArraysInstancedEXT, (), mode: GLenum, start: GLint, count: GLsizei, primcount: GLsizei);
     func!(DrawBuffer, (), buf: GLenum);
     func!(DrawBuffers, (), n: GLsizei, bufs: *const GLenum);
     func!(DrawBuffersARB, (), n: GLsizei, bufs: *const GLenum);
     func!(DrawBuffersATI, (), n: GLsizei, bufs: *const GLenum);
     func!(DrawCommandsAddressNV, (), primitiveMode: GLenum, indirects: *const GLuint64, sizes: *const GLsizei, count: GLuint);
     func!(DrawCommandsNV, (), primitiveMode: GLenum, buffer: GLuint, indirects: *const GLintptr, sizes: *const GLsizei, count: GLuint);
     func!(DrawCommandsStatesAddressNV, (), indirects: *const GLuint64, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);
     func!(DrawCommandsStatesNV, (), buffer: GLuint, indirects: *const GLintptr, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);
     func!(DrawElementArrayAPPLE, (), mode: GLenum, first: GLint, count: GLsizei);
     func!(DrawElementArrayATI, (), mode: GLenum, count: GLsizei);
     func!(DrawElements, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void);
     func!(DrawElementsBaseVertex, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint);
     func!(DrawElementsIndirect, (), mode: GLenum, type_: GLenum, indirect: *const c_void);
     func!(DrawElementsInstanced, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei);
     func!(DrawElementsInstancedARB, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, primcount: GLsizei);
     func!(DrawElementsInstancedBaseInstance, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint);
     func!(DrawElementsInstancedBaseVertex, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint);
     func!(DrawElementsInstancedBaseVertexBaseInstance, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);
     func!(DrawElementsInstancedEXT, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, primcount: GLsizei);
     func!(DrawMeshArraysSUN, (), mode: GLenum, first: GLint, count: GLsizei, width: GLsizei);
     func!(DrawMeshTasksIndirectNV, (), indirect: GLintptr);
     func!(DrawMeshTasksNV, (), first: GLuint, count: GLuint);
     func!(DrawRangeElementArrayAPPLE, (), mode: GLenum, start: GLuint, end: GLuint, first: GLint, count: GLsizei);
     func!(DrawRangeElementArrayATI, (), mode: GLenum, start: GLuint, end: GLuint, count: GLsizei);
     func!(DrawRangeElements, (), mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void);
     func!(DrawRangeElementsBaseVertex, (), mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint);
     func!(DrawRangeElementsEXT, (), mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void);
     func!(DrawTextureNV, (), texture: GLuint, sampler: GLuint, x0: GLfloat, y0: GLfloat, x1: GLfloat, y1: GLfloat, z: GLfloat, s0: GLfloat, t0: GLfloat, s1: GLfloat, t1: GLfloat);
     func!(DrawTransformFeedback, (), mode: GLenum, id: GLuint);
     func!(DrawTransformFeedbackInstanced, (), mode: GLenum, id: GLuint, instancecount: GLsizei);
     func!(DrawTransformFeedbackNV, (), mode: GLenum, id: GLuint);
     func!(DrawTransformFeedbackStream, (), mode: GLenum, id: GLuint, stream: GLuint);
     func!(DrawTransformFeedbackStreamInstanced, (), mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei);
     func!(DrawVkImageNV, (), vkImage: GLuint64, sampler: GLuint, x0: GLfloat, y0: GLfloat, x1: GLfloat, y1: GLfloat, z: GLfloat, s0: GLfloat, t0: GLfloat, s1: GLfloat, t1: GLfloat);
     func!(EGLImageTargetTexStorageEXT, (), target: GLenum, image: GLeglImageOES, attrib_list: *const GLint);
     func!(EGLImageTargetTextureStorageEXT, (), texture: GLuint, image: GLeglImageOES, attrib_list: *const GLint);
     func!(EdgeFlagFormatNV, (), stride: GLsizei);
     func!(EdgeFlagPointerEXT, (), stride: GLsizei, count: GLsizei, pointer: *const GLboolean);
     func!(EdgeFlagPointerListIBM, (), stride: GLint, pointer: *const *const GLboolean, ptrstride: GLint);
     func!(ElementPointerAPPLE, (), type_: GLenum, pointer: *const c_void);
     func!(ElementPointerATI, (), type_: GLenum, pointer: *const c_void);
     func!(Enable, (), cap: GLenum);
     func!(EnableClientStateIndexedEXT, (), array: GLenum, index: GLuint);
     func!(EnableClientStateiEXT, (), array: GLenum, index: GLuint);
     func!(EnableIndexedEXT, (), target: GLenum, index: GLuint);
     func!(EnableVariantClientStateEXT, (), id: GLuint);
     func!(EnableVertexArrayAttrib, (), vaobj: GLuint, index: GLuint);
     func!(EnableVertexArrayAttribEXT, (), vaobj: GLuint, index: GLuint);
     func!(EnableVertexArrayEXT, (), vaobj: GLuint, array: GLenum);
     func!(EnableVertexAttribAPPLE, (), index: GLuint, pname: GLenum);
     func!(EnableVertexAttribArray, (), index: GLuint);
     func!(EnableVertexAttribArrayARB, (), index: GLuint);
     func!(Enablei, (), target: GLenum, index: GLuint);
     func!(EndConditionalRender, (), );
     func!(EndConditionalRenderNV, (), );
     func!(EndConditionalRenderNVX, (), );
     func!(EndFragmentShaderATI, (), );
     func!(EndOcclusionQueryNV, (), );
     func!(EndPerfMonitorAMD, (), monitor: GLuint);
     func!(EndPerfQueryINTEL, (), queryHandle: GLuint);
     func!(EndQuery, (), target: GLenum);
     func!(EndQueryARB, (), target: GLenum);
     func!(EndQueryIndexed, (), target: GLenum, index: GLuint);
     func!(EndTransformFeedback, (), );
     func!(EndTransformFeedbackEXT, (), );
     func!(EndTransformFeedbackNV, (), );
     func!(EndVertexShaderEXT, (), );
     func!(EndVideoCaptureNV, (), video_capture_slot: GLuint);
     func!(EvalCoord1xOES, (), u: GLfixed);
     func!(EvalCoord1xvOES, (), coords: *const GLfixed);
     func!(EvalCoord2xOES, (), u: GLfixed, v: GLfixed);
     func!(EvalCoord2xvOES, (), coords: *const GLfixed);
     func!(EvalMapsNV, (), target: GLenum, mode: GLenum);
     func!(EvaluateDepthValuesARB, (), );
     func!(ExecuteProgramNV, (), target: GLenum, id: GLuint, params: *const GLfloat);
     func!(ExtractComponentEXT, (), res: GLuint, src: GLuint, num: GLuint);
     func!(FeedbackBufferxOES, (), n: GLsizei, type_: GLenum, buffer: *const GLfixed);
     func!(FenceSync, GLsync, condition: GLenum, flags: GLbitfield);
     func!(FinalCombinerInputNV, (), variable: GLenum, input: GLenum, mapping: GLenum, componentUsage: GLenum);
     func!(Finish, (), );
     func!(FinishAsyncSGIX, GLint, markerp: *mut GLuint);
     func!(FinishFenceAPPLE, (), fence: GLuint);
     func!(FinishFenceNV, (), fence: GLuint);
     func!(FinishObjectAPPLE, (), object: GLenum, name: GLint);
     func!(FinishTextureSUNX, (), );
     func!(Flush, (), );
     func!(FlushMappedBufferRange, (), target: GLenum, offset: GLintptr, length: GLsizeiptr);
     func!(FlushMappedBufferRangeAPPLE, (), target: GLenum, offset: GLintptr, size: GLsizeiptr);
     func!(FlushMappedNamedBufferRange, (), buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
     func!(FlushMappedNamedBufferRangeEXT, (), buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
     func!(FlushPixelDataRangeNV, (), target: GLenum);
     func!(FlushRasterSGIX, (), );
     func!(FlushStaticDataIBM, (), target: GLenum);
     func!(FlushVertexArrayRangeAPPLE, (), length: GLsizei, pointer: *mut c_void);
     func!(FlushVertexArrayRangeNV, (), );
     func!(FogCoordFormatNV, (), type_: GLenum, stride: GLsizei);
     func!(FogCoordPointerEXT, (), type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(FogCoordPointerListIBM, (), type_: GLenum, stride: GLint, pointer: *const *const c_void, ptrstride: GLint);
     func!(FogCoorddEXT, (), coord: GLdouble);
     func!(FogCoorddvEXT, (), coord: *const GLdouble);
     func!(FogCoordfEXT, (), coord: GLfloat);
     func!(FogCoordfvEXT, (), coord: *const GLfloat);
     func!(FogCoordhNV, (), fog: GLhalfNV);
     func!(FogCoordhvNV, (), fog: *const GLhalfNV);
     func!(FogFuncSGIS, (), n: GLsizei, points: *const GLfloat);
     func!(FogxOES, (), pname: GLenum, param: GLfixed);
     func!(FogxvOES, (), pname: GLenum, param: *const GLfixed);
     func!(FragmentColorMaterialSGIX, (), face: GLenum, mode: GLenum);
     func!(FragmentCoverageColorNV, (), color: GLuint);
     func!(FragmentLightModelfSGIX, (), pname: GLenum, param: GLfloat);
     func!(FragmentLightModelfvSGIX, (), pname: GLenum, params: *const GLfloat);
     func!(FragmentLightModeliSGIX, (), pname: GLenum, param: GLint);
     func!(FragmentLightModelivSGIX, (), pname: GLenum, params: *const GLint);
     func!(FragmentLightfSGIX, (), light: GLenum, pname: GLenum, param: GLfloat);
     func!(FragmentLightfvSGIX, (), light: GLenum, pname: GLenum, params: *const GLfloat);
     func!(FragmentLightiSGIX, (), light: GLenum, pname: GLenum, param: GLint);
     func!(FragmentLightivSGIX, (), light: GLenum, pname: GLenum, params: *const GLint);
     func!(FragmentMaterialfSGIX, (), face: GLenum, pname: GLenum, param: GLfloat);
     func!(FragmentMaterialfvSGIX, (), face: GLenum, pname: GLenum, params: *const GLfloat);
     func!(FragmentMaterialiSGIX, (), face: GLenum, pname: GLenum, param: GLint);
     func!(FragmentMaterialivSGIX, (), face: GLenum, pname: GLenum, params: *const GLint);
     func!(FrameTerminatorGREMEDY, (), );
     func!(FrameZoomSGIX, (), factor: GLint);
     func!(FramebufferDrawBufferEXT, (), framebuffer: GLuint, mode: GLenum);
     func!(FramebufferDrawBuffersEXT, (), framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);
     func!(FramebufferFetchBarrierEXT, (), );
     func!(FramebufferParameteri, (), target: GLenum, pname: GLenum, param: GLint);
     func!(FramebufferParameteriMESA, (), target: GLenum, pname: GLenum, param: GLint);
     func!(FramebufferReadBufferEXT, (), framebuffer: GLuint, mode: GLenum);
     func!(FramebufferRenderbuffer, (), target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
     func!(FramebufferRenderbufferEXT, (), target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
     func!(FramebufferSampleLocationsfvARB, (), target: GLenum, start: GLuint, count: GLsizei, v: *const GLfloat);
     func!(FramebufferSampleLocationsfvNV, (), target: GLenum, start: GLuint, count: GLsizei, v: *const GLfloat);
     func!(FramebufferSamplePositionsfvAMD, (), target: GLenum, numsamples: GLuint, pixelindex: GLuint, values: *const GLfloat);
     func!(FramebufferTexture, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture1D, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture1DEXT, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture2D, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture2DEXT, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture3D, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);
     func!(FramebufferTexture3DEXT, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);
     func!(FramebufferTextureARB, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTextureEXT, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTextureFaceARB, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);
     func!(FramebufferTextureFaceEXT, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);
     func!(FramebufferTextureLayer, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(FramebufferTextureLayerARB, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(FramebufferTextureLayerEXT, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(FramebufferTextureMultiviewOVR, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, baseViewIndex: GLint, numViews: GLsizei);
     func!(FreeObjectBufferATI, (), buffer: GLuint);
     func!(FrontFace, (), mode: GLenum);
     func!(FrustumfOES, (), l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);
     func!(FrustumxOES, (), l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);
     func!(GenAsyncMarkersSGIX, GLuint, range: GLsizei);
     func!(GenBuffers, (), n: GLsizei, buffers: *mut GLuint);
     func!(GenBuffersARB, (), n: GLsizei, buffers: *mut GLuint);
     func!(GenFencesAPPLE, (), n: GLsizei, fences: *mut GLuint);
     func!(GenFencesNV, (), n: GLsizei, fences: *mut GLuint);
     func!(GenFragmentShadersATI, GLuint, range: GLuint);
     func!(GenFramebuffers, (), n: GLsizei, framebuffers: *mut GLuint);
     func!(GenFramebuffersEXT, (), n: GLsizei, framebuffers: *mut GLuint);
     func!(GenNamesAMD, (), identifier: GLenum, num: GLuint, names: *mut GLuint);
     func!(GenOcclusionQueriesNV, (), n: GLsizei, ids: *mut GLuint);
     func!(GenPathsNV, GLuint, range: GLsizei);
     func!(GenPerfMonitorsAMD, (), n: GLsizei, monitors: *mut GLuint);
     func!(GenProgramPipelines, (), n: GLsizei, pipelines: *mut GLuint);
     func!(GenProgramsARB, (), n: GLsizei, programs: *mut GLuint);
     func!(GenProgramsNV, (), n: GLsizei, programs: *mut GLuint);
     func!(GenQueries, (), n: GLsizei, ids: *mut GLuint);
     func!(GenQueriesARB, (), n: GLsizei, ids: *mut GLuint);
     func!(GenQueryResourceTagNV, (), n: GLsizei, tagIds: *mut GLint);
     func!(GenRenderbuffers, (), n: GLsizei, renderbuffers: *mut GLuint);
     func!(GenRenderbuffersEXT, (), n: GLsizei, renderbuffers: *mut GLuint);
     func!(GenSamplers, (), count: GLsizei, samplers: *mut GLuint);
     func!(GenSemaphoresEXT, (), n: GLsizei, semaphores: *mut GLuint);
     func!(GenSymbolsEXT, GLuint, datatype: GLenum, storagetype: GLenum, range: GLenum, components: GLuint);
     func!(GenTextures, (), n: GLsizei, textures: *mut GLuint);
     func!(GenTexturesEXT, (), n: GLsizei, textures: *mut GLuint);
     func!(GenTransformFeedbacks, (), n: GLsizei, ids: *mut GLuint);
     func!(GenTransformFeedbacksNV, (), n: GLsizei, ids: *mut GLuint);
     func!(GenVertexArrays, (), n: GLsizei, arrays: *mut GLuint);
     func!(GenVertexArraysAPPLE, (), n: GLsizei, arrays: *mut GLuint);
     func!(GenVertexShadersEXT, GLuint, range: GLuint);
     func!(GenerateMipmap, (), target: GLenum);
     func!(GenerateMipmapEXT, (), target: GLenum);
     func!(GenerateMultiTexMipmapEXT, (), texunit: GLenum, target: GLenum);
     func!(GenerateTextureMipmap, (), texture: GLuint);
     func!(GenerateTextureMipmapEXT, (), texture: GLuint, target: GLenum);
     func!(GetActiveAtomicCounterBufferiv, (), program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetActiveAttrib, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
     func!(GetActiveAttribARB, (), programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLcharARB);
     func!(GetActiveSubroutineName, (), program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
     func!(GetActiveSubroutineUniformName, (), program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
     func!(GetActiveSubroutineUniformiv, (), program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint);
     func!(GetActiveUniform, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
     func!(GetActiveUniformARB, (), programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLcharARB);
     func!(GetActiveUniformBlockName, (), program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);
     func!(GetActiveUniformBlockiv, (), program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetActiveUniformName, (), program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);
     func!(GetActiveUniformsiv, (), program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint);
     func!(GetActiveVaryingNV, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);
     func!(GetArrayObjectfvATI, (), array: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetArrayObjectivATI, (), array: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetAttachedObjectsARB, (), containerObj: GLhandleARB, maxCount: GLsizei, count: *mut GLsizei, obj: *mut GLhandleARB);
     func!(GetAttachedShaders, (), program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);
     func!(GetAttribLocation, GLint, program: GLuint, name: *const GLchar);
     func!(GetAttribLocationARB, GLint, programObj: GLhandleARB, name: *const GLcharARB);
     func!(GetBooleanIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut GLboolean);
     func!(GetBooleani_v, (), target: GLenum, index: GLuint, data: *mut GLboolean);
     func!(GetBooleanv, (), pname: GLenum, data: *mut GLboolean);
     func!(GetBufferParameteri64v, (), target: GLenum, pname: GLenum, params: *mut GLint64);
     func!(GetBufferParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetBufferParameterivARB, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetBufferParameterui64vNV, (), target: GLenum, pname: GLenum, params: *mut GLuint64EXT);
     func!(GetBufferPointerv, (), target: GLenum, pname: GLenum, params: *mut *mut c_void);
     func!(GetBufferPointervARB, (), target: GLenum, pname: GLenum, params: *mut *mut c_void);
     func!(GetBufferSubData, (), target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
     func!(GetBufferSubDataARB, (), target: GLenum, offset: GLintptrARB, size: GLsizeiptrARB, data: *mut c_void);
     func!(GetClipPlanefOES, (), plane: GLenum, equation: *mut GLfloat);
     func!(GetClipPlanexOES, (), plane: GLenum, equation: *mut GLfixed);
     func!(GetColorTableEXT, (), target: GLenum, format: GLenum, type_: GLenum, data: *mut c_void);
     func!(GetColorTableParameterfvEXT, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetColorTableParameterfvSGI, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetColorTableParameterivEXT, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetColorTableParameterivSGI, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetColorTableSGI, (), target: GLenum, format: GLenum, type_: GLenum, table: *mut c_void);
     func!(GetCombinerInputParameterfvNV, (), stage: GLenum, portion: GLenum, variable: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetCombinerInputParameterivNV, (), stage: GLenum, portion: GLenum, variable: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetCombinerOutputParameterfvNV, (), stage: GLenum, portion: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetCombinerOutputParameterivNV, (), stage: GLenum, portion: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetCombinerStageParameterfvNV, (), stage: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetCommandHeaderNV, GLuint, tokenID: GLenum, size: GLuint);
     func!(GetCompressedMultiTexImageEXT, (), texunit: GLenum, target: GLenum, lod: GLint, img: *mut c_void);
     func!(GetCompressedTexImage, (), target: GLenum, level: GLint, img: *mut c_void);
     func!(GetCompressedTexImageARB, (), target: GLenum, level: GLint, img: *mut c_void);
     func!(GetCompressedTextureImage, (), texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetCompressedTextureImageEXT, (), texture: GLuint, target: GLenum, lod: GLint, img: *mut c_void);
     func!(GetCompressedTextureSubImage, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetConvolutionFilterEXT, (), target: GLenum, format: GLenum, type_: GLenum, image: *mut c_void);
     func!(GetConvolutionParameterfvEXT, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetConvolutionParameterivEXT, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetConvolutionParameterxvOES, (), target: GLenum, pname: GLenum, params: *mut GLfixed);
     func!(GetCoverageModulationTableNV, (), bufSize: GLsizei, v: *mut GLfloat);
     func!(GetDebugMessageLog, GLuint, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar);
     func!(GetDebugMessageLogAMD, GLuint, count: GLuint, bufSize: GLsizei, categories: *mut GLenum, severities: *mut GLenum, ids: *mut GLuint, lengths: *mut GLsizei, message: *mut GLchar);
     func!(GetDebugMessageLogARB, GLuint, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar);
     func!(GetDetailTexFuncSGIS, (), target: GLenum, points: *mut GLfloat);
     func!(GetDoubleIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut GLdouble);
     func!(GetDoublei_v, (), target: GLenum, index: GLuint, data: *mut GLdouble);
     func!(GetDoublei_vEXT, (), pname: GLenum, index: GLuint, params: *mut GLdouble);
     func!(GetDoublev, (), pname: GLenum, data: *mut GLdouble);
     func!(GetError, GLenum, );
     func!(GetFenceivNV, (), fence: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetFinalCombinerInputParameterfvNV, (), variable: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetFinalCombinerInputParameterivNV, (), variable: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetFirstPerfQueryIdINTEL, (), queryId: *mut GLuint);
     func!(GetFixedvOES, (), pname: GLenum, params: *mut GLfixed);
     func!(GetFloatIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut GLfloat);
     func!(GetFloati_v, (), target: GLenum, index: GLuint, data: *mut GLfloat);
     func!(GetFloati_vEXT, (), pname: GLenum, index: GLuint, params: *mut GLfloat);
     func!(GetFloatv, (), pname: GLenum, data: *mut GLfloat);
     func!(GetFogFuncSGIS, (), points: *mut GLfloat);
     func!(GetFragDataIndex, GLint, program: GLuint, name: *const GLchar);
     func!(GetFragDataLocation, GLint, program: GLuint, name: *const GLchar);
     func!(GetFragDataLocationEXT, GLint, program: GLuint, name: *const GLchar);
     func!(GetFragmentLightfvSGIX, (), light: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetFragmentLightivSGIX, (), light: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetFragmentMaterialfvSGIX, (), face: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetFragmentMaterialivSGIX, (), face: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetFramebufferAttachmentParameteriv, (), target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetFramebufferAttachmentParameterivEXT, (), target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetFramebufferParameterfvAMD, (), target: GLenum, pname: GLenum, numsamples: GLuint, pixelindex: GLuint, size: GLsizei, values: *mut GLfloat);
     func!(GetFramebufferParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetFramebufferParameterivEXT, (), framebuffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetFramebufferParameterivMESA, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetGraphicsResetStatus, GLenum, );
     func!(GetGraphicsResetStatusARB, GLenum, );
     func!(GetHandleARB, GLhandleARB, pname: GLenum);
     func!(GetHistogramEXT, (), target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, values: *mut c_void);
     func!(GetHistogramParameterfvEXT, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetHistogramParameterivEXT, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetHistogramParameterxvOES, (), target: GLenum, pname: GLenum, params: *mut GLfixed);
     func!(GetImageHandleARB, GLuint64, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: GLenum);
     func!(GetImageHandleNV, GLuint64, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: GLenum);
     func!(GetImageTransformParameterfvHP, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetImageTransformParameterivHP, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetInfoLogARB, (), obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLcharARB);
     func!(GetInstrumentsSGIX, GLint, );
     func!(GetInteger64i_v, (), target: GLenum, index: GLuint, data: *mut GLint64);
     func!(GetInteger64v, (), pname: GLenum, data: *mut GLint64);
     func!(GetIntegerIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut GLint);
     func!(GetIntegeri_v, (), target: GLenum, index: GLuint, data: *mut GLint);
     func!(GetIntegerui64i_vNV, (), value: GLenum, index: GLuint, result: *mut GLuint64EXT);
     func!(GetIntegerui64vNV, (), value: GLenum, result: *mut GLuint64EXT);
     func!(GetIntegerv, (), pname: GLenum, data: *mut GLint);
     func!(GetInternalformatSampleivNV, (), target: GLenum, internalformat: GLenum, samples: GLsizei, pname: GLenum, count: GLsizei, params: *mut GLint);
     func!(GetInternalformati64v, (), target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint64);
     func!(GetInternalformativ, (), target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint);
     func!(GetInvariantBooleanvEXT, (), id: GLuint, value: GLenum, data: *mut GLboolean);
     func!(GetInvariantFloatvEXT, (), id: GLuint, value: GLenum, data: *mut GLfloat);
     func!(GetInvariantIntegervEXT, (), id: GLuint, value: GLenum, data: *mut GLint);
     func!(GetLightxOES, (), light: GLenum, pname: GLenum, params: *mut GLfixed);
     func!(GetListParameterfvSGIX, (), list: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetListParameterivSGIX, (), list: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetLocalConstantBooleanvEXT, (), id: GLuint, value: GLenum, data: *mut GLboolean);
     func!(GetLocalConstantFloatvEXT, (), id: GLuint, value: GLenum, data: *mut GLfloat);
     func!(GetLocalConstantIntegervEXT, (), id: GLuint, value: GLenum, data: *mut GLint);
     func!(GetMapAttribParameterfvNV, (), target: GLenum, index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetMapAttribParameterivNV, (), target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetMapControlPointsNV, (), target: GLenum, index: GLuint, type_: GLenum, ustride: GLsizei, vstride: GLsizei, packed: GLboolean, points: *mut c_void);
     func!(GetMapParameterfvNV, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetMapParameterivNV, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMapxvOES, (), target: GLenum, query: GLenum, v: *mut GLfixed);
     func!(GetMaterialxOES, (), face: GLenum, pname: GLenum, param: GLfixed);
     func!(GetMemoryObjectDetachedResourcesuivNV, (), memory: GLuint, pname: GLenum, first: GLint, count: GLsizei, params: *mut GLuint);
     func!(GetMemoryObjectParameterivEXT, (), memoryObject: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetMinmaxEXT, (), target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, values: *mut c_void);
     func!(GetMinmaxParameterfvEXT, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetMinmaxParameterivEXT, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMultiTexEnvfvEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetMultiTexEnvivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMultiTexGendvEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *mut GLdouble);
     func!(GetMultiTexGenfvEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetMultiTexGenivEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMultiTexImageEXT, (), texunit: GLenum, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void);
     func!(GetMultiTexLevelParameterfvEXT, (), texunit: GLenum, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
     func!(GetMultiTexLevelParameterivEXT, (), texunit: GLenum, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
     func!(GetMultiTexParameterIivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMultiTexParameterIuivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLuint);
     func!(GetMultiTexParameterfvEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetMultiTexParameterivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMultisamplefv, (), pname: GLenum, index: GLuint, val: *mut GLfloat);
     func!(GetMultisamplefvNV, (), pname: GLenum, index: GLuint, val: *mut GLfloat);
     func!(GetNamedBufferParameteri64v, (), buffer: GLuint, pname: GLenum, params: *mut GLint64);
     func!(GetNamedBufferParameteriv, (), buffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetNamedBufferParameterivEXT, (), buffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetNamedBufferParameterui64vNV, (), buffer: GLuint, pname: GLenum, params: *mut GLuint64EXT);
     func!(GetNamedBufferPointerv, (), buffer: GLuint, pname: GLenum, params: *mut *mut c_void);
     func!(GetNamedBufferPointervEXT, (), buffer: GLuint, pname: GLenum, params: *mut *mut c_void);
     func!(GetNamedBufferSubData, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
     func!(GetNamedBufferSubDataEXT, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
     func!(GetNamedFramebufferAttachmentParameteriv, (), framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetNamedFramebufferAttachmentParameterivEXT, (), framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetNamedFramebufferParameterfvAMD, (), framebuffer: GLuint, pname: GLenum, numsamples: GLuint, pixelindex: GLuint, size: GLsizei, values: *mut GLfloat);
     func!(GetNamedFramebufferParameteriv, (), framebuffer: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetNamedFramebufferParameterivEXT, (), framebuffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetNamedProgramLocalParameterIivEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *mut GLint);
     func!(GetNamedProgramLocalParameterIuivEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *mut GLuint);
     func!(GetNamedProgramLocalParameterdvEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *mut GLdouble);
     func!(GetNamedProgramLocalParameterfvEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *mut GLfloat);
     func!(GetNamedProgramStringEXT, (), program: GLuint, target: GLenum, pname: GLenum, string: *mut c_void);
     func!(GetNamedProgramivEXT, (), program: GLuint, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetNamedRenderbufferParameteriv, (), renderbuffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetNamedRenderbufferParameterivEXT, (), renderbuffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetNamedStringARB, (), namelen: GLint, name: *const GLchar, bufSize: GLsizei, stringlen: *mut GLint, string: *mut GLchar);
     func!(GetNamedStringivARB, (), namelen: GLint, name: *const GLchar, pname: GLenum, params: *mut GLint);
     func!(GetNextPerfQueryIdINTEL, (), queryId: GLuint, nextQueryId: *mut GLuint);
     func!(GetObjectBufferfvATI, (), buffer: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetObjectBufferivATI, (), buffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetObjectLabel, (), identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
     func!(GetObjectLabelEXT, (), type_: GLenum, object: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
     func!(GetObjectParameterfvARB, (), obj: GLhandleARB, pname: GLenum, params: *mut GLfloat);
     func!(GetObjectParameterivAPPLE, (), objectType: GLenum, name: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetObjectParameterivARB, (), obj: GLhandleARB, pname: GLenum, params: *mut GLint);
     func!(GetObjectPtrLabel, (), ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
     func!(GetOcclusionQueryivNV, (), id: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetOcclusionQueryuivNV, (), id: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetPathCommandsNV, (), path: GLuint, commands: *mut GLubyte);
     func!(GetPathCoordsNV, (), path: GLuint, coords: *mut GLfloat);
     func!(GetPathDashArrayNV, (), path: GLuint, dashArray: *mut GLfloat);
     func!(GetPathLengthNV, GLfloat, path: GLuint, startSegment: GLsizei, numSegments: GLsizei);
     func!(GetPathMetricRangeNV, (), metricQueryMask: GLbitfield, firstPathName: GLuint, numPaths: GLsizei, stride: GLsizei, metrics: *mut GLfloat);
     func!(GetPathMetricsNV, (), metricQueryMask: GLbitfield, numPaths: GLsizei, pathNameType: GLenum, paths: *const c_void, pathBase: GLuint, stride: GLsizei, metrics: *mut GLfloat);
     func!(GetPathParameterfvNV, (), path: GLuint, pname: GLenum, value: *mut GLfloat);
     func!(GetPathParameterivNV, (), path: GLuint, pname: GLenum, value: *mut GLint);
     func!(GetPathSpacingNV, (), pathListMode: GLenum, numPaths: GLsizei, pathNameType: GLenum, paths: *const c_void, pathBase: GLuint, advanceScale: GLfloat, kerningScale: GLfloat, transformType: GLenum, returnedSpacing: *mut GLfloat);
     func!(GetPerfCounterInfoINTEL, (), queryId: GLuint, counterId: GLuint, counterNameLength: GLuint, counterName: *mut GLchar, counterDescLength: GLuint, counterDesc: *mut GLchar, counterOffset: *mut GLuint, counterDataSize: *mut GLuint, counterTypeEnum: *mut GLuint, counterDataTypeEnum: *mut GLuint, rawCounterMaxValue: *mut GLuint64);
     func!(GetPerfMonitorCounterDataAMD, (), monitor: GLuint, pname: GLenum, dataSize: GLsizei, data: *mut GLuint, bytesWritten: *mut GLint);
     func!(GetPerfMonitorCounterInfoAMD, (), group: GLuint, counter: GLuint, pname: GLenum, data: *mut c_void);
     func!(GetPerfMonitorCounterStringAMD, (), group: GLuint, counter: GLuint, bufSize: GLsizei, length: *mut GLsizei, counterString: *mut GLchar);
     func!(GetPerfMonitorCountersAMD, (), group: GLuint, numCounters: *mut GLint, maxActiveCounters: *mut GLint, counterSize: GLsizei, counters: *mut GLuint);
     func!(GetPerfMonitorGroupStringAMD, (), group: GLuint, bufSize: GLsizei, length: *mut GLsizei, groupString: *mut GLchar);
     func!(GetPerfMonitorGroupsAMD, (), numGroups: *mut GLint, groupsSize: GLsizei, groups: *mut GLuint);
     func!(GetPerfQueryDataINTEL, (), queryHandle: GLuint, flags: GLuint, dataSize: GLsizei, data: *mut c_void, bytesWritten: *mut GLuint);
     func!(GetPerfQueryIdByNameINTEL, (), queryName: *mut GLchar, queryId: *mut GLuint);
     func!(GetPerfQueryInfoINTEL, (), queryId: GLuint, queryNameLength: GLuint, queryName: *mut GLchar, dataSize: *mut GLuint, noCounters: *mut GLuint, noInstances: *mut GLuint, capsMask: *mut GLuint);
     func!(GetPixelMapxv, (), map: GLenum, size: GLint, values: *mut GLfixed);
     func!(GetPixelTexGenParameterfvSGIS, (), pname: GLenum, params: *mut GLfloat);
     func!(GetPixelTexGenParameterivSGIS, (), pname: GLenum, params: *mut GLint);
     func!(GetPixelTransformParameterfvEXT, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetPixelTransformParameterivEXT, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetPointerIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut *mut c_void);
     func!(GetPointeri_vEXT, (), pname: GLenum, index: GLuint, params: *mut *mut c_void);
     func!(GetPointerv, (), pname: GLenum, params: *mut *mut c_void);
     func!(GetPointervEXT, (), pname: GLenum, params: *mut *mut c_void);
     func!(GetProgramBinary, (), program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void);
     func!(GetProgramEnvParameterIivNV, (), target: GLenum, index: GLuint, params: *mut GLint);
     func!(GetProgramEnvParameterIuivNV, (), target: GLenum, index: GLuint, params: *mut GLuint);
     func!(GetProgramEnvParameterdvARB, (), target: GLenum, index: GLuint, params: *mut GLdouble);
     func!(GetProgramEnvParameterfvARB, (), target: GLenum, index: GLuint, params: *mut GLfloat);
     func!(GetProgramInfoLog, (), program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
     func!(GetProgramInterfaceiv, (), program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetProgramLocalParameterIivNV, (), target: GLenum, index: GLuint, params: *mut GLint);
     func!(GetProgramLocalParameterIuivNV, (), target: GLenum, index: GLuint, params: *mut GLuint);
     func!(GetProgramLocalParameterdvARB, (), target: GLenum, index: GLuint, params: *mut GLdouble);
     func!(GetProgramLocalParameterfvARB, (), target: GLenum, index: GLuint, params: *mut GLfloat);
     func!(GetProgramNamedParameterdvNV, (), id: GLuint, len: GLsizei, name: *const GLubyte, params: *mut GLdouble);
     func!(GetProgramNamedParameterfvNV, (), id: GLuint, len: GLsizei, name: *const GLubyte, params: *mut GLfloat);
     func!(GetProgramParameterdvNV, (), target: GLenum, index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetProgramParameterfvNV, (), target: GLenum, index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetProgramPipelineInfoLog, (), pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
     func!(GetProgramPipelineiv, (), pipeline: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetProgramResourceIndex, GLuint, program: GLuint, programInterface: GLenum, name: *const GLchar);
     func!(GetProgramResourceLocation, GLint, program: GLuint, programInterface: GLenum, name: *const GLchar);
     func!(GetProgramResourceLocationIndex, GLint, program: GLuint, programInterface: GLenum, name: *const GLchar);
     func!(GetProgramResourceName, (), program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
     func!(GetProgramResourcefvNV, (), program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLfloat);
     func!(GetProgramResourceiv, (), program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLint);
     func!(GetProgramStageiv, (), program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint);
     func!(GetProgramStringARB, (), target: GLenum, pname: GLenum, string: *mut c_void);
     func!(GetProgramStringNV, (), id: GLuint, pname: GLenum, program: *mut GLubyte);
     func!(GetProgramSubroutineParameteruivNV, (), target: GLenum, index: GLuint, param: *mut GLuint);
     func!(GetProgramiv, (), program: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetProgramivARB, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetProgramivNV, (), id: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetQueryBufferObjecti64v, (), id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
     func!(GetQueryBufferObjectiv, (), id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
     func!(GetQueryBufferObjectui64v, (), id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
     func!(GetQueryBufferObjectuiv, (), id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
     func!(GetQueryIndexediv, (), target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetQueryObjecti64v, (), id: GLuint, pname: GLenum, params: *mut GLint64);
     func!(GetQueryObjecti64vEXT, (), id: GLuint, pname: GLenum, params: *mut GLint64);
     func!(GetQueryObjectiv, (), id: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetQueryObjectivARB, (), id: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetQueryObjectui64v, (), id: GLuint, pname: GLenum, params: *mut GLuint64);
     func!(GetQueryObjectui64vEXT, (), id: GLuint, pname: GLenum, params: *mut GLuint64);
     func!(GetQueryObjectuiv, (), id: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetQueryObjectuivARB, (), id: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetQueryiv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetQueryivARB, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetRenderbufferParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetRenderbufferParameterivEXT, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetSamplerParameterIiv, (), sampler: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetSamplerParameterIuiv, (), sampler: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetSamplerParameterfv, (), sampler: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetSamplerParameteriv, (), sampler: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetSemaphoreParameterivNV, (), semaphore: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetSemaphoreParameterui64vEXT, (), semaphore: GLuint, pname: GLenum, params: *mut GLuint64);
     func!(GetSeparableFilterEXT, (), target: GLenum, format: GLenum, type_: GLenum, row: *mut c_void, column: *mut c_void, span: *mut c_void);
     func!(GetShaderInfoLog, (), shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
     func!(GetShaderPrecisionFormat, (), shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint);
     func!(GetShaderSource, (), shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);
     func!(GetShaderSourceARB, (), obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, source: *mut GLcharARB);
     func!(GetShaderiv, (), shader: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetShadingRateImagePaletteNV, (), viewport: GLuint, entry: GLuint, rate: *mut GLenum);
     func!(GetShadingRateSampleLocationivNV, (), rate: GLenum, samples: GLuint, index: GLuint, location: *mut GLint);
     func!(GetSharpenTexFuncSGIS, (), target: GLenum, points: *mut GLfloat);
     func!(GetStageIndexNV, GLushort, shadertype: GLenum);
     func!(GetString, *const GLubyte, name: GLenum);
     func!(GetStringi, *const GLubyte, name: GLenum, index: GLuint);
     func!(GetSubroutineIndex, GLuint, program: GLuint, shadertype: GLenum, name: *const GLchar);
     func!(GetSubroutineUniformLocation, GLint, program: GLuint, shadertype: GLenum, name: *const GLchar);
     func!(GetSynciv, (), sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);
     func!(GetTexBumpParameterfvATI, (), pname: GLenum, param: *mut GLfloat);
     func!(GetTexBumpParameterivATI, (), pname: GLenum, param: *mut GLint);
     func!(GetTexEnvxvOES, (), target: GLenum, pname: GLenum, params: *mut GLfixed);
     func!(GetTexFilterFuncSGIS, (), target: GLenum, filter: GLenum, weights: *mut GLfloat);
     func!(GetTexGenxvOES, (), coord: GLenum, pname: GLenum, params: *mut GLfixed);
     func!(GetTexImage, (), target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void);
     func!(GetTexLevelParameterfv, (), target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
     func!(GetTexLevelParameteriv, (), target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
     func!(GetTexLevelParameterxvOES, (), target: GLenum, level: GLint, pname: GLenum, params: *mut GLfixed);
     func!(GetTexParameterIiv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTexParameterIivEXT, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTexParameterIuiv, (), target: GLenum, pname: GLenum, params: *mut GLuint);
     func!(GetTexParameterIuivEXT, (), target: GLenum, pname: GLenum, params: *mut GLuint);
     func!(GetTexParameterPointervAPPLE, (), target: GLenum, pname: GLenum, params: *mut *mut c_void);
     func!(GetTexParameterfv, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetTexParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTexParameterxvOES, (), target: GLenum, pname: GLenum, params: *mut GLfixed);
     func!(GetTextureHandleARB, GLuint64, texture: GLuint);
     func!(GetTextureHandleNV, GLuint64, texture: GLuint);
     func!(GetTextureImage, (), texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetTextureImageEXT, (), texture: GLuint, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void);
     func!(GetTextureLevelParameterfv, (), texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat);
     func!(GetTextureLevelParameterfvEXT, (), texture: GLuint, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
     func!(GetTextureLevelParameteriv, (), texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint);
     func!(GetTextureLevelParameterivEXT, (), texture: GLuint, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
     func!(GetTextureParameterIiv, (), texture: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetTextureParameterIivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTextureParameterIuiv, (), texture: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetTextureParameterIuivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLuint);
     func!(GetTextureParameterfv, (), texture: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetTextureParameterfvEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetTextureParameteriv, (), texture: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetTextureParameterivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTextureSamplerHandleARB, GLuint64, texture: GLuint, sampler: GLuint);
     func!(GetTextureSamplerHandleNV, GLuint64, texture: GLuint, sampler: GLuint);
     func!(GetTextureSubImage, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetTrackMatrixivNV, (), target: GLenum, address: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetTransformFeedbackVarying, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);
     func!(GetTransformFeedbackVaryingEXT, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);
     func!(GetTransformFeedbackVaryingNV, (), program: GLuint, index: GLuint, location: *mut GLint);
     func!(GetTransformFeedbacki64_v, (), xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64);
     func!(GetTransformFeedbacki_v, (), xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint);
     func!(GetTransformFeedbackiv, (), xfb: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetUniformBlockIndex, GLuint, program: GLuint, uniformBlockName: *const GLchar);
     func!(GetUniformBufferSizeEXT, GLint, program: GLuint, location: GLint);
     func!(GetUniformIndices, (), program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);
     func!(GetUniformLocation, GLint, program: GLuint, name: *const GLchar);
     func!(GetUniformLocationARB, GLint, programObj: GLhandleARB, name: *const GLcharARB);
     func!(GetUniformOffsetEXT, GLintptr, program: GLuint, location: GLint);
     func!(GetUniformSubroutineuiv, (), shadertype: GLenum, location: GLint, params: *mut GLuint);
     func!(GetUniformdv, (), program: GLuint, location: GLint, params: *mut GLdouble);
     func!(GetUniformfv, (), program: GLuint, location: GLint, params: *mut GLfloat);
     func!(GetUniformfvARB, (), programObj: GLhandleARB, location: GLint, params: *mut GLfloat);
     func!(GetUniformi64vARB, (), program: GLuint, location: GLint, params: *mut GLint64);
     func!(GetUniformi64vNV, (), program: GLuint, location: GLint, params: *mut GLint64EXT);
     func!(GetUniformiv, (), program: GLuint, location: GLint, params: *mut GLint);
     func!(GetUniformivARB, (), programObj: GLhandleARB, location: GLint, params: *mut GLint);
     func!(GetUniformui64vARB, (), program: GLuint, location: GLint, params: *mut GLuint64);
     func!(GetUniformui64vNV, (), program: GLuint, location: GLint, params: *mut GLuint64EXT);
     func!(GetUniformuiv, (), program: GLuint, location: GLint, params: *mut GLuint);
     func!(GetUniformuivEXT, (), program: GLuint, location: GLint, params: *mut GLuint);
     func!(GetUnsignedBytei_vEXT, (), target: GLenum, index: GLuint, data: *mut GLubyte);
     func!(GetUnsignedBytevEXT, (), pname: GLenum, data: *mut GLubyte);
     func!(GetVariantArrayObjectfvATI, (), id: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetVariantArrayObjectivATI, (), id: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVariantBooleanvEXT, (), id: GLuint, value: GLenum, data: *mut GLboolean);
     func!(GetVariantFloatvEXT, (), id: GLuint, value: GLenum, data: *mut GLfloat);
     func!(GetVariantIntegervEXT, (), id: GLuint, value: GLenum, data: *mut GLint);
     func!(GetVariantPointervEXT, (), id: GLuint, value: GLenum, data: *mut *mut c_void);
     func!(GetVaryingLocationNV, GLint, program: GLuint, name: *const GLchar);
     func!(GetVertexArrayIndexed64iv, (), vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64);
     func!(GetVertexArrayIndexediv, (), vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetVertexArrayIntegeri_vEXT, (), vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetVertexArrayIntegervEXT, (), vaobj: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetVertexArrayPointeri_vEXT, (), vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut *mut c_void);
     func!(GetVertexArrayPointervEXT, (), vaobj: GLuint, pname: GLenum, param: *mut *mut c_void);
     func!(GetVertexArrayiv, (), vaobj: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetVertexAttribArrayObjectfvATI, (), index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetVertexAttribArrayObjectivATI, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVertexAttribIiv, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVertexAttribIivEXT, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVertexAttribIuiv, (), index: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetVertexAttribIuivEXT, (), index: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetVertexAttribLdv, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribLdvEXT, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribLi64vNV, (), index: GLuint, pname: GLenum, params: *mut GLint64EXT);
     func!(GetVertexAttribLui64vARB, (), index: GLuint, pname: GLenum, params: *mut GLuint64EXT);
     func!(GetVertexAttribLui64vNV, (), index: GLuint, pname: GLenum, params: *mut GLuint64EXT);
     func!(GetVertexAttribPointerv, (), index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
     func!(GetVertexAttribPointervARB, (), index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
     func!(GetVertexAttribPointervNV, (), index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
     func!(GetVertexAttribdv, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribdvARB, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribdvNV, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribfv, (), index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetVertexAttribfvARB, (), index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetVertexAttribfvNV, (), index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetVertexAttribiv, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVertexAttribivARB, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVertexAttribivNV, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVideoCaptureStreamdvNV, (), video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVideoCaptureStreamfvNV, (), video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetVideoCaptureStreamivNV, (), video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVideoCaptureivNV, (), video_capture_slot: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVideoi64vNV, (), video_slot: GLuint, pname: GLenum, params: *mut GLint64EXT);
     func!(GetVideoivNV, (), video_slot: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVideoui64vNV, (), video_slot: GLuint, pname: GLenum, params: *mut GLuint64EXT);
     func!(GetVideouivNV, (), video_slot: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetVkProcAddrNV, GLVULKANPROCNV, name: *const GLchar);
     func!(GetnCompressedTexImage, (), target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetnCompressedTexImageARB, (), target: GLenum, lod: GLint, bufSize: GLsizei, img: *mut c_void);
     func!(GetnTexImage, (), target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetnTexImageARB, (), target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, img: *mut c_void);
     func!(GetnUniformdv, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);
     func!(GetnUniformdvARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);
     func!(GetnUniformfv, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);
     func!(GetnUniformfvARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);
     func!(GetnUniformi64vARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint64);
     func!(GetnUniformiv, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);
     func!(GetnUniformivARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);
     func!(GetnUniformui64vARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint64);
     func!(GetnUniformuiv, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);
     func!(GetnUniformuivARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);
     func!(GlobalAlphaFactorbSUN, (), factor: GLbyte);
     func!(GlobalAlphaFactordSUN, (), factor: GLdouble);
     func!(GlobalAlphaFactorfSUN, (), factor: GLfloat);
     func!(GlobalAlphaFactoriSUN, (), factor: GLint);
     func!(GlobalAlphaFactorsSUN, (), factor: GLshort);
     func!(GlobalAlphaFactorubSUN, (), factor: GLubyte);
     func!(GlobalAlphaFactoruiSUN, (), factor: GLuint);
     func!(GlobalAlphaFactorusSUN, (), factor: GLushort);
     func!(Hint, (), target: GLenum, mode: GLenum);
     func!(HintPGI, (), target: GLenum, mode: GLint);
     func!(HistogramEXT, (), target: GLenum, width: GLsizei, internalformat: GLenum, sink: GLboolean);
     func!(IglooInterfaceSGIX, (), pname: GLenum, params: *const c_void);
     func!(ImageTransformParameterfHP, (), target: GLenum, pname: GLenum, param: GLfloat);
     func!(ImageTransformParameterfvHP, (), target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(ImageTransformParameteriHP, (), target: GLenum, pname: GLenum, param: GLint);
     func!(ImageTransformParameterivHP, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(ImportMemoryFdEXT, (), memory: GLuint, size: GLuint64, handleType: GLenum, fd: GLint);
     func!(ImportMemoryWin32HandleEXT, (), memory: GLuint, size: GLuint64, handleType: GLenum, handle: *mut c_void);
     func!(ImportMemoryWin32NameEXT, (), memory: GLuint, size: GLuint64, handleType: GLenum, name: *const c_void);
     func!(ImportSemaphoreFdEXT, (), semaphore: GLuint, handleType: GLenum, fd: GLint);
     func!(ImportSemaphoreWin32HandleEXT, (), semaphore: GLuint, handleType: GLenum, handle: *mut c_void);
     func!(ImportSemaphoreWin32NameEXT, (), semaphore: GLuint, handleType: GLenum, name: *const c_void);
     func!(ImportSyncEXT, GLsync, external_sync_type: GLenum, external_sync: GLintptr, flags: GLbitfield);
     func!(IndexFormatNV, (), type_: GLenum, stride: GLsizei);
     func!(IndexFuncEXT, (), func: GLenum, ref_: GLclampf);
     func!(IndexMaterialEXT, (), face: GLenum, mode: GLenum);
     func!(IndexPointerEXT, (), type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(IndexPointerListIBM, (), type_: GLenum, stride: GLint, pointer: *const *const c_void, ptrstride: GLint);
     func!(IndexxOES, (), component: GLfixed);
     func!(IndexxvOES, (), component: *const GLfixed);
     func!(InsertComponentEXT, (), res: GLuint, src: GLuint, num: GLuint);
     func!(InsertEventMarkerEXT, (), length: GLsizei, marker: *const GLchar);
     func!(InstrumentsBufferSGIX, (), size: GLsizei, buffer: *mut GLint);
     func!(InterpolatePathsNV, (), resultPath: GLuint, pathA: GLuint, pathB: GLuint, weight: GLfloat);
     func!(InvalidateBufferData, (), buffer: GLuint);
     func!(InvalidateBufferSubData, (), buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
     func!(InvalidateFramebuffer, (), target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);
     func!(InvalidateNamedFramebufferData, (), framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum);
     func!(InvalidateNamedFramebufferSubData, (), framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(InvalidateSubFramebuffer, (), target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(InvalidateTexImage, (), texture: GLuint, level: GLint);
     func!(InvalidateTexSubImage, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(IsAsyncMarkerSGIX, GLboolean, marker: GLuint);
     func!(IsBuffer, GLboolean, buffer: GLuint);
     func!(IsBufferARB, GLboolean, buffer: GLuint);
     func!(IsBufferResidentNV, GLboolean, target: GLenum);
     func!(IsCommandListNV, GLboolean, list: GLuint);
     func!(IsEnabled, GLboolean, cap: GLenum);
     func!(IsEnabledIndexedEXT, GLboolean, target: GLenum, index: GLuint);
     func!(IsEnabledi, GLboolean, target: GLenum, index: GLuint);
     func!(IsFenceAPPLE, GLboolean, fence: GLuint);
     func!(IsFenceNV, GLboolean, fence: GLuint);
     func!(IsFramebuffer, GLboolean, framebuffer: GLuint);
     func!(IsFramebufferEXT, GLboolean, framebuffer: GLuint);
     func!(IsImageHandleResidentARB, GLboolean, handle: GLuint64);
     func!(IsImageHandleResidentNV, GLboolean, handle: GLuint64);
     func!(IsMemoryObjectEXT, GLboolean, memoryObject: GLuint);
     func!(IsNameAMD, GLboolean, identifier: GLenum, name: GLuint);
     func!(IsNamedBufferResidentNV, GLboolean, buffer: GLuint);
     func!(IsNamedStringARB, GLboolean, namelen: GLint, name: *const GLchar);
     func!(IsObjectBufferATI, GLboolean, buffer: GLuint);
     func!(IsOcclusionQueryNV, GLboolean, id: GLuint);
     func!(IsPathNV, GLboolean, path: GLuint);
     func!(IsPointInFillPathNV, GLboolean, path: GLuint, mask: GLuint, x: GLfloat, y: GLfloat);
     func!(IsPointInStrokePathNV, GLboolean, path: GLuint, x: GLfloat, y: GLfloat);
     func!(IsProgram, GLboolean, program: GLuint);
     func!(IsProgramARB, GLboolean, program: GLuint);
     func!(IsProgramNV, GLboolean, id: GLuint);
     func!(IsProgramPipeline, GLboolean, pipeline: GLuint);
     func!(IsQuery, GLboolean, id: GLuint);
     func!(IsQueryARB, GLboolean, id: GLuint);
     func!(IsRenderbuffer, GLboolean, renderbuffer: GLuint);
     func!(IsRenderbufferEXT, GLboolean, renderbuffer: GLuint);
     func!(IsSampler, GLboolean, sampler: GLuint);
     func!(IsSemaphoreEXT, GLboolean, semaphore: GLuint);
     func!(IsShader, GLboolean, shader: GLuint);
     func!(IsStateNV, GLboolean, state: GLuint);
     func!(IsSync, GLboolean, sync: GLsync);
     func!(IsTexture, GLboolean, texture: GLuint);
     func!(IsTextureEXT, GLboolean, texture: GLuint);
     func!(IsTextureHandleResidentARB, GLboolean, handle: GLuint64);
     func!(IsTextureHandleResidentNV, GLboolean, handle: GLuint64);
     func!(IsTransformFeedback, GLboolean, id: GLuint);
     func!(IsTransformFeedbackNV, GLboolean, id: GLuint);
     func!(IsVariantEnabledEXT, GLboolean, id: GLuint, cap: GLenum);
     func!(IsVertexArray, GLboolean, array: GLuint);
     func!(IsVertexArrayAPPLE, GLboolean, array: GLuint);
     func!(IsVertexAttribEnabledAPPLE, GLboolean, index: GLuint, pname: GLenum);
     func!(LGPUCopyImageSubDataNVX, (), sourceGpu: GLuint, destinationGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srxY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(LGPUInterlockNVX, (), );
     func!(LGPUNamedBufferSubDataNVX, (), gpuMask: GLbitfield, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
     func!(LabelObjectEXT, (), type_: GLenum, object: GLuint, length: GLsizei, label: *const GLchar);
     func!(LightEnviSGIX, (), pname: GLenum, param: GLint);
     func!(LightModelxOES, (), pname: GLenum, param: GLfixed);
     func!(LightModelxvOES, (), pname: GLenum, param: *const GLfixed);
     func!(LightxOES, (), light: GLenum, pname: GLenum, param: GLfixed);
     func!(LightxvOES, (), light: GLenum, pname: GLenum, params: *const GLfixed);
     func!(LineWidth, (), width: GLfloat);
     func!(LineWidthxOES, (), width: GLfixed);
     func!(LinkProgram, (), program: GLuint);
     func!(LinkProgramARB, (), programObj: GLhandleARB);
     func!(ListDrawCommandsStatesClientNV, (), list: GLuint, segment: GLuint, indirects: *const *const c_void, sizes: *const GLsizei, states: *const GLuint, fbos: *const GLuint, count: GLuint);
     func!(ListParameterfSGIX, (), list: GLuint, pname: GLenum, param: GLfloat);
     func!(ListParameterfvSGIX, (), list: GLuint, pname: GLenum, params: *const GLfloat);
     func!(ListParameteriSGIX, (), list: GLuint, pname: GLenum, param: GLint);
     func!(ListParameterivSGIX, (), list: GLuint, pname: GLenum, params: *const GLint);
     func!(LoadIdentityDeformationMapSGIX, (), mask: GLbitfield);
     func!(LoadMatrixxOES, (), m: *const GLfixed);
     func!(LoadProgramNV, (), target: GLenum, id: GLuint, len: GLsizei, program: *const GLubyte);
     func!(LoadTransposeMatrixdARB, (), m: *const GLdouble);
     func!(LoadTransposeMatrixfARB, (), m: *const GLfloat);
     func!(LoadTransposeMatrixxOES, (), m: *const GLfixed);
     func!(LockArraysEXT, (), first: GLint, count: GLsizei);
     func!(LogicOp, (), opcode: GLenum);
     func!(MakeBufferNonResidentNV, (), target: GLenum);
     func!(MakeBufferResidentNV, (), target: GLenum, access: GLenum);
     func!(MakeImageHandleNonResidentARB, (), handle: GLuint64);
     func!(MakeImageHandleNonResidentNV, (), handle: GLuint64);
     func!(MakeImageHandleResidentARB, (), handle: GLuint64, access: GLenum);
     func!(MakeImageHandleResidentNV, (), handle: GLuint64, access: GLenum);
     func!(MakeNamedBufferNonResidentNV, (), buffer: GLuint);
     func!(MakeNamedBufferResidentNV, (), buffer: GLuint, access: GLenum);
     func!(MakeTextureHandleNonResidentARB, (), handle: GLuint64);
     func!(MakeTextureHandleNonResidentNV, (), handle: GLuint64);
     func!(MakeTextureHandleResidentARB, (), handle: GLuint64);
     func!(MakeTextureHandleResidentNV, (), handle: GLuint64);
     func!(Map1xOES, (), target: GLenum, u1: GLfixed, u2: GLfixed, stride: GLint, order: GLint, points: GLfixed);
     func!(Map2xOES, (), target: GLenum, u1: GLfixed, u2: GLfixed, ustride: GLint, uorder: GLint, v1: GLfixed, v2: GLfixed, vstride: GLint, vorder: GLint, points: GLfixed);
     func!(MapBuffer, *mut c_void, target: GLenum, access: GLenum);
     func!(MapBufferARB, *mut c_void, target: GLenum, access: GLenum);
     func!(MapBufferRange, *mut c_void, target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield);
     func!(MapControlPointsNV, (), target: GLenum, index: GLuint, type_: GLenum, ustride: GLsizei, vstride: GLsizei, uorder: GLint, vorder: GLint, packed: GLboolean, points: *const c_void);
     func!(MapGrid1xOES, (), n: GLint, u1: GLfixed, u2: GLfixed);
     func!(MapGrid2xOES, (), n: GLint, u1: GLfixed, u2: GLfixed, v1: GLfixed, v2: GLfixed);
     func!(MapNamedBuffer, *mut c_void, buffer: GLuint, access: GLenum);
     func!(MapNamedBufferEXT, *mut c_void, buffer: GLuint, access: GLenum);
     func!(MapNamedBufferRange, *mut c_void, buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield);
     func!(MapNamedBufferRangeEXT, *mut c_void, buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield);
     func!(MapObjectBufferATI, *mut c_void, buffer: GLuint);
     func!(MapParameterfvNV, (), target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(MapParameterivNV, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(MapTexture2DINTEL, *mut c_void, texture: GLuint, level: GLint, access: GLbitfield, stride: *mut GLint, layout: *mut GLenum);
     func!(MapVertexAttrib1dAPPLE, (), index: GLuint, size: GLuint, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble);
     func!(MapVertexAttrib1fAPPLE, (), index: GLuint, size: GLuint, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat);
     func!(MapVertexAttrib2dAPPLE, (), index: GLuint, size: GLuint, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble);
     func!(MapVertexAttrib2fAPPLE, (), index: GLuint, size: GLuint, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat);
     func!(MaterialxOES, (), face: GLenum, pname: GLenum, param: GLfixed);
     func!(MaterialxvOES, (), face: GLenum, pname: GLenum, param: *const GLfixed);
     func!(MatrixFrustumEXT, (), mode: GLenum, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);
     func!(MatrixIndexPointerARB, (), size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(MatrixIndexubvARB, (), size: GLint, indices: *const GLubyte);
     func!(MatrixIndexuivARB, (), size: GLint, indices: *const GLuint);
     func!(MatrixIndexusvARB, (), size: GLint, indices: *const GLushort);
     func!(MatrixLoad3x2fNV, (), matrixMode: GLenum, m: *const GLfloat);
     func!(MatrixLoad3x3fNV, (), matrixMode: GLenum, m: *const GLfloat);
     func!(MatrixLoadIdentityEXT, (), mode: GLenum);
     func!(MatrixLoadTranspose3x3fNV, (), matrixMode: GLenum, m: *const GLfloat);
     func!(MatrixLoadTransposedEXT, (), mode: GLenum, m: *const GLdouble);
     func!(MatrixLoadTransposefEXT, (), mode: GLenum, m: *const GLfloat);
     func!(MatrixLoaddEXT, (), mode: GLenum, m: *const GLdouble);
     func!(MatrixLoadfEXT, (), mode: GLenum, m: *const GLfloat);
     func!(MatrixMult3x2fNV, (), matrixMode: GLenum, m: *const GLfloat);
     func!(MatrixMult3x3fNV, (), matrixMode: GLenum, m: *const GLfloat);
     func!(MatrixMultTranspose3x3fNV, (), matrixMode: GLenum, m: *const GLfloat);
     func!(MatrixMultTransposedEXT, (), mode: GLenum, m: *const GLdouble);
     func!(MatrixMultTransposefEXT, (), mode: GLenum, m: *const GLfloat);
     func!(MatrixMultdEXT, (), mode: GLenum, m: *const GLdouble);
     func!(MatrixMultfEXT, (), mode: GLenum, m: *const GLfloat);
     func!(MatrixOrthoEXT, (), mode: GLenum, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);
     func!(MatrixPopEXT, (), mode: GLenum);
     func!(MatrixPushEXT, (), mode: GLenum);
     func!(MatrixRotatedEXT, (), mode: GLenum, angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(MatrixRotatefEXT, (), mode: GLenum, angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(MatrixScaledEXT, (), mode: GLenum, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(MatrixScalefEXT, (), mode: GLenum, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(MatrixTranslatedEXT, (), mode: GLenum, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(MatrixTranslatefEXT, (), mode: GLenum, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(MaxShaderCompilerThreadsARB, (), count: GLuint);
     func!(MaxShaderCompilerThreadsKHR, (), count: GLuint);
     func!(MemoryBarrier, (), barriers: GLbitfield);
     func!(MemoryBarrierByRegion, (), barriers: GLbitfield);
     func!(MemoryBarrierEXT, (), barriers: GLbitfield);
     func!(MemoryObjectParameterivEXT, (), memoryObject: GLuint, pname: GLenum, params: *const GLint);
     func!(MinSampleShading, (), value: GLfloat);
     func!(MinSampleShadingARB, (), value: GLfloat);
     func!(MinmaxEXT, (), target: GLenum, internalformat: GLenum, sink: GLboolean);
     func!(MultMatrixxOES, (), m: *const GLfixed);
     func!(MultTransposeMatrixdARB, (), m: *const GLdouble);
     func!(MultTransposeMatrixfARB, (), m: *const GLfloat);
     func!(MultTransposeMatrixxOES, (), m: *const GLfixed);
     func!(MultiDrawArrays, (), mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);
     func!(MultiDrawArraysEXT, (), mode: GLenum, first: *const GLint, count: *const GLsizei, primcount: GLsizei);
     func!(MultiDrawArraysIndirect, (), mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawArraysIndirectAMD, (), mode: GLenum, indirect: *const c_void, primcount: GLsizei, stride: GLsizei);
     func!(MultiDrawArraysIndirectBindlessCountNV, (), mode: GLenum, indirect: *const c_void, drawCount: GLsizei, maxDrawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);
     func!(MultiDrawArraysIndirectBindlessNV, (), mode: GLenum, indirect: *const c_void, drawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);
     func!(MultiDrawArraysIndirectCount, (), mode: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawArraysIndirectCountARB, (), mode: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawElementArrayAPPLE, (), mode: GLenum, first: *const GLint, count: *const GLsizei, primcount: GLsizei);
     func!(MultiDrawElements, (), mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei);
     func!(MultiDrawElementsBaseVertex, (), mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint);
     func!(MultiDrawElementsEXT, (), mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, primcount: GLsizei);
     func!(MultiDrawElementsIndirect, (), mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawElementsIndirectAMD, (), mode: GLenum, type_: GLenum, indirect: *const c_void, primcount: GLsizei, stride: GLsizei);
     func!(MultiDrawElementsIndirectBindlessCountNV, (), mode: GLenum, type_: GLenum, indirect: *const c_void, drawCount: GLsizei, maxDrawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);
     func!(MultiDrawElementsIndirectBindlessNV, (), mode: GLenum, type_: GLenum, indirect: *const c_void, drawCount: GLsizei, stride: GLsizei, vertexBufferCount: GLint);
     func!(MultiDrawElementsIndirectCount, (), mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawElementsIndirectCountARB, (), mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawMeshTasksIndirectCountNV, (), indirect: GLintptr, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawMeshTasksIndirectNV, (), indirect: GLintptr, drawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawRangeElementArrayAPPLE, (), mode: GLenum, start: GLuint, end: GLuint, first: *const GLint, count: *const GLsizei, primcount: GLsizei);
     func!(MultiModeDrawArraysIBM, (), mode: *const GLenum, first: *const GLint, count: *const GLsizei, primcount: GLsizei, modestride: GLint);
     func!(MultiModeDrawElementsIBM, (), mode: *const GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, primcount: GLsizei, modestride: GLint);
     func!(MultiTexBufferEXT, (), texunit: GLenum, target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(MultiTexCoord1bOES, (), texture: GLenum, s: GLbyte);
     func!(MultiTexCoord1bvOES, (), texture: GLenum, coords: *const GLbyte);
     func!(MultiTexCoord1dARB, (), target: GLenum, s: GLdouble);
     func!(MultiTexCoord1dvARB, (), target: GLenum, v: *const GLdouble);
     func!(MultiTexCoord1fARB, (), target: GLenum, s: GLfloat);
     func!(MultiTexCoord1fvARB, (), target: GLenum, v: *const GLfloat);
     func!(MultiTexCoord1hNV, (), target: GLenum, s: GLhalfNV);
     func!(MultiTexCoord1hvNV, (), target: GLenum, v: *const GLhalfNV);
     func!(MultiTexCoord1iARB, (), target: GLenum, s: GLint);
     func!(MultiTexCoord1ivARB, (), target: GLenum, v: *const GLint);
     func!(MultiTexCoord1sARB, (), target: GLenum, s: GLshort);
     func!(MultiTexCoord1svARB, (), target: GLenum, v: *const GLshort);
     func!(MultiTexCoord1xOES, (), texture: GLenum, s: GLfixed);
     func!(MultiTexCoord1xvOES, (), texture: GLenum, coords: *const GLfixed);
     func!(MultiTexCoord2bOES, (), texture: GLenum, s: GLbyte, t: GLbyte);
     func!(MultiTexCoord2bvOES, (), texture: GLenum, coords: *const GLbyte);
     func!(MultiTexCoord2dARB, (), target: GLenum, s: GLdouble, t: GLdouble);
     func!(MultiTexCoord2dvARB, (), target: GLenum, v: *const GLdouble);
     func!(MultiTexCoord2fARB, (), target: GLenum, s: GLfloat, t: GLfloat);
     func!(MultiTexCoord2fvARB, (), target: GLenum, v: *const GLfloat);
     func!(MultiTexCoord2hNV, (), target: GLenum, s: GLhalfNV, t: GLhalfNV);
     func!(MultiTexCoord2hvNV, (), target: GLenum, v: *const GLhalfNV);
     func!(MultiTexCoord2iARB, (), target: GLenum, s: GLint, t: GLint);
     func!(MultiTexCoord2ivARB, (), target: GLenum, v: *const GLint);
     func!(MultiTexCoord2sARB, (), target: GLenum, s: GLshort, t: GLshort);
     func!(MultiTexCoord2svARB, (), target: GLenum, v: *const GLshort);
     func!(MultiTexCoord2xOES, (), texture: GLenum, s: GLfixed, t: GLfixed);
     func!(MultiTexCoord2xvOES, (), texture: GLenum, coords: *const GLfixed);
     func!(MultiTexCoord3bOES, (), texture: GLenum, s: GLbyte, t: GLbyte, r: GLbyte);
     func!(MultiTexCoord3bvOES, (), texture: GLenum, coords: *const GLbyte);
     func!(MultiTexCoord3dARB, (), target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble);
     func!(MultiTexCoord3dvARB, (), target: GLenum, v: *const GLdouble);
     func!(MultiTexCoord3fARB, (), target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat);
     func!(MultiTexCoord3fvARB, (), target: GLenum, v: *const GLfloat);
     func!(MultiTexCoord3hNV, (), target: GLenum, s: GLhalfNV, t: GLhalfNV, r: GLhalfNV);
     func!(MultiTexCoord3hvNV, (), target: GLenum, v: *const GLhalfNV);
     func!(MultiTexCoord3iARB, (), target: GLenum, s: GLint, t: GLint, r: GLint);
     func!(MultiTexCoord3ivARB, (), target: GLenum, v: *const GLint);
     func!(MultiTexCoord3sARB, (), target: GLenum, s: GLshort, t: GLshort, r: GLshort);
     func!(MultiTexCoord3svARB, (), target: GLenum, v: *const GLshort);
     func!(MultiTexCoord3xOES, (), texture: GLenum, s: GLfixed, t: GLfixed, r: GLfixed);
     func!(MultiTexCoord3xvOES, (), texture: GLenum, coords: *const GLfixed);
     func!(MultiTexCoord4bOES, (), texture: GLenum, s: GLbyte, t: GLbyte, r: GLbyte, q: GLbyte);
     func!(MultiTexCoord4bvOES, (), texture: GLenum, coords: *const GLbyte);
     func!(MultiTexCoord4dARB, (), target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);
     func!(MultiTexCoord4dvARB, (), target: GLenum, v: *const GLdouble);
     func!(MultiTexCoord4fARB, (), target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);
     func!(MultiTexCoord4fvARB, (), target: GLenum, v: *const GLfloat);
     func!(MultiTexCoord4hNV, (), target: GLenum, s: GLhalfNV, t: GLhalfNV, r: GLhalfNV, q: GLhalfNV);
     func!(MultiTexCoord4hvNV, (), target: GLenum, v: *const GLhalfNV);
     func!(MultiTexCoord4iARB, (), target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint);
     func!(MultiTexCoord4ivARB, (), target: GLenum, v: *const GLint);
     func!(MultiTexCoord4sARB, (), target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort);
     func!(MultiTexCoord4svARB, (), target: GLenum, v: *const GLshort);
     func!(MultiTexCoord4xOES, (), texture: GLenum, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);
     func!(MultiTexCoord4xvOES, (), texture: GLenum, coords: *const GLfixed);
     func!(MultiTexCoordPointerEXT, (), texunit: GLenum, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(MultiTexEnvfEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, param: GLfloat);
     func!(MultiTexEnvfvEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(MultiTexEnviEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, param: GLint);
     func!(MultiTexEnvivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLint);
     func!(MultiTexGendEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, param: GLdouble);
     func!(MultiTexGendvEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *const GLdouble);
     func!(MultiTexGenfEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, param: GLfloat);
     func!(MultiTexGenfvEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *const GLfloat);
     func!(MultiTexGeniEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, param: GLint);
     func!(MultiTexGenivEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *const GLint);
     func!(MultiTexImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexParameterIivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLint);
     func!(MultiTexParameterIuivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLuint);
     func!(MultiTexParameterfEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, param: GLfloat);
     func!(MultiTexParameterfvEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(MultiTexParameteriEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, param: GLint);
     func!(MultiTexParameterivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLint);
     func!(MultiTexRenderbufferEXT, (), texunit: GLenum, target: GLenum, renderbuffer: GLuint);
     func!(MultiTexSubImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexSubImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexSubImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MulticastBarrierNV, (), );
     func!(MulticastBlitFramebufferNV, (), srcGpu: GLuint, dstGpu: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(MulticastBufferSubDataNV, (), gpuMask: GLbitfield, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
     func!(MulticastCopyBufferSubDataNV, (), readGpu: GLuint, writeGpuMask: GLbitfield, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
     func!(MulticastCopyImageSubDataNV, (), srcGpu: GLuint, dstGpuMask: GLbitfield, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);
     func!(MulticastFramebufferSampleLocationsfvNV, (), gpu: GLuint, framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);
     func!(MulticastGetQueryObjecti64vNV, (), gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLint64);
     func!(MulticastGetQueryObjectivNV, (), gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLint);
     func!(MulticastGetQueryObjectui64vNV, (), gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLuint64);
     func!(MulticastGetQueryObjectuivNV, (), gpu: GLuint, id: GLuint, pname: GLenum, params: *mut GLuint);
     func!(MulticastScissorArrayvNVX, (), gpu: GLuint, first: GLuint, count: GLsizei, v: *const GLint);
     func!(MulticastViewportArrayvNVX, (), gpu: GLuint, first: GLuint, count: GLsizei, v: *const GLfloat);
     func!(MulticastViewportPositionWScaleNVX, (), gpu: GLuint, index: GLuint, xcoeff: GLfloat, ycoeff: GLfloat);
     func!(MulticastWaitSyncNV, (), signalGpu: GLuint, waitGpuMask: GLbitfield);
     func!(NamedBufferAttachMemoryNV, (), buffer: GLuint, memory: GLuint, offset: GLuint64);
     func!(NamedBufferData, (), buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum);
     func!(NamedBufferDataEXT, (), buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum);
     func!(NamedBufferPageCommitmentARB, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);
     func!(NamedBufferPageCommitmentEXT, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, commit: GLboolean);
     func!(NamedBufferPageCommitmentMemNV, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, memory: GLuint, memOffset: GLuint64, commit: GLboolean);
     func!(NamedBufferStorage, (), buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
     func!(NamedBufferStorageEXT, (), buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
     func!(NamedBufferStorageExternalEXT, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, clientBuffer: GLeglClientBufferEXT, flags: GLbitfield);
     func!(NamedBufferStorageMemEXT, (), buffer: GLuint, size: GLsizeiptr, memory: GLuint, offset: GLuint64);
     func!(NamedBufferSubData, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
     func!(NamedBufferSubDataEXT, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
     func!(NamedCopyBufferSubDataEXT, (), readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
     func!(NamedFramebufferDrawBuffer, (), framebuffer: GLuint, buf: GLenum);
     func!(NamedFramebufferDrawBuffers, (), framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);
     func!(NamedFramebufferParameteri, (), framebuffer: GLuint, pname: GLenum, param: GLint);
     func!(NamedFramebufferParameteriEXT, (), framebuffer: GLuint, pname: GLenum, param: GLint);
     func!(NamedFramebufferReadBuffer, (), framebuffer: GLuint, src: GLenum);
     func!(NamedFramebufferRenderbuffer, (), framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
     func!(NamedFramebufferRenderbufferEXT, (), framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
     func!(NamedFramebufferSampleLocationsfvARB, (), framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);
     func!(NamedFramebufferSampleLocationsfvNV, (), framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);
     func!(NamedFramebufferSamplePositionsfvAMD, (), framebuffer: GLuint, numsamples: GLuint, pixelindex: GLuint, values: *const GLfloat);
     func!(NamedFramebufferTexture, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);
     func!(NamedFramebufferTexture1DEXT, (), framebuffer: GLuint, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(NamedFramebufferTexture2DEXT, (), framebuffer: GLuint, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(NamedFramebufferTexture3DEXT, (), framebuffer: GLuint, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);
     func!(NamedFramebufferTextureEXT, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);
     func!(NamedFramebufferTextureFaceEXT, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);
     func!(NamedFramebufferTextureLayer, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(NamedFramebufferTextureLayerEXT, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(NamedProgramLocalParameter4dEXT, (), program: GLuint, target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(NamedProgramLocalParameter4dvEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *const GLdouble);
     func!(NamedProgramLocalParameter4fEXT, (), program: GLuint, target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(NamedProgramLocalParameter4fvEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *const GLfloat);
     func!(NamedProgramLocalParameterI4iEXT, (), program: GLuint, target: GLenum, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
     func!(NamedProgramLocalParameterI4ivEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *const GLint);
     func!(NamedProgramLocalParameterI4uiEXT, (), program: GLuint, target: GLenum, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
     func!(NamedProgramLocalParameterI4uivEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *const GLuint);
     func!(NamedProgramLocalParameters4fvEXT, (), program: GLuint, target: GLenum, index: GLuint, count: GLsizei, params: *const GLfloat);
     func!(NamedProgramLocalParametersI4ivEXT, (), program: GLuint, target: GLenum, index: GLuint, count: GLsizei, params: *const GLint);
     func!(NamedProgramLocalParametersI4uivEXT, (), program: GLuint, target: GLenum, index: GLuint, count: GLsizei, params: *const GLuint);
     func!(NamedProgramStringEXT, (), program: GLuint, target: GLenum, format: GLenum, len: GLsizei, string: *const c_void);
     func!(NamedRenderbufferStorage, (), renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedRenderbufferStorageEXT, (), renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedRenderbufferStorageMultisample, (), renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedRenderbufferStorageMultisampleAdvancedAMD, (), renderbuffer: GLuint, samples: GLsizei, storageSamples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedRenderbufferStorageMultisampleCoverageEXT, (), renderbuffer: GLuint, coverageSamples: GLsizei, colorSamples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedRenderbufferStorageMultisampleEXT, (), renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedStringARB, (), type_: GLenum, namelen: GLint, name: *const GLchar, stringlen: GLint, string: *const GLchar);
     func!(NewObjectBufferATI, GLuint, size: GLsizei, pointer: *const c_void, usage: GLenum);
     func!(Normal3fVertex3fSUN, (), nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(Normal3fVertex3fvSUN, (), n: *const GLfloat, v: *const GLfloat);
     func!(Normal3hNV, (), nx: GLhalfNV, ny: GLhalfNV, nz: GLhalfNV);
     func!(Normal3hvNV, (), v: *const GLhalfNV);
     func!(Normal3xOES, (), nx: GLfixed, ny: GLfixed, nz: GLfixed);
     func!(Normal3xvOES, (), coords: *const GLfixed);
     func!(NormalFormatNV, (), type_: GLenum, stride: GLsizei);
     func!(NormalPointerEXT, (), type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(NormalPointerListIBM, (), type_: GLenum, stride: GLint, pointer: *const *const c_void, ptrstride: GLint);
     func!(NormalPointervINTEL, (), type_: GLenum, pointer: *const *const c_void);
     func!(NormalStream3bATI, (), stream: GLenum, nx: GLbyte, ny: GLbyte, nz: GLbyte);
     func!(NormalStream3bvATI, (), stream: GLenum, coords: *const GLbyte);
     func!(NormalStream3dATI, (), stream: GLenum, nx: GLdouble, ny: GLdouble, nz: GLdouble);
     func!(NormalStream3dvATI, (), stream: GLenum, coords: *const GLdouble);
     func!(NormalStream3fATI, (), stream: GLenum, nx: GLfloat, ny: GLfloat, nz: GLfloat);
     func!(NormalStream3fvATI, (), stream: GLenum, coords: *const GLfloat);
     func!(NormalStream3iATI, (), stream: GLenum, nx: GLint, ny: GLint, nz: GLint);
     func!(NormalStream3ivATI, (), stream: GLenum, coords: *const GLint);
     func!(NormalStream3sATI, (), stream: GLenum, nx: GLshort, ny: GLshort, nz: GLshort);
     func!(NormalStream3svATI, (), stream: GLenum, coords: *const GLshort);
     func!(ObjectLabel, (), identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar);
     func!(ObjectPtrLabel, (), ptr: *const c_void, length: GLsizei, label: *const GLchar);
     func!(ObjectPurgeableAPPLE, GLenum, objectType: GLenum, name: GLuint, option: GLenum);
     func!(ObjectUnpurgeableAPPLE, GLenum, objectType: GLenum, name: GLuint, option: GLenum);
     func!(OrthofOES, (), l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);
     func!(OrthoxOES, (), l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);
     func!(PNTrianglesfATI, (), pname: GLenum, param: GLfloat);
     func!(PNTrianglesiATI, (), pname: GLenum, param: GLint);
     func!(PassTexCoordATI, (), dst: GLuint, coord: GLuint, swizzle: GLenum);
     func!(PassThroughxOES, (), token: GLfixed);
     func!(PatchParameterfv, (), pname: GLenum, values: *const GLfloat);
     func!(PatchParameteri, (), pname: GLenum, value: GLint);
     func!(PathCommandsNV, (), path: GLuint, numCommands: GLsizei, commands: *const GLubyte, numCoords: GLsizei, coordType: GLenum, coords: *const c_void);
     func!(PathCoordsNV, (), path: GLuint, numCoords: GLsizei, coordType: GLenum, coords: *const c_void);
     func!(PathCoverDepthFuncNV, (), func: GLenum);
     func!(PathDashArrayNV, (), path: GLuint, dashCount: GLsizei, dashArray: *const GLfloat);
     func!(PathGlyphIndexArrayNV, GLenum, firstPathName: GLuint, fontTarget: GLenum, fontName: *const c_void, fontStyle: GLbitfield, firstGlyphIndex: GLuint, numGlyphs: GLsizei, pathParameterTemplate: GLuint, emScale: GLfloat);
     func!(PathGlyphIndexRangeNV, GLenum, fontTarget: GLenum, fontName: *const c_void, fontStyle: GLbitfield, pathParameterTemplate: GLuint, emScale: GLfloat, baseAndCount: *mut GLuint);
     func!(PathGlyphRangeNV, (), firstPathName: GLuint, fontTarget: GLenum, fontName: *const c_void, fontStyle: GLbitfield, firstGlyph: GLuint, numGlyphs: GLsizei, handleMissingGlyphs: GLenum, pathParameterTemplate: GLuint, emScale: GLfloat);
     func!(PathGlyphsNV, (), firstPathName: GLuint, fontTarget: GLenum, fontName: *const c_void, fontStyle: GLbitfield, numGlyphs: GLsizei, type_: GLenum, charcodes: *const c_void, handleMissingGlyphs: GLenum, pathParameterTemplate: GLuint, emScale: GLfloat);
     func!(PathMemoryGlyphIndexArrayNV, GLenum, firstPathName: GLuint, fontTarget: GLenum, fontSize: GLsizeiptr, fontData: *const c_void, faceIndex: GLsizei, firstGlyphIndex: GLuint, numGlyphs: GLsizei, pathParameterTemplate: GLuint, emScale: GLfloat);
     func!(PathParameterfNV, (), path: GLuint, pname: GLenum, value: GLfloat);
     func!(PathParameterfvNV, (), path: GLuint, pname: GLenum, value: *const GLfloat);
     func!(PathParameteriNV, (), path: GLuint, pname: GLenum, value: GLint);
     func!(PathParameterivNV, (), path: GLuint, pname: GLenum, value: *const GLint);
     func!(PathStencilDepthOffsetNV, (), factor: GLfloat, units: GLfloat);
     func!(PathStencilFuncNV, (), func: GLenum, ref_: GLint, mask: GLuint);
     func!(PathStringNV, (), path: GLuint, format: GLenum, length: GLsizei, pathString: *const c_void);
     func!(PathSubCommandsNV, (), path: GLuint, commandStart: GLsizei, commandsToDelete: GLsizei, numCommands: GLsizei, commands: *const GLubyte, numCoords: GLsizei, coordType: GLenum, coords: *const c_void);
     func!(PathSubCoordsNV, (), path: GLuint, coordStart: GLsizei, numCoords: GLsizei, coordType: GLenum, coords: *const c_void);
     func!(PauseTransformFeedback, (), );
     func!(PauseTransformFeedbackNV, (), );
     func!(PixelDataRangeNV, (), target: GLenum, length: GLsizei, pointer: *const c_void);
     func!(PixelMapx, (), map: GLenum, size: GLint, values: *const GLfixed);
     func!(PixelStoref, (), pname: GLenum, param: GLfloat);
     func!(PixelStorei, (), pname: GLenum, param: GLint);
     func!(PixelStorex, (), pname: GLenum, param: GLfixed);
     func!(PixelTexGenParameterfSGIS, (), pname: GLenum, param: GLfloat);
     func!(PixelTexGenParameterfvSGIS, (), pname: GLenum, params: *const GLfloat);
     func!(PixelTexGenParameteriSGIS, (), pname: GLenum, param: GLint);
     func!(PixelTexGenParameterivSGIS, (), pname: GLenum, params: *const GLint);
     func!(PixelTexGenSGIX, (), mode: GLenum);
     func!(PixelTransferxOES, (), pname: GLenum, param: GLfixed);
     func!(PixelTransformParameterfEXT, (), target: GLenum, pname: GLenum, param: GLfloat);
     func!(PixelTransformParameterfvEXT, (), target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(PixelTransformParameteriEXT, (), target: GLenum, pname: GLenum, param: GLint);
     func!(PixelTransformParameterivEXT, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(PixelZoomxOES, (), xfactor: GLfixed, yfactor: GLfixed);
     func!(PointAlongPathNV, GLboolean, path: GLuint, startSegment: GLsizei, numSegments: GLsizei, distance: GLfloat, x: *mut GLfloat, y: *mut GLfloat, tangentX: *mut GLfloat, tangentY: *mut GLfloat);
     func!(PointParameterf, (), pname: GLenum, param: GLfloat);
     func!(PointParameterfARB, (), pname: GLenum, param: GLfloat);
     func!(PointParameterfEXT, (), pname: GLenum, param: GLfloat);
     func!(PointParameterfSGIS, (), pname: GLenum, param: GLfloat);
     func!(PointParameterfv, (), pname: GLenum, params: *const GLfloat);
     func!(PointParameterfvARB, (), pname: GLenum, params: *const GLfloat);
     func!(PointParameterfvEXT, (), pname: GLenum, params: *const GLfloat);
     func!(PointParameterfvSGIS, (), pname: GLenum, params: *const GLfloat);
     func!(PointParameteri, (), pname: GLenum, param: GLint);
     func!(PointParameteriNV, (), pname: GLenum, param: GLint);
     func!(PointParameteriv, (), pname: GLenum, params: *const GLint);
     func!(PointParameterivNV, (), pname: GLenum, params: *const GLint);
     func!(PointParameterxvOES, (), pname: GLenum, params: *const GLfixed);
     func!(PointSize, (), size: GLfloat);
     func!(PointSizexOES, (), size: GLfixed);
     func!(PollAsyncSGIX, GLint, markerp: *mut GLuint);
     func!(PollInstrumentsSGIX, GLint, marker_p: *mut GLint);
     func!(PolygonMode, (), face: GLenum, mode: GLenum);
     func!(PolygonOffset, (), factor: GLfloat, units: GLfloat);
     func!(PolygonOffsetClamp, (), factor: GLfloat, units: GLfloat, clamp: GLfloat);
     func!(PolygonOffsetClampEXT, (), factor: GLfloat, units: GLfloat, clamp: GLfloat);
     func!(PolygonOffsetEXT, (), factor: GLfloat, bias: GLfloat);
     func!(PolygonOffsetxOES, (), factor: GLfixed, units: GLfixed);
     func!(PopDebugGroup, (), );
     func!(PopGroupMarkerEXT, (), );
     func!(PresentFrameDualFillNV, (), video_slot: GLuint, minPresentTime: GLuint64EXT, beginPresentTimeId: GLuint, presentDurationId: GLuint, type_: GLenum, target0: GLenum, fill0: GLuint, target1: GLenum, fill1: GLuint, target2: GLenum, fill2: GLuint, target3: GLenum, fill3: GLuint);
     func!(PresentFrameKeyedNV, (), video_slot: GLuint, minPresentTime: GLuint64EXT, beginPresentTimeId: GLuint, presentDurationId: GLuint, type_: GLenum, target0: GLenum, fill0: GLuint, key0: GLuint, target1: GLenum, fill1: GLuint, key1: GLuint);
     func!(PrimitiveBoundingBoxARB, (), minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);
     func!(PrimitiveRestartIndex, (), index: GLuint);
     func!(PrimitiveRestartIndexNV, (), index: GLuint);
     func!(PrimitiveRestartNV, (), );
     func!(PrioritizeTexturesEXT, (), n: GLsizei, textures: *const GLuint, priorities: *const GLclampf);
     func!(PrioritizeTexturesxOES, (), n: GLsizei, textures: *const GLuint, priorities: *const GLfixed);
     func!(ProgramBinary, (), program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei);
     func!(ProgramBufferParametersIivNV, (), target: GLenum, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLint);
     func!(ProgramBufferParametersIuivNV, (), target: GLenum, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLuint);
     func!(ProgramBufferParametersfvNV, (), target: GLenum, bindingIndex: GLuint, wordIndex: GLuint, count: GLsizei, params: *const GLfloat);
     func!(ProgramEnvParameter4dARB, (), target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(ProgramEnvParameter4dvARB, (), target: GLenum, index: GLuint, params: *const GLdouble);
     func!(ProgramEnvParameter4fARB, (), target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(ProgramEnvParameter4fvARB, (), target: GLenum, index: GLuint, params: *const GLfloat);
     func!(ProgramEnvParameterI4iNV, (), target: GLenum, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
     func!(ProgramEnvParameterI4ivNV, (), target: GLenum, index: GLuint, params: *const GLint);
     func!(ProgramEnvParameterI4uiNV, (), target: GLenum, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
     func!(ProgramEnvParameterI4uivNV, (), target: GLenum, index: GLuint, params: *const GLuint);
     func!(ProgramEnvParameters4fvEXT, (), target: GLenum, index: GLuint, count: GLsizei, params: *const GLfloat);
     func!(ProgramEnvParametersI4ivNV, (), target: GLenum, index: GLuint, count: GLsizei, params: *const GLint);
     func!(ProgramEnvParametersI4uivNV, (), target: GLenum, index: GLuint, count: GLsizei, params: *const GLuint);
     func!(ProgramLocalParameter4dARB, (), target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(ProgramLocalParameter4dvARB, (), target: GLenum, index: GLuint, params: *const GLdouble);
     func!(ProgramLocalParameter4fARB, (), target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(ProgramLocalParameter4fvARB, (), target: GLenum, index: GLuint, params: *const GLfloat);
     func!(ProgramLocalParameterI4iNV, (), target: GLenum, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
     func!(ProgramLocalParameterI4ivNV, (), target: GLenum, index: GLuint, params: *const GLint);
     func!(ProgramLocalParameterI4uiNV, (), target: GLenum, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
     func!(ProgramLocalParameterI4uivNV, (), target: GLenum, index: GLuint, params: *const GLuint);
     func!(ProgramLocalParameters4fvEXT, (), target: GLenum, index: GLuint, count: GLsizei, params: *const GLfloat);
     func!(ProgramLocalParametersI4ivNV, (), target: GLenum, index: GLuint, count: GLsizei, params: *const GLint);
     func!(ProgramLocalParametersI4uivNV, (), target: GLenum, index: GLuint, count: GLsizei, params: *const GLuint);
     func!(ProgramNamedParameter4dNV, (), id: GLuint, len: GLsizei, name: *const GLubyte, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(ProgramNamedParameter4dvNV, (), id: GLuint, len: GLsizei, name: *const GLubyte, v: *const GLdouble);
     func!(ProgramNamedParameter4fNV, (), id: GLuint, len: GLsizei, name: *const GLubyte, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(ProgramNamedParameter4fvNV, (), id: GLuint, len: GLsizei, name: *const GLubyte, v: *const GLfloat);
     func!(ProgramParameter4dNV, (), target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(ProgramParameter4dvNV, (), target: GLenum, index: GLuint, v: *const GLdouble);
     func!(ProgramParameter4fNV, (), target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(ProgramParameter4fvNV, (), target: GLenum, index: GLuint, v: *const GLfloat);
     func!(ProgramParameteri, (), program: GLuint, pname: GLenum, value: GLint);
     func!(ProgramParameteriARB, (), program: GLuint, pname: GLenum, value: GLint);
     func!(ProgramParameteriEXT, (), program: GLuint, pname: GLenum, value: GLint);
     func!(ProgramParameters4dvNV, (), target: GLenum, index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(ProgramParameters4fvNV, (), target: GLenum, index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(ProgramPathFragmentInputGenNV, (), program: GLuint, location: GLint, genMode: GLenum, components: GLint, coeffs: *const GLfloat);
     func!(ProgramStringARB, (), target: GLenum, format: GLenum, len: GLsizei, string: *const c_void);
     func!(ProgramSubroutineParametersuivNV, (), target: GLenum, count: GLsizei, params: *const GLuint);
     func!(ProgramUniform1d, (), program: GLuint, location: GLint, v0: GLdouble);
     func!(ProgramUniform1dEXT, (), program: GLuint, location: GLint, x: GLdouble);
     func!(ProgramUniform1dv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform1dvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform1f, (), program: GLuint, location: GLint, v0: GLfloat);
     func!(ProgramUniform1fEXT, (), program: GLuint, location: GLint, v0: GLfloat);
     func!(ProgramUniform1fv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform1fvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform1i, (), program: GLuint, location: GLint, v0: GLint);
     func!(ProgramUniform1i64ARB, (), program: GLuint, location: GLint, x: GLint64);
     func!(ProgramUniform1i64NV, (), program: GLuint, location: GLint, x: GLint64EXT);
     func!(ProgramUniform1i64vARB, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);
     func!(ProgramUniform1i64vNV, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);
     func!(ProgramUniform1iEXT, (), program: GLuint, location: GLint, v0: GLint);
     func!(ProgramUniform1iv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform1ivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform1ui, (), program: GLuint, location: GLint, v0: GLuint);
     func!(ProgramUniform1ui64ARB, (), program: GLuint, location: GLint, x: GLuint64);
     func!(ProgramUniform1ui64NV, (), program: GLuint, location: GLint, x: GLuint64EXT);
     func!(ProgramUniform1ui64vARB, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);
     func!(ProgramUniform1ui64vNV, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(ProgramUniform1uiEXT, (), program: GLuint, location: GLint, v0: GLuint);
     func!(ProgramUniform1uiv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform1uivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform2d, (), program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);
     func!(ProgramUniform2dEXT, (), program: GLuint, location: GLint, x: GLdouble, y: GLdouble);
     func!(ProgramUniform2dv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform2dvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform2f, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
     func!(ProgramUniform2fEXT, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
     func!(ProgramUniform2fv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform2fvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform2i, (), program: GLuint, location: GLint, v0: GLint, v1: GLint);
     func!(ProgramUniform2i64ARB, (), program: GLuint, location: GLint, x: GLint64, y: GLint64);
     func!(ProgramUniform2i64NV, (), program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT);
     func!(ProgramUniform2i64vARB, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);
     func!(ProgramUniform2i64vNV, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);
     func!(ProgramUniform2iEXT, (), program: GLuint, location: GLint, v0: GLint, v1: GLint);
     func!(ProgramUniform2iv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform2ivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform2ui, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
     func!(ProgramUniform2ui64ARB, (), program: GLuint, location: GLint, x: GLuint64, y: GLuint64);
     func!(ProgramUniform2ui64NV, (), program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT);
     func!(ProgramUniform2ui64vARB, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);
     func!(ProgramUniform2ui64vNV, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(ProgramUniform2uiEXT, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
     func!(ProgramUniform2uiv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform2uivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform3d, (), program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble);
     func!(ProgramUniform3dEXT, (), program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(ProgramUniform3dv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform3dvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform3f, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
     func!(ProgramUniform3fEXT, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
     func!(ProgramUniform3fv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform3fvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform3i, (), program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
     func!(ProgramUniform3i64ARB, (), program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64);
     func!(ProgramUniform3i64NV, (), program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);
     func!(ProgramUniform3i64vARB, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);
     func!(ProgramUniform3i64vNV, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);
     func!(ProgramUniform3iEXT, (), program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
     func!(ProgramUniform3iv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform3ivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform3ui, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
     func!(ProgramUniform3ui64ARB, (), program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);
     func!(ProgramUniform3ui64NV, (), program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);
     func!(ProgramUniform3ui64vARB, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);
     func!(ProgramUniform3ui64vNV, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(ProgramUniform3uiEXT, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
     func!(ProgramUniform3uiv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform3uivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform4d, (), program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble);
     func!(ProgramUniform4dEXT, (), program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(ProgramUniform4dv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform4dvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform4f, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
     func!(ProgramUniform4fEXT, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
     func!(ProgramUniform4fv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform4fvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform4i, (), program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
     func!(ProgramUniform4i64ARB, (), program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);
     func!(ProgramUniform4i64NV, (), program: GLuint, location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);
     func!(ProgramUniform4i64vARB, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);
     func!(ProgramUniform4i64vNV, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint64EXT);
     func!(ProgramUniform4iEXT, (), program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
     func!(ProgramUniform4iv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform4ivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform4ui, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
     func!(ProgramUniform4ui64ARB, (), program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);
     func!(ProgramUniform4ui64NV, (), program: GLuint, location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);
     func!(ProgramUniform4ui64vARB, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);
     func!(ProgramUniform4ui64vNV, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(ProgramUniform4uiEXT, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
     func!(ProgramUniform4uiv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform4uivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniformHandleui64ARB, (), program: GLuint, location: GLint, value: GLuint64);
     func!(ProgramUniformHandleui64NV, (), program: GLuint, location: GLint, value: GLuint64);
     func!(ProgramUniformHandleui64vARB, (), program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);
     func!(ProgramUniformHandleui64vNV, (), program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);
     func!(ProgramUniformMatrix2dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2x3dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2x3dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2x3fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2x3fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2x4dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2x4dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2x4fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2x4fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3x2dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3x2dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3x2fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3x2fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3x4dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3x4dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3x4fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3x4fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4x2dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4x2dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4x2fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4x2fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4x3dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4x3dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4x3fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4x3fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformui64NV, (), program: GLuint, location: GLint, value: GLuint64EXT);
     func!(ProgramUniformui64vNV, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(ProgramVertexLimitNV, (), target: GLenum, limit: GLint);
     func!(ProvokingVertex, (), mode: GLenum);
     func!(ProvokingVertexEXT, (), mode: GLenum);
     func!(PushClientAttribDefaultEXT, (), mask: GLbitfield);
     func!(PushDebugGroup, (), source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);
     func!(PushGroupMarkerEXT, (), length: GLsizei, marker: *const GLchar);
     func!(QueryCounter, (), id: GLuint, target: GLenum);
     func!(QueryMatrixxOES, GLbitfield, mantissa: *mut GLfixed, exponent: *mut GLint);
     func!(QueryObjectParameteruiAMD, (), target: GLenum, id: GLuint, pname: GLenum, param: GLuint);
     func!(QueryResourceNV, GLint, queryType: GLenum, tagId: GLint, count: GLuint, buffer: *mut GLint);
     func!(QueryResourceTagNV, (), tagId: GLint, tagString: *const GLchar);
     func!(RasterPos2xOES, (), x: GLfixed, y: GLfixed);
     func!(RasterPos2xvOES, (), coords: *const GLfixed);
     func!(RasterPos3xOES, (), x: GLfixed, y: GLfixed, z: GLfixed);
     func!(RasterPos3xvOES, (), coords: *const GLfixed);
     func!(RasterPos4xOES, (), x: GLfixed, y: GLfixed, z: GLfixed, w: GLfixed);
     func!(RasterPos4xvOES, (), coords: *const GLfixed);
     func!(RasterSamplesEXT, (), samples: GLuint, fixedsamplelocations: GLboolean);
     func!(ReadBuffer, (), src: GLenum);
     func!(ReadInstrumentsSGIX, (), marker: GLint);
     func!(ReadPixels, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void);
     func!(ReadnPixels, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void);
     func!(ReadnPixelsARB, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void);
     func!(RectxOES, (), x1: GLfixed, y1: GLfixed, x2: GLfixed, y2: GLfixed);
     func!(RectxvOES, (), v1: *const GLfixed, v2: *const GLfixed);
     func!(ReferencePlaneSGIX, (), equation: *const GLdouble);
     func!(ReleaseKeyedMutexWin32EXT, GLboolean, memory: GLuint, key: GLuint64);
     func!(ReleaseShaderCompiler, (), );
     func!(RenderGpuMaskNV, (), mask: GLbitfield);
     func!(RenderbufferStorage, (), target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(RenderbufferStorageEXT, (), target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(RenderbufferStorageMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(RenderbufferStorageMultisampleAdvancedAMD, (), target: GLenum, samples: GLsizei, storageSamples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(RenderbufferStorageMultisampleCoverageNV, (), target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(RenderbufferStorageMultisampleEXT, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(ReplacementCodePointerSUN, (), type_: GLenum, stride: GLsizei, pointer: *const *const c_void);
     func!(ReplacementCodeubSUN, (), code: GLubyte);
     func!(ReplacementCodeubvSUN, (), code: *const GLubyte);
     func!(ReplacementCodeuiColor3fVertex3fSUN, (), rc: GLuint, r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(ReplacementCodeuiColor3fVertex3fvSUN, (), rc: *const GLuint, c: *const GLfloat, v: *const GLfloat);
     func!(ReplacementCodeuiColor4fNormal3fVertex3fSUN, (), rc: GLuint, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(ReplacementCodeuiColor4fNormal3fVertex3fvSUN, (), rc: *const GLuint, c: *const GLfloat, n: *const GLfloat, v: *const GLfloat);
     func!(ReplacementCodeuiColor4ubVertex3fSUN, (), rc: GLuint, r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(ReplacementCodeuiColor4ubVertex3fvSUN, (), rc: *const GLuint, c: *const GLubyte, v: *const GLfloat);
     func!(ReplacementCodeuiNormal3fVertex3fSUN, (), rc: GLuint, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(ReplacementCodeuiNormal3fVertex3fvSUN, (), rc: *const GLuint, n: *const GLfloat, v: *const GLfloat);
     func!(ReplacementCodeuiSUN, (), code: GLuint);
     func!(ReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN, (), rc: GLuint, s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(ReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN, (), rc: *const GLuint, tc: *const GLfloat, c: *const GLfloat, n: *const GLfloat, v: *const GLfloat);
     func!(ReplacementCodeuiTexCoord2fNormal3fVertex3fSUN, (), rc: GLuint, s: GLfloat, t: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(ReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN, (), rc: *const GLuint, tc: *const GLfloat, n: *const GLfloat, v: *const GLfloat);
     func!(ReplacementCodeuiTexCoord2fVertex3fSUN, (), rc: GLuint, s: GLfloat, t: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(ReplacementCodeuiTexCoord2fVertex3fvSUN, (), rc: *const GLuint, tc: *const GLfloat, v: *const GLfloat);
     func!(ReplacementCodeuiVertex3fSUN, (), rc: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(ReplacementCodeuiVertex3fvSUN, (), rc: *const GLuint, v: *const GLfloat);
     func!(ReplacementCodeuivSUN, (), code: *const GLuint);
     func!(ReplacementCodeusSUN, (), code: GLushort);
     func!(ReplacementCodeusvSUN, (), code: *const GLushort);
     func!(RequestResidentProgramsNV, (), n: GLsizei, programs: *const GLuint);
     func!(ResetHistogramEXT, (), target: GLenum);
     func!(ResetMemoryObjectParameterNV, (), memory: GLuint, pname: GLenum);
     func!(ResetMinmaxEXT, (), target: GLenum);
     func!(ResizeBuffersMESA, (), );
     func!(ResolveDepthValuesNV, (), );
     func!(ResumeTransformFeedback, (), );
     func!(ResumeTransformFeedbackNV, (), );
     func!(RotatexOES, (), angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);
     func!(SampleCoverage, (), value: GLfloat, invert: GLboolean);
     func!(SampleCoverageARB, (), value: GLfloat, invert: GLboolean);
     func!(SampleMapATI, (), dst: GLuint, interp: GLuint, swizzle: GLenum);
     func!(SampleMaskEXT, (), value: GLclampf, invert: GLboolean);
     func!(SampleMaskIndexedNV, (), index: GLuint, mask: GLbitfield);
     func!(SampleMaskSGIS, (), value: GLclampf, invert: GLboolean);
     func!(SampleMaski, (), maskNumber: GLuint, mask: GLbitfield);
     func!(SamplePatternEXT, (), pattern: GLenum);
     func!(SamplePatternSGIS, (), pattern: GLenum);
     func!(SamplerParameterIiv, (), sampler: GLuint, pname: GLenum, param: *const GLint);
     func!(SamplerParameterIuiv, (), sampler: GLuint, pname: GLenum, param: *const GLuint);
     func!(SamplerParameterf, (), sampler: GLuint, pname: GLenum, param: GLfloat);
     func!(SamplerParameterfv, (), sampler: GLuint, pname: GLenum, param: *const GLfloat);
     func!(SamplerParameteri, (), sampler: GLuint, pname: GLenum, param: GLint);
     func!(SamplerParameteriv, (), sampler: GLuint, pname: GLenum, param: *const GLint);
     func!(ScalexOES, (), x: GLfixed, y: GLfixed, z: GLfixed);
     func!(Scissor, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(ScissorArrayv, (), first: GLuint, count: GLsizei, v: *const GLint);
     func!(ScissorExclusiveArrayvNV, (), first: GLuint, count: GLsizei, v: *const GLint);
     func!(ScissorExclusiveNV, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(ScissorIndexed, (), index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);
     func!(ScissorIndexedv, (), index: GLuint, v: *const GLint);
     func!(SecondaryColor3bEXT, (), red: GLbyte, green: GLbyte, blue: GLbyte);
     func!(SecondaryColor3bvEXT, (), v: *const GLbyte);
     func!(SecondaryColor3dEXT, (), red: GLdouble, green: GLdouble, blue: GLdouble);
     func!(SecondaryColor3dvEXT, (), v: *const GLdouble);
     func!(SecondaryColor3fEXT, (), red: GLfloat, green: GLfloat, blue: GLfloat);
     func!(SecondaryColor3fvEXT, (), v: *const GLfloat);
     func!(SecondaryColor3hNV, (), red: GLhalfNV, green: GLhalfNV, blue: GLhalfNV);
     func!(SecondaryColor3hvNV, (), v: *const GLhalfNV);
     func!(SecondaryColor3iEXT, (), red: GLint, green: GLint, blue: GLint);
     func!(SecondaryColor3ivEXT, (), v: *const GLint);
     func!(SecondaryColor3sEXT, (), red: GLshort, green: GLshort, blue: GLshort);
     func!(SecondaryColor3svEXT, (), v: *const GLshort);
     func!(SecondaryColor3ubEXT, (), red: GLubyte, green: GLubyte, blue: GLubyte);
     func!(SecondaryColor3ubvEXT, (), v: *const GLubyte);
     func!(SecondaryColor3uiEXT, (), red: GLuint, green: GLuint, blue: GLuint);
     func!(SecondaryColor3uivEXT, (), v: *const GLuint);
     func!(SecondaryColor3usEXT, (), red: GLushort, green: GLushort, blue: GLushort);
     func!(SecondaryColor3usvEXT, (), v: *const GLushort);
     func!(SecondaryColorFormatNV, (), size: GLint, type_: GLenum, stride: GLsizei);
     func!(SecondaryColorPointerEXT, (), size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(SecondaryColorPointerListIBM, (), size: GLint, type_: GLenum, stride: GLint, pointer: *const *const c_void, ptrstride: GLint);
     func!(SelectPerfMonitorCountersAMD, (), monitor: GLuint, enable: GLboolean, group: GLuint, numCounters: GLint, counterList: *mut GLuint);
     func!(SemaphoreParameterivNV, (), semaphore: GLuint, pname: GLenum, params: *const GLint);
     func!(SemaphoreParameterui64vEXT, (), semaphore: GLuint, pname: GLenum, params: *const GLuint64);
     func!(SeparableFilter2DEXT, (), target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, row: *const c_void, column: *const c_void);
     func!(SetFenceAPPLE, (), fence: GLuint);
     func!(SetFenceNV, (), fence: GLuint, condition: GLenum);
     func!(SetFragmentShaderConstantATI, (), dst: GLuint, value: *const GLfloat);
     func!(SetInvariantEXT, (), id: GLuint, type_: GLenum, addr: *const c_void);
     func!(SetLocalConstantEXT, (), id: GLuint, type_: GLenum, addr: *const c_void);
     func!(SetMultisamplefvAMD, (), pname: GLenum, index: GLuint, val: *const GLfloat);
     func!(ShaderBinary, (), count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei);
     func!(ShaderOp1EXT, (), op: GLenum, res: GLuint, arg1: GLuint);
     func!(ShaderOp2EXT, (), op: GLenum, res: GLuint, arg1: GLuint, arg2: GLuint);
     func!(ShaderOp3EXT, (), op: GLenum, res: GLuint, arg1: GLuint, arg2: GLuint, arg3: GLuint);
     func!(ShaderSource, (), shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);
     func!(ShaderSourceARB, (), shaderObj: GLhandleARB, count: GLsizei, string: *const *const GLcharARB, length: *const GLint);
     func!(ShaderStorageBlockBinding, (), program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint);
     func!(ShadingRateImageBarrierNV, (), synchronize: GLboolean);
     func!(ShadingRateImagePaletteNV, (), viewport: GLuint, first: GLuint, count: GLsizei, rates: *const GLenum);
     func!(ShadingRateSampleOrderCustomNV, (), rate: GLenum, samples: GLuint, locations: *const GLint);
     func!(ShadingRateSampleOrderNV, (), order: GLenum);
     func!(SharpenTexFuncSGIS, (), target: GLenum, n: GLsizei, points: *const GLfloat);
     func!(SignalSemaphoreEXT, (), semaphore: GLuint, numBufferBarriers: GLuint, buffers: *const GLuint, numTextureBarriers: GLuint, textures: *const GLuint, dstLayouts: *const GLenum);
     func!(SignalSemaphoreui64NVX, (), signalGpu: GLuint, fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);
     func!(SignalVkFenceNV, (), vkFence: GLuint64);
     func!(SignalVkSemaphoreNV, (), vkSemaphore: GLuint64);
     func!(SpecializeShader, (), shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);
     func!(SpecializeShaderARB, (), shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);
     func!(SpriteParameterfSGIX, (), pname: GLenum, param: GLfloat);
     func!(SpriteParameterfvSGIX, (), pname: GLenum, params: *const GLfloat);
     func!(SpriteParameteriSGIX, (), pname: GLenum, param: GLint);
     func!(SpriteParameterivSGIX, (), pname: GLenum, params: *const GLint);
     func!(StartInstrumentsSGIX, (), );
     func!(StateCaptureNV, (), state: GLuint, mode: GLenum);
     func!(StencilClearTagEXT, (), stencilTagBits: GLsizei, stencilClearTag: GLuint);
     func!(StencilFillPathInstancedNV, (), numPaths: GLsizei, pathNameType: GLenum, paths: *const c_void, pathBase: GLuint, fillMode: GLenum, mask: GLuint, transformType: GLenum, transformValues: *const GLfloat);
     func!(StencilFillPathNV, (), path: GLuint, fillMode: GLenum, mask: GLuint);
     func!(StencilFunc, (), func: GLenum, ref_: GLint, mask: GLuint);
     func!(StencilFuncSeparate, (), face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
     func!(StencilFuncSeparateATI, (), frontfunc: GLenum, backfunc: GLenum, ref_: GLint, mask: GLuint);
     func!(StencilMask, (), mask: GLuint);
     func!(StencilMaskSeparate, (), face: GLenum, mask: GLuint);
     func!(StencilOp, (), fail: GLenum, zfail: GLenum, zpass: GLenum);
     func!(StencilOpSeparate, (), face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
     func!(StencilOpSeparateATI, (), face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
     func!(StencilOpValueAMD, (), face: GLenum, value: GLuint);
     func!(StencilStrokePathInstancedNV, (), numPaths: GLsizei, pathNameType: GLenum, paths: *const c_void, pathBase: GLuint, reference: GLint, mask: GLuint, transformType: GLenum, transformValues: *const GLfloat);
     func!(StencilStrokePathNV, (), path: GLuint, reference: GLint, mask: GLuint);
     func!(StencilThenCoverFillPathInstancedNV, (), numPaths: GLsizei, pathNameType: GLenum, paths: *const c_void, pathBase: GLuint, fillMode: GLenum, mask: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);
     func!(StencilThenCoverFillPathNV, (), path: GLuint, fillMode: GLenum, mask: GLuint, coverMode: GLenum);
     func!(StencilThenCoverStrokePathInstancedNV, (), numPaths: GLsizei, pathNameType: GLenum, paths: *const c_void, pathBase: GLuint, reference: GLint, mask: GLuint, coverMode: GLenum, transformType: GLenum, transformValues: *const GLfloat);
     func!(StencilThenCoverStrokePathNV, (), path: GLuint, reference: GLint, mask: GLuint, coverMode: GLenum);
     func!(StopInstrumentsSGIX, (), marker: GLint);
     func!(StringMarkerGREMEDY, (), len: GLsizei, string: *const c_void);
     func!(SubpixelPrecisionBiasNV, (), xbits: GLuint, ybits: GLuint);
     func!(SwizzleEXT, (), res: GLuint, in_: GLuint, outX: GLenum, outY: GLenum, outZ: GLenum, outW: GLenum);
     func!(SyncTextureINTEL, (), texture: GLuint);
     func!(TagSampleBufferSGIX, (), );
     func!(Tangent3bEXT, (), tx: GLbyte, ty: GLbyte, tz: GLbyte);
     func!(Tangent3bvEXT, (), v: *const GLbyte);
     func!(Tangent3dEXT, (), tx: GLdouble, ty: GLdouble, tz: GLdouble);
     func!(Tangent3dvEXT, (), v: *const GLdouble);
     func!(Tangent3fEXT, (), tx: GLfloat, ty: GLfloat, tz: GLfloat);
     func!(Tangent3fvEXT, (), v: *const GLfloat);
     func!(Tangent3iEXT, (), tx: GLint, ty: GLint, tz: GLint);
     func!(Tangent3ivEXT, (), v: *const GLint);
     func!(Tangent3sEXT, (), tx: GLshort, ty: GLshort, tz: GLshort);
     func!(Tangent3svEXT, (), v: *const GLshort);
     func!(TangentPointerEXT, (), type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(TbufferMask3DFX, (), mask: GLuint);
     func!(TessellationFactorAMD, (), factor: GLfloat);
     func!(TessellationModeAMD, (), mode: GLenum);
     func!(TestFenceAPPLE, GLboolean, fence: GLuint);
     func!(TestFenceNV, GLboolean, fence: GLuint);
     func!(TestObjectAPPLE, GLboolean, object: GLenum, name: GLuint);
     func!(TexAttachMemoryNV, (), target: GLenum, memory: GLuint, offset: GLuint64);
     func!(TexBuffer, (), target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(TexBufferARB, (), target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(TexBufferEXT, (), target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(TexBufferRange, (), target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(TexBumpParameterfvATI, (), pname: GLenum, param: *const GLfloat);
     func!(TexBumpParameterivATI, (), pname: GLenum, param: *const GLint);
     func!(TexCoord1bOES, (), s: GLbyte);
     func!(TexCoord1bvOES, (), coords: *const GLbyte);
     func!(TexCoord1hNV, (), s: GLhalfNV);
     func!(TexCoord1hvNV, (), v: *const GLhalfNV);
     func!(TexCoord1xOES, (), s: GLfixed);
     func!(TexCoord1xvOES, (), coords: *const GLfixed);
     func!(TexCoord2bOES, (), s: GLbyte, t: GLbyte);
     func!(TexCoord2bvOES, (), coords: *const GLbyte);
     func!(TexCoord2fColor3fVertex3fSUN, (), s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(TexCoord2fColor3fVertex3fvSUN, (), tc: *const GLfloat, c: *const GLfloat, v: *const GLfloat);
     func!(TexCoord2fColor4fNormal3fVertex3fSUN, (), s: GLfloat, t: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(TexCoord2fColor4fNormal3fVertex3fvSUN, (), tc: *const GLfloat, c: *const GLfloat, n: *const GLfloat, v: *const GLfloat);
     func!(TexCoord2fColor4ubVertex3fSUN, (), s: GLfloat, t: GLfloat, r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(TexCoord2fColor4ubVertex3fvSUN, (), tc: *const GLfloat, c: *const GLubyte, v: *const GLfloat);
     func!(TexCoord2fNormal3fVertex3fSUN, (), s: GLfloat, t: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(TexCoord2fNormal3fVertex3fvSUN, (), tc: *const GLfloat, n: *const GLfloat, v: *const GLfloat);
     func!(TexCoord2fVertex3fSUN, (), s: GLfloat, t: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(TexCoord2fVertex3fvSUN, (), tc: *const GLfloat, v: *const GLfloat);
     func!(TexCoord2hNV, (), s: GLhalfNV, t: GLhalfNV);
     func!(TexCoord2hvNV, (), v: *const GLhalfNV);
     func!(TexCoord2xOES, (), s: GLfixed, t: GLfixed);
     func!(TexCoord2xvOES, (), coords: *const GLfixed);
     func!(TexCoord3bOES, (), s: GLbyte, t: GLbyte, r: GLbyte);
     func!(TexCoord3bvOES, (), coords: *const GLbyte);
     func!(TexCoord3hNV, (), s: GLhalfNV, t: GLhalfNV, r: GLhalfNV);
     func!(TexCoord3hvNV, (), v: *const GLhalfNV);
     func!(TexCoord3xOES, (), s: GLfixed, t: GLfixed, r: GLfixed);
     func!(TexCoord3xvOES, (), coords: *const GLfixed);
     func!(TexCoord4bOES, (), s: GLbyte, t: GLbyte, r: GLbyte, q: GLbyte);
     func!(TexCoord4bvOES, (), coords: *const GLbyte);
     func!(TexCoord4fColor4fNormal3fVertex4fSUN, (), s: GLfloat, t: GLfloat, p: GLfloat, q: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat, nx: GLfloat, ny: GLfloat, nz: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(TexCoord4fColor4fNormal3fVertex4fvSUN, (), tc: *const GLfloat, c: *const GLfloat, n: *const GLfloat, v: *const GLfloat);
     func!(TexCoord4fVertex4fSUN, (), s: GLfloat, t: GLfloat, p: GLfloat, q: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(TexCoord4fVertex4fvSUN, (), tc: *const GLfloat, v: *const GLfloat);
     func!(TexCoord4hNV, (), s: GLhalfNV, t: GLhalfNV, r: GLhalfNV, q: GLhalfNV);
     func!(TexCoord4hvNV, (), v: *const GLhalfNV);
     func!(TexCoord4xOES, (), s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);
     func!(TexCoord4xvOES, (), coords: *const GLfixed);
     func!(TexCoordFormatNV, (), size: GLint, type_: GLenum, stride: GLsizei);
     func!(TexCoordPointerEXT, (), size: GLint, type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(TexCoordPointerListIBM, (), size: GLint, type_: GLenum, stride: GLint, pointer: *const *const c_void, ptrstride: GLint);
     func!(TexCoordPointervINTEL, (), size: GLint, type_: GLenum, pointer: *const *const c_void);
     func!(TexEnvxOES, (), target: GLenum, pname: GLenum, param: GLfixed);
     func!(TexEnvxvOES, (), target: GLenum, pname: GLenum, params: *const GLfixed);
     func!(TexFilterFuncSGIS, (), target: GLenum, filter: GLenum, n: GLsizei, weights: *const GLfloat);
     func!(TexGenxOES, (), coord: GLenum, pname: GLenum, param: GLfixed);
     func!(TexGenxvOES, (), coord: GLenum, pname: GLenum, params: *const GLfixed);
     func!(TexImage1D, (), target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexImage2D, (), target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexImage2DMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
     func!(TexImage2DMultisampleCoverageNV, (), target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);
     func!(TexImage3D, (), target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexImage3DEXT, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexImage3DMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
     func!(TexImage3DMultisampleCoverageNV, (), target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);
     func!(TexImage4DSGIS, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, size4d: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexPageCommitmentARB, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);
     func!(TexPageCommitmentMemNV, (), target: GLenum, layer: GLint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64, commit: GLboolean);
     func!(TexParameterIiv, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(TexParameterIivEXT, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(TexParameterIuiv, (), target: GLenum, pname: GLenum, params: *const GLuint);
     func!(TexParameterIuivEXT, (), target: GLenum, pname: GLenum, params: *const GLuint);
     func!(TexParameterf, (), target: GLenum, pname: GLenum, param: GLfloat);
     func!(TexParameterfv, (), target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(TexParameteri, (), target: GLenum, pname: GLenum, param: GLint);
     func!(TexParameteriv, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(TexParameterxOES, (), target: GLenum, pname: GLenum, param: GLfixed);
     func!(TexParameterxvOES, (), target: GLenum, pname: GLenum, params: *const GLfixed);
     func!(TexRenderbufferNV, (), target: GLenum, renderbuffer: GLuint);
     func!(TexStorage1D, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);
     func!(TexStorage1DEXT, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);
     func!(TexStorage2D, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(TexStorage2DEXT, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(TexStorage2DMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
     func!(TexStorage3D, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(TexStorage3DEXT, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(TexStorage3DMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
     func!(TexStorageMem1DEXT, (), target: GLenum, levels: GLsizei, internalFormat: GLenum, width: GLsizei, memory: GLuint, offset: GLuint64);
     func!(TexStorageMem2DEXT, (), target: GLenum, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, memory: GLuint, offset: GLuint64);
     func!(TexStorageMem2DMultisampleEXT, (), target: GLenum, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);
     func!(TexStorageMem3DEXT, (), target: GLenum, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64);
     func!(TexStorageMem3DMultisampleEXT, (), target: GLenum, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);
     func!(TexStorageSparseAMD, (), target: GLenum, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, layers: GLsizei, flags: GLbitfield);
     func!(TexSubImage1D, (), target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage1DEXT, (), target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage2D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage2DEXT, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage3D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage3DEXT, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage4DSGIS, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, woffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, size4d: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureAttachMemoryNV, (), texture: GLuint, memory: GLuint, offset: GLuint64);
     func!(TextureBarrier, (), );
     func!(TextureBarrierNV, (), );
     func!(TextureBuffer, (), texture: GLuint, internalformat: GLenum, buffer: GLuint);
     func!(TextureBufferEXT, (), texture: GLuint, target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(TextureBufferRange, (), texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(TextureBufferRangeEXT, (), texture: GLuint, target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(TextureColorMaskSGIS, (), red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
     func!(TextureImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureImage2DMultisampleCoverageNV, (), texture: GLuint, target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);
     func!(TextureImage2DMultisampleNV, (), texture: GLuint, target: GLenum, samples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean);
     func!(TextureImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureImage3DMultisampleCoverageNV, (), texture: GLuint, target: GLenum, coverageSamples: GLsizei, colorSamples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);
     func!(TextureImage3DMultisampleNV, (), texture: GLuint, target: GLenum, samples: GLsizei, internalFormat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean);
     func!(TextureLightEXT, (), pname: GLenum);
     func!(TextureMaterialEXT, (), face: GLenum, mode: GLenum);
     func!(TextureNormalEXT, (), mode: GLenum);
     func!(TexturePageCommitmentEXT, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);
     func!(TexturePageCommitmentMemNV, (), texture: GLuint, layer: GLint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64, commit: GLboolean);
     func!(TextureParameterIiv, (), texture: GLuint, pname: GLenum, params: *const GLint);
     func!(TextureParameterIivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *const GLint);
     func!(TextureParameterIuiv, (), texture: GLuint, pname: GLenum, params: *const GLuint);
     func!(TextureParameterIuivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *const GLuint);
     func!(TextureParameterf, (), texture: GLuint, pname: GLenum, param: GLfloat);
     func!(TextureParameterfEXT, (), texture: GLuint, target: GLenum, pname: GLenum, param: GLfloat);
     func!(TextureParameterfv, (), texture: GLuint, pname: GLenum, param: *const GLfloat);
     func!(TextureParameterfvEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(TextureParameteri, (), texture: GLuint, pname: GLenum, param: GLint);
     func!(TextureParameteriEXT, (), texture: GLuint, target: GLenum, pname: GLenum, param: GLint);
     func!(TextureParameteriv, (), texture: GLuint, pname: GLenum, param: *const GLint);
     func!(TextureParameterivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *const GLint);
     func!(TextureRangeAPPLE, (), target: GLenum, length: GLsizei, pointer: *const c_void);
     func!(TextureRenderbufferEXT, (), texture: GLuint, target: GLenum, renderbuffer: GLuint);
     func!(TextureStorage1D, (), texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei);
     func!(TextureStorage1DEXT, (), texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);
     func!(TextureStorage2D, (), texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(TextureStorage2DEXT, (), texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(TextureStorage2DMultisample, (), texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
     func!(TextureStorage2DMultisampleEXT, (), texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
     func!(TextureStorage3D, (), texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(TextureStorage3DEXT, (), texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(TextureStorage3DMultisample, (), texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
     func!(TextureStorage3DMultisampleEXT, (), texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
     func!(TextureStorageMem1DEXT, (), texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, memory: GLuint, offset: GLuint64);
     func!(TextureStorageMem2DEXT, (), texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, memory: GLuint, offset: GLuint64);
     func!(TextureStorageMem2DMultisampleEXT, (), texture: GLuint, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);
     func!(TextureStorageMem3DEXT, (), texture: GLuint, levels: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, memory: GLuint, offset: GLuint64);
     func!(TextureStorageMem3DMultisampleEXT, (), texture: GLuint, samples: GLsizei, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedSampleLocations: GLboolean, memory: GLuint, offset: GLuint64);
     func!(TextureStorageSparseAMD, (), texture: GLuint, target: GLenum, internalFormat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, layers: GLsizei, flags: GLbitfield);
     func!(TextureSubImage1D, (), texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage2D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage3D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureView, (), texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);
     func!(TrackMatrixNV, (), target: GLenum, address: GLuint, matrix: GLenum, transform: GLenum);
     func!(TransformFeedbackAttribsNV, (), count: GLsizei, attribs: *const GLint, bufferMode: GLenum);
     func!(TransformFeedbackBufferBase, (), xfb: GLuint, index: GLuint, buffer: GLuint);
     func!(TransformFeedbackBufferRange, (), xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(TransformFeedbackStreamAttribsNV, (), count: GLsizei, attribs: *const GLint, nbuffers: GLsizei, bufstreams: *const GLint, bufferMode: GLenum);
     func!(TransformFeedbackVaryings, (), program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);
     func!(TransformFeedbackVaryingsEXT, (), program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);
     func!(TransformFeedbackVaryingsNV, (), program: GLuint, count: GLsizei, locations: *const GLint, bufferMode: GLenum);
     func!(TransformPathNV, (), resultPath: GLuint, srcPath: GLuint, transformType: GLenum, transformValues: *const GLfloat);
     func!(TranslatexOES, (), x: GLfixed, y: GLfixed, z: GLfixed);
     func!(Uniform1d, (), location: GLint, x: GLdouble);
     func!(Uniform1dv, (), location: GLint, count: GLsizei, value: *const GLdouble);
     func!(Uniform1f, (), location: GLint, v0: GLfloat);
     func!(Uniform1fARB, (), location: GLint, v0: GLfloat);
     func!(Uniform1fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform1fvARB, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform1i, (), location: GLint, v0: GLint);
     func!(Uniform1i64ARB, (), location: GLint, x: GLint64);
     func!(Uniform1i64NV, (), location: GLint, x: GLint64EXT);
     func!(Uniform1i64vARB, (), location: GLint, count: GLsizei, value: *const GLint64);
     func!(Uniform1i64vNV, (), location: GLint, count: GLsizei, value: *const GLint64EXT);
     func!(Uniform1iARB, (), location: GLint, v0: GLint);
     func!(Uniform1iv, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform1ivARB, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform1ui, (), location: GLint, v0: GLuint);
     func!(Uniform1ui64ARB, (), location: GLint, x: GLuint64);
     func!(Uniform1ui64NV, (), location: GLint, x: GLuint64EXT);
     func!(Uniform1ui64vARB, (), location: GLint, count: GLsizei, value: *const GLuint64);
     func!(Uniform1ui64vNV, (), location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(Uniform1uiEXT, (), location: GLint, v0: GLuint);
     func!(Uniform1uiv, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform1uivEXT, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform2d, (), location: GLint, x: GLdouble, y: GLdouble);
     func!(Uniform2dv, (), location: GLint, count: GLsizei, value: *const GLdouble);
     func!(Uniform2f, (), location: GLint, v0: GLfloat, v1: GLfloat);
     func!(Uniform2fARB, (), location: GLint, v0: GLfloat, v1: GLfloat);
     func!(Uniform2fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform2fvARB, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform2i, (), location: GLint, v0: GLint, v1: GLint);
     func!(Uniform2i64ARB, (), location: GLint, x: GLint64, y: GLint64);
     func!(Uniform2i64NV, (), location: GLint, x: GLint64EXT, y: GLint64EXT);
     func!(Uniform2i64vARB, (), location: GLint, count: GLsizei, value: *const GLint64);
     func!(Uniform2i64vNV, (), location: GLint, count: GLsizei, value: *const GLint64EXT);
     func!(Uniform2iARB, (), location: GLint, v0: GLint, v1: GLint);
     func!(Uniform2iv, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform2ivARB, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform2ui, (), location: GLint, v0: GLuint, v1: GLuint);
     func!(Uniform2ui64ARB, (), location: GLint, x: GLuint64, y: GLuint64);
     func!(Uniform2ui64NV, (), location: GLint, x: GLuint64EXT, y: GLuint64EXT);
     func!(Uniform2ui64vARB, (), location: GLint, count: GLsizei, value: *const GLuint64);
     func!(Uniform2ui64vNV, (), location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(Uniform2uiEXT, (), location: GLint, v0: GLuint, v1: GLuint);
     func!(Uniform2uiv, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform2uivEXT, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform3d, (), location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(Uniform3dv, (), location: GLint, count: GLsizei, value: *const GLdouble);
     func!(Uniform3f, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
     func!(Uniform3fARB, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
     func!(Uniform3fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform3fvARB, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform3i, (), location: GLint, v0: GLint, v1: GLint, v2: GLint);
     func!(Uniform3i64ARB, (), location: GLint, x: GLint64, y: GLint64, z: GLint64);
     func!(Uniform3i64NV, (), location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);
     func!(Uniform3i64vARB, (), location: GLint, count: GLsizei, value: *const GLint64);
     func!(Uniform3i64vNV, (), location: GLint, count: GLsizei, value: *const GLint64EXT);
     func!(Uniform3iARB, (), location: GLint, v0: GLint, v1: GLint, v2: GLint);
     func!(Uniform3iv, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform3ivARB, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform3ui, (), location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
     func!(Uniform3ui64ARB, (), location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);
     func!(Uniform3ui64NV, (), location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);
     func!(Uniform3ui64vARB, (), location: GLint, count: GLsizei, value: *const GLuint64);
     func!(Uniform3ui64vNV, (), location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(Uniform3uiEXT, (), location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
     func!(Uniform3uiv, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform3uivEXT, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform4d, (), location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(Uniform4dv, (), location: GLint, count: GLsizei, value: *const GLdouble);
     func!(Uniform4f, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
     func!(Uniform4fARB, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
     func!(Uniform4fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform4fvARB, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform4i, (), location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
     func!(Uniform4i64ARB, (), location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);
     func!(Uniform4i64NV, (), location: GLint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);
     func!(Uniform4i64vARB, (), location: GLint, count: GLsizei, value: *const GLint64);
     func!(Uniform4i64vNV, (), location: GLint, count: GLsizei, value: *const GLint64EXT);
     func!(Uniform4iARB, (), location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
     func!(Uniform4iv, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform4ivARB, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform4ui, (), location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
     func!(Uniform4ui64ARB, (), location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);
     func!(Uniform4ui64NV, (), location: GLint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);
     func!(Uniform4ui64vARB, (), location: GLint, count: GLsizei, value: *const GLuint64);
     func!(Uniform4ui64vNV, (), location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(Uniform4uiEXT, (), location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
     func!(Uniform4uiv, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform4uivEXT, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(UniformBlockBinding, (), program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);
     func!(UniformBufferEXT, (), program: GLuint, location: GLint, buffer: GLuint);
     func!(UniformHandleui64ARB, (), location: GLint, value: GLuint64);
     func!(UniformHandleui64NV, (), location: GLint, value: GLuint64);
     func!(UniformHandleui64vARB, (), location: GLint, count: GLsizei, value: *const GLuint64);
     func!(UniformHandleui64vNV, (), location: GLint, count: GLsizei, value: *const GLuint64);
     func!(UniformMatrix2dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix2fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix2fvARB, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix2x3dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix2x3fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix2x4dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix2x4fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix3dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix3fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix3fvARB, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix3x2dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix3x2fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix3x4dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix3x4fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix4dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix4fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix4fvARB, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix4x2dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix4x2fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix4x3dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix4x3fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformSubroutinesuiv, (), shadertype: GLenum, count: GLsizei, indices: *const GLuint);
     func!(Uniformui64NV, (), location: GLint, value: GLuint64EXT);
     func!(Uniformui64vNV, (), location: GLint, count: GLsizei, value: *const GLuint64EXT);
     func!(UnlockArraysEXT, (), );
     func!(UnmapBuffer, GLboolean, target: GLenum);
     func!(UnmapBufferARB, GLboolean, target: GLenum);
     func!(UnmapNamedBuffer, GLboolean, buffer: GLuint);
     func!(UnmapNamedBufferEXT, GLboolean, buffer: GLuint);
     func!(UnmapObjectBufferATI, (), buffer: GLuint);
     func!(UnmapTexture2DINTEL, (), texture: GLuint, level: GLint);
     func!(UpdateObjectBufferATI, (), buffer: GLuint, offset: GLuint, size: GLsizei, pointer: *const c_void, preserve: GLenum);
     func!(UploadGpuMaskNVX, (), mask: GLbitfield);
     func!(UseProgram, (), program: GLuint);
     func!(UseProgramObjectARB, (), programObj: GLhandleARB);
     func!(UseProgramStages, (), pipeline: GLuint, stages: GLbitfield, program: GLuint);
     func!(UseShaderProgramEXT, (), type_: GLenum, program: GLuint);
     func!(VDPAUFiniNV, (), );
     func!(VDPAUGetSurfaceivNV, (), surface: GLvdpauSurfaceNV, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);
     func!(VDPAUInitNV, (), vdpDevice: *const c_void, getProcAddress: *const c_void);
     func!(VDPAUIsSurfaceNV, GLboolean, surface: GLvdpauSurfaceNV);
     func!(VDPAUMapSurfacesNV, (), numSurfaces: GLsizei, surfaces: *const GLvdpauSurfaceNV);
     func!(VDPAURegisterOutputSurfaceNV, GLvdpauSurfaceNV, vdpSurface: *const c_void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint);
     func!(VDPAURegisterVideoSurfaceNV, GLvdpauSurfaceNV, vdpSurface: *const c_void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint);
     func!(VDPAURegisterVideoSurfaceWithPictureStructureNV, GLvdpauSurfaceNV, vdpSurface: *const c_void, target: GLenum, numTextureNames: GLsizei, textureNames: *const GLuint, isFrameStructure: GLboolean);
     func!(VDPAUSurfaceAccessNV, (), surface: GLvdpauSurfaceNV, access: GLenum);
     func!(VDPAUUnmapSurfacesNV, (), numSurface: GLsizei, surfaces: *const GLvdpauSurfaceNV);
     func!(VDPAUUnregisterSurfaceNV, (), surface: GLvdpauSurfaceNV);
     func!(ValidateProgram, (), program: GLuint);
     func!(ValidateProgramARB, (), programObj: GLhandleARB);
     func!(ValidateProgramPipeline, (), pipeline: GLuint);
     func!(VariantArrayObjectATI, (), id: GLuint, type_: GLenum, stride: GLsizei, buffer: GLuint, offset: GLuint);
     func!(VariantPointerEXT, (), id: GLuint, type_: GLenum, stride: GLuint, addr: *const c_void);
     func!(VariantbvEXT, (), id: GLuint, addr: *const GLbyte);
     func!(VariantdvEXT, (), id: GLuint, addr: *const GLdouble);
     func!(VariantfvEXT, (), id: GLuint, addr: *const GLfloat);
     func!(VariantivEXT, (), id: GLuint, addr: *const GLint);
     func!(VariantsvEXT, (), id: GLuint, addr: *const GLshort);
     func!(VariantubvEXT, (), id: GLuint, addr: *const GLubyte);
     func!(VariantuivEXT, (), id: GLuint, addr: *const GLuint);
     func!(VariantusvEXT, (), id: GLuint, addr: *const GLushort);
     func!(Vertex2bOES, (), x: GLbyte, y: GLbyte);
     func!(Vertex2bvOES, (), coords: *const GLbyte);
     func!(Vertex2hNV, (), x: GLhalfNV, y: GLhalfNV);
     func!(Vertex2hvNV, (), v: *const GLhalfNV);
     func!(Vertex2xOES, (), x: GLfixed);
     func!(Vertex2xvOES, (), coords: *const GLfixed);
     func!(Vertex3bOES, (), x: GLbyte, y: GLbyte, z: GLbyte);
     func!(Vertex3bvOES, (), coords: *const GLbyte);
     func!(Vertex3hNV, (), x: GLhalfNV, y: GLhalfNV, z: GLhalfNV);
     func!(Vertex3hvNV, (), v: *const GLhalfNV);
     func!(Vertex3xOES, (), x: GLfixed, y: GLfixed);
     func!(Vertex3xvOES, (), coords: *const GLfixed);
     func!(Vertex4bOES, (), x: GLbyte, y: GLbyte, z: GLbyte, w: GLbyte);
     func!(Vertex4bvOES, (), coords: *const GLbyte);
     func!(Vertex4hNV, (), x: GLhalfNV, y: GLhalfNV, z: GLhalfNV, w: GLhalfNV);
     func!(Vertex4hvNV, (), v: *const GLhalfNV);
     func!(Vertex4xOES, (), x: GLfixed, y: GLfixed, z: GLfixed);
     func!(Vertex4xvOES, (), coords: *const GLfixed);
     func!(VertexArrayAttribBinding, (), vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
     func!(VertexArrayAttribFormat, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);
     func!(VertexArrayAttribIFormat, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexArrayAttribLFormat, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexArrayBindVertexBufferEXT, (), vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
     func!(VertexArrayBindingDivisor, (), vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
     func!(VertexArrayColorOffsetEXT, (), vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayEdgeFlagOffsetEXT, (), vaobj: GLuint, buffer: GLuint, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayElementBuffer, (), vaobj: GLuint, buffer: GLuint);
     func!(VertexArrayFogCoordOffsetEXT, (), vaobj: GLuint, buffer: GLuint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayIndexOffsetEXT, (), vaobj: GLuint, buffer: GLuint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayMultiTexCoordOffsetEXT, (), vaobj: GLuint, buffer: GLuint, texunit: GLenum, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayNormalOffsetEXT, (), vaobj: GLuint, buffer: GLuint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayParameteriAPPLE, (), pname: GLenum, param: GLint);
     func!(VertexArrayRangeAPPLE, (), length: GLsizei, pointer: *mut c_void);
     func!(VertexArrayRangeNV, (), length: GLsizei, pointer: *const c_void);
     func!(VertexArraySecondaryColorOffsetEXT, (), vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayTexCoordOffsetEXT, (), vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayVertexAttribBindingEXT, (), vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
     func!(VertexArrayVertexAttribDivisorEXT, (), vaobj: GLuint, index: GLuint, divisor: GLuint);
     func!(VertexArrayVertexAttribFormatEXT, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);
     func!(VertexArrayVertexAttribIFormatEXT, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexArrayVertexAttribIOffsetEXT, (), vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayVertexAttribLFormatEXT, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexArrayVertexAttribLOffsetEXT, (), vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayVertexAttribOffsetEXT, (), vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayVertexBindingDivisorEXT, (), vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
     func!(VertexArrayVertexBuffer, (), vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
     func!(VertexArrayVertexBuffers, (), vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
     func!(VertexArrayVertexOffsetEXT, (), vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexAttrib1d, (), index: GLuint, x: GLdouble);
     func!(VertexAttrib1dARB, (), index: GLuint, x: GLdouble);
     func!(VertexAttrib1dNV, (), index: GLuint, x: GLdouble);
     func!(VertexAttrib1dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib1dvARB, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib1dvNV, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib1f, (), index: GLuint, x: GLfloat);
     func!(VertexAttrib1fARB, (), index: GLuint, x: GLfloat);
     func!(VertexAttrib1fNV, (), index: GLuint, x: GLfloat);
     func!(VertexAttrib1fv, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib1fvARB, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib1fvNV, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib1hNV, (), index: GLuint, x: GLhalfNV);
     func!(VertexAttrib1hvNV, (), index: GLuint, v: *const GLhalfNV);
     func!(VertexAttrib1s, (), index: GLuint, x: GLshort);
     func!(VertexAttrib1sARB, (), index: GLuint, x: GLshort);
     func!(VertexAttrib1sNV, (), index: GLuint, x: GLshort);
     func!(VertexAttrib1sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib1svARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib1svNV, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib2d, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttrib2dARB, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttrib2dNV, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttrib2dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib2dvARB, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib2dvNV, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib2f, (), index: GLuint, x: GLfloat, y: GLfloat);
     func!(VertexAttrib2fARB, (), index: GLuint, x: GLfloat, y: GLfloat);
     func!(VertexAttrib2fNV, (), index: GLuint, x: GLfloat, y: GLfloat);
     func!(VertexAttrib2fv, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib2fvARB, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib2fvNV, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib2hNV, (), index: GLuint, x: GLhalfNV, y: GLhalfNV);
     func!(VertexAttrib2hvNV, (), index: GLuint, v: *const GLhalfNV);
     func!(VertexAttrib2s, (), index: GLuint, x: GLshort, y: GLshort);
     func!(VertexAttrib2sARB, (), index: GLuint, x: GLshort, y: GLshort);
     func!(VertexAttrib2sNV, (), index: GLuint, x: GLshort, y: GLshort);
     func!(VertexAttrib2sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib2svARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib2svNV, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib3d, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttrib3dARB, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttrib3dNV, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttrib3dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib3dvARB, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib3dvNV, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib3f, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(VertexAttrib3fARB, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(VertexAttrib3fNV, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(VertexAttrib3fv, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib3fvARB, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib3fvNV, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib3hNV, (), index: GLuint, x: GLhalfNV, y: GLhalfNV, z: GLhalfNV);
     func!(VertexAttrib3hvNV, (), index: GLuint, v: *const GLhalfNV);
     func!(VertexAttrib3s, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort);
     func!(VertexAttrib3sARB, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort);
     func!(VertexAttrib3sNV, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort);
     func!(VertexAttrib3sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib3svARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib3svNV, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4Nbv, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttrib4NbvARB, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttrib4Niv, (), index: GLuint, v: *const GLint);
     func!(VertexAttrib4NivARB, (), index: GLuint, v: *const GLint);
     func!(VertexAttrib4Nsv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4NsvARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4Nub, (), index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
     func!(VertexAttrib4NubARB, (), index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
     func!(VertexAttrib4Nubv, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4NubvARB, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4Nuiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttrib4NuivARB, (), index: GLuint, v: *const GLuint);
     func!(VertexAttrib4Nusv, (), index: GLuint, v: *const GLushort);
     func!(VertexAttrib4NusvARB, (), index: GLuint, v: *const GLushort);
     func!(VertexAttrib4bv, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttrib4bvARB, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttrib4d, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttrib4dARB, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttrib4dNV, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttrib4dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib4dvARB, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib4dvNV, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib4f, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(VertexAttrib4fARB, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(VertexAttrib4fNV, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(VertexAttrib4fv, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib4fvARB, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib4fvNV, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib4hNV, (), index: GLuint, x: GLhalfNV, y: GLhalfNV, z: GLhalfNV, w: GLhalfNV);
     func!(VertexAttrib4hvNV, (), index: GLuint, v: *const GLhalfNV);
     func!(VertexAttrib4iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttrib4ivARB, (), index: GLuint, v: *const GLint);
     func!(VertexAttrib4s, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
     func!(VertexAttrib4sARB, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
     func!(VertexAttrib4sNV, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
     func!(VertexAttrib4sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4svARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4svNV, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4ubNV, (), index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
     func!(VertexAttrib4ubv, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4ubvARB, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4ubvNV, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttrib4uivARB, (), index: GLuint, v: *const GLuint);
     func!(VertexAttrib4usv, (), index: GLuint, v: *const GLushort);
     func!(VertexAttrib4usvARB, (), index: GLuint, v: *const GLushort);
     func!(VertexAttribArrayObjectATI, (), index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, buffer: GLuint, offset: GLuint);
     func!(VertexAttribBinding, (), attribindex: GLuint, bindingindex: GLuint);
     func!(VertexAttribDivisor, (), index: GLuint, divisor: GLuint);
     func!(VertexAttribDivisorARB, (), index: GLuint, divisor: GLuint);
     func!(VertexAttribFormat, (), attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);
     func!(VertexAttribFormatNV, (), index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei);
     func!(VertexAttribI1i, (), index: GLuint, x: GLint);
     func!(VertexAttribI1iEXT, (), index: GLuint, x: GLint);
     func!(VertexAttribI1iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI1ivEXT, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI1ui, (), index: GLuint, x: GLuint);
     func!(VertexAttribI1uiEXT, (), index: GLuint, x: GLuint);
     func!(VertexAttribI1uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI1uivEXT, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI2i, (), index: GLuint, x: GLint, y: GLint);
     func!(VertexAttribI2iEXT, (), index: GLuint, x: GLint, y: GLint);
     func!(VertexAttribI2iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI2ivEXT, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI2ui, (), index: GLuint, x: GLuint, y: GLuint);
     func!(VertexAttribI2uiEXT, (), index: GLuint, x: GLuint, y: GLuint);
     func!(VertexAttribI2uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI2uivEXT, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI3i, (), index: GLuint, x: GLint, y: GLint, z: GLint);
     func!(VertexAttribI3iEXT, (), index: GLuint, x: GLint, y: GLint, z: GLint);
     func!(VertexAttribI3iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI3ivEXT, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI3ui, (), index: GLuint, x: GLuint, y: GLuint, z: GLuint);
     func!(VertexAttribI3uiEXT, (), index: GLuint, x: GLuint, y: GLuint, z: GLuint);
     func!(VertexAttribI3uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI3uivEXT, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI4bv, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttribI4bvEXT, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttribI4i, (), index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
     func!(VertexAttribI4iEXT, (), index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
     func!(VertexAttribI4iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI4ivEXT, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI4sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttribI4svEXT, (), index: GLuint, v: *const GLshort);
     func!(VertexAttribI4ubv, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttribI4ubvEXT, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttribI4ui, (), index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
     func!(VertexAttribI4uiEXT, (), index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
     func!(VertexAttribI4uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI4uivEXT, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI4usv, (), index: GLuint, v: *const GLushort);
     func!(VertexAttribI4usvEXT, (), index: GLuint, v: *const GLushort);
     func!(VertexAttribIFormat, (), attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexAttribIFormatNV, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei);
     func!(VertexAttribIPointer, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribIPointerEXT, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribL1d, (), index: GLuint, x: GLdouble);
     func!(VertexAttribL1dEXT, (), index: GLuint, x: GLdouble);
     func!(VertexAttribL1dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL1dvEXT, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL1i64NV, (), index: GLuint, x: GLint64EXT);
     func!(VertexAttribL1i64vNV, (), index: GLuint, v: *const GLint64EXT);
     func!(VertexAttribL1ui64ARB, (), index: GLuint, x: GLuint64EXT);
     func!(VertexAttribL1ui64NV, (), index: GLuint, x: GLuint64EXT);
     func!(VertexAttribL1ui64vARB, (), index: GLuint, v: *const GLuint64EXT);
     func!(VertexAttribL1ui64vNV, (), index: GLuint, v: *const GLuint64EXT);
     func!(VertexAttribL2d, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttribL2dEXT, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttribL2dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL2dvEXT, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL2i64NV, (), index: GLuint, x: GLint64EXT, y: GLint64EXT);
     func!(VertexAttribL2i64vNV, (), index: GLuint, v: *const GLint64EXT);
     func!(VertexAttribL2ui64NV, (), index: GLuint, x: GLuint64EXT, y: GLuint64EXT);
     func!(VertexAttribL2ui64vNV, (), index: GLuint, v: *const GLuint64EXT);
     func!(VertexAttribL3d, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttribL3dEXT, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttribL3dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL3dvEXT, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL3i64NV, (), index: GLuint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT);
     func!(VertexAttribL3i64vNV, (), index: GLuint, v: *const GLint64EXT);
     func!(VertexAttribL3ui64NV, (), index: GLuint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT);
     func!(VertexAttribL3ui64vNV, (), index: GLuint, v: *const GLuint64EXT);
     func!(VertexAttribL4d, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttribL4dEXT, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttribL4dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL4dvEXT, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL4i64NV, (), index: GLuint, x: GLint64EXT, y: GLint64EXT, z: GLint64EXT, w: GLint64EXT);
     func!(VertexAttribL4i64vNV, (), index: GLuint, v: *const GLint64EXT);
     func!(VertexAttribL4ui64NV, (), index: GLuint, x: GLuint64EXT, y: GLuint64EXT, z: GLuint64EXT, w: GLuint64EXT);
     func!(VertexAttribL4ui64vNV, (), index: GLuint, v: *const GLuint64EXT);
     func!(VertexAttribLFormat, (), attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexAttribLFormatNV, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei);
     func!(VertexAttribLPointer, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribLPointerEXT, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribP1ui, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
     func!(VertexAttribP1uiv, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
     func!(VertexAttribP2ui, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
     func!(VertexAttribP2uiv, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
     func!(VertexAttribP3ui, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
     func!(VertexAttribP3uiv, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
     func!(VertexAttribP4ui, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
     func!(VertexAttribP4uiv, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
     func!(VertexAttribParameteriAMD, (), index: GLuint, pname: GLenum, param: GLint);
     func!(VertexAttribPointer, (), index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribPointerARB, (), index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribPointerNV, (), index: GLuint, fsize: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribs1dvNV, (), index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(VertexAttribs1fvNV, (), index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(VertexAttribs1hvNV, (), index: GLuint, n: GLsizei, v: *const GLhalfNV);
     func!(VertexAttribs1svNV, (), index: GLuint, count: GLsizei, v: *const GLshort);
     func!(VertexAttribs2dvNV, (), index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(VertexAttribs2fvNV, (), index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(VertexAttribs2hvNV, (), index: GLuint, n: GLsizei, v: *const GLhalfNV);
     func!(VertexAttribs2svNV, (), index: GLuint, count: GLsizei, v: *const GLshort);
     func!(VertexAttribs3dvNV, (), index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(VertexAttribs3fvNV, (), index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(VertexAttribs3hvNV, (), index: GLuint, n: GLsizei, v: *const GLhalfNV);
     func!(VertexAttribs3svNV, (), index: GLuint, count: GLsizei, v: *const GLshort);
     func!(VertexAttribs4dvNV, (), index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(VertexAttribs4fvNV, (), index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(VertexAttribs4hvNV, (), index: GLuint, n: GLsizei, v: *const GLhalfNV);
     func!(VertexAttribs4svNV, (), index: GLuint, count: GLsizei, v: *const GLshort);
     func!(VertexAttribs4ubvNV, (), index: GLuint, count: GLsizei, v: *const GLubyte);
     func!(VertexBindingDivisor, (), bindingindex: GLuint, divisor: GLuint);
     func!(VertexBlendARB, (), count: GLint);
     func!(VertexBlendEnvfATI, (), pname: GLenum, param: GLfloat);
     func!(VertexBlendEnviATI, (), pname: GLenum, param: GLint);
     func!(VertexFormatNV, (), size: GLint, type_: GLenum, stride: GLsizei);
     func!(VertexPointerEXT, (), size: GLint, type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(VertexPointerListIBM, (), size: GLint, type_: GLenum, stride: GLint, pointer: *const *const c_void, ptrstride: GLint);
     func!(VertexPointervINTEL, (), size: GLint, type_: GLenum, pointer: *const *const c_void);
     func!(VertexStream1dATI, (), stream: GLenum, x: GLdouble);
     func!(VertexStream1dvATI, (), stream: GLenum, coords: *const GLdouble);
     func!(VertexStream1fATI, (), stream: GLenum, x: GLfloat);
     func!(VertexStream1fvATI, (), stream: GLenum, coords: *const GLfloat);
     func!(VertexStream1iATI, (), stream: GLenum, x: GLint);
     func!(VertexStream1ivATI, (), stream: GLenum, coords: *const GLint);
     func!(VertexStream1sATI, (), stream: GLenum, x: GLshort);
     func!(VertexStream1svATI, (), stream: GLenum, coords: *const GLshort);
     func!(VertexStream2dATI, (), stream: GLenum, x: GLdouble, y: GLdouble);
     func!(VertexStream2dvATI, (), stream: GLenum, coords: *const GLdouble);
     func!(VertexStream2fATI, (), stream: GLenum, x: GLfloat, y: GLfloat);
     func!(VertexStream2fvATI, (), stream: GLenum, coords: *const GLfloat);
     func!(VertexStream2iATI, (), stream: GLenum, x: GLint, y: GLint);
     func!(VertexStream2ivATI, (), stream: GLenum, coords: *const GLint);
     func!(VertexStream2sATI, (), stream: GLenum, x: GLshort, y: GLshort);
     func!(VertexStream2svATI, (), stream: GLenum, coords: *const GLshort);
     func!(VertexStream3dATI, (), stream: GLenum, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexStream3dvATI, (), stream: GLenum, coords: *const GLdouble);
     func!(VertexStream3fATI, (), stream: GLenum, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(VertexStream3fvATI, (), stream: GLenum, coords: *const GLfloat);
     func!(VertexStream3iATI, (), stream: GLenum, x: GLint, y: GLint, z: GLint);
     func!(VertexStream3ivATI, (), stream: GLenum, coords: *const GLint);
     func!(VertexStream3sATI, (), stream: GLenum, x: GLshort, y: GLshort, z: GLshort);
     func!(VertexStream3svATI, (), stream: GLenum, coords: *const GLshort);
     func!(VertexStream4dATI, (), stream: GLenum, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexStream4dvATI, (), stream: GLenum, coords: *const GLdouble);
     func!(VertexStream4fATI, (), stream: GLenum, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(VertexStream4fvATI, (), stream: GLenum, coords: *const GLfloat);
     func!(VertexStream4iATI, (), stream: GLenum, x: GLint, y: GLint, z: GLint, w: GLint);
     func!(VertexStream4ivATI, (), stream: GLenum, coords: *const GLint);
     func!(VertexStream4sATI, (), stream: GLenum, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
     func!(VertexStream4svATI, (), stream: GLenum, coords: *const GLshort);
     func!(VertexWeightPointerEXT, (), size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexWeightfEXT, (), weight: GLfloat);
     func!(VertexWeightfvEXT, (), weight: *const GLfloat);
     func!(VertexWeighthNV, (), weight: GLhalfNV);
     func!(VertexWeighthvNV, (), weight: *const GLhalfNV);
     func!(VideoCaptureNV, GLenum, video_capture_slot: GLuint, sequence_num: *mut GLuint, capture_time: *mut GLuint64EXT);
     func!(VideoCaptureStreamParameterdvNV, (), video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLdouble);
     func!(VideoCaptureStreamParameterfvNV, (), video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLfloat);
     func!(VideoCaptureStreamParameterivNV, (), video_capture_slot: GLuint, stream: GLuint, pname: GLenum, params: *const GLint);
     func!(Viewport, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(ViewportArrayv, (), first: GLuint, count: GLsizei, v: *const GLfloat);
     func!(ViewportIndexedf, (), index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);
     func!(ViewportIndexedfv, (), index: GLuint, v: *const GLfloat);
     func!(ViewportPositionWScaleNV, (), index: GLuint, xcoeff: GLfloat, ycoeff: GLfloat);
     func!(ViewportSwizzleNV, (), index: GLuint, swizzlex: GLenum, swizzley: GLenum, swizzlez: GLenum, swizzlew: GLenum);
     func!(WaitSemaphoreEXT, (), semaphore: GLuint, numBufferBarriers: GLuint, buffers: *const GLuint, numTextureBarriers: GLuint, textures: *const GLuint, srcLayouts: *const GLenum);
     func!(WaitSemaphoreui64NVX, (), waitGpu: GLuint, fenceObjectCount: GLsizei, semaphoreArray: *const GLuint, fenceValueArray: *const GLuint64);
     func!(WaitSync, (), sync: GLsync, flags: GLbitfield, timeout: GLuint64);
     func!(WaitVkSemaphoreNV, (), vkSemaphore: GLuint64);
     func!(WeightPathsNV, (), resultPath: GLuint, numPaths: GLsizei, paths: *const GLuint, weights: *const GLfloat);
     func!(WeightPointerARB, (), size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(WeightbvARB, (), size: GLint, weights: *const GLbyte);
     func!(WeightdvARB, (), size: GLint, weights: *const GLdouble);
     func!(WeightfvARB, (), size: GLint, weights: *const GLfloat);
     func!(WeightivARB, (), size: GLint, weights: *const GLint);
     func!(WeightsvARB, (), size: GLint, weights: *const GLshort);
     func!(WeightubvARB, (), size: GLint, weights: *const GLubyte);
     func!(WeightuivARB, (), size: GLint, weights: *const GLuint);
     func!(WeightusvARB, (), size: GLint, weights: *const GLushort);
     func!(WindowPos2dARB, (), x: GLdouble, y: GLdouble);
     func!(WindowPos2dMESA, (), x: GLdouble, y: GLdouble);
     func!(WindowPos2dvARB, (), v: *const GLdouble);
     func!(WindowPos2dvMESA, (), v: *const GLdouble);
     func!(WindowPos2fARB, (), x: GLfloat, y: GLfloat);
     func!(WindowPos2fMESA, (), x: GLfloat, y: GLfloat);
     func!(WindowPos2fvARB, (), v: *const GLfloat);
     func!(WindowPos2fvMESA, (), v: *const GLfloat);
     func!(WindowPos2iARB, (), x: GLint, y: GLint);
     func!(WindowPos2iMESA, (), x: GLint, y: GLint);
     func!(WindowPos2ivARB, (), v: *const GLint);
     func!(WindowPos2ivMESA, (), v: *const GLint);
     func!(WindowPos2sARB, (), x: GLshort, y: GLshort);
     func!(WindowPos2sMESA, (), x: GLshort, y: GLshort);
     func!(WindowPos2svARB, (), v: *const GLshort);
     func!(WindowPos2svMESA, (), v: *const GLshort);
     func!(WindowPos3dARB, (), x: GLdouble, y: GLdouble, z: GLdouble);
     func!(WindowPos3dMESA, (), x: GLdouble, y: GLdouble, z: GLdouble);
     func!(WindowPos3dvARB, (), v: *const GLdouble);
     func!(WindowPos3dvMESA, (), v: *const GLdouble);
     func!(WindowPos3fARB, (), x: GLfloat, y: GLfloat, z: GLfloat);
     func!(WindowPos3fMESA, (), x: GLfloat, y: GLfloat, z: GLfloat);
     func!(WindowPos3fvARB, (), v: *const GLfloat);
     func!(WindowPos3fvMESA, (), v: *const GLfloat);
     func!(WindowPos3iARB, (), x: GLint, y: GLint, z: GLint);
     func!(WindowPos3iMESA, (), x: GLint, y: GLint, z: GLint);
     func!(WindowPos3ivARB, (), v: *const GLint);
     func!(WindowPos3ivMESA, (), v: *const GLint);
     func!(WindowPos3sARB, (), x: GLshort, y: GLshort, z: GLshort);
     func!(WindowPos3sMESA, (), x: GLshort, y: GLshort, z: GLshort);
     func!(WindowPos3svARB, (), v: *const GLshort);
     func!(WindowPos3svMESA, (), v: *const GLshort);
     func!(WindowPos4dMESA, (), x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(WindowPos4dvMESA, (), v: *const GLdouble);
     func!(WindowPos4fMESA, (), x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(WindowPos4fvMESA, (), v: *const GLfloat);
     func!(WindowPos4iMESA, (), x: GLint, y: GLint, z: GLint, w: GLint);
     func!(WindowPos4ivMESA, (), v: *const GLint);
     func!(WindowPos4sMESA, (), x: GLshort, y: GLshort, z: GLshort, w: GLshort);
     func!(WindowPos4svMESA, (), v: *const GLshort);
     func!(WindowRectanglesEXT, (), mode: GLenum, count: GLsizei, box_: *const GLint);
     func!(WriteMaskEXT, (), res: GLuint, in_: GLuint, outX: GLenum, outY: GLenum, outZ: GLenum, outW: GLenum);

    }
}


pub fn load<F>(mut loadfn: F) -> functions::Gl where F: FnMut(&'static str) -> *const c_void {
    #[allow(unused_mut)]
    let mut ctx = Gl {
         AccumxOES: FnPtr::new(loadfn("glAccumxOES")),
         AcquireKeyedMutexWin32EXT: FnPtr::new(loadfn("glAcquireKeyedMutexWin32EXT")),
         ActiveProgramEXT: FnPtr::new(loadfn("glActiveProgramEXT")),
         ActiveShaderProgram: FnPtr::new(loadfn("glActiveShaderProgram")),
         ActiveStencilFaceEXT: FnPtr::new(loadfn("glActiveStencilFaceEXT")),
         ActiveTexture: FnPtr::new(loadfn("glActiveTexture")),
         ActiveTextureARB: FnPtr::new(loadfn("glActiveTextureARB")),
         ActiveVaryingNV: FnPtr::new(loadfn("glActiveVaryingNV")),
         AlphaFragmentOp1ATI: FnPtr::new(loadfn("glAlphaFragmentOp1ATI")),
         AlphaFragmentOp2ATI: FnPtr::new(loadfn("glAlphaFragmentOp2ATI")),
         AlphaFragmentOp3ATI: FnPtr::new(loadfn("glAlphaFragmentOp3ATI")),
         AlphaFuncxOES: FnPtr::new(loadfn("glAlphaFuncxOES")),
         AlphaToCoverageDitherControlNV: FnPtr::new(loadfn("glAlphaToCoverageDitherControlNV")),
         ApplyFramebufferAttachmentCMAAINTEL: FnPtr::new(loadfn("glApplyFramebufferAttachmentCMAAINTEL")),
         ApplyTextureEXT: FnPtr::new(loadfn("glApplyTextureEXT")),
         AreProgramsResidentNV: FnPtr::new(loadfn("glAreProgramsResidentNV")),
         AreTexturesResidentEXT: FnPtr::new(loadfn("glAreTexturesResidentEXT")),
         ArrayElementEXT: FnPtr::new(loadfn("glArrayElementEXT")),
         ArrayObjectATI: FnPtr::new(loadfn("glArrayObjectATI")),
         AsyncCopyBufferSubDataNVX: FnPtr::new(loadfn("glAsyncCopyBufferSubDataNVX")),
         AsyncCopyImageSubDataNVX: FnPtr::new(loadfn("glAsyncCopyImageSubDataNVX")),
         AsyncMarkerSGIX: FnPtr::new(loadfn("glAsyncMarkerSGIX")),
         AttachObjectARB: FnPtr::new(loadfn("glAttachObjectARB")),
         AttachShader: FnPtr::new(loadfn("glAttachShader")),
         BeginConditionalRender: FnPtr::new(loadfn("glBeginConditionalRender")),
         BeginConditionalRenderNV: FnPtr::new(loadfn("glBeginConditionalRenderNV")),
         BeginConditionalRenderNVX: FnPtr::new(loadfn("glBeginConditionalRenderNVX")),
         BeginFragmentShaderATI: FnPtr::new(loadfn("glBeginFragmentShaderATI")),
         BeginOcclusionQueryNV: FnPtr::new(loadfn("glBeginOcclusionQueryNV")),
         BeginPerfMonitorAMD: FnPtr::new(loadfn("glBeginPerfMonitorAMD")),
         BeginPerfQueryINTEL: FnPtr::new(loadfn("glBeginPerfQueryINTEL")),
         BeginQuery: FnPtr::new(loadfn("glBeginQuery")),
         BeginQueryARB: FnPtr::new(loadfn("glBeginQueryARB")),
         BeginQueryIndexed: FnPtr::new(loadfn("glBeginQueryIndexed")),
         BeginTransformFeedback: FnPtr::new(loadfn("glBeginTransformFeedback")),
         BeginTransformFeedbackEXT: FnPtr::new(loadfn("glBeginTransformFeedbackEXT")),
         BeginTransformFeedbackNV: FnPtr::new(loadfn("glBeginTransformFeedbackNV")),
         BeginVertexShaderEXT: FnPtr::new(loadfn("glBeginVertexShaderEXT")),
         BeginVideoCaptureNV: FnPtr::new(loadfn("glBeginVideoCaptureNV")),
         BindAttribLocation: FnPtr::new(loadfn("glBindAttribLocation")),
         BindAttribLocationARB: FnPtr::new(loadfn("glBindAttribLocationARB")),
         BindBuffer: FnPtr::new(loadfn("glBindBuffer")),
         BindBufferARB: FnPtr::new(loadfn("glBindBufferARB")),
         BindBufferBase: FnPtr::new(loadfn("glBindBufferBase")),
         BindBufferBaseEXT: FnPtr::new(loadfn("glBindBufferBaseEXT")),
         BindBufferBaseNV: FnPtr::new(loadfn("glBindBufferBaseNV")),
         BindBufferOffsetEXT: FnPtr::new(loadfn("glBindBufferOffsetEXT")),
         BindBufferOffsetNV: FnPtr::new(loadfn("glBindBufferOffsetNV")),
         BindBufferRange: FnPtr::new(loadfn("glBindBufferRange")),
         BindBufferRangeEXT: FnPtr::new(loadfn("glBindBufferRangeEXT")),
         BindBufferRangeNV: FnPtr::new(loadfn("glBindBufferRangeNV")),
         BindBuffersBase: FnPtr::new(loadfn("glBindBuffersBase")),
         BindBuffersRange: FnPtr::new(loadfn("glBindBuffersRange")),
         BindFragDataLocation: FnPtr::new(loadfn("glBindFragDataLocation")),
         BindFragDataLocationEXT: FnPtr::new(loadfn("glBindFragDataLocationEXT")),
         BindFragDataLocationIndexed: FnPtr::new(loadfn("glBindFragDataLocationIndexed")),
         BindFragmentShaderATI: FnPtr::new(loadfn("glBindFragmentShaderATI")),
         BindFramebuffer: FnPtr::new(loadfn("glBindFramebuffer")),
         BindFramebufferEXT: FnPtr::new(loadfn("glBindFramebufferEXT")),
         BindImageTexture: FnPtr::new(loadfn("glBindImageTexture")),
         BindImageTextureEXT: FnPtr::new(loadfn("glBindImageTextureEXT")),
         BindImageTextures: FnPtr::new(loadfn("glBindImageTextures")),
         BindLightParameterEXT: FnPtr::new(loadfn("glBindLightParameterEXT")),
         BindMaterialParameterEXT: FnPtr::new(loadfn("glBindMaterialParameterEXT")),
         BindMultiTextureEXT: FnPtr::new(loadfn("glBindMultiTextureEXT")),
         BindParameterEXT: FnPtr::new(loadfn("glBindParameterEXT")),
         BindProgramARB: FnPtr::new(loadfn("glBindProgramARB")),
         BindProgramNV: FnPtr::new(loadfn("glBindProgramNV")),
         BindProgramPipeline: FnPtr::new(loadfn("glBindProgramPipeline")),
         BindRenderbuffer: FnPtr::new(loadfn("glBindRenderbuffer")),
         BindRenderbufferEXT: FnPtr::new(loadfn("glBindRenderbufferEXT")),
         BindSampler: FnPtr::new(loadfn("glBindSampler")),
         BindSamplers: FnPtr::new(loadfn("glBindSamplers")),
         BindShadingRateImageNV: FnPtr::new(loadfn("glBindShadingRateImageNV")),
         BindTexGenParameterEXT: FnPtr::new(loadfn("glBindTexGenParameterEXT")),
         BindTexture: FnPtr::new(loadfn("glBindTexture")),
         BindTextureEXT: FnPtr::new(loadfn("glBindTextureEXT")),
         BindTextureUnit: FnPtr::new(loadfn("glBindTextureUnit")),
         BindTextureUnitParameterEXT: FnPtr::new(loadfn("glBindTextureUnitParameterEXT")),
         BindTextures: FnPtr::new(loadfn("glBindTextures")),
         BindTransformFeedback: FnPtr::new(loadfn("glBindTransformFeedback")),
         BindTransformFeedbackNV: FnPtr::new(loadfn("glBindTransformFeedbackNV")),
         BindVertexArray: FnPtr::new(loadfn("glBindVertexArray")),
         BindVertexArrayAPPLE: FnPtr::new(loadfn("glBindVertexArrayAPPLE")),
         BindVertexBuffer: FnPtr::new(loadfn("glBindVertexBuffer")),
         BindVertexBuffers: FnPtr::new(loadfn("glBindVertexBuffers")),
         BindVertexShaderEXT: FnPtr::new(loadfn("glBindVertexShaderEXT")),
         BindVideoCaptureStreamBufferNV: FnPtr::new(loadfn("glBindVideoCaptureStreamBufferNV")),
         BindVideoCaptureStreamTextureNV: FnPtr::new(loadfn("glBindVideoCaptureStreamTextureNV")),
         Binormal3bEXT: FnPtr::new(loadfn("glBinormal3bEXT")),
         Binormal3bvEXT: FnPtr::new(loadfn("glBinormal3bvEXT")),
         Binormal3dEXT: FnPtr::new(loadfn("glBinormal3dEXT")),
         Binormal3dvEXT: FnPtr::new(loadfn("glBinormal3dvEXT")),
         Binormal3fEXT: FnPtr::new(loadfn("glBinormal3fEXT")),
         Binormal3fvEXT: FnPtr::new(loadfn("glBinormal3fvEXT")),
         Binormal3iEXT: FnPtr::new(loadfn("glBinormal3iEXT")),
         Binormal3ivEXT: FnPtr::new(loadfn("glBinormal3ivEXT")),
         Binormal3sEXT: FnPtr::new(loadfn("glBinormal3sEXT")),
         Binormal3svEXT: FnPtr::new(loadfn("glBinormal3svEXT")),
         BinormalPointerEXT: FnPtr::new(loadfn("glBinormalPointerEXT")),
         BitmapxOES: FnPtr::new(loadfn("glBitmapxOES")),
         BlendBarrierKHR: FnPtr::new(loadfn("glBlendBarrierKHR")),
         BlendBarrierNV: FnPtr::new(loadfn("glBlendBarrierNV")),
         BlendColor: FnPtr::new(loadfn("glBlendColor")),
         BlendColorEXT: FnPtr::new(loadfn("glBlendColorEXT")),
         BlendColorxOES: FnPtr::new(loadfn("glBlendColorxOES")),
         BlendEquation: FnPtr::new(loadfn("glBlendEquation")),
         BlendEquationEXT: FnPtr::new(loadfn("glBlendEquationEXT")),
         BlendEquationIndexedAMD: FnPtr::new(loadfn("glBlendEquationIndexedAMD")),
         BlendEquationSeparate: FnPtr::new(loadfn("glBlendEquationSeparate")),
         BlendEquationSeparateEXT: FnPtr::new(loadfn("glBlendEquationSeparateEXT")),
         BlendEquationSeparateIndexedAMD: FnPtr::new(loadfn("glBlendEquationSeparateIndexedAMD")),
         BlendEquationSeparatei: FnPtr::new(loadfn("glBlendEquationSeparatei")),
         BlendEquationSeparateiARB: FnPtr::new(loadfn("glBlendEquationSeparateiARB")),
         BlendEquationi: FnPtr::new(loadfn("glBlendEquationi")),
         BlendEquationiARB: FnPtr::new(loadfn("glBlendEquationiARB")),
         BlendFunc: FnPtr::new(loadfn("glBlendFunc")),
         BlendFuncIndexedAMD: FnPtr::new(loadfn("glBlendFuncIndexedAMD")),
         BlendFuncSeparate: FnPtr::new(loadfn("glBlendFuncSeparate")),
         BlendFuncSeparateEXT: FnPtr::new(loadfn("glBlendFuncSeparateEXT")),
         BlendFuncSeparateINGR: FnPtr::new(loadfn("glBlendFuncSeparateINGR")),
         BlendFuncSeparateIndexedAMD: FnPtr::new(loadfn("glBlendFuncSeparateIndexedAMD")),
         BlendFuncSeparatei: FnPtr::new(loadfn("glBlendFuncSeparatei")),
         BlendFuncSeparateiARB: FnPtr::new(loadfn("glBlendFuncSeparateiARB")),
         BlendFunci: FnPtr::new(loadfn("glBlendFunci")),
         BlendFunciARB: FnPtr::new(loadfn("glBlendFunciARB")),
         BlendParameteriNV: FnPtr::new(loadfn("glBlendParameteriNV")),
         BlitFramebuffer: FnPtr::new(loadfn("glBlitFramebuffer")),
         BlitFramebufferEXT: FnPtr::new(loadfn("glBlitFramebufferEXT")),
         BlitFramebufferLayerEXT: FnPtr::new(loadfn("glBlitFramebufferLayerEXT")),
         BlitFramebufferLayersEXT: FnPtr::new(loadfn("glBlitFramebufferLayersEXT")),
         BlitNamedFramebuffer: FnPtr::new(loadfn("glBlitNamedFramebuffer")),
         BufferAddressRangeNV: FnPtr::new(loadfn("glBufferAddressRangeNV")),
         BufferAttachMemoryNV: FnPtr::new(loadfn("glBufferAttachMemoryNV")),
         BufferData: FnPtr::new(loadfn("glBufferData")),
         BufferDataARB: FnPtr::new(loadfn("glBufferDataARB")),
         BufferPageCommitmentARB: FnPtr::new(loadfn("glBufferPageCommitmentARB")),
         BufferPageCommitmentMemNV: FnPtr::new(loadfn("glBufferPageCommitmentMemNV")),
         BufferParameteriAPPLE: FnPtr::new(loadfn("glBufferParameteriAPPLE")),
         BufferStorage: FnPtr::new(loadfn("glBufferStorage")),
         BufferStorageExternalEXT: FnPtr::new(loadfn("glBufferStorageExternalEXT")),
         BufferStorageMemEXT: FnPtr::new(loadfn("glBufferStorageMemEXT")),
         BufferSubData: FnPtr::new(loadfn("glBufferSubData")),
         BufferSubDataARB: FnPtr::new(loadfn("glBufferSubDataARB")),
         CallCommandListNV: FnPtr::new(loadfn("glCallCommandListNV")),
         CheckFramebufferStatus: FnPtr::new(loadfn("glCheckFramebufferStatus")),
         CheckFramebufferStatusEXT: FnPtr::new(loadfn("glCheckFramebufferStatusEXT")),
         CheckNamedFramebufferStatus: FnPtr::new(loadfn("glCheckNamedFramebufferStatus")),
         CheckNamedFramebufferStatusEXT: FnPtr::new(loadfn("glCheckNamedFramebufferStatusEXT")),
         ClampColor: FnPtr::new(loadfn("glClampColor")),
         ClampColorARB: FnPtr::new(loadfn("glClampColorARB")),
         Clear: FnPtr::new(loadfn("glClear")),
         ClearAccumxOES: FnPtr::new(loadfn("glClearAccumxOES")),
         ClearBufferData: FnPtr::new(loadfn("glClearBufferData")),
         ClearBufferSubData: FnPtr::new(loadfn("glClearBufferSubData")),
         ClearBufferfi: FnPtr::new(loadfn("glClearBufferfi")),
         ClearBufferfv: FnPtr::new(loadfn("glClearBufferfv")),
         ClearBufferiv: FnPtr::new(loadfn("glClearBufferiv")),
         ClearBufferuiv: FnPtr::new(loadfn("glClearBufferuiv")),
         ClearColor: FnPtr::new(loadfn("glClearColor")),
         ClearColorIiEXT: FnPtr::new(loadfn("glClearColorIiEXT")),
         ClearColorIuiEXT: FnPtr::new(loadfn("glClearColorIuiEXT")),
         ClearColorxOES: FnPtr::new(loadfn("glClearColorxOES")),
         ClearDepth: FnPtr::new(loadfn("glClearDepth")),
         ClearDepthdNV: FnPtr::new(loadfn("glClearDepthdNV")),
         ClearDepthf: FnPtr::new(loadfn("glClearDepthf")),
         ClearDepthfOES: FnPtr::new(loadfn("glClearDepthfOES")),
         ClearDepthxOES: FnPtr::new(loadfn("glClearDepthxOES")),
         ClearNamedBufferData: FnPtr::new(loadfn("glClearNamedBufferData")),
         ClearNamedBufferDataEXT: FnPtr::new(loadfn("glClearNamedBufferDataEXT")),
         ClearNamedBufferSubData: FnPtr::new(loadfn("glClearNamedBufferSubData")),
         ClearNamedBufferSubDataEXT: FnPtr::new(loadfn("glClearNamedBufferSubDataEXT")),
         ClearNamedFramebufferfi: FnPtr::new(loadfn("glClearNamedFramebufferfi")),
         ClearNamedFramebufferfv: FnPtr::new(loadfn("glClearNamedFramebufferfv")),
         ClearNamedFramebufferiv: FnPtr::new(loadfn("glClearNamedFramebufferiv")),
         ClearNamedFramebufferuiv: FnPtr::new(loadfn("glClearNamedFramebufferuiv")),
         ClearStencil: FnPtr::new(loadfn("glClearStencil")),
         ClearTexImage: FnPtr::new(loadfn("glClearTexImage")),
         ClearTexSubImage: FnPtr::new(loadfn("glClearTexSubImage")),
         ClientActiveTextureARB: FnPtr::new(loadfn("glClientActiveTextureARB")),
         ClientActiveVertexStreamATI: FnPtr::new(loadfn("glClientActiveVertexStreamATI")),
         ClientAttribDefaultEXT: FnPtr::new(loadfn("glClientAttribDefaultEXT")),
         ClientWaitSemaphoreui64NVX: FnPtr::new(loadfn("glClientWaitSemaphoreui64NVX")),
         ClientWaitSync: FnPtr::new(loadfn("glClientWaitSync")),
         ClipControl: FnPtr::new(loadfn("glClipControl")),
         ClipPlanefOES: FnPtr::new(loadfn("glClipPlanefOES")),
         ClipPlanexOES: FnPtr::new(loadfn("glClipPlanexOES")),
         Color3fVertex3fSUN: FnPtr::new(loadfn("glColor3fVertex3fSUN")),
         Color3fVertex3fvSUN: FnPtr::new(loadfn("glColor3fVertex3fvSUN")),
         Color3hNV: FnPtr::new(loadfn("glColor3hNV")),
         Color3hvNV: FnPtr::new(loadfn("glColor3hvNV")),
         Color3xOES: FnPtr::new(loadfn("glColor3xOES")),
         Color3xvOES: FnPtr::new(loadfn("glColor3xvOES")),
         Color4fNormal3fVertex3fSUN: FnPtr::new(loadfn("glColor4fNormal3fVertex3fSUN")),
         Color4fNormal3fVertex3fvSUN: FnPtr::new(loadfn("glColor4fNormal3fVertex3fvSUN")),
         Color4hNV: FnPtr::new(loadfn("glColor4hNV")),
         Color4hvNV: FnPtr::new(loadfn("glColor4hvNV")),
         Color4ubVertex2fSUN: FnPtr::new(loadfn("glColor4ubVertex2fSUN")),
         Color4ubVertex2fvSUN: FnPtr::new(loadfn("glColor4ubVertex2fvSUN")),
         Color4ubVertex3fSUN: FnPtr::new(loadfn("glColor4ubVertex3fSUN")),
         Color4ubVertex3fvSUN: FnPtr::new(loadfn("glColor4ubVertex3fvSUN")),
         Color4xOES: FnPtr::new(loadfn("glColor4xOES")),
         Color4xvOES: FnPtr::new(loadfn("glColor4xvOES")),
         ColorFormatNV: FnPtr::new(loadfn("glColorFormatNV")),
         ColorFragmentOp1ATI: FnPtr::new(loadfn("glColorFragmentOp1ATI")),
         ColorFragmentOp2ATI: FnPtr::new(loadfn("glColorFragmentOp2ATI")),
         ColorFragmentOp3ATI: FnPtr::new(loadfn("glColorFragmentOp3ATI")),
         ColorMask: FnPtr::new(loadfn("glColorMask")),
         ColorMaskIndexedEXT: FnPtr::new(loadfn("glColorMaskIndexedEXT")),
         ColorMaski: FnPtr::new(loadfn("glColorMaski")),
         ColorPointerEXT: FnPtr::new(loadfn("glColorPointerEXT")),
         ColorPointerListIBM: FnPtr::new(loadfn("glColorPointerListIBM")),
         ColorPointervINTEL: FnPtr::new(loadfn("glColorPointervINTEL")),
         ColorSubTableEXT: FnPtr::new(loadfn("glColorSubTableEXT")),
         ColorTableEXT: FnPtr::new(loadfn("glColorTableEXT")),
         ColorTableParameterfvSGI: FnPtr::new(loadfn("glColorTableParameterfvSGI")),
         ColorTableParameterivSGI: FnPtr::new(loadfn("glColorTableParameterivSGI")),
         ColorTableSGI: FnPtr::new(loadfn("glColorTableSGI")),
         CombinerInputNV: FnPtr::new(loadfn("glCombinerInputNV")),
         CombinerOutputNV: FnPtr::new(loadfn("glCombinerOutputNV")),
         CombinerParameterfNV: FnPtr::new(loadfn("glCombinerParameterfNV")),
         CombinerParameterfvNV: FnPtr::new(loadfn("glCombinerParameterfvNV")),
         CombinerParameteriNV: FnPtr::new(loadfn("glCombinerParameteriNV")),
         CombinerParameterivNV: FnPtr::new(loadfn("glCombinerParameterivNV")),
         CombinerStageParameterfvNV: FnPtr::new(loadfn("glCombinerStageParameterfvNV")),
         CommandListSegmentsNV: FnPtr::new(loadfn("glCommandListSegmentsNV")),
         CompileCommandListNV: FnPtr::new(loadfn("glCompileCommandListNV")),
         CompileShader: FnPtr::new(loadfn("glCompileShader")),
         CompileShaderARB: FnPtr::new(loadfn("glCompileShaderARB")),
         CompileShaderIncludeARB: FnPtr::new(loadfn("glCompileShaderIncludeARB")),
         CompressedMultiTexImage1DEXT: FnPtr::new(loadfn("glCompressedMultiTexImage1DEXT")),
         CompressedMultiTexImage2DEXT: FnPtr::new(loadfn("glCompressedMultiTexImage2DEXT")),
         CompressedMultiTexImage3DEXT: FnPtr::new(loadfn("glCompressedMultiTexImage3DEXT")),
         CompressedMultiTexSubImage1DEXT: FnPtr::new(loadfn("glCompressedMultiTexSubImage1DEXT")),
         CompressedMultiTexSubImage2DEXT: FnPtr::new(loadfn("glCompressedMultiTexSubImage2DEXT")),
         CompressedMultiTexSubImage3DEXT: FnPtr::new(loadfn("glCompressedMultiTexSubImage3DEXT")),
         CompressedTexImage1D: FnPtr::new(loadfn("glCompressedTexImage1D")),
         CompressedTexImage1DARB: FnPtr::new(loadfn("glCompressedTexImage1DARB")),
         CompressedTexImage2D: FnPtr::new(loadfn("glCompressedTexImage2D")),
         CompressedTexImage2DARB: FnPtr::new(loadfn("glCompressedTexImage2DARB")),
         CompressedTexImage3D: FnPtr::new(loadfn("glCompressedTexImage3D")),
         CompressedTexImage3DARB: FnPtr::new(loadfn("glCompressedTexImage3DARB")),
         CompressedTexSubImage1D: FnPtr::new(loadfn("glCompressedTexSubImage1D")),
         CompressedTexSubImage1DARB: FnPtr::new(loadfn("glCompressedTexSubImage1DARB")),
         CompressedTexSubImage2D: FnPtr::new(loadfn("glCompressedTexSubImage2D")),
         CompressedTexSubImage2DARB: FnPtr::new(loadfn("glCompressedTexSubImage2DARB")),
         CompressedTexSubImage3D: FnPtr::new(loadfn("glCompressedTexSubImage3D")),
         CompressedTexSubImage3DARB: FnPtr::new(loadfn("glCompressedTexSubImage3DARB")),
         CompressedTextureImage1DEXT: FnPtr::new(loadfn("glCompressedTextureImage1DEXT")),
         CompressedTextureImage2DEXT: FnPtr::new(loadfn("glCompressedTextureImage2DEXT")),
         CompressedTextureImage3DEXT: FnPtr::new(loadfn("glCompressedTextureImage3DEXT")),
         CompressedTextureSubImage1D: FnPtr::new(loadfn("glCompressedTextureSubImage1D")),
         CompressedTextureSubImage1DEXT: FnPtr::new(loadfn("glCompressedTextureSubImage1DEXT")),
         CompressedTextureSubImage2D: FnPtr::new(loadfn("glCompressedTextureSubImage2D")),
         CompressedTextureSubImage2DEXT: FnPtr::new(loadfn("glCompressedTextureSubImage2DEXT")),
         CompressedTextureSubImage3D: FnPtr::new(loadfn("glCompressedTextureSubImage3D")),
         CompressedTextureSubImage3DEXT: FnPtr::new(loadfn("glCompressedTextureSubImage3DEXT")),
         ConservativeRasterParameterfNV: FnPtr::new(loadfn("glConservativeRasterParameterfNV")),
         ConservativeRasterParameteriNV: FnPtr::new(loadfn("glConservativeRasterParameteriNV")),
         ConvolutionFilter1DEXT: FnPtr::new(loadfn("glConvolutionFilter1DEXT")),
         ConvolutionFilter2DEXT: FnPtr::new(loadfn("glConvolutionFilter2DEXT")),
         ConvolutionParameterfEXT: FnPtr::new(loadfn("glConvolutionParameterfEXT")),
         ConvolutionParameterfvEXT: FnPtr::new(loadfn("glConvolutionParameterfvEXT")),
         ConvolutionParameteriEXT: FnPtr::new(loadfn("glConvolutionParameteriEXT")),
         ConvolutionParameterivEXT: FnPtr::new(loadfn("glConvolutionParameterivEXT")),
         ConvolutionParameterxOES: FnPtr::new(loadfn("glConvolutionParameterxOES")),
         ConvolutionParameterxvOES: FnPtr::new(loadfn("glConvolutionParameterxvOES")),
         CopyBufferSubData: FnPtr::new(loadfn("glCopyBufferSubData")),
         CopyColorSubTableEXT: FnPtr::new(loadfn("glCopyColorSubTableEXT")),
         CopyColorTableSGI: FnPtr::new(loadfn("glCopyColorTableSGI")),
         CopyConvolutionFilter1DEXT: FnPtr::new(loadfn("glCopyConvolutionFilter1DEXT")),
         CopyConvolutionFilter2DEXT: FnPtr::new(loadfn("glCopyConvolutionFilter2DEXT")),
         CopyImageSubData: FnPtr::new(loadfn("glCopyImageSubData")),
         CopyImageSubDataNV: FnPtr::new(loadfn("glCopyImageSubDataNV")),
         CopyMultiTexImage1DEXT: FnPtr::new(loadfn("glCopyMultiTexImage1DEXT")),
         CopyMultiTexImage2DEXT: FnPtr::new(loadfn("glCopyMultiTexImage2DEXT")),
         CopyMultiTexSubImage1DEXT: FnPtr::new(loadfn("glCopyMultiTexSubImage1DEXT")),
         CopyMultiTexSubImage2DEXT: FnPtr::new(loadfn("glCopyMultiTexSubImage2DEXT")),
         CopyMultiTexSubImage3DEXT: FnPtr::new(loadfn("glCopyMultiTexSubImage3DEXT")),
         CopyNamedBufferSubData: FnPtr::new(loadfn("glCopyNamedBufferSubData")),
         CopyPathNV: FnPtr::new(loadfn("glCopyPathNV")),
         CopyTexImage1D: FnPtr::new(loadfn("glCopyTexImage1D")),
         CopyTexImage1DEXT: FnPtr::new(loadfn("glCopyTexImage1DEXT")),
         CopyTexImage2D: FnPtr::new(loadfn("glCopyTexImage2D")),
         CopyTexImage2DEXT: FnPtr::new(loadfn("glCopyTexImage2DEXT")),
         CopyTexSubImage1D: FnPtr::new(loadfn("glCopyTexSubImage1D")),
         CopyTexSubImage1DEXT: FnPtr::new(loadfn("glCopyTexSubImage1DEXT")),
         CopyTexSubImage2D: FnPtr::new(loadfn("glCopyTexSubImage2D")),
         CopyTexSubImage2DEXT: FnPtr::new(loadfn("glCopyTexSubImage2DEXT")),
         CopyTexSubImage3D: FnPtr::new(loadfn("glCopyTexSubImage3D")),
         CopyTexSubImage3DEXT: FnPtr::new(loadfn("glCopyTexSubImage3DEXT")),
         CopyTextureImage1DEXT: FnPtr::new(loadfn("glCopyTextureImage1DEXT")),
         CopyTextureImage2DEXT: FnPtr::new(loadfn("glCopyTextureImage2DEXT")),
         CopyTextureSubImage1D: FnPtr::new(loadfn("glCopyTextureSubImage1D")),
         CopyTextureSubImage1DEXT: FnPtr::new(loadfn("glCopyTextureSubImage1DEXT")),
         CopyTextureSubImage2D: FnPtr::new(loadfn("glCopyTextureSubImage2D")),
         CopyTextureSubImage2DEXT: FnPtr::new(loadfn("glCopyTextureSubImage2DEXT")),
         CopyTextureSubImage3D: FnPtr::new(loadfn("glCopyTextureSubImage3D")),
         CopyTextureSubImage3DEXT: FnPtr::new(loadfn("glCopyTextureSubImage3DEXT")),
         CoverFillPathInstancedNV: FnPtr::new(loadfn("glCoverFillPathInstancedNV")),
         CoverFillPathNV: FnPtr::new(loadfn("glCoverFillPathNV")),
         CoverStrokePathInstancedNV: FnPtr::new(loadfn("glCoverStrokePathInstancedNV")),
         CoverStrokePathNV: FnPtr::new(loadfn("glCoverStrokePathNV")),
         CoverageModulationNV: FnPtr::new(loadfn("glCoverageModulationNV")),
         CoverageModulationTableNV: FnPtr::new(loadfn("glCoverageModulationTableNV")),
         CreateBuffers: FnPtr::new(loadfn("glCreateBuffers")),
         CreateCommandListsNV: FnPtr::new(loadfn("glCreateCommandListsNV")),
         CreateFramebuffers: FnPtr::new(loadfn("glCreateFramebuffers")),
         CreateMemoryObjectsEXT: FnPtr::new(loadfn("glCreateMemoryObjectsEXT")),
         CreatePerfQueryINTEL: FnPtr::new(loadfn("glCreatePerfQueryINTEL")),
         CreateProgram: FnPtr::new(loadfn("glCreateProgram")),
         CreateProgramObjectARB: FnPtr::new(loadfn("glCreateProgramObjectARB")),
         CreateProgramPipelines: FnPtr::new(loadfn("glCreateProgramPipelines")),
         CreateProgressFenceNVX: FnPtr::new(loadfn("glCreateProgressFenceNVX")),
         CreateQueries: FnPtr::new(loadfn("glCreateQueries")),
         CreateRenderbuffers: FnPtr::new(loadfn("glCreateRenderbuffers")),
         CreateSamplers: FnPtr::new(loadfn("glCreateSamplers")),
         CreateSemaphoresNV: FnPtr::new(loadfn("glCreateSemaphoresNV")),
         CreateShader: FnPtr::new(loadfn("glCreateShader")),
         CreateShaderObjectARB: FnPtr::new(loadfn("glCreateShaderObjectARB")),
         CreateShaderProgramEXT: FnPtr::new(loadfn("glCreateShaderProgramEXT")),
         CreateShaderProgramv: FnPtr::new(loadfn("glCreateShaderProgramv")),
         CreateStatesNV: FnPtr::new(loadfn("glCreateStatesNV")),
         CreateSyncFromCLeventARB: FnPtr::new(loadfn("glCreateSyncFromCLeventARB")),
         CreateTextures: FnPtr::new(loadfn("glCreateTextures")),
         CreateTransformFeedbacks: FnPtr::new(loadfn("glCreateTransformFeedbacks")),
         CreateVertexArrays: FnPtr::new(loadfn("glCreateVertexArrays")),
         CullFace: FnPtr::new(loadfn("glCullFace")),
         CullParameterdvEXT: FnPtr::new(loadfn("glCullParameterdvEXT")),
         CullParameterfvEXT: FnPtr::new(loadfn("glCullParameterfvEXT")),
         CurrentPaletteMatrixARB: FnPtr::new(loadfn("glCurrentPaletteMatrixARB")),
         DebugMessageCallback: FnPtr::new(loadfn("glDebugMessageCallback")),
         DebugMessageCallbackAMD: FnPtr::new(loadfn("glDebugMessageCallbackAMD")),
         DebugMessageCallbackARB: FnPtr::new(loadfn("glDebugMessageCallbackARB")),
         DebugMessageControl: FnPtr::new(loadfn("glDebugMessageControl")),
         DebugMessageControlARB: FnPtr::new(loadfn("glDebugMessageControlARB")),
         DebugMessageEnableAMD: FnPtr::new(loadfn("glDebugMessageEnableAMD")),
         DebugMessageInsert: FnPtr::new(loadfn("glDebugMessageInsert")),
         DebugMessageInsertAMD: FnPtr::new(loadfn("glDebugMessageInsertAMD")),
         DebugMessageInsertARB: FnPtr::new(loadfn("glDebugMessageInsertARB")),
         DeformSGIX: FnPtr::new(loadfn("glDeformSGIX")),
         DeformationMap3dSGIX: FnPtr::new(loadfn("glDeformationMap3dSGIX")),
         DeformationMap3fSGIX: FnPtr::new(loadfn("glDeformationMap3fSGIX")),
         DeleteAsyncMarkersSGIX: FnPtr::new(loadfn("glDeleteAsyncMarkersSGIX")),
         DeleteBuffers: FnPtr::new(loadfn("glDeleteBuffers")),
         DeleteBuffersARB: FnPtr::new(loadfn("glDeleteBuffersARB")),
         DeleteCommandListsNV: FnPtr::new(loadfn("glDeleteCommandListsNV")),
         DeleteFencesAPPLE: FnPtr::new(loadfn("glDeleteFencesAPPLE")),
         DeleteFencesNV: FnPtr::new(loadfn("glDeleteFencesNV")),
         DeleteFragmentShaderATI: FnPtr::new(loadfn("glDeleteFragmentShaderATI")),
         DeleteFramebuffers: FnPtr::new(loadfn("glDeleteFramebuffers")),
         DeleteFramebuffersEXT: FnPtr::new(loadfn("glDeleteFramebuffersEXT")),
         DeleteMemoryObjectsEXT: FnPtr::new(loadfn("glDeleteMemoryObjectsEXT")),
         DeleteNamedStringARB: FnPtr::new(loadfn("glDeleteNamedStringARB")),
         DeleteNamesAMD: FnPtr::new(loadfn("glDeleteNamesAMD")),
         DeleteObjectARB: FnPtr::new(loadfn("glDeleteObjectARB")),
         DeleteOcclusionQueriesNV: FnPtr::new(loadfn("glDeleteOcclusionQueriesNV")),
         DeletePathsNV: FnPtr::new(loadfn("glDeletePathsNV")),
         DeletePerfMonitorsAMD: FnPtr::new(loadfn("glDeletePerfMonitorsAMD")),
         DeletePerfQueryINTEL: FnPtr::new(loadfn("glDeletePerfQueryINTEL")),
         DeleteProgram: FnPtr::new(loadfn("glDeleteProgram")),
         DeleteProgramPipelines: FnPtr::new(loadfn("glDeleteProgramPipelines")),
         DeleteProgramsARB: FnPtr::new(loadfn("glDeleteProgramsARB")),
         DeleteProgramsNV: FnPtr::new(loadfn("glDeleteProgramsNV")),
         DeleteQueries: FnPtr::new(loadfn("glDeleteQueries")),
         DeleteQueriesARB: FnPtr::new(loadfn("glDeleteQueriesARB")),
         DeleteQueryResourceTagNV: FnPtr::new(loadfn("glDeleteQueryResourceTagNV")),
         DeleteRenderbuffers: FnPtr::new(loadfn("glDeleteRenderbuffers")),
         DeleteRenderbuffersEXT: FnPtr::new(loadfn("glDeleteRenderbuffersEXT")),
         DeleteSamplers: FnPtr::new(loadfn("glDeleteSamplers")),
         DeleteSemaphoresEXT: FnPtr::new(loadfn("glDeleteSemaphoresEXT")),
         DeleteShader: FnPtr::new(loadfn("glDeleteShader")),
         DeleteStatesNV: FnPtr::new(loadfn("glDeleteStatesNV")),
         DeleteSync: FnPtr::new(loadfn("glDeleteSync")),
         DeleteTextures: FnPtr::new(loadfn("glDeleteTextures")),
         DeleteTexturesEXT: FnPtr::new(loadfn("glDeleteTexturesEXT")),
         DeleteTransformFeedbacks: FnPtr::new(loadfn("glDeleteTransformFeedbacks")),
         DeleteTransformFeedbacksNV: FnPtr::new(loadfn("glDeleteTransformFeedbacksNV")),
         DeleteVertexArrays: FnPtr::new(loadfn("glDeleteVertexArrays")),
         DeleteVertexArraysAPPLE: FnPtr::new(loadfn("glDeleteVertexArraysAPPLE")),
         DeleteVertexShaderEXT: FnPtr::new(loadfn("glDeleteVertexShaderEXT")),
         DepthBoundsEXT: FnPtr::new(loadfn("glDepthBoundsEXT")),
         DepthBoundsdNV: FnPtr::new(loadfn("glDepthBoundsdNV")),
         DepthFunc: FnPtr::new(loadfn("glDepthFunc")),
         DepthMask: FnPtr::new(loadfn("glDepthMask")),
         DepthRange: FnPtr::new(loadfn("glDepthRange")),
         DepthRangeArraydvNV: FnPtr::new(loadfn("glDepthRangeArraydvNV")),
         DepthRangeArrayv: FnPtr::new(loadfn("glDepthRangeArrayv")),
         DepthRangeIndexed: FnPtr::new(loadfn("glDepthRangeIndexed")),
         DepthRangeIndexeddNV: FnPtr::new(loadfn("glDepthRangeIndexeddNV")),
         DepthRangedNV: FnPtr::new(loadfn("glDepthRangedNV")),
         DepthRangef: FnPtr::new(loadfn("glDepthRangef")),
         DepthRangefOES: FnPtr::new(loadfn("glDepthRangefOES")),
         DepthRangexOES: FnPtr::new(loadfn("glDepthRangexOES")),
         DetachObjectARB: FnPtr::new(loadfn("glDetachObjectARB")),
         DetachShader: FnPtr::new(loadfn("glDetachShader")),
         DetailTexFuncSGIS: FnPtr::new(loadfn("glDetailTexFuncSGIS")),
         Disable: FnPtr::new(loadfn("glDisable")),
         DisableClientStateIndexedEXT: FnPtr::new(loadfn("glDisableClientStateIndexedEXT")),
         DisableClientStateiEXT: FnPtr::new(loadfn("glDisableClientStateiEXT")),
         DisableIndexedEXT: FnPtr::new(loadfn("glDisableIndexedEXT")),
         DisableVariantClientStateEXT: FnPtr::new(loadfn("glDisableVariantClientStateEXT")),
         DisableVertexArrayAttrib: FnPtr::new(loadfn("glDisableVertexArrayAttrib")),
         DisableVertexArrayAttribEXT: FnPtr::new(loadfn("glDisableVertexArrayAttribEXT")),
         DisableVertexArrayEXT: FnPtr::new(loadfn("glDisableVertexArrayEXT")),
         DisableVertexAttribAPPLE: FnPtr::new(loadfn("glDisableVertexAttribAPPLE")),
         DisableVertexAttribArray: FnPtr::new(loadfn("glDisableVertexAttribArray")),
         DisableVertexAttribArrayARB: FnPtr::new(loadfn("glDisableVertexAttribArrayARB")),
         Disablei: FnPtr::new(loadfn("glDisablei")),
         DispatchCompute: FnPtr::new(loadfn("glDispatchCompute")),
         DispatchComputeGroupSizeARB: FnPtr::new(loadfn("glDispatchComputeGroupSizeARB")),
         DispatchComputeIndirect: FnPtr::new(loadfn("glDispatchComputeIndirect")),
         DrawArrays: FnPtr::new(loadfn("glDrawArrays")),
         DrawArraysEXT: FnPtr::new(loadfn("glDrawArraysEXT")),
         DrawArraysIndirect: FnPtr::new(loadfn("glDrawArraysIndirect")),
         DrawArraysInstanced: FnPtr::new(loadfn("glDrawArraysInstanced")),
         DrawArraysInstancedARB: FnPtr::new(loadfn("glDrawArraysInstancedARB")),
         DrawArraysInstancedBaseInstance: FnPtr::new(loadfn("glDrawArraysInstancedBaseInstance")),
         DrawArraysInstancedEXT: FnPtr::new(loadfn("glDrawArraysInstancedEXT")),
         DrawBuffer: FnPtr::new(loadfn("glDrawBuffer")),
         DrawBuffers: FnPtr::new(loadfn("glDrawBuffers")),
         DrawBuffersARB: FnPtr::new(loadfn("glDrawBuffersARB")),
         DrawBuffersATI: FnPtr::new(loadfn("glDrawBuffersATI")),
         DrawCommandsAddressNV: FnPtr::new(loadfn("glDrawCommandsAddressNV")),
         DrawCommandsNV: FnPtr::new(loadfn("glDrawCommandsNV")),
         DrawCommandsStatesAddressNV: FnPtr::new(loadfn("glDrawCommandsStatesAddressNV")),
         DrawCommandsStatesNV: FnPtr::new(loadfn("glDrawCommandsStatesNV")),
         DrawElementArrayAPPLE: FnPtr::new(loadfn("glDrawElementArrayAPPLE")),
         DrawElementArrayATI: FnPtr::new(loadfn("glDrawElementArrayATI")),
         DrawElements: FnPtr::new(loadfn("glDrawElements")),
         DrawElementsBaseVertex: FnPtr::new(loadfn("glDrawElementsBaseVertex")),
         DrawElementsIndirect: FnPtr::new(loadfn("glDrawElementsIndirect")),
         DrawElementsInstanced: FnPtr::new(loadfn("glDrawElementsInstanced")),
         DrawElementsInstancedARB: FnPtr::new(loadfn("glDrawElementsInstancedARB")),
         DrawElementsInstancedBaseInstance: FnPtr::new(loadfn("glDrawElementsInstancedBaseInstance")),
         DrawElementsInstancedBaseVertex: FnPtr::new(loadfn("glDrawElementsInstancedBaseVertex")),
         DrawElementsInstancedBaseVertexBaseInstance: FnPtr::new(loadfn("glDrawElementsInstancedBaseVertexBaseInstance")),
         DrawElementsInstancedEXT: FnPtr::new(loadfn("glDrawElementsInstancedEXT")),
         DrawMeshArraysSUN: FnPtr::new(loadfn("glDrawMeshArraysSUN")),
         DrawMeshTasksIndirectNV: FnPtr::new(loadfn("glDrawMeshTasksIndirectNV")),
         DrawMeshTasksNV: FnPtr::new(loadfn("glDrawMeshTasksNV")),
         DrawRangeElementArrayAPPLE: FnPtr::new(loadfn("glDrawRangeElementArrayAPPLE")),
         DrawRangeElementArrayATI: FnPtr::new(loadfn("glDrawRangeElementArrayATI")),
         DrawRangeElements: FnPtr::new(loadfn("glDrawRangeElements")),
         DrawRangeElementsBaseVertex: FnPtr::new(loadfn("glDrawRangeElementsBaseVertex")),
         DrawRangeElementsEXT: FnPtr::new(loadfn("glDrawRangeElementsEXT")),
         DrawTextureNV: FnPtr::new(loadfn("glDrawTextureNV")),
         DrawTransformFeedback: FnPtr::new(loadfn("glDrawTransformFeedback")),
         DrawTransformFeedbackInstanced: FnPtr::new(loadfn("glDrawTransformFeedbackInstanced")),
         DrawTransformFeedbackNV: FnPtr::new(loadfn("glDrawTransformFeedbackNV")),
         DrawTransformFeedbackStream: FnPtr::new(loadfn("glDrawTransformFeedbackStream")),
         DrawTransformFeedbackStreamInstanced: FnPtr::new(loadfn("glDrawTransformFeedbackStreamInstanced")),
         DrawVkImageNV: FnPtr::new(loadfn("glDrawVkImageNV")),
         EGLImageTargetTexStorageEXT: FnPtr::new(loadfn("glEGLImageTargetTexStorageEXT")),
         EGLImageTargetTextureStorageEXT: FnPtr::new(loadfn("glEGLImageTargetTextureStorageEXT")),
         EdgeFlagFormatNV: FnPtr::new(loadfn("glEdgeFlagFormatNV")),
         EdgeFlagPointerEXT: FnPtr::new(loadfn("glEdgeFlagPointerEXT")),
         EdgeFlagPointerListIBM: FnPtr::new(loadfn("glEdgeFlagPointerListIBM")),
         ElementPointerAPPLE: FnPtr::new(loadfn("glElementPointerAPPLE")),
         ElementPointerATI: FnPtr::new(loadfn("glElementPointerATI")),
         Enable: FnPtr::new(loadfn("glEnable")),
         EnableClientStateIndexedEXT: FnPtr::new(loadfn("glEnableClientStateIndexedEXT")),
         EnableClientStateiEXT: FnPtr::new(loadfn("glEnableClientStateiEXT")),
         EnableIndexedEXT: FnPtr::new(loadfn("glEnableIndexedEXT")),
         EnableVariantClientStateEXT: FnPtr::new(loadfn("glEnableVariantClientStateEXT")),
         EnableVertexArrayAttrib: FnPtr::new(loadfn("glEnableVertexArrayAttrib")),
         EnableVertexArrayAttribEXT: FnPtr::new(loadfn("glEnableVertexArrayAttribEXT")),
         EnableVertexArrayEXT: FnPtr::new(loadfn("glEnableVertexArrayEXT")),
         EnableVertexAttribAPPLE: FnPtr::new(loadfn("glEnableVertexAttribAPPLE")),
         EnableVertexAttribArray: FnPtr::new(loadfn("glEnableVertexAttribArray")),
         EnableVertexAttribArrayARB: FnPtr::new(loadfn("glEnableVertexAttribArrayARB")),
         Enablei: FnPtr::new(loadfn("glEnablei")),
         EndConditionalRender: FnPtr::new(loadfn("glEndConditionalRender")),
         EndConditionalRenderNV: FnPtr::new(loadfn("glEndConditionalRenderNV")),
         EndConditionalRenderNVX: FnPtr::new(loadfn("glEndConditionalRenderNVX")),
         EndFragmentShaderATI: FnPtr::new(loadfn("glEndFragmentShaderATI")),
         EndOcclusionQueryNV: FnPtr::new(loadfn("glEndOcclusionQueryNV")),
         EndPerfMonitorAMD: FnPtr::new(loadfn("glEndPerfMonitorAMD")),
         EndPerfQueryINTEL: FnPtr::new(loadfn("glEndPerfQueryINTEL")),
         EndQuery: FnPtr::new(loadfn("glEndQuery")),
         EndQueryARB: FnPtr::new(loadfn("glEndQueryARB")),
         EndQueryIndexed: FnPtr::new(loadfn("glEndQueryIndexed")),
         EndTransformFeedback: FnPtr::new(loadfn("glEndTransformFeedback")),
         EndTransformFeedbackEXT: FnPtr::new(loadfn("glEndTransformFeedbackEXT")),
         EndTransformFeedbackNV: FnPtr::new(loadfn("glEndTransformFeedbackNV")),
         EndVertexShaderEXT: FnPtr::new(loadfn("glEndVertexShaderEXT")),
         EndVideoCaptureNV: FnPtr::new(loadfn("glEndVideoCaptureNV")),
         EvalCoord1xOES: FnPtr::new(loadfn("glEvalCoord1xOES")),
         EvalCoord1xvOES: FnPtr::new(loadfn("glEvalCoord1xvOES")),
         EvalCoord2xOES: FnPtr::new(loadfn("glEvalCoord2xOES")),
         EvalCoord2xvOES: FnPtr::new(loadfn("glEvalCoord2xvOES")),
         EvalMapsNV: FnPtr::new(loadfn("glEvalMapsNV")),
         EvaluateDepthValuesARB: FnPtr::new(loadfn("glEvaluateDepthValuesARB")),
         ExecuteProgramNV: FnPtr::new(loadfn("glExecuteProgramNV")),
         ExtractComponentEXT: FnPtr::new(loadfn("glExtractComponentEXT")),
         FeedbackBufferxOES: FnPtr::new(loadfn("glFeedbackBufferxOES")),
         FenceSync: FnPtr::new(loadfn("glFenceSync")),
         FinalCombinerInputNV: FnPtr::new(loadfn("glFinalCombinerInputNV")),
         Finish: FnPtr::new(loadfn("glFinish")),
         FinishAsyncSGIX: FnPtr::new(loadfn("glFinishAsyncSGIX")),
         FinishFenceAPPLE: FnPtr::new(loadfn("glFinishFenceAPPLE")),
         FinishFenceNV: FnPtr::new(loadfn("glFinishFenceNV")),
         FinishObjectAPPLE: FnPtr::new(loadfn("glFinishObjectAPPLE")),
         FinishTextureSUNX: FnPtr::new(loadfn("glFinishTextureSUNX")),
         Flush: FnPtr::new(loadfn("glFlush")),
         FlushMappedBufferRange: FnPtr::new(loadfn("glFlushMappedBufferRange")),
         FlushMappedBufferRangeAPPLE: FnPtr::new(loadfn("glFlushMappedBufferRangeAPPLE")),
         FlushMappedNamedBufferRange: FnPtr::new(loadfn("glFlushMappedNamedBufferRange")),
         FlushMappedNamedBufferRangeEXT: FnPtr::new(loadfn("glFlushMappedNamedBufferRangeEXT")),
         FlushPixelDataRangeNV: FnPtr::new(loadfn("glFlushPixelDataRangeNV")),
         FlushRasterSGIX: FnPtr::new(loadfn("glFlushRasterSGIX")),
         FlushStaticDataIBM: FnPtr::new(loadfn("glFlushStaticDataIBM")),
         FlushVertexArrayRangeAPPLE: FnPtr::new(loadfn("glFlushVertexArrayRangeAPPLE")),
         FlushVertexArrayRangeNV: FnPtr::new(loadfn("glFlushVertexArrayRangeNV")),
         FogCoordFormatNV: FnPtr::new(loadfn("glFogCoordFormatNV")),
         FogCoordPointerEXT: FnPtr::new(loadfn("glFogCoordPointerEXT")),
         FogCoordPointerListIBM: FnPtr::new(loadfn("glFogCoordPointerListIBM")),
         FogCoorddEXT: FnPtr::new(loadfn("glFogCoorddEXT")),
         FogCoorddvEXT: FnPtr::new(loadfn("glFogCoorddvEXT")),
         FogCoordfEXT: FnPtr::new(loadfn("glFogCoordfEXT")),
         FogCoordfvEXT: FnPtr::new(loadfn("glFogCoordfvEXT")),
         FogCoordhNV: FnPtr::new(loadfn("glFogCoordhNV")),
         FogCoordhvNV: FnPtr::new(loadfn("glFogCoordhvNV")),
         FogFuncSGIS: FnPtr::new(loadfn("glFogFuncSGIS")),
         FogxOES: FnPtr::new(loadfn("glFogxOES")),
         FogxvOES: FnPtr::new(loadfn("glFogxvOES")),
         FragmentColorMaterialSGIX: FnPtr::new(loadfn("glFragmentColorMaterialSGIX")),
         FragmentCoverageColorNV: FnPtr::new(loadfn("glFragmentCoverageColorNV")),
         FragmentLightModelfSGIX: FnPtr::new(loadfn("glFragmentLightModelfSGIX")),
         FragmentLightModelfvSGIX: FnPtr::new(loadfn("glFragmentLightModelfvSGIX")),
         FragmentLightModeliSGIX: FnPtr::new(loadfn("glFragmentLightModeliSGIX")),
         FragmentLightModelivSGIX: FnPtr::new(loadfn("glFragmentLightModelivSGIX")),
         FragmentLightfSGIX: FnPtr::new(loadfn("glFragmentLightfSGIX")),
         FragmentLightfvSGIX: FnPtr::new(loadfn("glFragmentLightfvSGIX")),
         FragmentLightiSGIX: FnPtr::new(loadfn("glFragmentLightiSGIX")),
         FragmentLightivSGIX: FnPtr::new(loadfn("glFragmentLightivSGIX")),
         FragmentMaterialfSGIX: FnPtr::new(loadfn("glFragmentMaterialfSGIX")),
         FragmentMaterialfvSGIX: FnPtr::new(loadfn("glFragmentMaterialfvSGIX")),
         FragmentMaterialiSGIX: FnPtr::new(loadfn("glFragmentMaterialiSGIX")),
         FragmentMaterialivSGIX: FnPtr::new(loadfn("glFragmentMaterialivSGIX")),
         FrameTerminatorGREMEDY: FnPtr::new(loadfn("glFrameTerminatorGREMEDY")),
         FrameZoomSGIX: FnPtr::new(loadfn("glFrameZoomSGIX")),
         FramebufferDrawBufferEXT: FnPtr::new(loadfn("glFramebufferDrawBufferEXT")),
         FramebufferDrawBuffersEXT: FnPtr::new(loadfn("glFramebufferDrawBuffersEXT")),
         FramebufferFetchBarrierEXT: FnPtr::new(loadfn("glFramebufferFetchBarrierEXT")),
         FramebufferParameteri: FnPtr::new(loadfn("glFramebufferParameteri")),
         FramebufferParameteriMESA: FnPtr::new(loadfn("glFramebufferParameteriMESA")),
         FramebufferReadBufferEXT: FnPtr::new(loadfn("glFramebufferReadBufferEXT")),
         FramebufferRenderbuffer: FnPtr::new(loadfn("glFramebufferRenderbuffer")),
         FramebufferRenderbufferEXT: FnPtr::new(loadfn("glFramebufferRenderbufferEXT")),
         FramebufferSampleLocationsfvARB: FnPtr::new(loadfn("glFramebufferSampleLocationsfvARB")),
         FramebufferSampleLocationsfvNV: FnPtr::new(loadfn("glFramebufferSampleLocationsfvNV")),
         FramebufferSamplePositionsfvAMD: FnPtr::new(loadfn("glFramebufferSamplePositionsfvAMD")),
         FramebufferTexture: FnPtr::new(loadfn("glFramebufferTexture")),
         FramebufferTexture1D: FnPtr::new(loadfn("glFramebufferTexture1D")),
         FramebufferTexture1DEXT: FnPtr::new(loadfn("glFramebufferTexture1DEXT")),
         FramebufferTexture2D: FnPtr::new(loadfn("glFramebufferTexture2D")),
         FramebufferTexture2DEXT: FnPtr::new(loadfn("glFramebufferTexture2DEXT")),
         FramebufferTexture3D: FnPtr::new(loadfn("glFramebufferTexture3D")),
         FramebufferTexture3DEXT: FnPtr::new(loadfn("glFramebufferTexture3DEXT")),
         FramebufferTextureARB: FnPtr::new(loadfn("glFramebufferTextureARB")),
         FramebufferTextureEXT: FnPtr::new(loadfn("glFramebufferTextureEXT")),
         FramebufferTextureFaceARB: FnPtr::new(loadfn("glFramebufferTextureFaceARB")),
         FramebufferTextureFaceEXT: FnPtr::new(loadfn("glFramebufferTextureFaceEXT")),
         FramebufferTextureLayer: FnPtr::new(loadfn("glFramebufferTextureLayer")),
         FramebufferTextureLayerARB: FnPtr::new(loadfn("glFramebufferTextureLayerARB")),
         FramebufferTextureLayerEXT: FnPtr::new(loadfn("glFramebufferTextureLayerEXT")),
         FramebufferTextureMultiviewOVR: FnPtr::new(loadfn("glFramebufferTextureMultiviewOVR")),
         FreeObjectBufferATI: FnPtr::new(loadfn("glFreeObjectBufferATI")),
         FrontFace: FnPtr::new(loadfn("glFrontFace")),
         FrustumfOES: FnPtr::new(loadfn("glFrustumfOES")),
         FrustumxOES: FnPtr::new(loadfn("glFrustumxOES")),
         GenAsyncMarkersSGIX: FnPtr::new(loadfn("glGenAsyncMarkersSGIX")),
         GenBuffers: FnPtr::new(loadfn("glGenBuffers")),
         GenBuffersARB: FnPtr::new(loadfn("glGenBuffersARB")),
         GenFencesAPPLE: FnPtr::new(loadfn("glGenFencesAPPLE")),
         GenFencesNV: FnPtr::new(loadfn("glGenFencesNV")),
         GenFragmentShadersATI: FnPtr::new(loadfn("glGenFragmentShadersATI")),
         GenFramebuffers: FnPtr::new(loadfn("glGenFramebuffers")),
         GenFramebuffersEXT: FnPtr::new(loadfn("glGenFramebuffersEXT")),
         GenNamesAMD: FnPtr::new(loadfn("glGenNamesAMD")),
         GenOcclusionQueriesNV: FnPtr::new(loadfn("glGenOcclusionQueriesNV")),
         GenPathsNV: FnPtr::new(loadfn("glGenPathsNV")),
         GenPerfMonitorsAMD: FnPtr::new(loadfn("glGenPerfMonitorsAMD")),
         GenProgramPipelines: FnPtr::new(loadfn("glGenProgramPipelines")),
         GenProgramsARB: FnPtr::new(loadfn("glGenProgramsARB")),
         GenProgramsNV: FnPtr::new(loadfn("glGenProgramsNV")),
         GenQueries: FnPtr::new(loadfn("glGenQueries")),
         GenQueriesARB: FnPtr::new(loadfn("glGenQueriesARB")),
         GenQueryResourceTagNV: FnPtr::new(loadfn("glGenQueryResourceTagNV")),
         GenRenderbuffers: FnPtr::new(loadfn("glGenRenderbuffers")),
         GenRenderbuffersEXT: FnPtr::new(loadfn("glGenRenderbuffersEXT")),
         GenSamplers: FnPtr::new(loadfn("glGenSamplers")),
         GenSemaphoresEXT: FnPtr::new(loadfn("glGenSemaphoresEXT")),
         GenSymbolsEXT: FnPtr::new(loadfn("glGenSymbolsEXT")),
         GenTextures: FnPtr::new(loadfn("glGenTextures")),
         GenTexturesEXT: FnPtr::new(loadfn("glGenTexturesEXT")),
         GenTransformFeedbacks: FnPtr::new(loadfn("glGenTransformFeedbacks")),
         GenTransformFeedbacksNV: FnPtr::new(loadfn("glGenTransformFeedbacksNV")),
         GenVertexArrays: FnPtr::new(loadfn("glGenVertexArrays")),
         GenVertexArraysAPPLE: FnPtr::new(loadfn("glGenVertexArraysAPPLE")),
         GenVertexShadersEXT: FnPtr::new(loadfn("glGenVertexShadersEXT")),
         GenerateMipmap: FnPtr::new(loadfn("glGenerateMipmap")),
         GenerateMipmapEXT: FnPtr::new(loadfn("glGenerateMipmapEXT")),
         GenerateMultiTexMipmapEXT: FnPtr::new(loadfn("glGenerateMultiTexMipmapEXT")),
         GenerateTextureMipmap: FnPtr::new(loadfn("glGenerateTextureMipmap")),
         GenerateTextureMipmapEXT: FnPtr::new(loadfn("glGenerateTextureMipmapEXT")),
         GetActiveAtomicCounterBufferiv: FnPtr::new(loadfn("glGetActiveAtomicCounterBufferiv")),
         GetActiveAttrib: FnPtr::new(loadfn("glGetActiveAttrib")),
         GetActiveAttribARB: FnPtr::new(loadfn("glGetActiveAttribARB")),
         GetActiveSubroutineName: FnPtr::new(loadfn("glGetActiveSubroutineName")),
         GetActiveSubroutineUniformName: FnPtr::new(loadfn("glGetActiveSubroutineUniformName")),
         GetActiveSubroutineUniformiv: FnPtr::new(loadfn("glGetActiveSubroutineUniformiv")),
         GetActiveUniform: FnPtr::new(loadfn("glGetActiveUniform")),
         GetActiveUniformARB: FnPtr::new(loadfn("glGetActiveUniformARB")),
         GetActiveUniformBlockName: FnPtr::new(loadfn("glGetActiveUniformBlockName")),
         GetActiveUniformBlockiv: FnPtr::new(loadfn("glGetActiveUniformBlockiv")),
         GetActiveUniformName: FnPtr::new(loadfn("glGetActiveUniformName")),
         GetActiveUniformsiv: FnPtr::new(loadfn("glGetActiveUniformsiv")),
         GetActiveVaryingNV: FnPtr::new(loadfn("glGetActiveVaryingNV")),
         GetArrayObjectfvATI: FnPtr::new(loadfn("glGetArrayObjectfvATI")),
         GetArrayObjectivATI: FnPtr::new(loadfn("glGetArrayObjectivATI")),
         GetAttachedObjectsARB: FnPtr::new(loadfn("glGetAttachedObjectsARB")),
         GetAttachedShaders: FnPtr::new(loadfn("glGetAttachedShaders")),
         GetAttribLocation: FnPtr::new(loadfn("glGetAttribLocation")),
         GetAttribLocationARB: FnPtr::new(loadfn("glGetAttribLocationARB")),
         GetBooleanIndexedvEXT: FnPtr::new(loadfn("glGetBooleanIndexedvEXT")),
         GetBooleani_v: FnPtr::new(loadfn("glGetBooleani_v")),
         GetBooleanv: FnPtr::new(loadfn("glGetBooleanv")),
         GetBufferParameteri64v: FnPtr::new(loadfn("glGetBufferParameteri64v")),
         GetBufferParameteriv: FnPtr::new(loadfn("glGetBufferParameteriv")),
         GetBufferParameterivARB: FnPtr::new(loadfn("glGetBufferParameterivARB")),
         GetBufferParameterui64vNV: FnPtr::new(loadfn("glGetBufferParameterui64vNV")),
         GetBufferPointerv: FnPtr::new(loadfn("glGetBufferPointerv")),
         GetBufferPointervARB: FnPtr::new(loadfn("glGetBufferPointervARB")),
         GetBufferSubData: FnPtr::new(loadfn("glGetBufferSubData")),
         GetBufferSubDataARB: FnPtr::new(loadfn("glGetBufferSubDataARB")),
         GetClipPlanefOES: FnPtr::new(loadfn("glGetClipPlanefOES")),
         GetClipPlanexOES: FnPtr::new(loadfn("glGetClipPlanexOES")),
         GetColorTableEXT: FnPtr::new(loadfn("glGetColorTableEXT")),
         GetColorTableParameterfvEXT: FnPtr::new(loadfn("glGetColorTableParameterfvEXT")),
         GetColorTableParameterfvSGI: FnPtr::new(loadfn("glGetColorTableParameterfvSGI")),
         GetColorTableParameterivEXT: FnPtr::new(loadfn("glGetColorTableParameterivEXT")),
         GetColorTableParameterivSGI: FnPtr::new(loadfn("glGetColorTableParameterivSGI")),
         GetColorTableSGI: FnPtr::new(loadfn("glGetColorTableSGI")),
         GetCombinerInputParameterfvNV: FnPtr::new(loadfn("glGetCombinerInputParameterfvNV")),
         GetCombinerInputParameterivNV: FnPtr::new(loadfn("glGetCombinerInputParameterivNV")),
         GetCombinerOutputParameterfvNV: FnPtr::new(loadfn("glGetCombinerOutputParameterfvNV")),
         GetCombinerOutputParameterivNV: FnPtr::new(loadfn("glGetCombinerOutputParameterivNV")),
         GetCombinerStageParameterfvNV: FnPtr::new(loadfn("glGetCombinerStageParameterfvNV")),
         GetCommandHeaderNV: FnPtr::new(loadfn("glGetCommandHeaderNV")),
         GetCompressedMultiTexImageEXT: FnPtr::new(loadfn("glGetCompressedMultiTexImageEXT")),
         GetCompressedTexImage: FnPtr::new(loadfn("glGetCompressedTexImage")),
         GetCompressedTexImageARB: FnPtr::new(loadfn("glGetCompressedTexImageARB")),
         GetCompressedTextureImage: FnPtr::new(loadfn("glGetCompressedTextureImage")),
         GetCompressedTextureImageEXT: FnPtr::new(loadfn("glGetCompressedTextureImageEXT")),
         GetCompressedTextureSubImage: FnPtr::new(loadfn("glGetCompressedTextureSubImage")),
         GetConvolutionFilterEXT: FnPtr::new(loadfn("glGetConvolutionFilterEXT")),
         GetConvolutionParameterfvEXT: FnPtr::new(loadfn("glGetConvolutionParameterfvEXT")),
         GetConvolutionParameterivEXT: FnPtr::new(loadfn("glGetConvolutionParameterivEXT")),
         GetConvolutionParameterxvOES: FnPtr::new(loadfn("glGetConvolutionParameterxvOES")),
         GetCoverageModulationTableNV: FnPtr::new(loadfn("glGetCoverageModulationTableNV")),
         GetDebugMessageLog: FnPtr::new(loadfn("glGetDebugMessageLog")),
         GetDebugMessageLogAMD: FnPtr::new(loadfn("glGetDebugMessageLogAMD")),
         GetDebugMessageLogARB: FnPtr::new(loadfn("glGetDebugMessageLogARB")),
         GetDetailTexFuncSGIS: FnPtr::new(loadfn("glGetDetailTexFuncSGIS")),
         GetDoubleIndexedvEXT: FnPtr::new(loadfn("glGetDoubleIndexedvEXT")),
         GetDoublei_v: FnPtr::new(loadfn("glGetDoublei_v")),
         GetDoublei_vEXT: FnPtr::new(loadfn("glGetDoublei_vEXT")),
         GetDoublev: FnPtr::new(loadfn("glGetDoublev")),
         GetError: FnPtr::new(loadfn("glGetError")),
         GetFenceivNV: FnPtr::new(loadfn("glGetFenceivNV")),
         GetFinalCombinerInputParameterfvNV: FnPtr::new(loadfn("glGetFinalCombinerInputParameterfvNV")),
         GetFinalCombinerInputParameterivNV: FnPtr::new(loadfn("glGetFinalCombinerInputParameterivNV")),
         GetFirstPerfQueryIdINTEL: FnPtr::new(loadfn("glGetFirstPerfQueryIdINTEL")),
         GetFixedvOES: FnPtr::new(loadfn("glGetFixedvOES")),
         GetFloatIndexedvEXT: FnPtr::new(loadfn("glGetFloatIndexedvEXT")),
         GetFloati_v: FnPtr::new(loadfn("glGetFloati_v")),
         GetFloati_vEXT: FnPtr::new(loadfn("glGetFloati_vEXT")),
         GetFloatv: FnPtr::new(loadfn("glGetFloatv")),
         GetFogFuncSGIS: FnPtr::new(loadfn("glGetFogFuncSGIS")),
         GetFragDataIndex: FnPtr::new(loadfn("glGetFragDataIndex")),
         GetFragDataLocation: FnPtr::new(loadfn("glGetFragDataLocation")),
         GetFragDataLocationEXT: FnPtr::new(loadfn("glGetFragDataLocationEXT")),
         GetFragmentLightfvSGIX: FnPtr::new(loadfn("glGetFragmentLightfvSGIX")),
         GetFragmentLightivSGIX: FnPtr::new(loadfn("glGetFragmentLightivSGIX")),
         GetFragmentMaterialfvSGIX: FnPtr::new(loadfn("glGetFragmentMaterialfvSGIX")),
         GetFragmentMaterialivSGIX: FnPtr::new(loadfn("glGetFragmentMaterialivSGIX")),
         GetFramebufferAttachmentParameteriv: FnPtr::new(loadfn("glGetFramebufferAttachmentParameteriv")),
         GetFramebufferAttachmentParameterivEXT: FnPtr::new(loadfn("glGetFramebufferAttachmentParameterivEXT")),
         GetFramebufferParameterfvAMD: FnPtr::new(loadfn("glGetFramebufferParameterfvAMD")),
         GetFramebufferParameteriv: FnPtr::new(loadfn("glGetFramebufferParameteriv")),
         GetFramebufferParameterivEXT: FnPtr::new(loadfn("glGetFramebufferParameterivEXT")),
         GetFramebufferParameterivMESA: FnPtr::new(loadfn("glGetFramebufferParameterivMESA")),
         GetGraphicsResetStatus: FnPtr::new(loadfn("glGetGraphicsResetStatus")),
         GetGraphicsResetStatusARB: FnPtr::new(loadfn("glGetGraphicsResetStatusARB")),
         GetHandleARB: FnPtr::new(loadfn("glGetHandleARB")),
         GetHistogramEXT: FnPtr::new(loadfn("glGetHistogramEXT")),
         GetHistogramParameterfvEXT: FnPtr::new(loadfn("glGetHistogramParameterfvEXT")),
         GetHistogramParameterivEXT: FnPtr::new(loadfn("glGetHistogramParameterivEXT")),
         GetHistogramParameterxvOES: FnPtr::new(loadfn("glGetHistogramParameterxvOES")),
         GetImageHandleARB: FnPtr::new(loadfn("glGetImageHandleARB")),
         GetImageHandleNV: FnPtr::new(loadfn("glGetImageHandleNV")),
         GetImageTransformParameterfvHP: FnPtr::new(loadfn("glGetImageTransformParameterfvHP")),
         GetImageTransformParameterivHP: FnPtr::new(loadfn("glGetImageTransformParameterivHP")),
         GetInfoLogARB: FnPtr::new(loadfn("glGetInfoLogARB")),
         GetInstrumentsSGIX: FnPtr::new(loadfn("glGetInstrumentsSGIX")),
         GetInteger64i_v: FnPtr::new(loadfn("glGetInteger64i_v")),
         GetInteger64v: FnPtr::new(loadfn("glGetInteger64v")),
         GetIntegerIndexedvEXT: FnPtr::new(loadfn("glGetIntegerIndexedvEXT")),
         GetIntegeri_v: FnPtr::new(loadfn("glGetIntegeri_v")),
         GetIntegerui64i_vNV: FnPtr::new(loadfn("glGetIntegerui64i_vNV")),
         GetIntegerui64vNV: FnPtr::new(loadfn("glGetIntegerui64vNV")),
         GetIntegerv: FnPtr::new(loadfn("glGetIntegerv")),
         GetInternalformatSampleivNV: FnPtr::new(loadfn("glGetInternalformatSampleivNV")),
         GetInternalformati64v: FnPtr::new(loadfn("glGetInternalformati64v")),
         GetInternalformativ: FnPtr::new(loadfn("glGetInternalformativ")),
         GetInvariantBooleanvEXT: FnPtr::new(loadfn("glGetInvariantBooleanvEXT")),
         GetInvariantFloatvEXT: FnPtr::new(loadfn("glGetInvariantFloatvEXT")),
         GetInvariantIntegervEXT: FnPtr::new(loadfn("glGetInvariantIntegervEXT")),
         GetLightxOES: FnPtr::new(loadfn("glGetLightxOES")),
         GetListParameterfvSGIX: FnPtr::new(loadfn("glGetListParameterfvSGIX")),
         GetListParameterivSGIX: FnPtr::new(loadfn("glGetListParameterivSGIX")),
         GetLocalConstantBooleanvEXT: FnPtr::new(loadfn("glGetLocalConstantBooleanvEXT")),
         GetLocalConstantFloatvEXT: FnPtr::new(loadfn("glGetLocalConstantFloatvEXT")),
         GetLocalConstantIntegervEXT: FnPtr::new(loadfn("glGetLocalConstantIntegervEXT")),
         GetMapAttribParameterfvNV: FnPtr::new(loadfn("glGetMapAttribParameterfvNV")),
         GetMapAttribParameterivNV: FnPtr::new(loadfn("glGetMapAttribParameterivNV")),
         GetMapControlPointsNV: FnPtr::new(loadfn("glGetMapControlPointsNV")),
         GetMapParameterfvNV: FnPtr::new(loadfn("glGetMapParameterfvNV")),
         GetMapParameterivNV: FnPtr::new(loadfn("glGetMapParameterivNV")),
         GetMapxvOES: FnPtr::new(loadfn("glGetMapxvOES")),
         GetMaterialxOES: FnPtr::new(loadfn("glGetMaterialxOES")),
         GetMemoryObjectDetachedResourcesuivNV: FnPtr::new(loadfn("glGetMemoryObjectDetachedResourcesuivNV")),
         GetMemoryObjectParameterivEXT: FnPtr::new(loadfn("glGetMemoryObjectParameterivEXT")),
         GetMinmaxEXT: FnPtr::new(loadfn("glGetMinmaxEXT")),
         GetMinmaxParameterfvEXT: FnPtr::new(loadfn("glGetMinmaxParameterfvEXT")),
         GetMinmaxParameterivEXT: FnPtr::new(loadfn("glGetMinmaxParameterivEXT")),
         GetMultiTexEnvfvEXT: FnPtr::new(loadfn("glGetMultiTexEnvfvEXT")),
         GetMultiTexEnvivEXT: FnPtr::new(loadfn("glGetMultiTexEnvivEXT")),
         GetMultiTexGendvEXT: FnPtr::new(loadfn("glGetMultiTexGendvEXT")),
         GetMultiTexGenfvEXT: FnPtr::new(loadfn("glGetMultiTexGenfvEXT")),
         GetMultiTexGenivEXT: FnPtr::new(loadfn("glGetMultiTexGenivEXT")),
         GetMultiTexImageEXT: FnPtr::new(loadfn("glGetMultiTexImageEXT")),
         GetMultiTexLevelParameterfvEXT: FnPtr::new(loadfn("glGetMultiTexLevelParameterfvEXT")),
         GetMultiTexLevelParameterivEXT: FnPtr::new(loadfn("glGetMultiTexLevelParameterivEXT")),
         GetMultiTexParameterIivEXT: FnPtr::new(loadfn("glGetMultiTexParameterIivEXT")),
         GetMultiTexParameterIuivEXT: FnPtr::new(loadfn("glGetMultiTexParameterIuivEXT")),
         GetMultiTexParameterfvEXT: FnPtr::new(loadfn("glGetMultiTexParameterfvEXT")),
         GetMultiTexParameterivEXT: FnPtr::new(loadfn("glGetMultiTexParameterivEXT")),
         GetMultisamplefv: FnPtr::new(loadfn("glGetMultisamplefv")),
         GetMultisamplefvNV: FnPtr::new(loadfn("glGetMultisamplefvNV")),
         GetNamedBufferParameteri64v: FnPtr::new(loadfn("glGetNamedBufferParameteri64v")),
         GetNamedBufferParameteriv: FnPtr::new(loadfn("glGetNamedBufferParameteriv")),
         GetNamedBufferParameterivEXT: FnPtr::new(loadfn("glGetNamedBufferParameterivEXT")),
         GetNamedBufferParameterui64vNV: FnPtr::new(loadfn("glGetNamedBufferParameterui64vNV")),
         GetNamedBufferPointerv: FnPtr::new(loadfn("glGetNamedBufferPointerv")),
         GetNamedBufferPointervEXT: FnPtr::new(loadfn("glGetNamedBufferPointervEXT")),
         GetNamedBufferSubData: FnPtr::new(loadfn("glGetNamedBufferSubData")),
         GetNamedBufferSubDataEXT: FnPtr::new(loadfn("glGetNamedBufferSubDataEXT")),
         GetNamedFramebufferAttachmentParameteriv: FnPtr::new(loadfn("glGetNamedFramebufferAttachmentParameteriv")),
         GetNamedFramebufferAttachmentParameterivEXT: FnPtr::new(loadfn("glGetNamedFramebufferAttachmentParameterivEXT")),
         GetNamedFramebufferParameterfvAMD: FnPtr::new(loadfn("glGetNamedFramebufferParameterfvAMD")),
         GetNamedFramebufferParameteriv: FnPtr::new(loadfn("glGetNamedFramebufferParameteriv")),
         GetNamedFramebufferParameterivEXT: FnPtr::new(loadfn("glGetNamedFramebufferParameterivEXT")),
         GetNamedProgramLocalParameterIivEXT: FnPtr::new(loadfn("glGetNamedProgramLocalParameterIivEXT")),
         GetNamedProgramLocalParameterIuivEXT: FnPtr::new(loadfn("glGetNamedProgramLocalParameterIuivEXT")),
         GetNamedProgramLocalParameterdvEXT: FnPtr::new(loadfn("glGetNamedProgramLocalParameterdvEXT")),
         GetNamedProgramLocalParameterfvEXT: FnPtr::new(loadfn("glGetNamedProgramLocalParameterfvEXT")),
         GetNamedProgramStringEXT: FnPtr::new(loadfn("glGetNamedProgramStringEXT")),
         GetNamedProgramivEXT: FnPtr::new(loadfn("glGetNamedProgramivEXT")),
         GetNamedRenderbufferParameteriv: FnPtr::new(loadfn("glGetNamedRenderbufferParameteriv")),
         GetNamedRenderbufferParameterivEXT: FnPtr::new(loadfn("glGetNamedRenderbufferParameterivEXT")),
         GetNamedStringARB: FnPtr::new(loadfn("glGetNamedStringARB")),
         GetNamedStringivARB: FnPtr::new(loadfn("glGetNamedStringivARB")),
         GetNextPerfQueryIdINTEL: FnPtr::new(loadfn("glGetNextPerfQueryIdINTEL")),
         GetObjectBufferfvATI: FnPtr::new(loadfn("glGetObjectBufferfvATI")),
         GetObjectBufferivATI: FnPtr::new(loadfn("glGetObjectBufferivATI")),
         GetObjectLabel: FnPtr::new(loadfn("glGetObjectLabel")),
         GetObjectLabelEXT: FnPtr::new(loadfn("glGetObjectLabelEXT")),
         GetObjectParameterfvARB: FnPtr::new(loadfn("glGetObjectParameterfvARB")),
         GetObjectParameterivAPPLE: FnPtr::new(loadfn("glGetObjectParameterivAPPLE")),
         GetObjectParameterivARB: FnPtr::new(loadfn("glGetObjectParameterivARB")),
         GetObjectPtrLabel: FnPtr::new(loadfn("glGetObjectPtrLabel")),
         GetOcclusionQueryivNV: FnPtr::new(loadfn("glGetOcclusionQueryivNV")),
         GetOcclusionQueryuivNV: FnPtr::new(loadfn("glGetOcclusionQueryuivNV")),
         GetPathCommandsNV: FnPtr::new(loadfn("glGetPathCommandsNV")),
         GetPathCoordsNV: FnPtr::new(loadfn("glGetPathCoordsNV")),
         GetPathDashArrayNV: FnPtr::new(loadfn("glGetPathDashArrayNV")),
         GetPathLengthNV: FnPtr::new(loadfn("glGetPathLengthNV")),
         GetPathMetricRangeNV: FnPtr::new(loadfn("glGetPathMetricRangeNV")),
         GetPathMetricsNV: FnPtr::new(loadfn("glGetPathMetricsNV")),
         GetPathParameterfvNV: FnPtr::new(loadfn("glGetPathParameterfvNV")),
         GetPathParameterivNV: FnPtr::new(loadfn("glGetPathParameterivNV")),
         GetPathSpacingNV: FnPtr::new(loadfn("glGetPathSpacingNV")),
         GetPerfCounterInfoINTEL: FnPtr::new(loadfn("glGetPerfCounterInfoINTEL")),
         GetPerfMonitorCounterDataAMD: FnPtr::new(loadfn("glGetPerfMonitorCounterDataAMD")),
         GetPerfMonitorCounterInfoAMD: FnPtr::new(loadfn("glGetPerfMonitorCounterInfoAMD")),
         GetPerfMonitorCounterStringAMD: FnPtr::new(loadfn("glGetPerfMonitorCounterStringAMD")),
         GetPerfMonitorCountersAMD: FnPtr::new(loadfn("glGetPerfMonitorCountersAMD")),
         GetPerfMonitorGroupStringAMD: FnPtr::new(loadfn("glGetPerfMonitorGroupStringAMD")),
         GetPerfMonitorGroupsAMD: FnPtr::new(loadfn("glGetPerfMonitorGroupsAMD")),
         GetPerfQueryDataINTEL: FnPtr::new(loadfn("glGetPerfQueryDataINTEL")),
         GetPerfQueryIdByNameINTEL: FnPtr::new(loadfn("glGetPerfQueryIdByNameINTEL")),
         GetPerfQueryInfoINTEL: FnPtr::new(loadfn("glGetPerfQueryInfoINTEL")),
         GetPixelMapxv: FnPtr::new(loadfn("glGetPixelMapxv")),
         GetPixelTexGenParameterfvSGIS: FnPtr::new(loadfn("glGetPixelTexGenParameterfvSGIS")),
         GetPixelTexGenParameterivSGIS: FnPtr::new(loadfn("glGetPixelTexGenParameterivSGIS")),
         GetPixelTransformParameterfvEXT: FnPtr::new(loadfn("glGetPixelTransformParameterfvEXT")),
         GetPixelTransformParameterivEXT: FnPtr::new(loadfn("glGetPixelTransformParameterivEXT")),
         GetPointerIndexedvEXT: FnPtr::new(loadfn("glGetPointerIndexedvEXT")),
         GetPointeri_vEXT: FnPtr::new(loadfn("glGetPointeri_vEXT")),
         GetPointerv: FnPtr::new(loadfn("glGetPointerv")),
         GetPointervEXT: FnPtr::new(loadfn("glGetPointervEXT")),
         GetProgramBinary: FnPtr::new(loadfn("glGetProgramBinary")),
         GetProgramEnvParameterIivNV: FnPtr::new(loadfn("glGetProgramEnvParameterIivNV")),
         GetProgramEnvParameterIuivNV: FnPtr::new(loadfn("glGetProgramEnvParameterIuivNV")),
         GetProgramEnvParameterdvARB: FnPtr::new(loadfn("glGetProgramEnvParameterdvARB")),
         GetProgramEnvParameterfvARB: FnPtr::new(loadfn("glGetProgramEnvParameterfvARB")),
         GetProgramInfoLog: FnPtr::new(loadfn("glGetProgramInfoLog")),
         GetProgramInterfaceiv: FnPtr::new(loadfn("glGetProgramInterfaceiv")),
         GetProgramLocalParameterIivNV: FnPtr::new(loadfn("glGetProgramLocalParameterIivNV")),
         GetProgramLocalParameterIuivNV: FnPtr::new(loadfn("glGetProgramLocalParameterIuivNV")),
         GetProgramLocalParameterdvARB: FnPtr::new(loadfn("glGetProgramLocalParameterdvARB")),
         GetProgramLocalParameterfvARB: FnPtr::new(loadfn("glGetProgramLocalParameterfvARB")),
         GetProgramNamedParameterdvNV: FnPtr::new(loadfn("glGetProgramNamedParameterdvNV")),
         GetProgramNamedParameterfvNV: FnPtr::new(loadfn("glGetProgramNamedParameterfvNV")),
         GetProgramParameterdvNV: FnPtr::new(loadfn("glGetProgramParameterdvNV")),
         GetProgramParameterfvNV: FnPtr::new(loadfn("glGetProgramParameterfvNV")),
         GetProgramPipelineInfoLog: FnPtr::new(loadfn("glGetProgramPipelineInfoLog")),
         GetProgramPipelineiv: FnPtr::new(loadfn("glGetProgramPipelineiv")),
         GetProgramResourceIndex: FnPtr::new(loadfn("glGetProgramResourceIndex")),
         GetProgramResourceLocation: FnPtr::new(loadfn("glGetProgramResourceLocation")),
         GetProgramResourceLocationIndex: FnPtr::new(loadfn("glGetProgramResourceLocationIndex")),
         GetProgramResourceName: FnPtr::new(loadfn("glGetProgramResourceName")),
         GetProgramResourcefvNV: FnPtr::new(loadfn("glGetProgramResourcefvNV")),
         GetProgramResourceiv: FnPtr::new(loadfn("glGetProgramResourceiv")),
         GetProgramStageiv: FnPtr::new(loadfn("glGetProgramStageiv")),
         GetProgramStringARB: FnPtr::new(loadfn("glGetProgramStringARB")),
         GetProgramStringNV: FnPtr::new(loadfn("glGetProgramStringNV")),
         GetProgramSubroutineParameteruivNV: FnPtr::new(loadfn("glGetProgramSubroutineParameteruivNV")),
         GetProgramiv: FnPtr::new(loadfn("glGetProgramiv")),
         GetProgramivARB: FnPtr::new(loadfn("glGetProgramivARB")),
         GetProgramivNV: FnPtr::new(loadfn("glGetProgramivNV")),
         GetQueryBufferObjecti64v: FnPtr::new(loadfn("glGetQueryBufferObjecti64v")),
         GetQueryBufferObjectiv: FnPtr::new(loadfn("glGetQueryBufferObjectiv")),
         GetQueryBufferObjectui64v: FnPtr::new(loadfn("glGetQueryBufferObjectui64v")),
         GetQueryBufferObjectuiv: FnPtr::new(loadfn("glGetQueryBufferObjectuiv")),
         GetQueryIndexediv: FnPtr::new(loadfn("glGetQueryIndexediv")),
         GetQueryObjecti64v: FnPtr::new(loadfn("glGetQueryObjecti64v")),
         GetQueryObjecti64vEXT: FnPtr::new(loadfn("glGetQueryObjecti64vEXT")),
         GetQueryObjectiv: FnPtr::new(loadfn("glGetQueryObjectiv")),
         GetQueryObjectivARB: FnPtr::new(loadfn("glGetQueryObjectivARB")),
         GetQueryObjectui64v: FnPtr::new(loadfn("glGetQueryObjectui64v")),
         GetQueryObjectui64vEXT: FnPtr::new(loadfn("glGetQueryObjectui64vEXT")),
         GetQueryObjectuiv: FnPtr::new(loadfn("glGetQueryObjectuiv")),
         GetQueryObjectuivARB: FnPtr::new(loadfn("glGetQueryObjectuivARB")),
         GetQueryiv: FnPtr::new(loadfn("glGetQueryiv")),
         GetQueryivARB: FnPtr::new(loadfn("glGetQueryivARB")),
         GetRenderbufferParameteriv: FnPtr::new(loadfn("glGetRenderbufferParameteriv")),
         GetRenderbufferParameterivEXT: FnPtr::new(loadfn("glGetRenderbufferParameterivEXT")),
         GetSamplerParameterIiv: FnPtr::new(loadfn("glGetSamplerParameterIiv")),
         GetSamplerParameterIuiv: FnPtr::new(loadfn("glGetSamplerParameterIuiv")),
         GetSamplerParameterfv: FnPtr::new(loadfn("glGetSamplerParameterfv")),
         GetSamplerParameteriv: FnPtr::new(loadfn("glGetSamplerParameteriv")),
         GetSemaphoreParameterivNV: FnPtr::new(loadfn("glGetSemaphoreParameterivNV")),
         GetSemaphoreParameterui64vEXT: FnPtr::new(loadfn("glGetSemaphoreParameterui64vEXT")),
         GetSeparableFilterEXT: FnPtr::new(loadfn("glGetSeparableFilterEXT")),
         GetShaderInfoLog: FnPtr::new(loadfn("glGetShaderInfoLog")),
         GetShaderPrecisionFormat: FnPtr::new(loadfn("glGetShaderPrecisionFormat")),
         GetShaderSource: FnPtr::new(loadfn("glGetShaderSource")),
         GetShaderSourceARB: FnPtr::new(loadfn("glGetShaderSourceARB")),
         GetShaderiv: FnPtr::new(loadfn("glGetShaderiv")),
         GetShadingRateImagePaletteNV: FnPtr::new(loadfn("glGetShadingRateImagePaletteNV")),
         GetShadingRateSampleLocationivNV: FnPtr::new(loadfn("glGetShadingRateSampleLocationivNV")),
         GetSharpenTexFuncSGIS: FnPtr::new(loadfn("glGetSharpenTexFuncSGIS")),
         GetStageIndexNV: FnPtr::new(loadfn("glGetStageIndexNV")),
         GetString: FnPtr::new(loadfn("glGetString")),
         GetStringi: FnPtr::new(loadfn("glGetStringi")),
         GetSubroutineIndex: FnPtr::new(loadfn("glGetSubroutineIndex")),
         GetSubroutineUniformLocation: FnPtr::new(loadfn("glGetSubroutineUniformLocation")),
         GetSynciv: FnPtr::new(loadfn("glGetSynciv")),
         GetTexBumpParameterfvATI: FnPtr::new(loadfn("glGetTexBumpParameterfvATI")),
         GetTexBumpParameterivATI: FnPtr::new(loadfn("glGetTexBumpParameterivATI")),
         GetTexEnvxvOES: FnPtr::new(loadfn("glGetTexEnvxvOES")),
         GetTexFilterFuncSGIS: FnPtr::new(loadfn("glGetTexFilterFuncSGIS")),
         GetTexGenxvOES: FnPtr::new(loadfn("glGetTexGenxvOES")),
         GetTexImage: FnPtr::new(loadfn("glGetTexImage")),
         GetTexLevelParameterfv: FnPtr::new(loadfn("glGetTexLevelParameterfv")),
         GetTexLevelParameteriv: FnPtr::new(loadfn("glGetTexLevelParameteriv")),
         GetTexLevelParameterxvOES: FnPtr::new(loadfn("glGetTexLevelParameterxvOES")),
         GetTexParameterIiv: FnPtr::new(loadfn("glGetTexParameterIiv")),
         GetTexParameterIivEXT: FnPtr::new(loadfn("glGetTexParameterIivEXT")),
         GetTexParameterIuiv: FnPtr::new(loadfn("glGetTexParameterIuiv")),
         GetTexParameterIuivEXT: FnPtr::new(loadfn("glGetTexParameterIuivEXT")),
         GetTexParameterPointervAPPLE: FnPtr::new(loadfn("glGetTexParameterPointervAPPLE")),
         GetTexParameterfv: FnPtr::new(loadfn("glGetTexParameterfv")),
         GetTexParameteriv: FnPtr::new(loadfn("glGetTexParameteriv")),
         GetTexParameterxvOES: FnPtr::new(loadfn("glGetTexParameterxvOES")),
         GetTextureHandleARB: FnPtr::new(loadfn("glGetTextureHandleARB")),
         GetTextureHandleNV: FnPtr::new(loadfn("glGetTextureHandleNV")),
         GetTextureImage: FnPtr::new(loadfn("glGetTextureImage")),
         GetTextureImageEXT: FnPtr::new(loadfn("glGetTextureImageEXT")),
         GetTextureLevelParameterfv: FnPtr::new(loadfn("glGetTextureLevelParameterfv")),
         GetTextureLevelParameterfvEXT: FnPtr::new(loadfn("glGetTextureLevelParameterfvEXT")),
         GetTextureLevelParameteriv: FnPtr::new(loadfn("glGetTextureLevelParameteriv")),
         GetTextureLevelParameterivEXT: FnPtr::new(loadfn("glGetTextureLevelParameterivEXT")),
         GetTextureParameterIiv: FnPtr::new(loadfn("glGetTextureParameterIiv")),
         GetTextureParameterIivEXT: FnPtr::new(loadfn("glGetTextureParameterIivEXT")),
         GetTextureParameterIuiv: FnPtr::new(loadfn("glGetTextureParameterIuiv")),
         GetTextureParameterIuivEXT: FnPtr::new(loadfn("glGetTextureParameterIuivEXT")),
         GetTextureParameterfv: FnPtr::new(loadfn("glGetTextureParameterfv")),
         GetTextureParameterfvEXT: FnPtr::new(loadfn("glGetTextureParameterfvEXT")),
         GetTextureParameteriv: FnPtr::new(loadfn("glGetTextureParameteriv")),
         GetTextureParameterivEXT: FnPtr::new(loadfn("glGetTextureParameterivEXT")),
         GetTextureSamplerHandleARB: FnPtr::new(loadfn("glGetTextureSamplerHandleARB")),
         GetTextureSamplerHandleNV: FnPtr::new(loadfn("glGetTextureSamplerHandleNV")),
         GetTextureSubImage: FnPtr::new(loadfn("glGetTextureSubImage")),
         GetTrackMatrixivNV: FnPtr::new(loadfn("glGetTrackMatrixivNV")),
         GetTransformFeedbackVarying: FnPtr::new(loadfn("glGetTransformFeedbackVarying")),
         GetTransformFeedbackVaryingEXT: FnPtr::new(loadfn("glGetTransformFeedbackVaryingEXT")),
         GetTransformFeedbackVaryingNV: FnPtr::new(loadfn("glGetTransformFeedbackVaryingNV")),
         GetTransformFeedbacki64_v: FnPtr::new(loadfn("glGetTransformFeedbacki64_v")),
         GetTransformFeedbacki_v: FnPtr::new(loadfn("glGetTransformFeedbacki_v")),
         GetTransformFeedbackiv: FnPtr::new(loadfn("glGetTransformFeedbackiv")),
         GetUniformBlockIndex: FnPtr::new(loadfn("glGetUniformBlockIndex")),
         GetUniformBufferSizeEXT: FnPtr::new(loadfn("glGetUniformBufferSizeEXT")),
         GetUniformIndices: FnPtr::new(loadfn("glGetUniformIndices")),
         GetUniformLocation: FnPtr::new(loadfn("glGetUniformLocation")),
         GetUniformLocationARB: FnPtr::new(loadfn("glGetUniformLocationARB")),
         GetUniformOffsetEXT: FnPtr::new(loadfn("glGetUniformOffsetEXT")),
         GetUniformSubroutineuiv: FnPtr::new(loadfn("glGetUniformSubroutineuiv")),
         GetUniformdv: FnPtr::new(loadfn("glGetUniformdv")),
         GetUniformfv: FnPtr::new(loadfn("glGetUniformfv")),
         GetUniformfvARB: FnPtr::new(loadfn("glGetUniformfvARB")),
         GetUniformi64vARB: FnPtr::new(loadfn("glGetUniformi64vARB")),
         GetUniformi64vNV: FnPtr::new(loadfn("glGetUniformi64vNV")),
         GetUniformiv: FnPtr::new(loadfn("glGetUniformiv")),
         GetUniformivARB: FnPtr::new(loadfn("glGetUniformivARB")),
         GetUniformui64vARB: FnPtr::new(loadfn("glGetUniformui64vARB")),
         GetUniformui64vNV: FnPtr::new(loadfn("glGetUniformui64vNV")),
         GetUniformuiv: FnPtr::new(loadfn("glGetUniformuiv")),
         GetUniformuivEXT: FnPtr::new(loadfn("glGetUniformuivEXT")),
         GetUnsignedBytei_vEXT: FnPtr::new(loadfn("glGetUnsignedBytei_vEXT")),
         GetUnsignedBytevEXT: FnPtr::new(loadfn("glGetUnsignedBytevEXT")),
         GetVariantArrayObjectfvATI: FnPtr::new(loadfn("glGetVariantArrayObjectfvATI")),
         GetVariantArrayObjectivATI: FnPtr::new(loadfn("glGetVariantArrayObjectivATI")),
         GetVariantBooleanvEXT: FnPtr::new(loadfn("glGetVariantBooleanvEXT")),
         GetVariantFloatvEXT: FnPtr::new(loadfn("glGetVariantFloatvEXT")),
         GetVariantIntegervEXT: FnPtr::new(loadfn("glGetVariantIntegervEXT")),
         GetVariantPointervEXT: FnPtr::new(loadfn("glGetVariantPointervEXT")),
         GetVaryingLocationNV: FnPtr::new(loadfn("glGetVaryingLocationNV")),
         GetVertexArrayIndexed64iv: FnPtr::new(loadfn("glGetVertexArrayIndexed64iv")),
         GetVertexArrayIndexediv: FnPtr::new(loadfn("glGetVertexArrayIndexediv")),
         GetVertexArrayIntegeri_vEXT: FnPtr::new(loadfn("glGetVertexArrayIntegeri_vEXT")),
         GetVertexArrayIntegervEXT: FnPtr::new(loadfn("glGetVertexArrayIntegervEXT")),
         GetVertexArrayPointeri_vEXT: FnPtr::new(loadfn("glGetVertexArrayPointeri_vEXT")),
         GetVertexArrayPointervEXT: FnPtr::new(loadfn("glGetVertexArrayPointervEXT")),
         GetVertexArrayiv: FnPtr::new(loadfn("glGetVertexArrayiv")),
         GetVertexAttribArrayObjectfvATI: FnPtr::new(loadfn("glGetVertexAttribArrayObjectfvATI")),
         GetVertexAttribArrayObjectivATI: FnPtr::new(loadfn("glGetVertexAttribArrayObjectivATI")),
         GetVertexAttribIiv: FnPtr::new(loadfn("glGetVertexAttribIiv")),
         GetVertexAttribIivEXT: FnPtr::new(loadfn("glGetVertexAttribIivEXT")),
         GetVertexAttribIuiv: FnPtr::new(loadfn("glGetVertexAttribIuiv")),
         GetVertexAttribIuivEXT: FnPtr::new(loadfn("glGetVertexAttribIuivEXT")),
         GetVertexAttribLdv: FnPtr::new(loadfn("glGetVertexAttribLdv")),
         GetVertexAttribLdvEXT: FnPtr::new(loadfn("glGetVertexAttribLdvEXT")),
         GetVertexAttribLi64vNV: FnPtr::new(loadfn("glGetVertexAttribLi64vNV")),
         GetVertexAttribLui64vARB: FnPtr::new(loadfn("glGetVertexAttribLui64vARB")),
         GetVertexAttribLui64vNV: FnPtr::new(loadfn("glGetVertexAttribLui64vNV")),
         GetVertexAttribPointerv: FnPtr::new(loadfn("glGetVertexAttribPointerv")),
         GetVertexAttribPointervARB: FnPtr::new(loadfn("glGetVertexAttribPointervARB")),
         GetVertexAttribPointervNV: FnPtr::new(loadfn("glGetVertexAttribPointervNV")),
         GetVertexAttribdv: FnPtr::new(loadfn("glGetVertexAttribdv")),
         GetVertexAttribdvARB: FnPtr::new(loadfn("glGetVertexAttribdvARB")),
         GetVertexAttribdvNV: FnPtr::new(loadfn("glGetVertexAttribdvNV")),
         GetVertexAttribfv: FnPtr::new(loadfn("glGetVertexAttribfv")),
         GetVertexAttribfvARB: FnPtr::new(loadfn("glGetVertexAttribfvARB")),
         GetVertexAttribfvNV: FnPtr::new(loadfn("glGetVertexAttribfvNV")),
         GetVertexAttribiv: FnPtr::new(loadfn("glGetVertexAttribiv")),
         GetVertexAttribivARB: FnPtr::new(loadfn("glGetVertexAttribivARB")),
         GetVertexAttribivNV: FnPtr::new(loadfn("glGetVertexAttribivNV")),
         GetVideoCaptureStreamdvNV: FnPtr::new(loadfn("glGetVideoCaptureStreamdvNV")),
         GetVideoCaptureStreamfvNV: FnPtr::new(loadfn("glGetVideoCaptureStreamfvNV")),
         GetVideoCaptureStreamivNV: FnPtr::new(loadfn("glGetVideoCaptureStreamivNV")),
         GetVideoCaptureivNV: FnPtr::new(loadfn("glGetVideoCaptureivNV")),
         GetVideoi64vNV: FnPtr::new(loadfn("glGetVideoi64vNV")),
         GetVideoivNV: FnPtr::new(loadfn("glGetVideoivNV")),
         GetVideoui64vNV: FnPtr::new(loadfn("glGetVideoui64vNV")),
         GetVideouivNV: FnPtr::new(loadfn("glGetVideouivNV")),
         GetVkProcAddrNV: FnPtr::new(loadfn("glGetVkProcAddrNV")),
         GetnCompressedTexImage: FnPtr::new(loadfn("glGetnCompressedTexImage")),
         GetnCompressedTexImageARB: FnPtr::new(loadfn("glGetnCompressedTexImageARB")),
         GetnTexImage: FnPtr::new(loadfn("glGetnTexImage")),
         GetnTexImageARB: FnPtr::new(loadfn("glGetnTexImageARB")),
         GetnUniformdv: FnPtr::new(loadfn("glGetnUniformdv")),
         GetnUniformdvARB: FnPtr::new(loadfn("glGetnUniformdvARB")),
         GetnUniformfv: FnPtr::new(loadfn("glGetnUniformfv")),
         GetnUniformfvARB: FnPtr::new(loadfn("glGetnUniformfvARB")),
         GetnUniformi64vARB: FnPtr::new(loadfn("glGetnUniformi64vARB")),
         GetnUniformiv: FnPtr::new(loadfn("glGetnUniformiv")),
         GetnUniformivARB: FnPtr::new(loadfn("glGetnUniformivARB")),
         GetnUniformui64vARB: FnPtr::new(loadfn("glGetnUniformui64vARB")),
         GetnUniformuiv: FnPtr::new(loadfn("glGetnUniformuiv")),
         GetnUniformuivARB: FnPtr::new(loadfn("glGetnUniformuivARB")),
         GlobalAlphaFactorbSUN: FnPtr::new(loadfn("glGlobalAlphaFactorbSUN")),
         GlobalAlphaFactordSUN: FnPtr::new(loadfn("glGlobalAlphaFactordSUN")),
         GlobalAlphaFactorfSUN: FnPtr::new(loadfn("glGlobalAlphaFactorfSUN")),
         GlobalAlphaFactoriSUN: FnPtr::new(loadfn("glGlobalAlphaFactoriSUN")),
         GlobalAlphaFactorsSUN: FnPtr::new(loadfn("glGlobalAlphaFactorsSUN")),
         GlobalAlphaFactorubSUN: FnPtr::new(loadfn("glGlobalAlphaFactorubSUN")),
         GlobalAlphaFactoruiSUN: FnPtr::new(loadfn("glGlobalAlphaFactoruiSUN")),
         GlobalAlphaFactorusSUN: FnPtr::new(loadfn("glGlobalAlphaFactorusSUN")),
         Hint: FnPtr::new(loadfn("glHint")),
         HintPGI: FnPtr::new(loadfn("glHintPGI")),
         HistogramEXT: FnPtr::new(loadfn("glHistogramEXT")),
         IglooInterfaceSGIX: FnPtr::new(loadfn("glIglooInterfaceSGIX")),
         ImageTransformParameterfHP: FnPtr::new(loadfn("glImageTransformParameterfHP")),
         ImageTransformParameterfvHP: FnPtr::new(loadfn("glImageTransformParameterfvHP")),
         ImageTransformParameteriHP: FnPtr::new(loadfn("glImageTransformParameteriHP")),
         ImageTransformParameterivHP: FnPtr::new(loadfn("glImageTransformParameterivHP")),
         ImportMemoryFdEXT: FnPtr::new(loadfn("glImportMemoryFdEXT")),
         ImportMemoryWin32HandleEXT: FnPtr::new(loadfn("glImportMemoryWin32HandleEXT")),
         ImportMemoryWin32NameEXT: FnPtr::new(loadfn("glImportMemoryWin32NameEXT")),
         ImportSemaphoreFdEXT: FnPtr::new(loadfn("glImportSemaphoreFdEXT")),
         ImportSemaphoreWin32HandleEXT: FnPtr::new(loadfn("glImportSemaphoreWin32HandleEXT")),
         ImportSemaphoreWin32NameEXT: FnPtr::new(loadfn("glImportSemaphoreWin32NameEXT")),
         ImportSyncEXT: FnPtr::new(loadfn("glImportSyncEXT")),
         IndexFormatNV: FnPtr::new(loadfn("glIndexFormatNV")),
         IndexFuncEXT: FnPtr::new(loadfn("glIndexFuncEXT")),
         IndexMaterialEXT: FnPtr::new(loadfn("glIndexMaterialEXT")),
         IndexPointerEXT: FnPtr::new(loadfn("glIndexPointerEXT")),
         IndexPointerListIBM: FnPtr::new(loadfn("glIndexPointerListIBM")),
         IndexxOES: FnPtr::new(loadfn("glIndexxOES")),
         IndexxvOES: FnPtr::new(loadfn("glIndexxvOES")),
         InsertComponentEXT: FnPtr::new(loadfn("glInsertComponentEXT")),
         InsertEventMarkerEXT: FnPtr::new(loadfn("glInsertEventMarkerEXT")),
         InstrumentsBufferSGIX: FnPtr::new(loadfn("glInstrumentsBufferSGIX")),
         InterpolatePathsNV: FnPtr::new(loadfn("glInterpolatePathsNV")),
         InvalidateBufferData: FnPtr::new(loadfn("glInvalidateBufferData")),
         InvalidateBufferSubData: FnPtr::new(loadfn("glInvalidateBufferSubData")),
         InvalidateFramebuffer: FnPtr::new(loadfn("glInvalidateFramebuffer")),
         InvalidateNamedFramebufferData: FnPtr::new(loadfn("glInvalidateNamedFramebufferData")),
         InvalidateNamedFramebufferSubData: FnPtr::new(loadfn("glInvalidateNamedFramebufferSubData")),
         InvalidateSubFramebuffer: FnPtr::new(loadfn("glInvalidateSubFramebuffer")),
         InvalidateTexImage: FnPtr::new(loadfn("glInvalidateTexImage")),
         InvalidateTexSubImage: FnPtr::new(loadfn("glInvalidateTexSubImage")),
         IsAsyncMarkerSGIX: FnPtr::new(loadfn("glIsAsyncMarkerSGIX")),
         IsBuffer: FnPtr::new(loadfn("glIsBuffer")),
         IsBufferARB: FnPtr::new(loadfn("glIsBufferARB")),
         IsBufferResidentNV: FnPtr::new(loadfn("glIsBufferResidentNV")),
         IsCommandListNV: FnPtr::new(loadfn("glIsCommandListNV")),
         IsEnabled: FnPtr::new(loadfn("glIsEnabled")),
         IsEnabledIndexedEXT: FnPtr::new(loadfn("glIsEnabledIndexedEXT")),
         IsEnabledi: FnPtr::new(loadfn("glIsEnabledi")),
         IsFenceAPPLE: FnPtr::new(loadfn("glIsFenceAPPLE")),
         IsFenceNV: FnPtr::new(loadfn("glIsFenceNV")),
         IsFramebuffer: FnPtr::new(loadfn("glIsFramebuffer")),
         IsFramebufferEXT: FnPtr::new(loadfn("glIsFramebufferEXT")),
         IsImageHandleResidentARB: FnPtr::new(loadfn("glIsImageHandleResidentARB")),
         IsImageHandleResidentNV: FnPtr::new(loadfn("glIsImageHandleResidentNV")),
         IsMemoryObjectEXT: FnPtr::new(loadfn("glIsMemoryObjectEXT")),
         IsNameAMD: FnPtr::new(loadfn("glIsNameAMD")),
         IsNamedBufferResidentNV: FnPtr::new(loadfn("glIsNamedBufferResidentNV")),
         IsNamedStringARB: FnPtr::new(loadfn("glIsNamedStringARB")),
         IsObjectBufferATI: FnPtr::new(loadfn("glIsObjectBufferATI")),
         IsOcclusionQueryNV: FnPtr::new(loadfn("glIsOcclusionQueryNV")),
         IsPathNV: FnPtr::new(loadfn("glIsPathNV")),
         IsPointInFillPathNV: FnPtr::new(loadfn("glIsPointInFillPathNV")),
         IsPointInStrokePathNV: FnPtr::new(loadfn("glIsPointInStrokePathNV")),
         IsProgram: FnPtr::new(loadfn("glIsProgram")),
         IsProgramARB: FnPtr::new(loadfn("glIsProgramARB")),
         IsProgramNV: FnPtr::new(loadfn("glIsProgramNV")),
         IsProgramPipeline: FnPtr::new(loadfn("glIsProgramPipeline")),
         IsQuery: FnPtr::new(loadfn("glIsQuery")),
         IsQueryARB: FnPtr::new(loadfn("glIsQueryARB")),
         IsRenderbuffer: FnPtr::new(loadfn("glIsRenderbuffer")),
         IsRenderbufferEXT: FnPtr::new(loadfn("glIsRenderbufferEXT")),
         IsSampler: FnPtr::new(loadfn("glIsSampler")),
         IsSemaphoreEXT: FnPtr::new(loadfn("glIsSemaphoreEXT")),
         IsShader: FnPtr::new(loadfn("glIsShader")),
         IsStateNV: FnPtr::new(loadfn("glIsStateNV")),
         IsSync: FnPtr::new(loadfn("glIsSync")),
         IsTexture: FnPtr::new(loadfn("glIsTexture")),
         IsTextureEXT: FnPtr::new(loadfn("glIsTextureEXT")),
         IsTextureHandleResidentARB: FnPtr::new(loadfn("glIsTextureHandleResidentARB")),
         IsTextureHandleResidentNV: FnPtr::new(loadfn("glIsTextureHandleResidentNV")),
         IsTransformFeedback: FnPtr::new(loadfn("glIsTransformFeedback")),
         IsTransformFeedbackNV: FnPtr::new(loadfn("glIsTransformFeedbackNV")),
         IsVariantEnabledEXT: FnPtr::new(loadfn("glIsVariantEnabledEXT")),
         IsVertexArray: FnPtr::new(loadfn("glIsVertexArray")),
         IsVertexArrayAPPLE: FnPtr::new(loadfn("glIsVertexArrayAPPLE")),
         IsVertexAttribEnabledAPPLE: FnPtr::new(loadfn("glIsVertexAttribEnabledAPPLE")),
         LGPUCopyImageSubDataNVX: FnPtr::new(loadfn("glLGPUCopyImageSubDataNVX")),
         LGPUInterlockNVX: FnPtr::new(loadfn("glLGPUInterlockNVX")),
         LGPUNamedBufferSubDataNVX: FnPtr::new(loadfn("glLGPUNamedBufferSubDataNVX")),
         LabelObjectEXT: FnPtr::new(loadfn("glLabelObjectEXT")),
         LightEnviSGIX: FnPtr::new(loadfn("glLightEnviSGIX")),
         LightModelxOES: FnPtr::new(loadfn("glLightModelxOES")),
         LightModelxvOES: FnPtr::new(loadfn("glLightModelxvOES")),
         LightxOES: FnPtr::new(loadfn("glLightxOES")),
         LightxvOES: FnPtr::new(loadfn("glLightxvOES")),
         LineWidth: FnPtr::new(loadfn("glLineWidth")),
         LineWidthxOES: FnPtr::new(loadfn("glLineWidthxOES")),
         LinkProgram: FnPtr::new(loadfn("glLinkProgram")),
         LinkProgramARB: FnPtr::new(loadfn("glLinkProgramARB")),
         ListDrawCommandsStatesClientNV: FnPtr::new(loadfn("glListDrawCommandsStatesClientNV")),
         ListParameterfSGIX: FnPtr::new(loadfn("glListParameterfSGIX")),
         ListParameterfvSGIX: FnPtr::new(loadfn("glListParameterfvSGIX")),
         ListParameteriSGIX: FnPtr::new(loadfn("glListParameteriSGIX")),
         ListParameterivSGIX: FnPtr::new(loadfn("glListParameterivSGIX")),
         LoadIdentityDeformationMapSGIX: FnPtr::new(loadfn("glLoadIdentityDeformationMapSGIX")),
         LoadMatrixxOES: FnPtr::new(loadfn("glLoadMatrixxOES")),
         LoadProgramNV: FnPtr::new(loadfn("glLoadProgramNV")),
         LoadTransposeMatrixdARB: FnPtr::new(loadfn("glLoadTransposeMatrixdARB")),
         LoadTransposeMatrixfARB: FnPtr::new(loadfn("glLoadTransposeMatrixfARB")),
         LoadTransposeMatrixxOES: FnPtr::new(loadfn("glLoadTransposeMatrixxOES")),
         LockArraysEXT: FnPtr::new(loadfn("glLockArraysEXT")),
         LogicOp: FnPtr::new(loadfn("glLogicOp")),
         MakeBufferNonResidentNV: FnPtr::new(loadfn("glMakeBufferNonResidentNV")),
         MakeBufferResidentNV: FnPtr::new(loadfn("glMakeBufferResidentNV")),
         MakeImageHandleNonResidentARB: FnPtr::new(loadfn("glMakeImageHandleNonResidentARB")),
         MakeImageHandleNonResidentNV: FnPtr::new(loadfn("glMakeImageHandleNonResidentNV")),
         MakeImageHandleResidentARB: FnPtr::new(loadfn("glMakeImageHandleResidentARB")),
         MakeImageHandleResidentNV: FnPtr::new(loadfn("glMakeImageHandleResidentNV")),
         MakeNamedBufferNonResidentNV: FnPtr::new(loadfn("glMakeNamedBufferNonResidentNV")),
         MakeNamedBufferResidentNV: FnPtr::new(loadfn("glMakeNamedBufferResidentNV")),
         MakeTextureHandleNonResidentARB: FnPtr::new(loadfn("glMakeTextureHandleNonResidentARB")),
         MakeTextureHandleNonResidentNV: FnPtr::new(loadfn("glMakeTextureHandleNonResidentNV")),
         MakeTextureHandleResidentARB: FnPtr::new(loadfn("glMakeTextureHandleResidentARB")),
         MakeTextureHandleResidentNV: FnPtr::new(loadfn("glMakeTextureHandleResidentNV")),
         Map1xOES: FnPtr::new(loadfn("glMap1xOES")),
         Map2xOES: FnPtr::new(loadfn("glMap2xOES")),
         MapBuffer: FnPtr::new(loadfn("glMapBuffer")),
         MapBufferARB: FnPtr::new(loadfn("glMapBufferARB")),
         MapBufferRange: FnPtr::new(loadfn("glMapBufferRange")),
         MapControlPointsNV: FnPtr::new(loadfn("glMapControlPointsNV")),
         MapGrid1xOES: FnPtr::new(loadfn("glMapGrid1xOES")),
         MapGrid2xOES: FnPtr::new(loadfn("glMapGrid2xOES")),
         MapNamedBuffer: FnPtr::new(loadfn("glMapNamedBuffer")),
         MapNamedBufferEXT: FnPtr::new(loadfn("glMapNamedBufferEXT")),
         MapNamedBufferRange: FnPtr::new(loadfn("glMapNamedBufferRange")),
         MapNamedBufferRangeEXT: FnPtr::new(loadfn("glMapNamedBufferRangeEXT")),
         MapObjectBufferATI: FnPtr::new(loadfn("glMapObjectBufferATI")),
         MapParameterfvNV: FnPtr::new(loadfn("glMapParameterfvNV")),
         MapParameterivNV: FnPtr::new(loadfn("glMapParameterivNV")),
         MapTexture2DINTEL: FnPtr::new(loadfn("glMapTexture2DINTEL")),
         MapVertexAttrib1dAPPLE: FnPtr::new(loadfn("glMapVertexAttrib1dAPPLE")),
         MapVertexAttrib1fAPPLE: FnPtr::new(loadfn("glMapVertexAttrib1fAPPLE")),
         MapVertexAttrib2dAPPLE: FnPtr::new(loadfn("glMapVertexAttrib2dAPPLE")),
         MapVertexAttrib2fAPPLE: FnPtr::new(loadfn("glMapVertexAttrib2fAPPLE")),
         MaterialxOES: FnPtr::new(loadfn("glMaterialxOES")),
         MaterialxvOES: FnPtr::new(loadfn("glMaterialxvOES")),
         MatrixFrustumEXT: FnPtr::new(loadfn("glMatrixFrustumEXT")),
         MatrixIndexPointerARB: FnPtr::new(loadfn("glMatrixIndexPointerARB")),
         MatrixIndexubvARB: FnPtr::new(loadfn("glMatrixIndexubvARB")),
         MatrixIndexuivARB: FnPtr::new(loadfn("glMatrixIndexuivARB")),
         MatrixIndexusvARB: FnPtr::new(loadfn("glMatrixIndexusvARB")),
         MatrixLoad3x2fNV: FnPtr::new(loadfn("glMatrixLoad3x2fNV")),
         MatrixLoad3x3fNV: FnPtr::new(loadfn("glMatrixLoad3x3fNV")),
         MatrixLoadIdentityEXT: FnPtr::new(loadfn("glMatrixLoadIdentityEXT")),
         MatrixLoadTranspose3x3fNV: FnPtr::new(loadfn("glMatrixLoadTranspose3x3fNV")),
         MatrixLoadTransposedEXT: FnPtr::new(loadfn("glMatrixLoadTransposedEXT")),
         MatrixLoadTransposefEXT: FnPtr::new(loadfn("glMatrixLoadTransposefEXT")),
         MatrixLoaddEXT: FnPtr::new(loadfn("glMatrixLoaddEXT")),
         MatrixLoadfEXT: FnPtr::new(loadfn("glMatrixLoadfEXT")),
         MatrixMult3x2fNV: FnPtr::new(loadfn("glMatrixMult3x2fNV")),
         MatrixMult3x3fNV: FnPtr::new(loadfn("glMatrixMult3x3fNV")),
         MatrixMultTranspose3x3fNV: FnPtr::new(loadfn("glMatrixMultTranspose3x3fNV")),
         MatrixMultTransposedEXT: FnPtr::new(loadfn("glMatrixMultTransposedEXT")),
         MatrixMultTransposefEXT: FnPtr::new(loadfn("glMatrixMultTransposefEXT")),
         MatrixMultdEXT: FnPtr::new(loadfn("glMatrixMultdEXT")),
         MatrixMultfEXT: FnPtr::new(loadfn("glMatrixMultfEXT")),
         MatrixOrthoEXT: FnPtr::new(loadfn("glMatrixOrthoEXT")),
         MatrixPopEXT: FnPtr::new(loadfn("glMatrixPopEXT")),
         MatrixPushEXT: FnPtr::new(loadfn("glMatrixPushEXT")),
         MatrixRotatedEXT: FnPtr::new(loadfn("glMatrixRotatedEXT")),
         MatrixRotatefEXT: FnPtr::new(loadfn("glMatrixRotatefEXT")),
         MatrixScaledEXT: FnPtr::new(loadfn("glMatrixScaledEXT")),
         MatrixScalefEXT: FnPtr::new(loadfn("glMatrixScalefEXT")),
         MatrixTranslatedEXT: FnPtr::new(loadfn("glMatrixTranslatedEXT")),
         MatrixTranslatefEXT: FnPtr::new(loadfn("glMatrixTranslatefEXT")),
         MaxShaderCompilerThreadsARB: FnPtr::new(loadfn("glMaxShaderCompilerThreadsARB")),
         MaxShaderCompilerThreadsKHR: FnPtr::new(loadfn("glMaxShaderCompilerThreadsKHR")),
         MemoryBarrier: FnPtr::new(loadfn("glMemoryBarrier")),
         MemoryBarrierByRegion: FnPtr::new(loadfn("glMemoryBarrierByRegion")),
         MemoryBarrierEXT: FnPtr::new(loadfn("glMemoryBarrierEXT")),
         MemoryObjectParameterivEXT: FnPtr::new(loadfn("glMemoryObjectParameterivEXT")),
         MinSampleShading: FnPtr::new(loadfn("glMinSampleShading")),
         MinSampleShadingARB: FnPtr::new(loadfn("glMinSampleShadingARB")),
         MinmaxEXT: FnPtr::new(loadfn("glMinmaxEXT")),
         MultMatrixxOES: FnPtr::new(loadfn("glMultMatrixxOES")),
         MultTransposeMatrixdARB: FnPtr::new(loadfn("glMultTransposeMatrixdARB")),
         MultTransposeMatrixfARB: FnPtr::new(loadfn("glMultTransposeMatrixfARB")),
         MultTransposeMatrixxOES: FnPtr::new(loadfn("glMultTransposeMatrixxOES")),
         MultiDrawArrays: FnPtr::new(loadfn("glMultiDrawArrays")),
         MultiDrawArraysEXT: FnPtr::new(loadfn("glMultiDrawArraysEXT")),
         MultiDrawArraysIndirect: FnPtr::new(loadfn("glMultiDrawArraysIndirect")),
         MultiDrawArraysIndirectAMD: FnPtr::new(loadfn("glMultiDrawArraysIndirectAMD")),
         MultiDrawArraysIndirectBindlessCountNV: FnPtr::new(loadfn("glMultiDrawArraysIndirectBindlessCountNV")),
         MultiDrawArraysIndirectBindlessNV: FnPtr::new(loadfn("glMultiDrawArraysIndirectBindlessNV")),
         MultiDrawArraysIndirectCount: FnPtr::new(loadfn("glMultiDrawArraysIndirectCount")),
         MultiDrawArraysIndirectCountARB: FnPtr::new(loadfn("glMultiDrawArraysIndirectCountARB")),
         MultiDrawElementArrayAPPLE: FnPtr::new(loadfn("glMultiDrawElementArrayAPPLE")),
         MultiDrawElements: FnPtr::new(loadfn("glMultiDrawElements")),
         MultiDrawElementsBaseVertex: FnPtr::new(loadfn("glMultiDrawElementsBaseVertex")),
         MultiDrawElementsEXT: FnPtr::new(loadfn("glMultiDrawElementsEXT")),
         MultiDrawElementsIndirect: FnPtr::new(loadfn("glMultiDrawElementsIndirect")),
         MultiDrawElementsIndirectAMD: FnPtr::new(loadfn("glMultiDrawElementsIndirectAMD")),
         MultiDrawElementsIndirectBindlessCountNV: FnPtr::new(loadfn("glMultiDrawElementsIndirectBindlessCountNV")),
         MultiDrawElementsIndirectBindlessNV: FnPtr::new(loadfn("glMultiDrawElementsIndirectBindlessNV")),
         MultiDrawElementsIndirectCount: FnPtr::new(loadfn("glMultiDrawElementsIndirectCount")),
         MultiDrawElementsIndirectCountARB: FnPtr::new(loadfn("glMultiDrawElementsIndirectCountARB")),
         MultiDrawMeshTasksIndirectCountNV: FnPtr::new(loadfn("glMultiDrawMeshTasksIndirectCountNV")),
         MultiDrawMeshTasksIndirectNV: FnPtr::new(loadfn("glMultiDrawMeshTasksIndirectNV")),
         MultiDrawRangeElementArrayAPPLE: FnPtr::new(loadfn("glMultiDrawRangeElementArrayAPPLE")),
         MultiModeDrawArraysIBM: FnPtr::new(loadfn("glMultiModeDrawArraysIBM")),
         MultiModeDrawElementsIBM: FnPtr::new(loadfn("glMultiModeDrawElementsIBM")),
         MultiTexBufferEXT: FnPtr::new(loadfn("glMultiTexBufferEXT")),
         MultiTexCoord1bOES: FnPtr::new(loadfn("glMultiTexCoord1bOES")),
         MultiTexCoord1bvOES: FnPtr::new(loadfn("glMultiTexCoord1bvOES")),
         MultiTexCoord1dARB: FnPtr::new(loadfn("glMultiTexCoord1dARB")),
         MultiTexCoord1dvARB: FnPtr::new(loadfn("glMultiTexCoord1dvARB")),
         MultiTexCoord1fARB: FnPtr::new(loadfn("glMultiTexCoord1fARB")),
         MultiTexCoord1fvARB: FnPtr::new(loadfn("glMultiTexCoord1fvARB")),
         MultiTexCoord1hNV: FnPtr::new(loadfn("glMultiTexCoord1hNV")),
         MultiTexCoord1hvNV: FnPtr::new(loadfn("glMultiTexCoord1hvNV")),
         MultiTexCoord1iARB: FnPtr::new(loadfn("glMultiTexCoord1iARB")),
         MultiTexCoord1ivARB: FnPtr::new(loadfn("glMultiTexCoord1ivARB")),
         MultiTexCoord1sARB: FnPtr::new(loadfn("glMultiTexCoord1sARB")),
         MultiTexCoord1svARB: FnPtr::new(loadfn("glMultiTexCoord1svARB")),
         MultiTexCoord1xOES: FnPtr::new(loadfn("glMultiTexCoord1xOES")),
         MultiTexCoord1xvOES: FnPtr::new(loadfn("glMultiTexCoord1xvOES")),
         MultiTexCoord2bOES: FnPtr::new(loadfn("glMultiTexCoord2bOES")),
         MultiTexCoord2bvOES: FnPtr::new(loadfn("glMultiTexCoord2bvOES")),
         MultiTexCoord2dARB: FnPtr::new(loadfn("glMultiTexCoord2dARB")),
         MultiTexCoord2dvARB: FnPtr::new(loadfn("glMultiTexCoord2dvARB")),
         MultiTexCoord2fARB: FnPtr::new(loadfn("glMultiTexCoord2fARB")),
         MultiTexCoord2fvARB: FnPtr::new(loadfn("glMultiTexCoord2fvARB")),
         MultiTexCoord2hNV: FnPtr::new(loadfn("glMultiTexCoord2hNV")),
         MultiTexCoord2hvNV: FnPtr::new(loadfn("glMultiTexCoord2hvNV")),
         MultiTexCoord2iARB: FnPtr::new(loadfn("glMultiTexCoord2iARB")),
         MultiTexCoord2ivARB: FnPtr::new(loadfn("glMultiTexCoord2ivARB")),
         MultiTexCoord2sARB: FnPtr::new(loadfn("glMultiTexCoord2sARB")),
         MultiTexCoord2svARB: FnPtr::new(loadfn("glMultiTexCoord2svARB")),
         MultiTexCoord2xOES: FnPtr::new(loadfn("glMultiTexCoord2xOES")),
         MultiTexCoord2xvOES: FnPtr::new(loadfn("glMultiTexCoord2xvOES")),
         MultiTexCoord3bOES: FnPtr::new(loadfn("glMultiTexCoord3bOES")),
         MultiTexCoord3bvOES: FnPtr::new(loadfn("glMultiTexCoord3bvOES")),
         MultiTexCoord3dARB: FnPtr::new(loadfn("glMultiTexCoord3dARB")),
         MultiTexCoord3dvARB: FnPtr::new(loadfn("glMultiTexCoord3dvARB")),
         MultiTexCoord3fARB: FnPtr::new(loadfn("glMultiTexCoord3fARB")),
         MultiTexCoord3fvARB: FnPtr::new(loadfn("glMultiTexCoord3fvARB")),
         MultiTexCoord3hNV: FnPtr::new(loadfn("glMultiTexCoord3hNV")),
         MultiTexCoord3hvNV: FnPtr::new(loadfn("glMultiTexCoord3hvNV")),
         MultiTexCoord3iARB: FnPtr::new(loadfn("glMultiTexCoord3iARB")),
         MultiTexCoord3ivARB: FnPtr::new(loadfn("glMultiTexCoord3ivARB")),
         MultiTexCoord3sARB: FnPtr::new(loadfn("glMultiTexCoord3sARB")),
         MultiTexCoord3svARB: FnPtr::new(loadfn("glMultiTexCoord3svARB")),
         MultiTexCoord3xOES: FnPtr::new(loadfn("glMultiTexCoord3xOES")),
         MultiTexCoord3xvOES: FnPtr::new(loadfn("glMultiTexCoord3xvOES")),
         MultiTexCoord4bOES: FnPtr::new(loadfn("glMultiTexCoord4bOES")),
         MultiTexCoord4bvOES: FnPtr::new(loadfn("glMultiTexCoord4bvOES")),
         MultiTexCoord4dARB: FnPtr::new(loadfn("glMultiTexCoord4dARB")),
         MultiTexCoord4dvARB: FnPtr::new(loadfn("glMultiTexCoord4dvARB")),
         MultiTexCoord4fARB: FnPtr::new(loadfn("glMultiTexCoord4fARB")),
         MultiTexCoord4fvARB: FnPtr::new(loadfn("glMultiTexCoord4fvARB")),
         MultiTexCoord4hNV: FnPtr::new(loadfn("glMultiTexCoord4hNV")),
         MultiTexCoord4hvNV: FnPtr::new(loadfn("glMultiTexCoord4hvNV")),
         MultiTexCoord4iARB: FnPtr::new(loadfn("glMultiTexCoord4iARB")),
         MultiTexCoord4ivARB: FnPtr::new(loadfn("glMultiTexCoord4ivARB")),
         MultiTexCoord4sARB: FnPtr::new(loadfn("glMultiTexCoord4sARB")),
         MultiTexCoord4svARB: FnPtr::new(loadfn("glMultiTexCoord4svARB")),
         MultiTexCoord4xOES: FnPtr::new(loadfn("glMultiTexCoord4xOES")),
         MultiTexCoord4xvOES: FnPtr::new(loadfn("glMultiTexCoord4xvOES")),
         MultiTexCoordPointerEXT: FnPtr::new(loadfn("glMultiTexCoordPointerEXT")),
         MultiTexEnvfEXT: FnPtr::new(loadfn("glMultiTexEnvfEXT")),
         MultiTexEnvfvEXT: FnPtr::new(loadfn("glMultiTexEnvfvEXT")),
         MultiTexEnviEXT: FnPtr::new(loadfn("glMultiTexEnviEXT")),
         MultiTexEnvivEXT: FnPtr::new(loadfn("glMultiTexEnvivEXT")),
         MultiTexGendEXT: FnPtr::new(loadfn("glMultiTexGendEXT")),
         MultiTexGendvEXT: FnPtr::new(loadfn("glMultiTexGendvEXT")),
         MultiTexGenfEXT: FnPtr::new(loadfn("glMultiTexGenfEXT")),
         MultiTexGenfvEXT: FnPtr::new(loadfn("glMultiTexGenfvEXT")),
         MultiTexGeniEXT: FnPtr::new(loadfn("glMultiTexGeniEXT")),
         MultiTexGenivEXT: FnPtr::new(loadfn("glMultiTexGenivEXT")),
         MultiTexImage1DEXT: FnPtr::new(loadfn("glMultiTexImage1DEXT")),
         MultiTexImage2DEXT: FnPtr::new(loadfn("glMultiTexImage2DEXT")),
         MultiTexImage3DEXT: FnPtr::new(loadfn("glMultiTexImage3DEXT")),
         MultiTexParameterIivEXT: FnPtr::new(loadfn("glMultiTexParameterIivEXT")),
         MultiTexParameterIuivEXT: FnPtr::new(loadfn("glMultiTexParameterIuivEXT")),
         MultiTexParameterfEXT: FnPtr::new(loadfn("glMultiTexParameterfEXT")),
         MultiTexParameterfvEXT: FnPtr::new(loadfn("glMultiTexParameterfvEXT")),
         MultiTexParameteriEXT: FnPtr::new(loadfn("glMultiTexParameteriEXT")),
         MultiTexParameterivEXT: FnPtr::new(loadfn("glMultiTexParameterivEXT")),
         MultiTexRenderbufferEXT: FnPtr::new(loadfn("glMultiTexRenderbufferEXT")),
         MultiTexSubImage1DEXT: FnPtr::new(loadfn("glMultiTexSubImage1DEXT")),
         MultiTexSubImage2DEXT: FnPtr::new(loadfn("glMultiTexSubImage2DEXT")),
         MultiTexSubImage3DEXT: FnPtr::new(loadfn("glMultiTexSubImage3DEXT")),
         MulticastBarrierNV: FnPtr::new(loadfn("glMulticastBarrierNV")),
         MulticastBlitFramebufferNV: FnPtr::new(loadfn("glMulticastBlitFramebufferNV")),
         MulticastBufferSubDataNV: FnPtr::new(loadfn("glMulticastBufferSubDataNV")),
         MulticastCopyBufferSubDataNV: FnPtr::new(loadfn("glMulticastCopyBufferSubDataNV")),
         MulticastCopyImageSubDataNV: FnPtr::new(loadfn("glMulticastCopyImageSubDataNV")),
         MulticastFramebufferSampleLocationsfvNV: FnPtr::new(loadfn("glMulticastFramebufferSampleLocationsfvNV")),
         MulticastGetQueryObjecti64vNV: FnPtr::new(loadfn("glMulticastGetQueryObjecti64vNV")),
         MulticastGetQueryObjectivNV: FnPtr::new(loadfn("glMulticastGetQueryObjectivNV")),
         MulticastGetQueryObjectui64vNV: FnPtr::new(loadfn("glMulticastGetQueryObjectui64vNV")),
         MulticastGetQueryObjectuivNV: FnPtr::new(loadfn("glMulticastGetQueryObjectuivNV")),
         MulticastScissorArrayvNVX: FnPtr::new(loadfn("glMulticastScissorArrayvNVX")),
         MulticastViewportArrayvNVX: FnPtr::new(loadfn("glMulticastViewportArrayvNVX")),
         MulticastViewportPositionWScaleNVX: FnPtr::new(loadfn("glMulticastViewportPositionWScaleNVX")),
         MulticastWaitSyncNV: FnPtr::new(loadfn("glMulticastWaitSyncNV")),
         NamedBufferAttachMemoryNV: FnPtr::new(loadfn("glNamedBufferAttachMemoryNV")),
         NamedBufferData: FnPtr::new(loadfn("glNamedBufferData")),
         NamedBufferDataEXT: FnPtr::new(loadfn("glNamedBufferDataEXT")),
         NamedBufferPageCommitmentARB: FnPtr::new(loadfn("glNamedBufferPageCommitmentARB")),
         NamedBufferPageCommitmentEXT: FnPtr::new(loadfn("glNamedBufferPageCommitmentEXT")),
         NamedBufferPageCommitmentMemNV: FnPtr::new(loadfn("glNamedBufferPageCommitmentMemNV")),
         NamedBufferStorage: FnPtr::new(loadfn("glNamedBufferStorage")),
         NamedBufferStorageEXT: FnPtr::new(loadfn("glNamedBufferStorageEXT")),
         NamedBufferStorageExternalEXT: FnPtr::new(loadfn("glNamedBufferStorageExternalEXT")),
         NamedBufferStorageMemEXT: FnPtr::new(loadfn("glNamedBufferStorageMemEXT")),
         NamedBufferSubData: FnPtr::new(loadfn("glNamedBufferSubData")),
         NamedBufferSubDataEXT: FnPtr::new(loadfn("glNamedBufferSubDataEXT")),
         NamedCopyBufferSubDataEXT: FnPtr::new(loadfn("glNamedCopyBufferSubDataEXT")),
         NamedFramebufferDrawBuffer: FnPtr::new(loadfn("glNamedFramebufferDrawBuffer")),
         NamedFramebufferDrawBuffers: FnPtr::new(loadfn("glNamedFramebufferDrawBuffers")),
         NamedFramebufferParameteri: FnPtr::new(loadfn("glNamedFramebufferParameteri")),
         NamedFramebufferParameteriEXT: FnPtr::new(loadfn("glNamedFramebufferParameteriEXT")),
         NamedFramebufferReadBuffer: FnPtr::new(loadfn("glNamedFramebufferReadBuffer")),
         NamedFramebufferRenderbuffer: FnPtr::new(loadfn("glNamedFramebufferRenderbuffer")),
         NamedFramebufferRenderbufferEXT: FnPtr::new(loadfn("glNamedFramebufferRenderbufferEXT")),
         NamedFramebufferSampleLocationsfvARB: FnPtr::new(loadfn("glNamedFramebufferSampleLocationsfvARB")),
         NamedFramebufferSampleLocationsfvNV: FnPtr::new(loadfn("glNamedFramebufferSampleLocationsfvNV")),
         NamedFramebufferSamplePositionsfvAMD: FnPtr::new(loadfn("glNamedFramebufferSamplePositionsfvAMD")),
         NamedFramebufferTexture: FnPtr::new(loadfn("glNamedFramebufferTexture")),
         NamedFramebufferTexture1DEXT: FnPtr::new(loadfn("glNamedFramebufferTexture1DEXT")),
         NamedFramebufferTexture2DEXT: FnPtr::new(loadfn("glNamedFramebufferTexture2DEXT")),
         NamedFramebufferTexture3DEXT: FnPtr::new(loadfn("glNamedFramebufferTexture3DEXT")),
         NamedFramebufferTextureEXT: FnPtr::new(loadfn("glNamedFramebufferTextureEXT")),
         NamedFramebufferTextureFaceEXT: FnPtr::new(loadfn("glNamedFramebufferTextureFaceEXT")),
         NamedFramebufferTextureLayer: FnPtr::new(loadfn("glNamedFramebufferTextureLayer")),
         NamedFramebufferTextureLayerEXT: FnPtr::new(loadfn("glNamedFramebufferTextureLayerEXT")),
         NamedProgramLocalParameter4dEXT: FnPtr::new(loadfn("glNamedProgramLocalParameter4dEXT")),
         NamedProgramLocalParameter4dvEXT: FnPtr::new(loadfn("glNamedProgramLocalParameter4dvEXT")),
         NamedProgramLocalParameter4fEXT: FnPtr::new(loadfn("glNamedProgramLocalParameter4fEXT")),
         NamedProgramLocalParameter4fvEXT: FnPtr::new(loadfn("glNamedProgramLocalParameter4fvEXT")),
         NamedProgramLocalParameterI4iEXT: FnPtr::new(loadfn("glNamedProgramLocalParameterI4iEXT")),
         NamedProgramLocalParameterI4ivEXT: FnPtr::new(loadfn("glNamedProgramLocalParameterI4ivEXT")),
         NamedProgramLocalParameterI4uiEXT: FnPtr::new(loadfn("glNamedProgramLocalParameterI4uiEXT")),
         NamedProgramLocalParameterI4uivEXT: FnPtr::new(loadfn("glNamedProgramLocalParameterI4uivEXT")),
         NamedProgramLocalParameters4fvEXT: FnPtr::new(loadfn("glNamedProgramLocalParameters4fvEXT")),
         NamedProgramLocalParametersI4ivEXT: FnPtr::new(loadfn("glNamedProgramLocalParametersI4ivEXT")),
         NamedProgramLocalParametersI4uivEXT: FnPtr::new(loadfn("glNamedProgramLocalParametersI4uivEXT")),
         NamedProgramStringEXT: FnPtr::new(loadfn("glNamedProgramStringEXT")),
         NamedRenderbufferStorage: FnPtr::new(loadfn("glNamedRenderbufferStorage")),
         NamedRenderbufferStorageEXT: FnPtr::new(loadfn("glNamedRenderbufferStorageEXT")),
         NamedRenderbufferStorageMultisample: FnPtr::new(loadfn("glNamedRenderbufferStorageMultisample")),
         NamedRenderbufferStorageMultisampleAdvancedAMD: FnPtr::new(loadfn("glNamedRenderbufferStorageMultisampleAdvancedAMD")),
         NamedRenderbufferStorageMultisampleCoverageEXT: FnPtr::new(loadfn("glNamedRenderbufferStorageMultisampleCoverageEXT")),
         NamedRenderbufferStorageMultisampleEXT: FnPtr::new(loadfn("glNamedRenderbufferStorageMultisampleEXT")),
         NamedStringARB: FnPtr::new(loadfn("glNamedStringARB")),
         NewObjectBufferATI: FnPtr::new(loadfn("glNewObjectBufferATI")),
         Normal3fVertex3fSUN: FnPtr::new(loadfn("glNormal3fVertex3fSUN")),
         Normal3fVertex3fvSUN: FnPtr::new(loadfn("glNormal3fVertex3fvSUN")),
         Normal3hNV: FnPtr::new(loadfn("glNormal3hNV")),
         Normal3hvNV: FnPtr::new(loadfn("glNormal3hvNV")),
         Normal3xOES: FnPtr::new(loadfn("glNormal3xOES")),
         Normal3xvOES: FnPtr::new(loadfn("glNormal3xvOES")),
         NormalFormatNV: FnPtr::new(loadfn("glNormalFormatNV")),
         NormalPointerEXT: FnPtr::new(loadfn("glNormalPointerEXT")),
         NormalPointerListIBM: FnPtr::new(loadfn("glNormalPointerListIBM")),
         NormalPointervINTEL: FnPtr::new(loadfn("glNormalPointervINTEL")),
         NormalStream3bATI: FnPtr::new(loadfn("glNormalStream3bATI")),
         NormalStream3bvATI: FnPtr::new(loadfn("glNormalStream3bvATI")),
         NormalStream3dATI: FnPtr::new(loadfn("glNormalStream3dATI")),
         NormalStream3dvATI: FnPtr::new(loadfn("glNormalStream3dvATI")),
         NormalStream3fATI: FnPtr::new(loadfn("glNormalStream3fATI")),
         NormalStream3fvATI: FnPtr::new(loadfn("glNormalStream3fvATI")),
         NormalStream3iATI: FnPtr::new(loadfn("glNormalStream3iATI")),
         NormalStream3ivATI: FnPtr::new(loadfn("glNormalStream3ivATI")),
         NormalStream3sATI: FnPtr::new(loadfn("glNormalStream3sATI")),
         NormalStream3svATI: FnPtr::new(loadfn("glNormalStream3svATI")),
         ObjectLabel: FnPtr::new(loadfn("glObjectLabel")),
         ObjectPtrLabel: FnPtr::new(loadfn("glObjectPtrLabel")),
         ObjectPurgeableAPPLE: FnPtr::new(loadfn("glObjectPurgeableAPPLE")),
         ObjectUnpurgeableAPPLE: FnPtr::new(loadfn("glObjectUnpurgeableAPPLE")),
         OrthofOES: FnPtr::new(loadfn("glOrthofOES")),
         OrthoxOES: FnPtr::new(loadfn("glOrthoxOES")),
         PNTrianglesfATI: FnPtr::new(loadfn("glPNTrianglesfATI")),
         PNTrianglesiATI: FnPtr::new(loadfn("glPNTrianglesiATI")),
         PassTexCoordATI: FnPtr::new(loadfn("glPassTexCoordATI")),
         PassThroughxOES: FnPtr::new(loadfn("glPassThroughxOES")),
         PatchParameterfv: FnPtr::new(loadfn("glPatchParameterfv")),
         PatchParameteri: FnPtr::new(loadfn("glPatchParameteri")),
         PathCommandsNV: FnPtr::new(loadfn("glPathCommandsNV")),
         PathCoordsNV: FnPtr::new(loadfn("glPathCoordsNV")),
         PathCoverDepthFuncNV: FnPtr::new(loadfn("glPathCoverDepthFuncNV")),
         PathDashArrayNV: FnPtr::new(loadfn("glPathDashArrayNV")),
         PathGlyphIndexArrayNV: FnPtr::new(loadfn("glPathGlyphIndexArrayNV")),
         PathGlyphIndexRangeNV: FnPtr::new(loadfn("glPathGlyphIndexRangeNV")),
         PathGlyphRangeNV: FnPtr::new(loadfn("glPathGlyphRangeNV")),
         PathGlyphsNV: FnPtr::new(loadfn("glPathGlyphsNV")),
         PathMemoryGlyphIndexArrayNV: FnPtr::new(loadfn("glPathMemoryGlyphIndexArrayNV")),
         PathParameterfNV: FnPtr::new(loadfn("glPathParameterfNV")),
         PathParameterfvNV: FnPtr::new(loadfn("glPathParameterfvNV")),
         PathParameteriNV: FnPtr::new(loadfn("glPathParameteriNV")),
         PathParameterivNV: FnPtr::new(loadfn("glPathParameterivNV")),
         PathStencilDepthOffsetNV: FnPtr::new(loadfn("glPathStencilDepthOffsetNV")),
         PathStencilFuncNV: FnPtr::new(loadfn("glPathStencilFuncNV")),
         PathStringNV: FnPtr::new(loadfn("glPathStringNV")),
         PathSubCommandsNV: FnPtr::new(loadfn("glPathSubCommandsNV")),
         PathSubCoordsNV: FnPtr::new(loadfn("glPathSubCoordsNV")),
         PauseTransformFeedback: FnPtr::new(loadfn("glPauseTransformFeedback")),
         PauseTransformFeedbackNV: FnPtr::new(loadfn("glPauseTransformFeedbackNV")),
         PixelDataRangeNV: FnPtr::new(loadfn("glPixelDataRangeNV")),
         PixelMapx: FnPtr::new(loadfn("glPixelMapx")),
         PixelStoref: FnPtr::new(loadfn("glPixelStoref")),
         PixelStorei: FnPtr::new(loadfn("glPixelStorei")),
         PixelStorex: FnPtr::new(loadfn("glPixelStorex")),
         PixelTexGenParameterfSGIS: FnPtr::new(loadfn("glPixelTexGenParameterfSGIS")),
         PixelTexGenParameterfvSGIS: FnPtr::new(loadfn("glPixelTexGenParameterfvSGIS")),
         PixelTexGenParameteriSGIS: FnPtr::new(loadfn("glPixelTexGenParameteriSGIS")),
         PixelTexGenParameterivSGIS: FnPtr::new(loadfn("glPixelTexGenParameterivSGIS")),
         PixelTexGenSGIX: FnPtr::new(loadfn("glPixelTexGenSGIX")),
         PixelTransferxOES: FnPtr::new(loadfn("glPixelTransferxOES")),
         PixelTransformParameterfEXT: FnPtr::new(loadfn("glPixelTransformParameterfEXT")),
         PixelTransformParameterfvEXT: FnPtr::new(loadfn("glPixelTransformParameterfvEXT")),
         PixelTransformParameteriEXT: FnPtr::new(loadfn("glPixelTransformParameteriEXT")),
         PixelTransformParameterivEXT: FnPtr::new(loadfn("glPixelTransformParameterivEXT")),
         PixelZoomxOES: FnPtr::new(loadfn("glPixelZoomxOES")),
         PointAlongPathNV: FnPtr::new(loadfn("glPointAlongPathNV")),
         PointParameterf: FnPtr::new(loadfn("glPointParameterf")),
         PointParameterfARB: FnPtr::new(loadfn("glPointParameterfARB")),
         PointParameterfEXT: FnPtr::new(loadfn("glPointParameterfEXT")),
         PointParameterfSGIS: FnPtr::new(loadfn("glPointParameterfSGIS")),
         PointParameterfv: FnPtr::new(loadfn("glPointParameterfv")),
         PointParameterfvARB: FnPtr::new(loadfn("glPointParameterfvARB")),
         PointParameterfvEXT: FnPtr::new(loadfn("glPointParameterfvEXT")),
         PointParameterfvSGIS: FnPtr::new(loadfn("glPointParameterfvSGIS")),
         PointParameteri: FnPtr::new(loadfn("glPointParameteri")),
         PointParameteriNV: FnPtr::new(loadfn("glPointParameteriNV")),
         PointParameteriv: FnPtr::new(loadfn("glPointParameteriv")),
         PointParameterivNV: FnPtr::new(loadfn("glPointParameterivNV")),
         PointParameterxvOES: FnPtr::new(loadfn("glPointParameterxvOES")),
         PointSize: FnPtr::new(loadfn("glPointSize")),
         PointSizexOES: FnPtr::new(loadfn("glPointSizexOES")),
         PollAsyncSGIX: FnPtr::new(loadfn("glPollAsyncSGIX")),
         PollInstrumentsSGIX: FnPtr::new(loadfn("glPollInstrumentsSGIX")),
         PolygonMode: FnPtr::new(loadfn("glPolygonMode")),
         PolygonOffset: FnPtr::new(loadfn("glPolygonOffset")),
         PolygonOffsetClamp: FnPtr::new(loadfn("glPolygonOffsetClamp")),
         PolygonOffsetClampEXT: FnPtr::new(loadfn("glPolygonOffsetClampEXT")),
         PolygonOffsetEXT: FnPtr::new(loadfn("glPolygonOffsetEXT")),
         PolygonOffsetxOES: FnPtr::new(loadfn("glPolygonOffsetxOES")),
         PopDebugGroup: FnPtr::new(loadfn("glPopDebugGroup")),
         PopGroupMarkerEXT: FnPtr::new(loadfn("glPopGroupMarkerEXT")),
         PresentFrameDualFillNV: FnPtr::new(loadfn("glPresentFrameDualFillNV")),
         PresentFrameKeyedNV: FnPtr::new(loadfn("glPresentFrameKeyedNV")),
         PrimitiveBoundingBoxARB: FnPtr::new(loadfn("glPrimitiveBoundingBoxARB")),
         PrimitiveRestartIndex: FnPtr::new(loadfn("glPrimitiveRestartIndex")),
         PrimitiveRestartIndexNV: FnPtr::new(loadfn("glPrimitiveRestartIndexNV")),
         PrimitiveRestartNV: FnPtr::new(loadfn("glPrimitiveRestartNV")),
         PrioritizeTexturesEXT: FnPtr::new(loadfn("glPrioritizeTexturesEXT")),
         PrioritizeTexturesxOES: FnPtr::new(loadfn("glPrioritizeTexturesxOES")),
         ProgramBinary: FnPtr::new(loadfn("glProgramBinary")),
         ProgramBufferParametersIivNV: FnPtr::new(loadfn("glProgramBufferParametersIivNV")),
         ProgramBufferParametersIuivNV: FnPtr::new(loadfn("glProgramBufferParametersIuivNV")),
         ProgramBufferParametersfvNV: FnPtr::new(loadfn("glProgramBufferParametersfvNV")),
         ProgramEnvParameter4dARB: FnPtr::new(loadfn("glProgramEnvParameter4dARB")),
         ProgramEnvParameter4dvARB: FnPtr::new(loadfn("glProgramEnvParameter4dvARB")),
         ProgramEnvParameter4fARB: FnPtr::new(loadfn("glProgramEnvParameter4fARB")),
         ProgramEnvParameter4fvARB: FnPtr::new(loadfn("glProgramEnvParameter4fvARB")),
         ProgramEnvParameterI4iNV: FnPtr::new(loadfn("glProgramEnvParameterI4iNV")),
         ProgramEnvParameterI4ivNV: FnPtr::new(loadfn("glProgramEnvParameterI4ivNV")),
         ProgramEnvParameterI4uiNV: FnPtr::new(loadfn("glProgramEnvParameterI4uiNV")),
         ProgramEnvParameterI4uivNV: FnPtr::new(loadfn("glProgramEnvParameterI4uivNV")),
         ProgramEnvParameters4fvEXT: FnPtr::new(loadfn("glProgramEnvParameters4fvEXT")),
         ProgramEnvParametersI4ivNV: FnPtr::new(loadfn("glProgramEnvParametersI4ivNV")),
         ProgramEnvParametersI4uivNV: FnPtr::new(loadfn("glProgramEnvParametersI4uivNV")),
         ProgramLocalParameter4dARB: FnPtr::new(loadfn("glProgramLocalParameter4dARB")),
         ProgramLocalParameter4dvARB: FnPtr::new(loadfn("glProgramLocalParameter4dvARB")),
         ProgramLocalParameter4fARB: FnPtr::new(loadfn("glProgramLocalParameter4fARB")),
         ProgramLocalParameter4fvARB: FnPtr::new(loadfn("glProgramLocalParameter4fvARB")),
         ProgramLocalParameterI4iNV: FnPtr::new(loadfn("glProgramLocalParameterI4iNV")),
         ProgramLocalParameterI4ivNV: FnPtr::new(loadfn("glProgramLocalParameterI4ivNV")),
         ProgramLocalParameterI4uiNV: FnPtr::new(loadfn("glProgramLocalParameterI4uiNV")),
         ProgramLocalParameterI4uivNV: FnPtr::new(loadfn("glProgramLocalParameterI4uivNV")),
         ProgramLocalParameters4fvEXT: FnPtr::new(loadfn("glProgramLocalParameters4fvEXT")),
         ProgramLocalParametersI4ivNV: FnPtr::new(loadfn("glProgramLocalParametersI4ivNV")),
         ProgramLocalParametersI4uivNV: FnPtr::new(loadfn("glProgramLocalParametersI4uivNV")),
         ProgramNamedParameter4dNV: FnPtr::new(loadfn("glProgramNamedParameter4dNV")),
         ProgramNamedParameter4dvNV: FnPtr::new(loadfn("glProgramNamedParameter4dvNV")),
         ProgramNamedParameter4fNV: FnPtr::new(loadfn("glProgramNamedParameter4fNV")),
         ProgramNamedParameter4fvNV: FnPtr::new(loadfn("glProgramNamedParameter4fvNV")),
         ProgramParameter4dNV: FnPtr::new(loadfn("glProgramParameter4dNV")),
         ProgramParameter4dvNV: FnPtr::new(loadfn("glProgramParameter4dvNV")),
         ProgramParameter4fNV: FnPtr::new(loadfn("glProgramParameter4fNV")),
         ProgramParameter4fvNV: FnPtr::new(loadfn("glProgramParameter4fvNV")),
         ProgramParameteri: FnPtr::new(loadfn("glProgramParameteri")),
         ProgramParameteriARB: FnPtr::new(loadfn("glProgramParameteriARB")),
         ProgramParameteriEXT: FnPtr::new(loadfn("glProgramParameteriEXT")),
         ProgramParameters4dvNV: FnPtr::new(loadfn("glProgramParameters4dvNV")),
         ProgramParameters4fvNV: FnPtr::new(loadfn("glProgramParameters4fvNV")),
         ProgramPathFragmentInputGenNV: FnPtr::new(loadfn("glProgramPathFragmentInputGenNV")),
         ProgramStringARB: FnPtr::new(loadfn("glProgramStringARB")),
         ProgramSubroutineParametersuivNV: FnPtr::new(loadfn("glProgramSubroutineParametersuivNV")),
         ProgramUniform1d: FnPtr::new(loadfn("glProgramUniform1d")),
         ProgramUniform1dEXT: FnPtr::new(loadfn("glProgramUniform1dEXT")),
         ProgramUniform1dv: FnPtr::new(loadfn("glProgramUniform1dv")),
         ProgramUniform1dvEXT: FnPtr::new(loadfn("glProgramUniform1dvEXT")),
         ProgramUniform1f: FnPtr::new(loadfn("glProgramUniform1f")),
         ProgramUniform1fEXT: FnPtr::new(loadfn("glProgramUniform1fEXT")),
         ProgramUniform1fv: FnPtr::new(loadfn("glProgramUniform1fv")),
         ProgramUniform1fvEXT: FnPtr::new(loadfn("glProgramUniform1fvEXT")),
         ProgramUniform1i: FnPtr::new(loadfn("glProgramUniform1i")),
         ProgramUniform1i64ARB: FnPtr::new(loadfn("glProgramUniform1i64ARB")),
         ProgramUniform1i64NV: FnPtr::new(loadfn("glProgramUniform1i64NV")),
         ProgramUniform1i64vARB: FnPtr::new(loadfn("glProgramUniform1i64vARB")),
         ProgramUniform1i64vNV: FnPtr::new(loadfn("glProgramUniform1i64vNV")),
         ProgramUniform1iEXT: FnPtr::new(loadfn("glProgramUniform1iEXT")),
         ProgramUniform1iv: FnPtr::new(loadfn("glProgramUniform1iv")),
         ProgramUniform1ivEXT: FnPtr::new(loadfn("glProgramUniform1ivEXT")),
         ProgramUniform1ui: FnPtr::new(loadfn("glProgramUniform1ui")),
         ProgramUniform1ui64ARB: FnPtr::new(loadfn("glProgramUniform1ui64ARB")),
         ProgramUniform1ui64NV: FnPtr::new(loadfn("glProgramUniform1ui64NV")),
         ProgramUniform1ui64vARB: FnPtr::new(loadfn("glProgramUniform1ui64vARB")),
         ProgramUniform1ui64vNV: FnPtr::new(loadfn("glProgramUniform1ui64vNV")),
         ProgramUniform1uiEXT: FnPtr::new(loadfn("glProgramUniform1uiEXT")),
         ProgramUniform1uiv: FnPtr::new(loadfn("glProgramUniform1uiv")),
         ProgramUniform1uivEXT: FnPtr::new(loadfn("glProgramUniform1uivEXT")),
         ProgramUniform2d: FnPtr::new(loadfn("glProgramUniform2d")),
         ProgramUniform2dEXT: FnPtr::new(loadfn("glProgramUniform2dEXT")),
         ProgramUniform2dv: FnPtr::new(loadfn("glProgramUniform2dv")),
         ProgramUniform2dvEXT: FnPtr::new(loadfn("glProgramUniform2dvEXT")),
         ProgramUniform2f: FnPtr::new(loadfn("glProgramUniform2f")),
         ProgramUniform2fEXT: FnPtr::new(loadfn("glProgramUniform2fEXT")),
         ProgramUniform2fv: FnPtr::new(loadfn("glProgramUniform2fv")),
         ProgramUniform2fvEXT: FnPtr::new(loadfn("glProgramUniform2fvEXT")),
         ProgramUniform2i: FnPtr::new(loadfn("glProgramUniform2i")),
         ProgramUniform2i64ARB: FnPtr::new(loadfn("glProgramUniform2i64ARB")),
         ProgramUniform2i64NV: FnPtr::new(loadfn("glProgramUniform2i64NV")),
         ProgramUniform2i64vARB: FnPtr::new(loadfn("glProgramUniform2i64vARB")),
         ProgramUniform2i64vNV: FnPtr::new(loadfn("glProgramUniform2i64vNV")),
         ProgramUniform2iEXT: FnPtr::new(loadfn("glProgramUniform2iEXT")),
         ProgramUniform2iv: FnPtr::new(loadfn("glProgramUniform2iv")),
         ProgramUniform2ivEXT: FnPtr::new(loadfn("glProgramUniform2ivEXT")),
         ProgramUniform2ui: FnPtr::new(loadfn("glProgramUniform2ui")),
         ProgramUniform2ui64ARB: FnPtr::new(loadfn("glProgramUniform2ui64ARB")),
         ProgramUniform2ui64NV: FnPtr::new(loadfn("glProgramUniform2ui64NV")),
         ProgramUniform2ui64vARB: FnPtr::new(loadfn("glProgramUniform2ui64vARB")),
         ProgramUniform2ui64vNV: FnPtr::new(loadfn("glProgramUniform2ui64vNV")),
         ProgramUniform2uiEXT: FnPtr::new(loadfn("glProgramUniform2uiEXT")),
         ProgramUniform2uiv: FnPtr::new(loadfn("glProgramUniform2uiv")),
         ProgramUniform2uivEXT: FnPtr::new(loadfn("glProgramUniform2uivEXT")),
         ProgramUniform3d: FnPtr::new(loadfn("glProgramUniform3d")),
         ProgramUniform3dEXT: FnPtr::new(loadfn("glProgramUniform3dEXT")),
         ProgramUniform3dv: FnPtr::new(loadfn("glProgramUniform3dv")),
         ProgramUniform3dvEXT: FnPtr::new(loadfn("glProgramUniform3dvEXT")),
         ProgramUniform3f: FnPtr::new(loadfn("glProgramUniform3f")),
         ProgramUniform3fEXT: FnPtr::new(loadfn("glProgramUniform3fEXT")),
         ProgramUniform3fv: FnPtr::new(loadfn("glProgramUniform3fv")),
         ProgramUniform3fvEXT: FnPtr::new(loadfn("glProgramUniform3fvEXT")),
         ProgramUniform3i: FnPtr::new(loadfn("glProgramUniform3i")),
         ProgramUniform3i64ARB: FnPtr::new(loadfn("glProgramUniform3i64ARB")),
         ProgramUniform3i64NV: FnPtr::new(loadfn("glProgramUniform3i64NV")),
         ProgramUniform3i64vARB: FnPtr::new(loadfn("glProgramUniform3i64vARB")),
         ProgramUniform3i64vNV: FnPtr::new(loadfn("glProgramUniform3i64vNV")),
         ProgramUniform3iEXT: FnPtr::new(loadfn("glProgramUniform3iEXT")),
         ProgramUniform3iv: FnPtr::new(loadfn("glProgramUniform3iv")),
         ProgramUniform3ivEXT: FnPtr::new(loadfn("glProgramUniform3ivEXT")),
         ProgramUniform3ui: FnPtr::new(loadfn("glProgramUniform3ui")),
         ProgramUniform3ui64ARB: FnPtr::new(loadfn("glProgramUniform3ui64ARB")),
         ProgramUniform3ui64NV: FnPtr::new(loadfn("glProgramUniform3ui64NV")),
         ProgramUniform3ui64vARB: FnPtr::new(loadfn("glProgramUniform3ui64vARB")),
         ProgramUniform3ui64vNV: FnPtr::new(loadfn("glProgramUniform3ui64vNV")),
         ProgramUniform3uiEXT: FnPtr::new(loadfn("glProgramUniform3uiEXT")),
         ProgramUniform3uiv: FnPtr::new(loadfn("glProgramUniform3uiv")),
         ProgramUniform3uivEXT: FnPtr::new(loadfn("glProgramUniform3uivEXT")),
         ProgramUniform4d: FnPtr::new(loadfn("glProgramUniform4d")),
         ProgramUniform4dEXT: FnPtr::new(loadfn("glProgramUniform4dEXT")),
         ProgramUniform4dv: FnPtr::new(loadfn("glProgramUniform4dv")),
         ProgramUniform4dvEXT: FnPtr::new(loadfn("glProgramUniform4dvEXT")),
         ProgramUniform4f: FnPtr::new(loadfn("glProgramUniform4f")),
         ProgramUniform4fEXT: FnPtr::new(loadfn("glProgramUniform4fEXT")),
         ProgramUniform4fv: FnPtr::new(loadfn("glProgramUniform4fv")),
         ProgramUniform4fvEXT: FnPtr::new(loadfn("glProgramUniform4fvEXT")),
         ProgramUniform4i: FnPtr::new(loadfn("glProgramUniform4i")),
         ProgramUniform4i64ARB: FnPtr::new(loadfn("glProgramUniform4i64ARB")),
         ProgramUniform4i64NV: FnPtr::new(loadfn("glProgramUniform4i64NV")),
         ProgramUniform4i64vARB: FnPtr::new(loadfn("glProgramUniform4i64vARB")),
         ProgramUniform4i64vNV: FnPtr::new(loadfn("glProgramUniform4i64vNV")),
         ProgramUniform4iEXT: FnPtr::new(loadfn("glProgramUniform4iEXT")),
         ProgramUniform4iv: FnPtr::new(loadfn("glProgramUniform4iv")),
         ProgramUniform4ivEXT: FnPtr::new(loadfn("glProgramUniform4ivEXT")),
         ProgramUniform4ui: FnPtr::new(loadfn("glProgramUniform4ui")),
         ProgramUniform4ui64ARB: FnPtr::new(loadfn("glProgramUniform4ui64ARB")),
         ProgramUniform4ui64NV: FnPtr::new(loadfn("glProgramUniform4ui64NV")),
         ProgramUniform4ui64vARB: FnPtr::new(loadfn("glProgramUniform4ui64vARB")),
         ProgramUniform4ui64vNV: FnPtr::new(loadfn("glProgramUniform4ui64vNV")),
         ProgramUniform4uiEXT: FnPtr::new(loadfn("glProgramUniform4uiEXT")),
         ProgramUniform4uiv: FnPtr::new(loadfn("glProgramUniform4uiv")),
         ProgramUniform4uivEXT: FnPtr::new(loadfn("glProgramUniform4uivEXT")),
         ProgramUniformHandleui64ARB: FnPtr::new(loadfn("glProgramUniformHandleui64ARB")),
         ProgramUniformHandleui64NV: FnPtr::new(loadfn("glProgramUniformHandleui64NV")),
         ProgramUniformHandleui64vARB: FnPtr::new(loadfn("glProgramUniformHandleui64vARB")),
         ProgramUniformHandleui64vNV: FnPtr::new(loadfn("glProgramUniformHandleui64vNV")),
         ProgramUniformMatrix2dv: FnPtr::new(loadfn("glProgramUniformMatrix2dv")),
         ProgramUniformMatrix2dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2dvEXT")),
         ProgramUniformMatrix2fv: FnPtr::new(loadfn("glProgramUniformMatrix2fv")),
         ProgramUniformMatrix2fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2fvEXT")),
         ProgramUniformMatrix2x3dv: FnPtr::new(loadfn("glProgramUniformMatrix2x3dv")),
         ProgramUniformMatrix2x3dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2x3dvEXT")),
         ProgramUniformMatrix2x3fv: FnPtr::new(loadfn("glProgramUniformMatrix2x3fv")),
         ProgramUniformMatrix2x3fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2x3fvEXT")),
         ProgramUniformMatrix2x4dv: FnPtr::new(loadfn("glProgramUniformMatrix2x4dv")),
         ProgramUniformMatrix2x4dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2x4dvEXT")),
         ProgramUniformMatrix2x4fv: FnPtr::new(loadfn("glProgramUniformMatrix2x4fv")),
         ProgramUniformMatrix2x4fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2x4fvEXT")),
         ProgramUniformMatrix3dv: FnPtr::new(loadfn("glProgramUniformMatrix3dv")),
         ProgramUniformMatrix3dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3dvEXT")),
         ProgramUniformMatrix3fv: FnPtr::new(loadfn("glProgramUniformMatrix3fv")),
         ProgramUniformMatrix3fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3fvEXT")),
         ProgramUniformMatrix3x2dv: FnPtr::new(loadfn("glProgramUniformMatrix3x2dv")),
         ProgramUniformMatrix3x2dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3x2dvEXT")),
         ProgramUniformMatrix3x2fv: FnPtr::new(loadfn("glProgramUniformMatrix3x2fv")),
         ProgramUniformMatrix3x2fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3x2fvEXT")),
         ProgramUniformMatrix3x4dv: FnPtr::new(loadfn("glProgramUniformMatrix3x4dv")),
         ProgramUniformMatrix3x4dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3x4dvEXT")),
         ProgramUniformMatrix3x4fv: FnPtr::new(loadfn("glProgramUniformMatrix3x4fv")),
         ProgramUniformMatrix3x4fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3x4fvEXT")),
         ProgramUniformMatrix4dv: FnPtr::new(loadfn("glProgramUniformMatrix4dv")),
         ProgramUniformMatrix4dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4dvEXT")),
         ProgramUniformMatrix4fv: FnPtr::new(loadfn("glProgramUniformMatrix4fv")),
         ProgramUniformMatrix4fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4fvEXT")),
         ProgramUniformMatrix4x2dv: FnPtr::new(loadfn("glProgramUniformMatrix4x2dv")),
         ProgramUniformMatrix4x2dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4x2dvEXT")),
         ProgramUniformMatrix4x2fv: FnPtr::new(loadfn("glProgramUniformMatrix4x2fv")),
         ProgramUniformMatrix4x2fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4x2fvEXT")),
         ProgramUniformMatrix4x3dv: FnPtr::new(loadfn("glProgramUniformMatrix4x3dv")),
         ProgramUniformMatrix4x3dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4x3dvEXT")),
         ProgramUniformMatrix4x3fv: FnPtr::new(loadfn("glProgramUniformMatrix4x3fv")),
         ProgramUniformMatrix4x3fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4x3fvEXT")),
         ProgramUniformui64NV: FnPtr::new(loadfn("glProgramUniformui64NV")),
         ProgramUniformui64vNV: FnPtr::new(loadfn("glProgramUniformui64vNV")),
         ProgramVertexLimitNV: FnPtr::new(loadfn("glProgramVertexLimitNV")),
         ProvokingVertex: FnPtr::new(loadfn("glProvokingVertex")),
         ProvokingVertexEXT: FnPtr::new(loadfn("glProvokingVertexEXT")),
         PushClientAttribDefaultEXT: FnPtr::new(loadfn("glPushClientAttribDefaultEXT")),
         PushDebugGroup: FnPtr::new(loadfn("glPushDebugGroup")),
         PushGroupMarkerEXT: FnPtr::new(loadfn("glPushGroupMarkerEXT")),
         QueryCounter: FnPtr::new(loadfn("glQueryCounter")),
         QueryMatrixxOES: FnPtr::new(loadfn("glQueryMatrixxOES")),
         QueryObjectParameteruiAMD: FnPtr::new(loadfn("glQueryObjectParameteruiAMD")),
         QueryResourceNV: FnPtr::new(loadfn("glQueryResourceNV")),
         QueryResourceTagNV: FnPtr::new(loadfn("glQueryResourceTagNV")),
         RasterPos2xOES: FnPtr::new(loadfn("glRasterPos2xOES")),
         RasterPos2xvOES: FnPtr::new(loadfn("glRasterPos2xvOES")),
         RasterPos3xOES: FnPtr::new(loadfn("glRasterPos3xOES")),
         RasterPos3xvOES: FnPtr::new(loadfn("glRasterPos3xvOES")),
         RasterPos4xOES: FnPtr::new(loadfn("glRasterPos4xOES")),
         RasterPos4xvOES: FnPtr::new(loadfn("glRasterPos4xvOES")),
         RasterSamplesEXT: FnPtr::new(loadfn("glRasterSamplesEXT")),
         ReadBuffer: FnPtr::new(loadfn("glReadBuffer")),
         ReadInstrumentsSGIX: FnPtr::new(loadfn("glReadInstrumentsSGIX")),
         ReadPixels: FnPtr::new(loadfn("glReadPixels")),
         ReadnPixels: FnPtr::new(loadfn("glReadnPixels")),
         ReadnPixelsARB: FnPtr::new(loadfn("glReadnPixelsARB")),
         RectxOES: FnPtr::new(loadfn("glRectxOES")),
         RectxvOES: FnPtr::new(loadfn("glRectxvOES")),
         ReferencePlaneSGIX: FnPtr::new(loadfn("glReferencePlaneSGIX")),
         ReleaseKeyedMutexWin32EXT: FnPtr::new(loadfn("glReleaseKeyedMutexWin32EXT")),
         ReleaseShaderCompiler: FnPtr::new(loadfn("glReleaseShaderCompiler")),
         RenderGpuMaskNV: FnPtr::new(loadfn("glRenderGpuMaskNV")),
         RenderbufferStorage: FnPtr::new(loadfn("glRenderbufferStorage")),
         RenderbufferStorageEXT: FnPtr::new(loadfn("glRenderbufferStorageEXT")),
         RenderbufferStorageMultisample: FnPtr::new(loadfn("glRenderbufferStorageMultisample")),
         RenderbufferStorageMultisampleAdvancedAMD: FnPtr::new(loadfn("glRenderbufferStorageMultisampleAdvancedAMD")),
         RenderbufferStorageMultisampleCoverageNV: FnPtr::new(loadfn("glRenderbufferStorageMultisampleCoverageNV")),
         RenderbufferStorageMultisampleEXT: FnPtr::new(loadfn("glRenderbufferStorageMultisampleEXT")),
         ReplacementCodePointerSUN: FnPtr::new(loadfn("glReplacementCodePointerSUN")),
         ReplacementCodeubSUN: FnPtr::new(loadfn("glReplacementCodeubSUN")),
         ReplacementCodeubvSUN: FnPtr::new(loadfn("glReplacementCodeubvSUN")),
         ReplacementCodeuiColor3fVertex3fSUN: FnPtr::new(loadfn("glReplacementCodeuiColor3fVertex3fSUN")),
         ReplacementCodeuiColor3fVertex3fvSUN: FnPtr::new(loadfn("glReplacementCodeuiColor3fVertex3fvSUN")),
         ReplacementCodeuiColor4fNormal3fVertex3fSUN: FnPtr::new(loadfn("glReplacementCodeuiColor4fNormal3fVertex3fSUN")),
         ReplacementCodeuiColor4fNormal3fVertex3fvSUN: FnPtr::new(loadfn("glReplacementCodeuiColor4fNormal3fVertex3fvSUN")),
         ReplacementCodeuiColor4ubVertex3fSUN: FnPtr::new(loadfn("glReplacementCodeuiColor4ubVertex3fSUN")),
         ReplacementCodeuiColor4ubVertex3fvSUN: FnPtr::new(loadfn("glReplacementCodeuiColor4ubVertex3fvSUN")),
         ReplacementCodeuiNormal3fVertex3fSUN: FnPtr::new(loadfn("glReplacementCodeuiNormal3fVertex3fSUN")),
         ReplacementCodeuiNormal3fVertex3fvSUN: FnPtr::new(loadfn("glReplacementCodeuiNormal3fVertex3fvSUN")),
         ReplacementCodeuiSUN: FnPtr::new(loadfn("glReplacementCodeuiSUN")),
         ReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN: FnPtr::new(loadfn("glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN")),
         ReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN: FnPtr::new(loadfn("glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN")),
         ReplacementCodeuiTexCoord2fNormal3fVertex3fSUN: FnPtr::new(loadfn("glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN")),
         ReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN: FnPtr::new(loadfn("glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN")),
         ReplacementCodeuiTexCoord2fVertex3fSUN: FnPtr::new(loadfn("glReplacementCodeuiTexCoord2fVertex3fSUN")),
         ReplacementCodeuiTexCoord2fVertex3fvSUN: FnPtr::new(loadfn("glReplacementCodeuiTexCoord2fVertex3fvSUN")),
         ReplacementCodeuiVertex3fSUN: FnPtr::new(loadfn("glReplacementCodeuiVertex3fSUN")),
         ReplacementCodeuiVertex3fvSUN: FnPtr::new(loadfn("glReplacementCodeuiVertex3fvSUN")),
         ReplacementCodeuivSUN: FnPtr::new(loadfn("glReplacementCodeuivSUN")),
         ReplacementCodeusSUN: FnPtr::new(loadfn("glReplacementCodeusSUN")),
         ReplacementCodeusvSUN: FnPtr::new(loadfn("glReplacementCodeusvSUN")),
         RequestResidentProgramsNV: FnPtr::new(loadfn("glRequestResidentProgramsNV")),
         ResetHistogramEXT: FnPtr::new(loadfn("glResetHistogramEXT")),
         ResetMemoryObjectParameterNV: FnPtr::new(loadfn("glResetMemoryObjectParameterNV")),
         ResetMinmaxEXT: FnPtr::new(loadfn("glResetMinmaxEXT")),
         ResizeBuffersMESA: FnPtr::new(loadfn("glResizeBuffersMESA")),
         ResolveDepthValuesNV: FnPtr::new(loadfn("glResolveDepthValuesNV")),
         ResumeTransformFeedback: FnPtr::new(loadfn("glResumeTransformFeedback")),
         ResumeTransformFeedbackNV: FnPtr::new(loadfn("glResumeTransformFeedbackNV")),
         RotatexOES: FnPtr::new(loadfn("glRotatexOES")),
         SampleCoverage: FnPtr::new(loadfn("glSampleCoverage")),
         SampleCoverageARB: FnPtr::new(loadfn("glSampleCoverageARB")),
         SampleMapATI: FnPtr::new(loadfn("glSampleMapATI")),
         SampleMaskEXT: FnPtr::new(loadfn("glSampleMaskEXT")),
         SampleMaskIndexedNV: FnPtr::new(loadfn("glSampleMaskIndexedNV")),
         SampleMaskSGIS: FnPtr::new(loadfn("glSampleMaskSGIS")),
         SampleMaski: FnPtr::new(loadfn("glSampleMaski")),
         SamplePatternEXT: FnPtr::new(loadfn("glSamplePatternEXT")),
         SamplePatternSGIS: FnPtr::new(loadfn("glSamplePatternSGIS")),
         SamplerParameterIiv: FnPtr::new(loadfn("glSamplerParameterIiv")),
         SamplerParameterIuiv: FnPtr::new(loadfn("glSamplerParameterIuiv")),
         SamplerParameterf: FnPtr::new(loadfn("glSamplerParameterf")),
         SamplerParameterfv: FnPtr::new(loadfn("glSamplerParameterfv")),
         SamplerParameteri: FnPtr::new(loadfn("glSamplerParameteri")),
         SamplerParameteriv: FnPtr::new(loadfn("glSamplerParameteriv")),
         ScalexOES: FnPtr::new(loadfn("glScalexOES")),
         Scissor: FnPtr::new(loadfn("glScissor")),
         ScissorArrayv: FnPtr::new(loadfn("glScissorArrayv")),
         ScissorExclusiveArrayvNV: FnPtr::new(loadfn("glScissorExclusiveArrayvNV")),
         ScissorExclusiveNV: FnPtr::new(loadfn("glScissorExclusiveNV")),
         ScissorIndexed: FnPtr::new(loadfn("glScissorIndexed")),
         ScissorIndexedv: FnPtr::new(loadfn("glScissorIndexedv")),
         SecondaryColor3bEXT: FnPtr::new(loadfn("glSecondaryColor3bEXT")),
         SecondaryColor3bvEXT: FnPtr::new(loadfn("glSecondaryColor3bvEXT")),
         SecondaryColor3dEXT: FnPtr::new(loadfn("glSecondaryColor3dEXT")),
         SecondaryColor3dvEXT: FnPtr::new(loadfn("glSecondaryColor3dvEXT")),
         SecondaryColor3fEXT: FnPtr::new(loadfn("glSecondaryColor3fEXT")),
         SecondaryColor3fvEXT: FnPtr::new(loadfn("glSecondaryColor3fvEXT")),
         SecondaryColor3hNV: FnPtr::new(loadfn("glSecondaryColor3hNV")),
         SecondaryColor3hvNV: FnPtr::new(loadfn("glSecondaryColor3hvNV")),
         SecondaryColor3iEXT: FnPtr::new(loadfn("glSecondaryColor3iEXT")),
         SecondaryColor3ivEXT: FnPtr::new(loadfn("glSecondaryColor3ivEXT")),
         SecondaryColor3sEXT: FnPtr::new(loadfn("glSecondaryColor3sEXT")),
         SecondaryColor3svEXT: FnPtr::new(loadfn("glSecondaryColor3svEXT")),
         SecondaryColor3ubEXT: FnPtr::new(loadfn("glSecondaryColor3ubEXT")),
         SecondaryColor3ubvEXT: FnPtr::new(loadfn("glSecondaryColor3ubvEXT")),
         SecondaryColor3uiEXT: FnPtr::new(loadfn("glSecondaryColor3uiEXT")),
         SecondaryColor3uivEXT: FnPtr::new(loadfn("glSecondaryColor3uivEXT")),
         SecondaryColor3usEXT: FnPtr::new(loadfn("glSecondaryColor3usEXT")),
         SecondaryColor3usvEXT: FnPtr::new(loadfn("glSecondaryColor3usvEXT")),
         SecondaryColorFormatNV: FnPtr::new(loadfn("glSecondaryColorFormatNV")),
         SecondaryColorPointerEXT: FnPtr::new(loadfn("glSecondaryColorPointerEXT")),
         SecondaryColorPointerListIBM: FnPtr::new(loadfn("glSecondaryColorPointerListIBM")),
         SelectPerfMonitorCountersAMD: FnPtr::new(loadfn("glSelectPerfMonitorCountersAMD")),
         SemaphoreParameterivNV: FnPtr::new(loadfn("glSemaphoreParameterivNV")),
         SemaphoreParameterui64vEXT: FnPtr::new(loadfn("glSemaphoreParameterui64vEXT")),
         SeparableFilter2DEXT: FnPtr::new(loadfn("glSeparableFilter2DEXT")),
         SetFenceAPPLE: FnPtr::new(loadfn("glSetFenceAPPLE")),
         SetFenceNV: FnPtr::new(loadfn("glSetFenceNV")),
         SetFragmentShaderConstantATI: FnPtr::new(loadfn("glSetFragmentShaderConstantATI")),
         SetInvariantEXT: FnPtr::new(loadfn("glSetInvariantEXT")),
         SetLocalConstantEXT: FnPtr::new(loadfn("glSetLocalConstantEXT")),
         SetMultisamplefvAMD: FnPtr::new(loadfn("glSetMultisamplefvAMD")),
         ShaderBinary: FnPtr::new(loadfn("glShaderBinary")),
         ShaderOp1EXT: FnPtr::new(loadfn("glShaderOp1EXT")),
         ShaderOp2EXT: FnPtr::new(loadfn("glShaderOp2EXT")),
         ShaderOp3EXT: FnPtr::new(loadfn("glShaderOp3EXT")),
         ShaderSource: FnPtr::new(loadfn("glShaderSource")),
         ShaderSourceARB: FnPtr::new(loadfn("glShaderSourceARB")),
         ShaderStorageBlockBinding: FnPtr::new(loadfn("glShaderStorageBlockBinding")),
         ShadingRateImageBarrierNV: FnPtr::new(loadfn("glShadingRateImageBarrierNV")),
         ShadingRateImagePaletteNV: FnPtr::new(loadfn("glShadingRateImagePaletteNV")),
         ShadingRateSampleOrderCustomNV: FnPtr::new(loadfn("glShadingRateSampleOrderCustomNV")),
         ShadingRateSampleOrderNV: FnPtr::new(loadfn("glShadingRateSampleOrderNV")),
         SharpenTexFuncSGIS: FnPtr::new(loadfn("glSharpenTexFuncSGIS")),
         SignalSemaphoreEXT: FnPtr::new(loadfn("glSignalSemaphoreEXT")),
         SignalSemaphoreui64NVX: FnPtr::new(loadfn("glSignalSemaphoreui64NVX")),
         SignalVkFenceNV: FnPtr::new(loadfn("glSignalVkFenceNV")),
         SignalVkSemaphoreNV: FnPtr::new(loadfn("glSignalVkSemaphoreNV")),
         SpecializeShader: FnPtr::new(loadfn("glSpecializeShader")),
         SpecializeShaderARB: FnPtr::new(loadfn("glSpecializeShaderARB")),
         SpriteParameterfSGIX: FnPtr::new(loadfn("glSpriteParameterfSGIX")),
         SpriteParameterfvSGIX: FnPtr::new(loadfn("glSpriteParameterfvSGIX")),
         SpriteParameteriSGIX: FnPtr::new(loadfn("glSpriteParameteriSGIX")),
         SpriteParameterivSGIX: FnPtr::new(loadfn("glSpriteParameterivSGIX")),
         StartInstrumentsSGIX: FnPtr::new(loadfn("glStartInstrumentsSGIX")),
         StateCaptureNV: FnPtr::new(loadfn("glStateCaptureNV")),
         StencilClearTagEXT: FnPtr::new(loadfn("glStencilClearTagEXT")),
         StencilFillPathInstancedNV: FnPtr::new(loadfn("glStencilFillPathInstancedNV")),
         StencilFillPathNV: FnPtr::new(loadfn("glStencilFillPathNV")),
         StencilFunc: FnPtr::new(loadfn("glStencilFunc")),
         StencilFuncSeparate: FnPtr::new(loadfn("glStencilFuncSeparate")),
         StencilFuncSeparateATI: FnPtr::new(loadfn("glStencilFuncSeparateATI")),
         StencilMask: FnPtr::new(loadfn("glStencilMask")),
         StencilMaskSeparate: FnPtr::new(loadfn("glStencilMaskSeparate")),
         StencilOp: FnPtr::new(loadfn("glStencilOp")),
         StencilOpSeparate: FnPtr::new(loadfn("glStencilOpSeparate")),
         StencilOpSeparateATI: FnPtr::new(loadfn("glStencilOpSeparateATI")),
         StencilOpValueAMD: FnPtr::new(loadfn("glStencilOpValueAMD")),
         StencilStrokePathInstancedNV: FnPtr::new(loadfn("glStencilStrokePathInstancedNV")),
         StencilStrokePathNV: FnPtr::new(loadfn("glStencilStrokePathNV")),
         StencilThenCoverFillPathInstancedNV: FnPtr::new(loadfn("glStencilThenCoverFillPathInstancedNV")),
         StencilThenCoverFillPathNV: FnPtr::new(loadfn("glStencilThenCoverFillPathNV")),
         StencilThenCoverStrokePathInstancedNV: FnPtr::new(loadfn("glStencilThenCoverStrokePathInstancedNV")),
         StencilThenCoverStrokePathNV: FnPtr::new(loadfn("glStencilThenCoverStrokePathNV")),
         StopInstrumentsSGIX: FnPtr::new(loadfn("glStopInstrumentsSGIX")),
         StringMarkerGREMEDY: FnPtr::new(loadfn("glStringMarkerGREMEDY")),
         SubpixelPrecisionBiasNV: FnPtr::new(loadfn("glSubpixelPrecisionBiasNV")),
         SwizzleEXT: FnPtr::new(loadfn("glSwizzleEXT")),
         SyncTextureINTEL: FnPtr::new(loadfn("glSyncTextureINTEL")),
         TagSampleBufferSGIX: FnPtr::new(loadfn("glTagSampleBufferSGIX")),
         Tangent3bEXT: FnPtr::new(loadfn("glTangent3bEXT")),
         Tangent3bvEXT: FnPtr::new(loadfn("glTangent3bvEXT")),
         Tangent3dEXT: FnPtr::new(loadfn("glTangent3dEXT")),
         Tangent3dvEXT: FnPtr::new(loadfn("glTangent3dvEXT")),
         Tangent3fEXT: FnPtr::new(loadfn("glTangent3fEXT")),
         Tangent3fvEXT: FnPtr::new(loadfn("glTangent3fvEXT")),
         Tangent3iEXT: FnPtr::new(loadfn("glTangent3iEXT")),
         Tangent3ivEXT: FnPtr::new(loadfn("glTangent3ivEXT")),
         Tangent3sEXT: FnPtr::new(loadfn("glTangent3sEXT")),
         Tangent3svEXT: FnPtr::new(loadfn("glTangent3svEXT")),
         TangentPointerEXT: FnPtr::new(loadfn("glTangentPointerEXT")),
         TbufferMask3DFX: FnPtr::new(loadfn("glTbufferMask3DFX")),
         TessellationFactorAMD: FnPtr::new(loadfn("glTessellationFactorAMD")),
         TessellationModeAMD: FnPtr::new(loadfn("glTessellationModeAMD")),
         TestFenceAPPLE: FnPtr::new(loadfn("glTestFenceAPPLE")),
         TestFenceNV: FnPtr::new(loadfn("glTestFenceNV")),
         TestObjectAPPLE: FnPtr::new(loadfn("glTestObjectAPPLE")),
         TexAttachMemoryNV: FnPtr::new(loadfn("glTexAttachMemoryNV")),
         TexBuffer: FnPtr::new(loadfn("glTexBuffer")),
         TexBufferARB: FnPtr::new(loadfn("glTexBufferARB")),
         TexBufferEXT: FnPtr::new(loadfn("glTexBufferEXT")),
         TexBufferRange: FnPtr::new(loadfn("glTexBufferRange")),
         TexBumpParameterfvATI: FnPtr::new(loadfn("glTexBumpParameterfvATI")),
         TexBumpParameterivATI: FnPtr::new(loadfn("glTexBumpParameterivATI")),
         TexCoord1bOES: FnPtr::new(loadfn("glTexCoord1bOES")),
         TexCoord1bvOES: FnPtr::new(loadfn("glTexCoord1bvOES")),
         TexCoord1hNV: FnPtr::new(loadfn("glTexCoord1hNV")),
         TexCoord1hvNV: FnPtr::new(loadfn("glTexCoord1hvNV")),
         TexCoord1xOES: FnPtr::new(loadfn("glTexCoord1xOES")),
         TexCoord1xvOES: FnPtr::new(loadfn("glTexCoord1xvOES")),
         TexCoord2bOES: FnPtr::new(loadfn("glTexCoord2bOES")),
         TexCoord2bvOES: FnPtr::new(loadfn("glTexCoord2bvOES")),
         TexCoord2fColor3fVertex3fSUN: FnPtr::new(loadfn("glTexCoord2fColor3fVertex3fSUN")),
         TexCoord2fColor3fVertex3fvSUN: FnPtr::new(loadfn("glTexCoord2fColor3fVertex3fvSUN")),
         TexCoord2fColor4fNormal3fVertex3fSUN: FnPtr::new(loadfn("glTexCoord2fColor4fNormal3fVertex3fSUN")),
         TexCoord2fColor4fNormal3fVertex3fvSUN: FnPtr::new(loadfn("glTexCoord2fColor4fNormal3fVertex3fvSUN")),
         TexCoord2fColor4ubVertex3fSUN: FnPtr::new(loadfn("glTexCoord2fColor4ubVertex3fSUN")),
         TexCoord2fColor4ubVertex3fvSUN: FnPtr::new(loadfn("glTexCoord2fColor4ubVertex3fvSUN")),
         TexCoord2fNormal3fVertex3fSUN: FnPtr::new(loadfn("glTexCoord2fNormal3fVertex3fSUN")),
         TexCoord2fNormal3fVertex3fvSUN: FnPtr::new(loadfn("glTexCoord2fNormal3fVertex3fvSUN")),
         TexCoord2fVertex3fSUN: FnPtr::new(loadfn("glTexCoord2fVertex3fSUN")),
         TexCoord2fVertex3fvSUN: FnPtr::new(loadfn("glTexCoord2fVertex3fvSUN")),
         TexCoord2hNV: FnPtr::new(loadfn("glTexCoord2hNV")),
         TexCoord2hvNV: FnPtr::new(loadfn("glTexCoord2hvNV")),
         TexCoord2xOES: FnPtr::new(loadfn("glTexCoord2xOES")),
         TexCoord2xvOES: FnPtr::new(loadfn("glTexCoord2xvOES")),
         TexCoord3bOES: FnPtr::new(loadfn("glTexCoord3bOES")),
         TexCoord3bvOES: FnPtr::new(loadfn("glTexCoord3bvOES")),
         TexCoord3hNV: FnPtr::new(loadfn("glTexCoord3hNV")),
         TexCoord3hvNV: FnPtr::new(loadfn("glTexCoord3hvNV")),
         TexCoord3xOES: FnPtr::new(loadfn("glTexCoord3xOES")),
         TexCoord3xvOES: FnPtr::new(loadfn("glTexCoord3xvOES")),
         TexCoord4bOES: FnPtr::new(loadfn("glTexCoord4bOES")),
         TexCoord4bvOES: FnPtr::new(loadfn("glTexCoord4bvOES")),
         TexCoord4fColor4fNormal3fVertex4fSUN: FnPtr::new(loadfn("glTexCoord4fColor4fNormal3fVertex4fSUN")),
         TexCoord4fColor4fNormal3fVertex4fvSUN: FnPtr::new(loadfn("glTexCoord4fColor4fNormal3fVertex4fvSUN")),
         TexCoord4fVertex4fSUN: FnPtr::new(loadfn("glTexCoord4fVertex4fSUN")),
         TexCoord4fVertex4fvSUN: FnPtr::new(loadfn("glTexCoord4fVertex4fvSUN")),
         TexCoord4hNV: FnPtr::new(loadfn("glTexCoord4hNV")),
         TexCoord4hvNV: FnPtr::new(loadfn("glTexCoord4hvNV")),
         TexCoord4xOES: FnPtr::new(loadfn("glTexCoord4xOES")),
         TexCoord4xvOES: FnPtr::new(loadfn("glTexCoord4xvOES")),
         TexCoordFormatNV: FnPtr::new(loadfn("glTexCoordFormatNV")),
         TexCoordPointerEXT: FnPtr::new(loadfn("glTexCoordPointerEXT")),
         TexCoordPointerListIBM: FnPtr::new(loadfn("glTexCoordPointerListIBM")),
         TexCoordPointervINTEL: FnPtr::new(loadfn("glTexCoordPointervINTEL")),
         TexEnvxOES: FnPtr::new(loadfn("glTexEnvxOES")),
         TexEnvxvOES: FnPtr::new(loadfn("glTexEnvxvOES")),
         TexFilterFuncSGIS: FnPtr::new(loadfn("glTexFilterFuncSGIS")),
         TexGenxOES: FnPtr::new(loadfn("glTexGenxOES")),
         TexGenxvOES: FnPtr::new(loadfn("glTexGenxvOES")),
         TexImage1D: FnPtr::new(loadfn("glTexImage1D")),
         TexImage2D: FnPtr::new(loadfn("glTexImage2D")),
         TexImage2DMultisample: FnPtr::new(loadfn("glTexImage2DMultisample")),
         TexImage2DMultisampleCoverageNV: FnPtr::new(loadfn("glTexImage2DMultisampleCoverageNV")),
         TexImage3D: FnPtr::new(loadfn("glTexImage3D")),
         TexImage3DEXT: FnPtr::new(loadfn("glTexImage3DEXT")),
         TexImage3DMultisample: FnPtr::new(loadfn("glTexImage3DMultisample")),
         TexImage3DMultisampleCoverageNV: FnPtr::new(loadfn("glTexImage3DMultisampleCoverageNV")),
         TexImage4DSGIS: FnPtr::new(loadfn("glTexImage4DSGIS")),
         TexPageCommitmentARB: FnPtr::new(loadfn("glTexPageCommitmentARB")),
         TexPageCommitmentMemNV: FnPtr::new(loadfn("glTexPageCommitmentMemNV")),
         TexParameterIiv: FnPtr::new(loadfn("glTexParameterIiv")),
         TexParameterIivEXT: FnPtr::new(loadfn("glTexParameterIivEXT")),
         TexParameterIuiv: FnPtr::new(loadfn("glTexParameterIuiv")),
         TexParameterIuivEXT: FnPtr::new(loadfn("glTexParameterIuivEXT")),
         TexParameterf: FnPtr::new(loadfn("glTexParameterf")),
         TexParameterfv: FnPtr::new(loadfn("glTexParameterfv")),
         TexParameteri: FnPtr::new(loadfn("glTexParameteri")),
         TexParameteriv: FnPtr::new(loadfn("glTexParameteriv")),
         TexParameterxOES: FnPtr::new(loadfn("glTexParameterxOES")),
         TexParameterxvOES: FnPtr::new(loadfn("glTexParameterxvOES")),
         TexRenderbufferNV: FnPtr::new(loadfn("glTexRenderbufferNV")),
         TexStorage1D: FnPtr::new(loadfn("glTexStorage1D")),
         TexStorage1DEXT: FnPtr::new(loadfn("glTexStorage1DEXT")),
         TexStorage2D: FnPtr::new(loadfn("glTexStorage2D")),
         TexStorage2DEXT: FnPtr::new(loadfn("glTexStorage2DEXT")),
         TexStorage2DMultisample: FnPtr::new(loadfn("glTexStorage2DMultisample")),
         TexStorage3D: FnPtr::new(loadfn("glTexStorage3D")),
         TexStorage3DEXT: FnPtr::new(loadfn("glTexStorage3DEXT")),
         TexStorage3DMultisample: FnPtr::new(loadfn("glTexStorage3DMultisample")),
         TexStorageMem1DEXT: FnPtr::new(loadfn("glTexStorageMem1DEXT")),
         TexStorageMem2DEXT: FnPtr::new(loadfn("glTexStorageMem2DEXT")),
         TexStorageMem2DMultisampleEXT: FnPtr::new(loadfn("glTexStorageMem2DMultisampleEXT")),
         TexStorageMem3DEXT: FnPtr::new(loadfn("glTexStorageMem3DEXT")),
         TexStorageMem3DMultisampleEXT: FnPtr::new(loadfn("glTexStorageMem3DMultisampleEXT")),
         TexStorageSparseAMD: FnPtr::new(loadfn("glTexStorageSparseAMD")),
         TexSubImage1D: FnPtr::new(loadfn("glTexSubImage1D")),
         TexSubImage1DEXT: FnPtr::new(loadfn("glTexSubImage1DEXT")),
         TexSubImage2D: FnPtr::new(loadfn("glTexSubImage2D")),
         TexSubImage2DEXT: FnPtr::new(loadfn("glTexSubImage2DEXT")),
         TexSubImage3D: FnPtr::new(loadfn("glTexSubImage3D")),
         TexSubImage3DEXT: FnPtr::new(loadfn("glTexSubImage3DEXT")),
         TexSubImage4DSGIS: FnPtr::new(loadfn("glTexSubImage4DSGIS")),
         TextureAttachMemoryNV: FnPtr::new(loadfn("glTextureAttachMemoryNV")),
         TextureBarrier: FnPtr::new(loadfn("glTextureBarrier")),
         TextureBarrierNV: FnPtr::new(loadfn("glTextureBarrierNV")),
         TextureBuffer: FnPtr::new(loadfn("glTextureBuffer")),
         TextureBufferEXT: FnPtr::new(loadfn("glTextureBufferEXT")),
         TextureBufferRange: FnPtr::new(loadfn("glTextureBufferRange")),
         TextureBufferRangeEXT: FnPtr::new(loadfn("glTextureBufferRangeEXT")),
         TextureColorMaskSGIS: FnPtr::new(loadfn("glTextureColorMaskSGIS")),
         TextureImage1DEXT: FnPtr::new(loadfn("glTextureImage1DEXT")),
         TextureImage2DEXT: FnPtr::new(loadfn("glTextureImage2DEXT")),
         TextureImage2DMultisampleCoverageNV: FnPtr::new(loadfn("glTextureImage2DMultisampleCoverageNV")),
         TextureImage2DMultisampleNV: FnPtr::new(loadfn("glTextureImage2DMultisampleNV")),
         TextureImage3DEXT: FnPtr::new(loadfn("glTextureImage3DEXT")),
         TextureImage3DMultisampleCoverageNV: FnPtr::new(loadfn("glTextureImage3DMultisampleCoverageNV")),
         TextureImage3DMultisampleNV: FnPtr::new(loadfn("glTextureImage3DMultisampleNV")),
         TextureLightEXT: FnPtr::new(loadfn("glTextureLightEXT")),
         TextureMaterialEXT: FnPtr::new(loadfn("glTextureMaterialEXT")),
         TextureNormalEXT: FnPtr::new(loadfn("glTextureNormalEXT")),
         TexturePageCommitmentEXT: FnPtr::new(loadfn("glTexturePageCommitmentEXT")),
         TexturePageCommitmentMemNV: FnPtr::new(loadfn("glTexturePageCommitmentMemNV")),
         TextureParameterIiv: FnPtr::new(loadfn("glTextureParameterIiv")),
         TextureParameterIivEXT: FnPtr::new(loadfn("glTextureParameterIivEXT")),
         TextureParameterIuiv: FnPtr::new(loadfn("glTextureParameterIuiv")),
         TextureParameterIuivEXT: FnPtr::new(loadfn("glTextureParameterIuivEXT")),
         TextureParameterf: FnPtr::new(loadfn("glTextureParameterf")),
         TextureParameterfEXT: FnPtr::new(loadfn("glTextureParameterfEXT")),
         TextureParameterfv: FnPtr::new(loadfn("glTextureParameterfv")),
         TextureParameterfvEXT: FnPtr::new(loadfn("glTextureParameterfvEXT")),
         TextureParameteri: FnPtr::new(loadfn("glTextureParameteri")),
         TextureParameteriEXT: FnPtr::new(loadfn("glTextureParameteriEXT")),
         TextureParameteriv: FnPtr::new(loadfn("glTextureParameteriv")),
         TextureParameterivEXT: FnPtr::new(loadfn("glTextureParameterivEXT")),
         TextureRangeAPPLE: FnPtr::new(loadfn("glTextureRangeAPPLE")),
         TextureRenderbufferEXT: FnPtr::new(loadfn("glTextureRenderbufferEXT")),
         TextureStorage1D: FnPtr::new(loadfn("glTextureStorage1D")),
         TextureStorage1DEXT: FnPtr::new(loadfn("glTextureStorage1DEXT")),
         TextureStorage2D: FnPtr::new(loadfn("glTextureStorage2D")),
         TextureStorage2DEXT: FnPtr::new(loadfn("glTextureStorage2DEXT")),
         TextureStorage2DMultisample: FnPtr::new(loadfn("glTextureStorage2DMultisample")),
         TextureStorage2DMultisampleEXT: FnPtr::new(loadfn("glTextureStorage2DMultisampleEXT")),
         TextureStorage3D: FnPtr::new(loadfn("glTextureStorage3D")),
         TextureStorage3DEXT: FnPtr::new(loadfn("glTextureStorage3DEXT")),
         TextureStorage3DMultisample: FnPtr::new(loadfn("glTextureStorage3DMultisample")),
         TextureStorage3DMultisampleEXT: FnPtr::new(loadfn("glTextureStorage3DMultisampleEXT")),
         TextureStorageMem1DEXT: FnPtr::new(loadfn("glTextureStorageMem1DEXT")),
         TextureStorageMem2DEXT: FnPtr::new(loadfn("glTextureStorageMem2DEXT")),
         TextureStorageMem2DMultisampleEXT: FnPtr::new(loadfn("glTextureStorageMem2DMultisampleEXT")),
         TextureStorageMem3DEXT: FnPtr::new(loadfn("glTextureStorageMem3DEXT")),
         TextureStorageMem3DMultisampleEXT: FnPtr::new(loadfn("glTextureStorageMem3DMultisampleEXT")),
         TextureStorageSparseAMD: FnPtr::new(loadfn("glTextureStorageSparseAMD")),
         TextureSubImage1D: FnPtr::new(loadfn("glTextureSubImage1D")),
         TextureSubImage1DEXT: FnPtr::new(loadfn("glTextureSubImage1DEXT")),
         TextureSubImage2D: FnPtr::new(loadfn("glTextureSubImage2D")),
         TextureSubImage2DEXT: FnPtr::new(loadfn("glTextureSubImage2DEXT")),
         TextureSubImage3D: FnPtr::new(loadfn("glTextureSubImage3D")),
         TextureSubImage3DEXT: FnPtr::new(loadfn("glTextureSubImage3DEXT")),
         TextureView: FnPtr::new(loadfn("glTextureView")),
         TrackMatrixNV: FnPtr::new(loadfn("glTrackMatrixNV")),
         TransformFeedbackAttribsNV: FnPtr::new(loadfn("glTransformFeedbackAttribsNV")),
         TransformFeedbackBufferBase: FnPtr::new(loadfn("glTransformFeedbackBufferBase")),
         TransformFeedbackBufferRange: FnPtr::new(loadfn("glTransformFeedbackBufferRange")),
         TransformFeedbackStreamAttribsNV: FnPtr::new(loadfn("glTransformFeedbackStreamAttribsNV")),
         TransformFeedbackVaryings: FnPtr::new(loadfn("glTransformFeedbackVaryings")),
         TransformFeedbackVaryingsEXT: FnPtr::new(loadfn("glTransformFeedbackVaryingsEXT")),
         TransformFeedbackVaryingsNV: FnPtr::new(loadfn("glTransformFeedbackVaryingsNV")),
         TransformPathNV: FnPtr::new(loadfn("glTransformPathNV")),
         TranslatexOES: FnPtr::new(loadfn("glTranslatexOES")),
         Uniform1d: FnPtr::new(loadfn("glUniform1d")),
         Uniform1dv: FnPtr::new(loadfn("glUniform1dv")),
         Uniform1f: FnPtr::new(loadfn("glUniform1f")),
         Uniform1fARB: FnPtr::new(loadfn("glUniform1fARB")),
         Uniform1fv: FnPtr::new(loadfn("glUniform1fv")),
         Uniform1fvARB: FnPtr::new(loadfn("glUniform1fvARB")),
         Uniform1i: FnPtr::new(loadfn("glUniform1i")),
         Uniform1i64ARB: FnPtr::new(loadfn("glUniform1i64ARB")),
         Uniform1i64NV: FnPtr::new(loadfn("glUniform1i64NV")),
         Uniform1i64vARB: FnPtr::new(loadfn("glUniform1i64vARB")),
         Uniform1i64vNV: FnPtr::new(loadfn("glUniform1i64vNV")),
         Uniform1iARB: FnPtr::new(loadfn("glUniform1iARB")),
         Uniform1iv: FnPtr::new(loadfn("glUniform1iv")),
         Uniform1ivARB: FnPtr::new(loadfn("glUniform1ivARB")),
         Uniform1ui: FnPtr::new(loadfn("glUniform1ui")),
         Uniform1ui64ARB: FnPtr::new(loadfn("glUniform1ui64ARB")),
         Uniform1ui64NV: FnPtr::new(loadfn("glUniform1ui64NV")),
         Uniform1ui64vARB: FnPtr::new(loadfn("glUniform1ui64vARB")),
         Uniform1ui64vNV: FnPtr::new(loadfn("glUniform1ui64vNV")),
         Uniform1uiEXT: FnPtr::new(loadfn("glUniform1uiEXT")),
         Uniform1uiv: FnPtr::new(loadfn("glUniform1uiv")),
         Uniform1uivEXT: FnPtr::new(loadfn("glUniform1uivEXT")),
         Uniform2d: FnPtr::new(loadfn("glUniform2d")),
         Uniform2dv: FnPtr::new(loadfn("glUniform2dv")),
         Uniform2f: FnPtr::new(loadfn("glUniform2f")),
         Uniform2fARB: FnPtr::new(loadfn("glUniform2fARB")),
         Uniform2fv: FnPtr::new(loadfn("glUniform2fv")),
         Uniform2fvARB: FnPtr::new(loadfn("glUniform2fvARB")),
         Uniform2i: FnPtr::new(loadfn("glUniform2i")),
         Uniform2i64ARB: FnPtr::new(loadfn("glUniform2i64ARB")),
         Uniform2i64NV: FnPtr::new(loadfn("glUniform2i64NV")),
         Uniform2i64vARB: FnPtr::new(loadfn("glUniform2i64vARB")),
         Uniform2i64vNV: FnPtr::new(loadfn("glUniform2i64vNV")),
         Uniform2iARB: FnPtr::new(loadfn("glUniform2iARB")),
         Uniform2iv: FnPtr::new(loadfn("glUniform2iv")),
         Uniform2ivARB: FnPtr::new(loadfn("glUniform2ivARB")),
         Uniform2ui: FnPtr::new(loadfn("glUniform2ui")),
         Uniform2ui64ARB: FnPtr::new(loadfn("glUniform2ui64ARB")),
         Uniform2ui64NV: FnPtr::new(loadfn("glUniform2ui64NV")),
         Uniform2ui64vARB: FnPtr::new(loadfn("glUniform2ui64vARB")),
         Uniform2ui64vNV: FnPtr::new(loadfn("glUniform2ui64vNV")),
         Uniform2uiEXT: FnPtr::new(loadfn("glUniform2uiEXT")),
         Uniform2uiv: FnPtr::new(loadfn("glUniform2uiv")),
         Uniform2uivEXT: FnPtr::new(loadfn("glUniform2uivEXT")),
         Uniform3d: FnPtr::new(loadfn("glUniform3d")),
         Uniform3dv: FnPtr::new(loadfn("glUniform3dv")),
         Uniform3f: FnPtr::new(loadfn("glUniform3f")),
         Uniform3fARB: FnPtr::new(loadfn("glUniform3fARB")),
         Uniform3fv: FnPtr::new(loadfn("glUniform3fv")),
         Uniform3fvARB: FnPtr::new(loadfn("glUniform3fvARB")),
         Uniform3i: FnPtr::new(loadfn("glUniform3i")),
         Uniform3i64ARB: FnPtr::new(loadfn("glUniform3i64ARB")),
         Uniform3i64NV: FnPtr::new(loadfn("glUniform3i64NV")),
         Uniform3i64vARB: FnPtr::new(loadfn("glUniform3i64vARB")),
         Uniform3i64vNV: FnPtr::new(loadfn("glUniform3i64vNV")),
         Uniform3iARB: FnPtr::new(loadfn("glUniform3iARB")),
         Uniform3iv: FnPtr::new(loadfn("glUniform3iv")),
         Uniform3ivARB: FnPtr::new(loadfn("glUniform3ivARB")),
         Uniform3ui: FnPtr::new(loadfn("glUniform3ui")),
         Uniform3ui64ARB: FnPtr::new(loadfn("glUniform3ui64ARB")),
         Uniform3ui64NV: FnPtr::new(loadfn("glUniform3ui64NV")),
         Uniform3ui64vARB: FnPtr::new(loadfn("glUniform3ui64vARB")),
         Uniform3ui64vNV: FnPtr::new(loadfn("glUniform3ui64vNV")),
         Uniform3uiEXT: FnPtr::new(loadfn("glUniform3uiEXT")),
         Uniform3uiv: FnPtr::new(loadfn("glUniform3uiv")),
         Uniform3uivEXT: FnPtr::new(loadfn("glUniform3uivEXT")),
         Uniform4d: FnPtr::new(loadfn("glUniform4d")),
         Uniform4dv: FnPtr::new(loadfn("glUniform4dv")),
         Uniform4f: FnPtr::new(loadfn("glUniform4f")),
         Uniform4fARB: FnPtr::new(loadfn("glUniform4fARB")),
         Uniform4fv: FnPtr::new(loadfn("glUniform4fv")),
         Uniform4fvARB: FnPtr::new(loadfn("glUniform4fvARB")),
         Uniform4i: FnPtr::new(loadfn("glUniform4i")),
         Uniform4i64ARB: FnPtr::new(loadfn("glUniform4i64ARB")),
         Uniform4i64NV: FnPtr::new(loadfn("glUniform4i64NV")),
         Uniform4i64vARB: FnPtr::new(loadfn("glUniform4i64vARB")),
         Uniform4i64vNV: FnPtr::new(loadfn("glUniform4i64vNV")),
         Uniform4iARB: FnPtr::new(loadfn("glUniform4iARB")),
         Uniform4iv: FnPtr::new(loadfn("glUniform4iv")),
         Uniform4ivARB: FnPtr::new(loadfn("glUniform4ivARB")),
         Uniform4ui: FnPtr::new(loadfn("glUniform4ui")),
         Uniform4ui64ARB: FnPtr::new(loadfn("glUniform4ui64ARB")),
         Uniform4ui64NV: FnPtr::new(loadfn("glUniform4ui64NV")),
         Uniform4ui64vARB: FnPtr::new(loadfn("glUniform4ui64vARB")),
         Uniform4ui64vNV: FnPtr::new(loadfn("glUniform4ui64vNV")),
         Uniform4uiEXT: FnPtr::new(loadfn("glUniform4uiEXT")),
         Uniform4uiv: FnPtr::new(loadfn("glUniform4uiv")),
         Uniform4uivEXT: FnPtr::new(loadfn("glUniform4uivEXT")),
         UniformBlockBinding: FnPtr::new(loadfn("glUniformBlockBinding")),
         UniformBufferEXT: FnPtr::new(loadfn("glUniformBufferEXT")),
         UniformHandleui64ARB: FnPtr::new(loadfn("glUniformHandleui64ARB")),
         UniformHandleui64NV: FnPtr::new(loadfn("glUniformHandleui64NV")),
         UniformHandleui64vARB: FnPtr::new(loadfn("glUniformHandleui64vARB")),
         UniformHandleui64vNV: FnPtr::new(loadfn("glUniformHandleui64vNV")),
         UniformMatrix2dv: FnPtr::new(loadfn("glUniformMatrix2dv")),
         UniformMatrix2fv: FnPtr::new(loadfn("glUniformMatrix2fv")),
         UniformMatrix2fvARB: FnPtr::new(loadfn("glUniformMatrix2fvARB")),
         UniformMatrix2x3dv: FnPtr::new(loadfn("glUniformMatrix2x3dv")),
         UniformMatrix2x3fv: FnPtr::new(loadfn("glUniformMatrix2x3fv")),
         UniformMatrix2x4dv: FnPtr::new(loadfn("glUniformMatrix2x4dv")),
         UniformMatrix2x4fv: FnPtr::new(loadfn("glUniformMatrix2x4fv")),
         UniformMatrix3dv: FnPtr::new(loadfn("glUniformMatrix3dv")),
         UniformMatrix3fv: FnPtr::new(loadfn("glUniformMatrix3fv")),
         UniformMatrix3fvARB: FnPtr::new(loadfn("glUniformMatrix3fvARB")),
         UniformMatrix3x2dv: FnPtr::new(loadfn("glUniformMatrix3x2dv")),
         UniformMatrix3x2fv: FnPtr::new(loadfn("glUniformMatrix3x2fv")),
         UniformMatrix3x4dv: FnPtr::new(loadfn("glUniformMatrix3x4dv")),
         UniformMatrix3x4fv: FnPtr::new(loadfn("glUniformMatrix3x4fv")),
         UniformMatrix4dv: FnPtr::new(loadfn("glUniformMatrix4dv")),
         UniformMatrix4fv: FnPtr::new(loadfn("glUniformMatrix4fv")),
         UniformMatrix4fvARB: FnPtr::new(loadfn("glUniformMatrix4fvARB")),
         UniformMatrix4x2dv: FnPtr::new(loadfn("glUniformMatrix4x2dv")),
         UniformMatrix4x2fv: FnPtr::new(loadfn("glUniformMatrix4x2fv")),
         UniformMatrix4x3dv: FnPtr::new(loadfn("glUniformMatrix4x3dv")),
         UniformMatrix4x3fv: FnPtr::new(loadfn("glUniformMatrix4x3fv")),
         UniformSubroutinesuiv: FnPtr::new(loadfn("glUniformSubroutinesuiv")),
         Uniformui64NV: FnPtr::new(loadfn("glUniformui64NV")),
         Uniformui64vNV: FnPtr::new(loadfn("glUniformui64vNV")),
         UnlockArraysEXT: FnPtr::new(loadfn("glUnlockArraysEXT")),
         UnmapBuffer: FnPtr::new(loadfn("glUnmapBuffer")),
         UnmapBufferARB: FnPtr::new(loadfn("glUnmapBufferARB")),
         UnmapNamedBuffer: FnPtr::new(loadfn("glUnmapNamedBuffer")),
         UnmapNamedBufferEXT: FnPtr::new(loadfn("glUnmapNamedBufferEXT")),
         UnmapObjectBufferATI: FnPtr::new(loadfn("glUnmapObjectBufferATI")),
         UnmapTexture2DINTEL: FnPtr::new(loadfn("glUnmapTexture2DINTEL")),
         UpdateObjectBufferATI: FnPtr::new(loadfn("glUpdateObjectBufferATI")),
         UploadGpuMaskNVX: FnPtr::new(loadfn("glUploadGpuMaskNVX")),
         UseProgram: FnPtr::new(loadfn("glUseProgram")),
         UseProgramObjectARB: FnPtr::new(loadfn("glUseProgramObjectARB")),
         UseProgramStages: FnPtr::new(loadfn("glUseProgramStages")),
         UseShaderProgramEXT: FnPtr::new(loadfn("glUseShaderProgramEXT")),
         VDPAUFiniNV: FnPtr::new(loadfn("glVDPAUFiniNV")),
         VDPAUGetSurfaceivNV: FnPtr::new(loadfn("glVDPAUGetSurfaceivNV")),
         VDPAUInitNV: FnPtr::new(loadfn("glVDPAUInitNV")),
         VDPAUIsSurfaceNV: FnPtr::new(loadfn("glVDPAUIsSurfaceNV")),
         VDPAUMapSurfacesNV: FnPtr::new(loadfn("glVDPAUMapSurfacesNV")),
         VDPAURegisterOutputSurfaceNV: FnPtr::new(loadfn("glVDPAURegisterOutputSurfaceNV")),
         VDPAURegisterVideoSurfaceNV: FnPtr::new(loadfn("glVDPAURegisterVideoSurfaceNV")),
         VDPAURegisterVideoSurfaceWithPictureStructureNV: FnPtr::new(loadfn("glVDPAURegisterVideoSurfaceWithPictureStructureNV")),
         VDPAUSurfaceAccessNV: FnPtr::new(loadfn("glVDPAUSurfaceAccessNV")),
         VDPAUUnmapSurfacesNV: FnPtr::new(loadfn("glVDPAUUnmapSurfacesNV")),
         VDPAUUnregisterSurfaceNV: FnPtr::new(loadfn("glVDPAUUnregisterSurfaceNV")),
         ValidateProgram: FnPtr::new(loadfn("glValidateProgram")),
         ValidateProgramARB: FnPtr::new(loadfn("glValidateProgramARB")),
         ValidateProgramPipeline: FnPtr::new(loadfn("glValidateProgramPipeline")),
         VariantArrayObjectATI: FnPtr::new(loadfn("glVariantArrayObjectATI")),
         VariantPointerEXT: FnPtr::new(loadfn("glVariantPointerEXT")),
         VariantbvEXT: FnPtr::new(loadfn("glVariantbvEXT")),
         VariantdvEXT: FnPtr::new(loadfn("glVariantdvEXT")),
         VariantfvEXT: FnPtr::new(loadfn("glVariantfvEXT")),
         VariantivEXT: FnPtr::new(loadfn("glVariantivEXT")),
         VariantsvEXT: FnPtr::new(loadfn("glVariantsvEXT")),
         VariantubvEXT: FnPtr::new(loadfn("glVariantubvEXT")),
         VariantuivEXT: FnPtr::new(loadfn("glVariantuivEXT")),
         VariantusvEXT: FnPtr::new(loadfn("glVariantusvEXT")),
         Vertex2bOES: FnPtr::new(loadfn("glVertex2bOES")),
         Vertex2bvOES: FnPtr::new(loadfn("glVertex2bvOES")),
         Vertex2hNV: FnPtr::new(loadfn("glVertex2hNV")),
         Vertex2hvNV: FnPtr::new(loadfn("glVertex2hvNV")),
         Vertex2xOES: FnPtr::new(loadfn("glVertex2xOES")),
         Vertex2xvOES: FnPtr::new(loadfn("glVertex2xvOES")),
         Vertex3bOES: FnPtr::new(loadfn("glVertex3bOES")),
         Vertex3bvOES: FnPtr::new(loadfn("glVertex3bvOES")),
         Vertex3hNV: FnPtr::new(loadfn("glVertex3hNV")),
         Vertex3hvNV: FnPtr::new(loadfn("glVertex3hvNV")),
         Vertex3xOES: FnPtr::new(loadfn("glVertex3xOES")),
         Vertex3xvOES: FnPtr::new(loadfn("glVertex3xvOES")),
         Vertex4bOES: FnPtr::new(loadfn("glVertex4bOES")),
         Vertex4bvOES: FnPtr::new(loadfn("glVertex4bvOES")),
         Vertex4hNV: FnPtr::new(loadfn("glVertex4hNV")),
         Vertex4hvNV: FnPtr::new(loadfn("glVertex4hvNV")),
         Vertex4xOES: FnPtr::new(loadfn("glVertex4xOES")),
         Vertex4xvOES: FnPtr::new(loadfn("glVertex4xvOES")),
         VertexArrayAttribBinding: FnPtr::new(loadfn("glVertexArrayAttribBinding")),
         VertexArrayAttribFormat: FnPtr::new(loadfn("glVertexArrayAttribFormat")),
         VertexArrayAttribIFormat: FnPtr::new(loadfn("glVertexArrayAttribIFormat")),
         VertexArrayAttribLFormat: FnPtr::new(loadfn("glVertexArrayAttribLFormat")),
         VertexArrayBindVertexBufferEXT: FnPtr::new(loadfn("glVertexArrayBindVertexBufferEXT")),
         VertexArrayBindingDivisor: FnPtr::new(loadfn("glVertexArrayBindingDivisor")),
         VertexArrayColorOffsetEXT: FnPtr::new(loadfn("glVertexArrayColorOffsetEXT")),
         VertexArrayEdgeFlagOffsetEXT: FnPtr::new(loadfn("glVertexArrayEdgeFlagOffsetEXT")),
         VertexArrayElementBuffer: FnPtr::new(loadfn("glVertexArrayElementBuffer")),
         VertexArrayFogCoordOffsetEXT: FnPtr::new(loadfn("glVertexArrayFogCoordOffsetEXT")),
         VertexArrayIndexOffsetEXT: FnPtr::new(loadfn("glVertexArrayIndexOffsetEXT")),
         VertexArrayMultiTexCoordOffsetEXT: FnPtr::new(loadfn("glVertexArrayMultiTexCoordOffsetEXT")),
         VertexArrayNormalOffsetEXT: FnPtr::new(loadfn("glVertexArrayNormalOffsetEXT")),
         VertexArrayParameteriAPPLE: FnPtr::new(loadfn("glVertexArrayParameteriAPPLE")),
         VertexArrayRangeAPPLE: FnPtr::new(loadfn("glVertexArrayRangeAPPLE")),
         VertexArrayRangeNV: FnPtr::new(loadfn("glVertexArrayRangeNV")),
         VertexArraySecondaryColorOffsetEXT: FnPtr::new(loadfn("glVertexArraySecondaryColorOffsetEXT")),
         VertexArrayTexCoordOffsetEXT: FnPtr::new(loadfn("glVertexArrayTexCoordOffsetEXT")),
         VertexArrayVertexAttribBindingEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribBindingEXT")),
         VertexArrayVertexAttribDivisorEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribDivisorEXT")),
         VertexArrayVertexAttribFormatEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribFormatEXT")),
         VertexArrayVertexAttribIFormatEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribIFormatEXT")),
         VertexArrayVertexAttribIOffsetEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribIOffsetEXT")),
         VertexArrayVertexAttribLFormatEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribLFormatEXT")),
         VertexArrayVertexAttribLOffsetEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribLOffsetEXT")),
         VertexArrayVertexAttribOffsetEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribOffsetEXT")),
         VertexArrayVertexBindingDivisorEXT: FnPtr::new(loadfn("glVertexArrayVertexBindingDivisorEXT")),
         VertexArrayVertexBuffer: FnPtr::new(loadfn("glVertexArrayVertexBuffer")),
         VertexArrayVertexBuffers: FnPtr::new(loadfn("glVertexArrayVertexBuffers")),
         VertexArrayVertexOffsetEXT: FnPtr::new(loadfn("glVertexArrayVertexOffsetEXT")),
         VertexAttrib1d: FnPtr::new(loadfn("glVertexAttrib1d")),
         VertexAttrib1dARB: FnPtr::new(loadfn("glVertexAttrib1dARB")),
         VertexAttrib1dNV: FnPtr::new(loadfn("glVertexAttrib1dNV")),
         VertexAttrib1dv: FnPtr::new(loadfn("glVertexAttrib1dv")),
         VertexAttrib1dvARB: FnPtr::new(loadfn("glVertexAttrib1dvARB")),
         VertexAttrib1dvNV: FnPtr::new(loadfn("glVertexAttrib1dvNV")),
         VertexAttrib1f: FnPtr::new(loadfn("glVertexAttrib1f")),
         VertexAttrib1fARB: FnPtr::new(loadfn("glVertexAttrib1fARB")),
         VertexAttrib1fNV: FnPtr::new(loadfn("glVertexAttrib1fNV")),
         VertexAttrib1fv: FnPtr::new(loadfn("glVertexAttrib1fv")),
         VertexAttrib1fvARB: FnPtr::new(loadfn("glVertexAttrib1fvARB")),
         VertexAttrib1fvNV: FnPtr::new(loadfn("glVertexAttrib1fvNV")),
         VertexAttrib1hNV: FnPtr::new(loadfn("glVertexAttrib1hNV")),
         VertexAttrib1hvNV: FnPtr::new(loadfn("glVertexAttrib1hvNV")),
         VertexAttrib1s: FnPtr::new(loadfn("glVertexAttrib1s")),
         VertexAttrib1sARB: FnPtr::new(loadfn("glVertexAttrib1sARB")),
         VertexAttrib1sNV: FnPtr::new(loadfn("glVertexAttrib1sNV")),
         VertexAttrib1sv: FnPtr::new(loadfn("glVertexAttrib1sv")),
         VertexAttrib1svARB: FnPtr::new(loadfn("glVertexAttrib1svARB")),
         VertexAttrib1svNV: FnPtr::new(loadfn("glVertexAttrib1svNV")),
         VertexAttrib2d: FnPtr::new(loadfn("glVertexAttrib2d")),
         VertexAttrib2dARB: FnPtr::new(loadfn("glVertexAttrib2dARB")),
         VertexAttrib2dNV: FnPtr::new(loadfn("glVertexAttrib2dNV")),
         VertexAttrib2dv: FnPtr::new(loadfn("glVertexAttrib2dv")),
         VertexAttrib2dvARB: FnPtr::new(loadfn("glVertexAttrib2dvARB")),
         VertexAttrib2dvNV: FnPtr::new(loadfn("glVertexAttrib2dvNV")),
         VertexAttrib2f: FnPtr::new(loadfn("glVertexAttrib2f")),
         VertexAttrib2fARB: FnPtr::new(loadfn("glVertexAttrib2fARB")),
         VertexAttrib2fNV: FnPtr::new(loadfn("glVertexAttrib2fNV")),
         VertexAttrib2fv: FnPtr::new(loadfn("glVertexAttrib2fv")),
         VertexAttrib2fvARB: FnPtr::new(loadfn("glVertexAttrib2fvARB")),
         VertexAttrib2fvNV: FnPtr::new(loadfn("glVertexAttrib2fvNV")),
         VertexAttrib2hNV: FnPtr::new(loadfn("glVertexAttrib2hNV")),
         VertexAttrib2hvNV: FnPtr::new(loadfn("glVertexAttrib2hvNV")),
         VertexAttrib2s: FnPtr::new(loadfn("glVertexAttrib2s")),
         VertexAttrib2sARB: FnPtr::new(loadfn("glVertexAttrib2sARB")),
         VertexAttrib2sNV: FnPtr::new(loadfn("glVertexAttrib2sNV")),
         VertexAttrib2sv: FnPtr::new(loadfn("glVertexAttrib2sv")),
         VertexAttrib2svARB: FnPtr::new(loadfn("glVertexAttrib2svARB")),
         VertexAttrib2svNV: FnPtr::new(loadfn("glVertexAttrib2svNV")),
         VertexAttrib3d: FnPtr::new(loadfn("glVertexAttrib3d")),
         VertexAttrib3dARB: FnPtr::new(loadfn("glVertexAttrib3dARB")),
         VertexAttrib3dNV: FnPtr::new(loadfn("glVertexAttrib3dNV")),
         VertexAttrib3dv: FnPtr::new(loadfn("glVertexAttrib3dv")),
         VertexAttrib3dvARB: FnPtr::new(loadfn("glVertexAttrib3dvARB")),
         VertexAttrib3dvNV: FnPtr::new(loadfn("glVertexAttrib3dvNV")),
         VertexAttrib3f: FnPtr::new(loadfn("glVertexAttrib3f")),
         VertexAttrib3fARB: FnPtr::new(loadfn("glVertexAttrib3fARB")),
         VertexAttrib3fNV: FnPtr::new(loadfn("glVertexAttrib3fNV")),
         VertexAttrib3fv: FnPtr::new(loadfn("glVertexAttrib3fv")),
         VertexAttrib3fvARB: FnPtr::new(loadfn("glVertexAttrib3fvARB")),
         VertexAttrib3fvNV: FnPtr::new(loadfn("glVertexAttrib3fvNV")),
         VertexAttrib3hNV: FnPtr::new(loadfn("glVertexAttrib3hNV")),
         VertexAttrib3hvNV: FnPtr::new(loadfn("glVertexAttrib3hvNV")),
         VertexAttrib3s: FnPtr::new(loadfn("glVertexAttrib3s")),
         VertexAttrib3sARB: FnPtr::new(loadfn("glVertexAttrib3sARB")),
         VertexAttrib3sNV: FnPtr::new(loadfn("glVertexAttrib3sNV")),
         VertexAttrib3sv: FnPtr::new(loadfn("glVertexAttrib3sv")),
         VertexAttrib3svARB: FnPtr::new(loadfn("glVertexAttrib3svARB")),
         VertexAttrib3svNV: FnPtr::new(loadfn("glVertexAttrib3svNV")),
         VertexAttrib4Nbv: FnPtr::new(loadfn("glVertexAttrib4Nbv")),
         VertexAttrib4NbvARB: FnPtr::new(loadfn("glVertexAttrib4NbvARB")),
         VertexAttrib4Niv: FnPtr::new(loadfn("glVertexAttrib4Niv")),
         VertexAttrib4NivARB: FnPtr::new(loadfn("glVertexAttrib4NivARB")),
         VertexAttrib4Nsv: FnPtr::new(loadfn("glVertexAttrib4Nsv")),
         VertexAttrib4NsvARB: FnPtr::new(loadfn("glVertexAttrib4NsvARB")),
         VertexAttrib4Nub: FnPtr::new(loadfn("glVertexAttrib4Nub")),
         VertexAttrib4NubARB: FnPtr::new(loadfn("glVertexAttrib4NubARB")),
         VertexAttrib4Nubv: FnPtr::new(loadfn("glVertexAttrib4Nubv")),
         VertexAttrib4NubvARB: FnPtr::new(loadfn("glVertexAttrib4NubvARB")),
         VertexAttrib4Nuiv: FnPtr::new(loadfn("glVertexAttrib4Nuiv")),
         VertexAttrib4NuivARB: FnPtr::new(loadfn("glVertexAttrib4NuivARB")),
         VertexAttrib4Nusv: FnPtr::new(loadfn("glVertexAttrib4Nusv")),
         VertexAttrib4NusvARB: FnPtr::new(loadfn("glVertexAttrib4NusvARB")),
         VertexAttrib4bv: FnPtr::new(loadfn("glVertexAttrib4bv")),
         VertexAttrib4bvARB: FnPtr::new(loadfn("glVertexAttrib4bvARB")),
         VertexAttrib4d: FnPtr::new(loadfn("glVertexAttrib4d")),
         VertexAttrib4dARB: FnPtr::new(loadfn("glVertexAttrib4dARB")),
         VertexAttrib4dNV: FnPtr::new(loadfn("glVertexAttrib4dNV")),
         VertexAttrib4dv: FnPtr::new(loadfn("glVertexAttrib4dv")),
         VertexAttrib4dvARB: FnPtr::new(loadfn("glVertexAttrib4dvARB")),
         VertexAttrib4dvNV: FnPtr::new(loadfn("glVertexAttrib4dvNV")),
         VertexAttrib4f: FnPtr::new(loadfn("glVertexAttrib4f")),
         VertexAttrib4fARB: FnPtr::new(loadfn("glVertexAttrib4fARB")),
         VertexAttrib4fNV: FnPtr::new(loadfn("glVertexAttrib4fNV")),
         VertexAttrib4fv: FnPtr::new(loadfn("glVertexAttrib4fv")),
         VertexAttrib4fvARB: FnPtr::new(loadfn("glVertexAttrib4fvARB")),
         VertexAttrib4fvNV: FnPtr::new(loadfn("glVertexAttrib4fvNV")),
         VertexAttrib4hNV: FnPtr::new(loadfn("glVertexAttrib4hNV")),
         VertexAttrib4hvNV: FnPtr::new(loadfn("glVertexAttrib4hvNV")),
         VertexAttrib4iv: FnPtr::new(loadfn("glVertexAttrib4iv")),
         VertexAttrib4ivARB: FnPtr::new(loadfn("glVertexAttrib4ivARB")),
         VertexAttrib4s: FnPtr::new(loadfn("glVertexAttrib4s")),
         VertexAttrib4sARB: FnPtr::new(loadfn("glVertexAttrib4sARB")),
         VertexAttrib4sNV: FnPtr::new(loadfn("glVertexAttrib4sNV")),
         VertexAttrib4sv: FnPtr::new(loadfn("glVertexAttrib4sv")),
         VertexAttrib4svARB: FnPtr::new(loadfn("glVertexAttrib4svARB")),
         VertexAttrib4svNV: FnPtr::new(loadfn("glVertexAttrib4svNV")),
         VertexAttrib4ubNV: FnPtr::new(loadfn("glVertexAttrib4ubNV")),
         VertexAttrib4ubv: FnPtr::new(loadfn("glVertexAttrib4ubv")),
         VertexAttrib4ubvARB: FnPtr::new(loadfn("glVertexAttrib4ubvARB")),
         VertexAttrib4ubvNV: FnPtr::new(loadfn("glVertexAttrib4ubvNV")),
         VertexAttrib4uiv: FnPtr::new(loadfn("glVertexAttrib4uiv")),
         VertexAttrib4uivARB: FnPtr::new(loadfn("glVertexAttrib4uivARB")),
         VertexAttrib4usv: FnPtr::new(loadfn("glVertexAttrib4usv")),
         VertexAttrib4usvARB: FnPtr::new(loadfn("glVertexAttrib4usvARB")),
         VertexAttribArrayObjectATI: FnPtr::new(loadfn("glVertexAttribArrayObjectATI")),
         VertexAttribBinding: FnPtr::new(loadfn("glVertexAttribBinding")),
         VertexAttribDivisor: FnPtr::new(loadfn("glVertexAttribDivisor")),
         VertexAttribDivisorARB: FnPtr::new(loadfn("glVertexAttribDivisorARB")),
         VertexAttribFormat: FnPtr::new(loadfn("glVertexAttribFormat")),
         VertexAttribFormatNV: FnPtr::new(loadfn("glVertexAttribFormatNV")),
         VertexAttribI1i: FnPtr::new(loadfn("glVertexAttribI1i")),
         VertexAttribI1iEXT: FnPtr::new(loadfn("glVertexAttribI1iEXT")),
         VertexAttribI1iv: FnPtr::new(loadfn("glVertexAttribI1iv")),
         VertexAttribI1ivEXT: FnPtr::new(loadfn("glVertexAttribI1ivEXT")),
         VertexAttribI1ui: FnPtr::new(loadfn("glVertexAttribI1ui")),
         VertexAttribI1uiEXT: FnPtr::new(loadfn("glVertexAttribI1uiEXT")),
         VertexAttribI1uiv: FnPtr::new(loadfn("glVertexAttribI1uiv")),
         VertexAttribI1uivEXT: FnPtr::new(loadfn("glVertexAttribI1uivEXT")),
         VertexAttribI2i: FnPtr::new(loadfn("glVertexAttribI2i")),
         VertexAttribI2iEXT: FnPtr::new(loadfn("glVertexAttribI2iEXT")),
         VertexAttribI2iv: FnPtr::new(loadfn("glVertexAttribI2iv")),
         VertexAttribI2ivEXT: FnPtr::new(loadfn("glVertexAttribI2ivEXT")),
         VertexAttribI2ui: FnPtr::new(loadfn("glVertexAttribI2ui")),
         VertexAttribI2uiEXT: FnPtr::new(loadfn("glVertexAttribI2uiEXT")),
         VertexAttribI2uiv: FnPtr::new(loadfn("glVertexAttribI2uiv")),
         VertexAttribI2uivEXT: FnPtr::new(loadfn("glVertexAttribI2uivEXT")),
         VertexAttribI3i: FnPtr::new(loadfn("glVertexAttribI3i")),
         VertexAttribI3iEXT: FnPtr::new(loadfn("glVertexAttribI3iEXT")),
         VertexAttribI3iv: FnPtr::new(loadfn("glVertexAttribI3iv")),
         VertexAttribI3ivEXT: FnPtr::new(loadfn("glVertexAttribI3ivEXT")),
         VertexAttribI3ui: FnPtr::new(loadfn("glVertexAttribI3ui")),
         VertexAttribI3uiEXT: FnPtr::new(loadfn("glVertexAttribI3uiEXT")),
         VertexAttribI3uiv: FnPtr::new(loadfn("glVertexAttribI3uiv")),
         VertexAttribI3uivEXT: FnPtr::new(loadfn("glVertexAttribI3uivEXT")),
         VertexAttribI4bv: FnPtr::new(loadfn("glVertexAttribI4bv")),
         VertexAttribI4bvEXT: FnPtr::new(loadfn("glVertexAttribI4bvEXT")),
         VertexAttribI4i: FnPtr::new(loadfn("glVertexAttribI4i")),
         VertexAttribI4iEXT: FnPtr::new(loadfn("glVertexAttribI4iEXT")),
         VertexAttribI4iv: FnPtr::new(loadfn("glVertexAttribI4iv")),
         VertexAttribI4ivEXT: FnPtr::new(loadfn("glVertexAttribI4ivEXT")),
         VertexAttribI4sv: FnPtr::new(loadfn("glVertexAttribI4sv")),
         VertexAttribI4svEXT: FnPtr::new(loadfn("glVertexAttribI4svEXT")),
         VertexAttribI4ubv: FnPtr::new(loadfn("glVertexAttribI4ubv")),
         VertexAttribI4ubvEXT: FnPtr::new(loadfn("glVertexAttribI4ubvEXT")),
         VertexAttribI4ui: FnPtr::new(loadfn("glVertexAttribI4ui")),
         VertexAttribI4uiEXT: FnPtr::new(loadfn("glVertexAttribI4uiEXT")),
         VertexAttribI4uiv: FnPtr::new(loadfn("glVertexAttribI4uiv")),
         VertexAttribI4uivEXT: FnPtr::new(loadfn("glVertexAttribI4uivEXT")),
         VertexAttribI4usv: FnPtr::new(loadfn("glVertexAttribI4usv")),
         VertexAttribI4usvEXT: FnPtr::new(loadfn("glVertexAttribI4usvEXT")),
         VertexAttribIFormat: FnPtr::new(loadfn("glVertexAttribIFormat")),
         VertexAttribIFormatNV: FnPtr::new(loadfn("glVertexAttribIFormatNV")),
         VertexAttribIPointer: FnPtr::new(loadfn("glVertexAttribIPointer")),
         VertexAttribIPointerEXT: FnPtr::new(loadfn("glVertexAttribIPointerEXT")),
         VertexAttribL1d: FnPtr::new(loadfn("glVertexAttribL1d")),
         VertexAttribL1dEXT: FnPtr::new(loadfn("glVertexAttribL1dEXT")),
         VertexAttribL1dv: FnPtr::new(loadfn("glVertexAttribL1dv")),
         VertexAttribL1dvEXT: FnPtr::new(loadfn("glVertexAttribL1dvEXT")),
         VertexAttribL1i64NV: FnPtr::new(loadfn("glVertexAttribL1i64NV")),
         VertexAttribL1i64vNV: FnPtr::new(loadfn("glVertexAttribL1i64vNV")),
         VertexAttribL1ui64ARB: FnPtr::new(loadfn("glVertexAttribL1ui64ARB")),
         VertexAttribL1ui64NV: FnPtr::new(loadfn("glVertexAttribL1ui64NV")),
         VertexAttribL1ui64vARB: FnPtr::new(loadfn("glVertexAttribL1ui64vARB")),
         VertexAttribL1ui64vNV: FnPtr::new(loadfn("glVertexAttribL1ui64vNV")),
         VertexAttribL2d: FnPtr::new(loadfn("glVertexAttribL2d")),
         VertexAttribL2dEXT: FnPtr::new(loadfn("glVertexAttribL2dEXT")),
         VertexAttribL2dv: FnPtr::new(loadfn("glVertexAttribL2dv")),
         VertexAttribL2dvEXT: FnPtr::new(loadfn("glVertexAttribL2dvEXT")),
         VertexAttribL2i64NV: FnPtr::new(loadfn("glVertexAttribL2i64NV")),
         VertexAttribL2i64vNV: FnPtr::new(loadfn("glVertexAttribL2i64vNV")),
         VertexAttribL2ui64NV: FnPtr::new(loadfn("glVertexAttribL2ui64NV")),
         VertexAttribL2ui64vNV: FnPtr::new(loadfn("glVertexAttribL2ui64vNV")),
         VertexAttribL3d: FnPtr::new(loadfn("glVertexAttribL3d")),
         VertexAttribL3dEXT: FnPtr::new(loadfn("glVertexAttribL3dEXT")),
         VertexAttribL3dv: FnPtr::new(loadfn("glVertexAttribL3dv")),
         VertexAttribL3dvEXT: FnPtr::new(loadfn("glVertexAttribL3dvEXT")),
         VertexAttribL3i64NV: FnPtr::new(loadfn("glVertexAttribL3i64NV")),
         VertexAttribL3i64vNV: FnPtr::new(loadfn("glVertexAttribL3i64vNV")),
         VertexAttribL3ui64NV: FnPtr::new(loadfn("glVertexAttribL3ui64NV")),
         VertexAttribL3ui64vNV: FnPtr::new(loadfn("glVertexAttribL3ui64vNV")),
         VertexAttribL4d: FnPtr::new(loadfn("glVertexAttribL4d")),
         VertexAttribL4dEXT: FnPtr::new(loadfn("glVertexAttribL4dEXT")),
         VertexAttribL4dv: FnPtr::new(loadfn("glVertexAttribL4dv")),
         VertexAttribL4dvEXT: FnPtr::new(loadfn("glVertexAttribL4dvEXT")),
         VertexAttribL4i64NV: FnPtr::new(loadfn("glVertexAttribL4i64NV")),
         VertexAttribL4i64vNV: FnPtr::new(loadfn("glVertexAttribL4i64vNV")),
         VertexAttribL4ui64NV: FnPtr::new(loadfn("glVertexAttribL4ui64NV")),
         VertexAttribL4ui64vNV: FnPtr::new(loadfn("glVertexAttribL4ui64vNV")),
         VertexAttribLFormat: FnPtr::new(loadfn("glVertexAttribLFormat")),
         VertexAttribLFormatNV: FnPtr::new(loadfn("glVertexAttribLFormatNV")),
         VertexAttribLPointer: FnPtr::new(loadfn("glVertexAttribLPointer")),
         VertexAttribLPointerEXT: FnPtr::new(loadfn("glVertexAttribLPointerEXT")),
         VertexAttribP1ui: FnPtr::new(loadfn("glVertexAttribP1ui")),
         VertexAttribP1uiv: FnPtr::new(loadfn("glVertexAttribP1uiv")),
         VertexAttribP2ui: FnPtr::new(loadfn("glVertexAttribP2ui")),
         VertexAttribP2uiv: FnPtr::new(loadfn("glVertexAttribP2uiv")),
         VertexAttribP3ui: FnPtr::new(loadfn("glVertexAttribP3ui")),
         VertexAttribP3uiv: FnPtr::new(loadfn("glVertexAttribP3uiv")),
         VertexAttribP4ui: FnPtr::new(loadfn("glVertexAttribP4ui")),
         VertexAttribP4uiv: FnPtr::new(loadfn("glVertexAttribP4uiv")),
         VertexAttribParameteriAMD: FnPtr::new(loadfn("glVertexAttribParameteriAMD")),
         VertexAttribPointer: FnPtr::new(loadfn("glVertexAttribPointer")),
         VertexAttribPointerARB: FnPtr::new(loadfn("glVertexAttribPointerARB")),
         VertexAttribPointerNV: FnPtr::new(loadfn("glVertexAttribPointerNV")),
         VertexAttribs1dvNV: FnPtr::new(loadfn("glVertexAttribs1dvNV")),
         VertexAttribs1fvNV: FnPtr::new(loadfn("glVertexAttribs1fvNV")),
         VertexAttribs1hvNV: FnPtr::new(loadfn("glVertexAttribs1hvNV")),
         VertexAttribs1svNV: FnPtr::new(loadfn("glVertexAttribs1svNV")),
         VertexAttribs2dvNV: FnPtr::new(loadfn("glVertexAttribs2dvNV")),
         VertexAttribs2fvNV: FnPtr::new(loadfn("glVertexAttribs2fvNV")),
         VertexAttribs2hvNV: FnPtr::new(loadfn("glVertexAttribs2hvNV")),
         VertexAttribs2svNV: FnPtr::new(loadfn("glVertexAttribs2svNV")),
         VertexAttribs3dvNV: FnPtr::new(loadfn("glVertexAttribs3dvNV")),
         VertexAttribs3fvNV: FnPtr::new(loadfn("glVertexAttribs3fvNV")),
         VertexAttribs3hvNV: FnPtr::new(loadfn("glVertexAttribs3hvNV")),
         VertexAttribs3svNV: FnPtr::new(loadfn("glVertexAttribs3svNV")),
         VertexAttribs4dvNV: FnPtr::new(loadfn("glVertexAttribs4dvNV")),
         VertexAttribs4fvNV: FnPtr::new(loadfn("glVertexAttribs4fvNV")),
         VertexAttribs4hvNV: FnPtr::new(loadfn("glVertexAttribs4hvNV")),
         VertexAttribs4svNV: FnPtr::new(loadfn("glVertexAttribs4svNV")),
         VertexAttribs4ubvNV: FnPtr::new(loadfn("glVertexAttribs4ubvNV")),
         VertexBindingDivisor: FnPtr::new(loadfn("glVertexBindingDivisor")),
         VertexBlendARB: FnPtr::new(loadfn("glVertexBlendARB")),
         VertexBlendEnvfATI: FnPtr::new(loadfn("glVertexBlendEnvfATI")),
         VertexBlendEnviATI: FnPtr::new(loadfn("glVertexBlendEnviATI")),
         VertexFormatNV: FnPtr::new(loadfn("glVertexFormatNV")),
         VertexPointerEXT: FnPtr::new(loadfn("glVertexPointerEXT")),
         VertexPointerListIBM: FnPtr::new(loadfn("glVertexPointerListIBM")),
         VertexPointervINTEL: FnPtr::new(loadfn("glVertexPointervINTEL")),
         VertexStream1dATI: FnPtr::new(loadfn("glVertexStream1dATI")),
         VertexStream1dvATI: FnPtr::new(loadfn("glVertexStream1dvATI")),
         VertexStream1fATI: FnPtr::new(loadfn("glVertexStream1fATI")),
         VertexStream1fvATI: FnPtr::new(loadfn("glVertexStream1fvATI")),
         VertexStream1iATI: FnPtr::new(loadfn("glVertexStream1iATI")),
         VertexStream1ivATI: FnPtr::new(loadfn("glVertexStream1ivATI")),
         VertexStream1sATI: FnPtr::new(loadfn("glVertexStream1sATI")),
         VertexStream1svATI: FnPtr::new(loadfn("glVertexStream1svATI")),
         VertexStream2dATI: FnPtr::new(loadfn("glVertexStream2dATI")),
         VertexStream2dvATI: FnPtr::new(loadfn("glVertexStream2dvATI")),
         VertexStream2fATI: FnPtr::new(loadfn("glVertexStream2fATI")),
         VertexStream2fvATI: FnPtr::new(loadfn("glVertexStream2fvATI")),
         VertexStream2iATI: FnPtr::new(loadfn("glVertexStream2iATI")),
         VertexStream2ivATI: FnPtr::new(loadfn("glVertexStream2ivATI")),
         VertexStream2sATI: FnPtr::new(loadfn("glVertexStream2sATI")),
         VertexStream2svATI: FnPtr::new(loadfn("glVertexStream2svATI")),
         VertexStream3dATI: FnPtr::new(loadfn("glVertexStream3dATI")),
         VertexStream3dvATI: FnPtr::new(loadfn("glVertexStream3dvATI")),
         VertexStream3fATI: FnPtr::new(loadfn("glVertexStream3fATI")),
         VertexStream3fvATI: FnPtr::new(loadfn("glVertexStream3fvATI")),
         VertexStream3iATI: FnPtr::new(loadfn("glVertexStream3iATI")),
         VertexStream3ivATI: FnPtr::new(loadfn("glVertexStream3ivATI")),
         VertexStream3sATI: FnPtr::new(loadfn("glVertexStream3sATI")),
         VertexStream3svATI: FnPtr::new(loadfn("glVertexStream3svATI")),
         VertexStream4dATI: FnPtr::new(loadfn("glVertexStream4dATI")),
         VertexStream4dvATI: FnPtr::new(loadfn("glVertexStream4dvATI")),
         VertexStream4fATI: FnPtr::new(loadfn("glVertexStream4fATI")),
         VertexStream4fvATI: FnPtr::new(loadfn("glVertexStream4fvATI")),
         VertexStream4iATI: FnPtr::new(loadfn("glVertexStream4iATI")),
         VertexStream4ivATI: FnPtr::new(loadfn("glVertexStream4ivATI")),
         VertexStream4sATI: FnPtr::new(loadfn("glVertexStream4sATI")),
         VertexStream4svATI: FnPtr::new(loadfn("glVertexStream4svATI")),
         VertexWeightPointerEXT: FnPtr::new(loadfn("glVertexWeightPointerEXT")),
         VertexWeightfEXT: FnPtr::new(loadfn("glVertexWeightfEXT")),
         VertexWeightfvEXT: FnPtr::new(loadfn("glVertexWeightfvEXT")),
         VertexWeighthNV: FnPtr::new(loadfn("glVertexWeighthNV")),
         VertexWeighthvNV: FnPtr::new(loadfn("glVertexWeighthvNV")),
         VideoCaptureNV: FnPtr::new(loadfn("glVideoCaptureNV")),
         VideoCaptureStreamParameterdvNV: FnPtr::new(loadfn("glVideoCaptureStreamParameterdvNV")),
         VideoCaptureStreamParameterfvNV: FnPtr::new(loadfn("glVideoCaptureStreamParameterfvNV")),
         VideoCaptureStreamParameterivNV: FnPtr::new(loadfn("glVideoCaptureStreamParameterivNV")),
         Viewport: FnPtr::new(loadfn("glViewport")),
         ViewportArrayv: FnPtr::new(loadfn("glViewportArrayv")),
         ViewportIndexedf: FnPtr::new(loadfn("glViewportIndexedf")),
         ViewportIndexedfv: FnPtr::new(loadfn("glViewportIndexedfv")),
         ViewportPositionWScaleNV: FnPtr::new(loadfn("glViewportPositionWScaleNV")),
         ViewportSwizzleNV: FnPtr::new(loadfn("glViewportSwizzleNV")),
         WaitSemaphoreEXT: FnPtr::new(loadfn("glWaitSemaphoreEXT")),
         WaitSemaphoreui64NVX: FnPtr::new(loadfn("glWaitSemaphoreui64NVX")),
         WaitSync: FnPtr::new(loadfn("glWaitSync")),
         WaitVkSemaphoreNV: FnPtr::new(loadfn("glWaitVkSemaphoreNV")),
         WeightPathsNV: FnPtr::new(loadfn("glWeightPathsNV")),
         WeightPointerARB: FnPtr::new(loadfn("glWeightPointerARB")),
         WeightbvARB: FnPtr::new(loadfn("glWeightbvARB")),
         WeightdvARB: FnPtr::new(loadfn("glWeightdvARB")),
         WeightfvARB: FnPtr::new(loadfn("glWeightfvARB")),
         WeightivARB: FnPtr::new(loadfn("glWeightivARB")),
         WeightsvARB: FnPtr::new(loadfn("glWeightsvARB")),
         WeightubvARB: FnPtr::new(loadfn("glWeightubvARB")),
         WeightuivARB: FnPtr::new(loadfn("glWeightuivARB")),
         WeightusvARB: FnPtr::new(loadfn("glWeightusvARB")),
         WindowPos2dARB: FnPtr::new(loadfn("glWindowPos2dARB")),
         WindowPos2dMESA: FnPtr::new(loadfn("glWindowPos2dMESA")),
         WindowPos2dvARB: FnPtr::new(loadfn("glWindowPos2dvARB")),
         WindowPos2dvMESA: FnPtr::new(loadfn("glWindowPos2dvMESA")),
         WindowPos2fARB: FnPtr::new(loadfn("glWindowPos2fARB")),
         WindowPos2fMESA: FnPtr::new(loadfn("glWindowPos2fMESA")),
         WindowPos2fvARB: FnPtr::new(loadfn("glWindowPos2fvARB")),
         WindowPos2fvMESA: FnPtr::new(loadfn("glWindowPos2fvMESA")),
         WindowPos2iARB: FnPtr::new(loadfn("glWindowPos2iARB")),
         WindowPos2iMESA: FnPtr::new(loadfn("glWindowPos2iMESA")),
         WindowPos2ivARB: FnPtr::new(loadfn("glWindowPos2ivARB")),
         WindowPos2ivMESA: FnPtr::new(loadfn("glWindowPos2ivMESA")),
         WindowPos2sARB: FnPtr::new(loadfn("glWindowPos2sARB")),
         WindowPos2sMESA: FnPtr::new(loadfn("glWindowPos2sMESA")),
         WindowPos2svARB: FnPtr::new(loadfn("glWindowPos2svARB")),
         WindowPos2svMESA: FnPtr::new(loadfn("glWindowPos2svMESA")),
         WindowPos3dARB: FnPtr::new(loadfn("glWindowPos3dARB")),
         WindowPos3dMESA: FnPtr::new(loadfn("glWindowPos3dMESA")),
         WindowPos3dvARB: FnPtr::new(loadfn("glWindowPos3dvARB")),
         WindowPos3dvMESA: FnPtr::new(loadfn("glWindowPos3dvMESA")),
         WindowPos3fARB: FnPtr::new(loadfn("glWindowPos3fARB")),
         WindowPos3fMESA: FnPtr::new(loadfn("glWindowPos3fMESA")),
         WindowPos3fvARB: FnPtr::new(loadfn("glWindowPos3fvARB")),
         WindowPos3fvMESA: FnPtr::new(loadfn("glWindowPos3fvMESA")),
         WindowPos3iARB: FnPtr::new(loadfn("glWindowPos3iARB")),
         WindowPos3iMESA: FnPtr::new(loadfn("glWindowPos3iMESA")),
         WindowPos3ivARB: FnPtr::new(loadfn("glWindowPos3ivARB")),
         WindowPos3ivMESA: FnPtr::new(loadfn("glWindowPos3ivMESA")),
         WindowPos3sARB: FnPtr::new(loadfn("glWindowPos3sARB")),
         WindowPos3sMESA: FnPtr::new(loadfn("glWindowPos3sMESA")),
         WindowPos3svARB: FnPtr::new(loadfn("glWindowPos3svARB")),
         WindowPos3svMESA: FnPtr::new(loadfn("glWindowPos3svMESA")),
         WindowPos4dMESA: FnPtr::new(loadfn("glWindowPos4dMESA")),
         WindowPos4dvMESA: FnPtr::new(loadfn("glWindowPos4dvMESA")),
         WindowPos4fMESA: FnPtr::new(loadfn("glWindowPos4fMESA")),
         WindowPos4fvMESA: FnPtr::new(loadfn("glWindowPos4fvMESA")),
         WindowPos4iMESA: FnPtr::new(loadfn("glWindowPos4iMESA")),
         WindowPos4ivMESA: FnPtr::new(loadfn("glWindowPos4ivMESA")),
         WindowPos4sMESA: FnPtr::new(loadfn("glWindowPos4sMESA")),
         WindowPos4svMESA: FnPtr::new(loadfn("glWindowPos4svMESA")),
         WindowRectanglesEXT: FnPtr::new(loadfn("glWindowRectanglesEXT")),
         WriteMaskEXT: FnPtr::new(loadfn("glWriteMaskEXT")),
    };

     ctx.ActiveTexture.aliased(&ctx.ActiveTextureARB);
     ctx.ActiveTextureARB.aliased(&ctx.ActiveTexture);
     ctx.AttachObjectARB.aliased(&ctx.AttachShader);
     ctx.AttachShader.aliased(&ctx.AttachObjectARB);
     ctx.BeginConditionalRender.aliased(&ctx.BeginConditionalRenderNV);
     ctx.BeginConditionalRenderNV.aliased(&ctx.BeginConditionalRender);
     ctx.BeginQuery.aliased(&ctx.BeginQueryARB);
     ctx.BeginQueryARB.aliased(&ctx.BeginQuery);
     ctx.BeginTransformFeedback.aliased(&ctx.BeginTransformFeedbackEXT);
     ctx.BeginTransformFeedback.aliased(&ctx.BeginTransformFeedbackNV);
     ctx.BeginTransformFeedbackEXT.aliased(&ctx.BeginTransformFeedback);
     ctx.BeginTransformFeedbackEXT.aliased(&ctx.BeginTransformFeedbackNV);
     ctx.BeginTransformFeedbackNV.aliased(&ctx.BeginTransformFeedback);
     ctx.BeginTransformFeedbackNV.aliased(&ctx.BeginTransformFeedbackEXT);
     ctx.BindAttribLocation.aliased(&ctx.BindAttribLocationARB);
     ctx.BindAttribLocationARB.aliased(&ctx.BindAttribLocation);
     ctx.BindBuffer.aliased(&ctx.BindBufferARB);
     ctx.BindBufferARB.aliased(&ctx.BindBuffer);
     ctx.BindBufferBase.aliased(&ctx.BindBufferBaseEXT);
     ctx.BindBufferBase.aliased(&ctx.BindBufferBaseNV);
     ctx.BindBufferBaseEXT.aliased(&ctx.BindBufferBase);
     ctx.BindBufferBaseEXT.aliased(&ctx.BindBufferBaseNV);
     ctx.BindBufferBaseNV.aliased(&ctx.BindBufferBase);
     ctx.BindBufferBaseNV.aliased(&ctx.BindBufferBaseEXT);
     ctx.BindBufferOffsetEXT.aliased(&ctx.BindBufferOffsetNV);
     ctx.BindBufferOffsetNV.aliased(&ctx.BindBufferOffsetEXT);
     ctx.BindBufferRange.aliased(&ctx.BindBufferRangeEXT);
     ctx.BindBufferRange.aliased(&ctx.BindBufferRangeNV);
     ctx.BindBufferRangeEXT.aliased(&ctx.BindBufferRange);
     ctx.BindBufferRangeEXT.aliased(&ctx.BindBufferRangeNV);
     ctx.BindBufferRangeNV.aliased(&ctx.BindBufferRange);
     ctx.BindBufferRangeNV.aliased(&ctx.BindBufferRangeEXT);
     ctx.BindFragDataLocation.aliased(&ctx.BindFragDataLocationEXT);
     ctx.BindFragDataLocationEXT.aliased(&ctx.BindFragDataLocation);
     ctx.BindProgramARB.aliased(&ctx.BindProgramNV);
     ctx.BindProgramNV.aliased(&ctx.BindProgramARB);
     ctx.BindTexture.aliased(&ctx.BindTextureEXT);
     ctx.BindTextureEXT.aliased(&ctx.BindTexture);
     ctx.BlendColor.aliased(&ctx.BlendColorEXT);
     ctx.BlendColorEXT.aliased(&ctx.BlendColor);
     ctx.BlendEquation.aliased(&ctx.BlendEquationEXT);
     ctx.BlendEquationEXT.aliased(&ctx.BlendEquation);
     ctx.BlendEquationi.aliased(&ctx.BlendEquationIndexedAMD);
     ctx.BlendEquationi.aliased(&ctx.BlendEquationiARB);
     ctx.BlendEquationiARB.aliased(&ctx.BlendEquationIndexedAMD);
     ctx.BlendEquationiARB.aliased(&ctx.BlendEquationi);
     ctx.BlendEquationIndexedAMD.aliased(&ctx.BlendEquationi);
     ctx.BlendEquationIndexedAMD.aliased(&ctx.BlendEquationiARB);
     ctx.BlendEquationSeparate.aliased(&ctx.BlendEquationSeparateEXT);
     ctx.BlendEquationSeparateEXT.aliased(&ctx.BlendEquationSeparate);
     ctx.BlendEquationSeparatei.aliased(&ctx.BlendEquationSeparateIndexedAMD);
     ctx.BlendEquationSeparatei.aliased(&ctx.BlendEquationSeparateiARB);
     ctx.BlendEquationSeparateiARB.aliased(&ctx.BlendEquationSeparateIndexedAMD);
     ctx.BlendEquationSeparateiARB.aliased(&ctx.BlendEquationSeparatei);
     ctx.BlendEquationSeparateIndexedAMD.aliased(&ctx.BlendEquationSeparatei);
     ctx.BlendEquationSeparateIndexedAMD.aliased(&ctx.BlendEquationSeparateiARB);
     ctx.BlendFunci.aliased(&ctx.BlendFuncIndexedAMD);
     ctx.BlendFunci.aliased(&ctx.BlendFunciARB);
     ctx.BlendFunciARB.aliased(&ctx.BlendFuncIndexedAMD);
     ctx.BlendFunciARB.aliased(&ctx.BlendFunci);
     ctx.BlendFuncIndexedAMD.aliased(&ctx.BlendFunci);
     ctx.BlendFuncIndexedAMD.aliased(&ctx.BlendFunciARB);
     ctx.BlendFuncSeparate.aliased(&ctx.BlendFuncSeparateEXT);
     ctx.BlendFuncSeparate.aliased(&ctx.BlendFuncSeparateINGR);
     ctx.BlendFuncSeparateEXT.aliased(&ctx.BlendFuncSeparate);
     ctx.BlendFuncSeparateEXT.aliased(&ctx.BlendFuncSeparateINGR);
     ctx.BlendFuncSeparatei.aliased(&ctx.BlendFuncSeparateIndexedAMD);
     ctx.BlendFuncSeparatei.aliased(&ctx.BlendFuncSeparateiARB);
     ctx.BlendFuncSeparateiARB.aliased(&ctx.BlendFuncSeparateIndexedAMD);
     ctx.BlendFuncSeparateiARB.aliased(&ctx.BlendFuncSeparatei);
     ctx.BlendFuncSeparateIndexedAMD.aliased(&ctx.BlendFuncSeparatei);
     ctx.BlendFuncSeparateIndexedAMD.aliased(&ctx.BlendFuncSeparateiARB);
     ctx.BlendFuncSeparateINGR.aliased(&ctx.BlendFuncSeparate);
     ctx.BlendFuncSeparateINGR.aliased(&ctx.BlendFuncSeparateEXT);
     ctx.BlitFramebuffer.aliased(&ctx.BlitFramebufferEXT);
     ctx.BlitFramebufferEXT.aliased(&ctx.BlitFramebuffer);
     ctx.BufferData.aliased(&ctx.BufferDataARB);
     ctx.BufferDataARB.aliased(&ctx.BufferData);
     ctx.BufferSubData.aliased(&ctx.BufferSubDataARB);
     ctx.BufferSubDataARB.aliased(&ctx.BufferSubData);
     ctx.CheckFramebufferStatus.aliased(&ctx.CheckFramebufferStatusEXT);
     ctx.CheckFramebufferStatusEXT.aliased(&ctx.CheckFramebufferStatus);
     ctx.ClampColor.aliased(&ctx.ClampColorARB);
     ctx.ClampColorARB.aliased(&ctx.ClampColor);
     ctx.ClearDepthf.aliased(&ctx.ClearDepthfOES);
     ctx.ClearDepthfOES.aliased(&ctx.ClearDepthf);
     ctx.ColorMaski.aliased(&ctx.ColorMaskIndexedEXT);
     ctx.ColorMaskIndexedEXT.aliased(&ctx.ColorMaski);
     ctx.CompileShader.aliased(&ctx.CompileShaderARB);
     ctx.CompileShaderARB.aliased(&ctx.CompileShader);
     ctx.CompressedTexImage1D.aliased(&ctx.CompressedTexImage1DARB);
     ctx.CompressedTexImage1DARB.aliased(&ctx.CompressedTexImage1D);
     ctx.CompressedTexImage2D.aliased(&ctx.CompressedTexImage2DARB);
     ctx.CompressedTexImage2DARB.aliased(&ctx.CompressedTexImage2D);
     ctx.CompressedTexImage3D.aliased(&ctx.CompressedTexImage3DARB);
     ctx.CompressedTexImage3DARB.aliased(&ctx.CompressedTexImage3D);
     ctx.CompressedTexSubImage1D.aliased(&ctx.CompressedTexSubImage1DARB);
     ctx.CompressedTexSubImage1DARB.aliased(&ctx.CompressedTexSubImage1D);
     ctx.CompressedTexSubImage2D.aliased(&ctx.CompressedTexSubImage2DARB);
     ctx.CompressedTexSubImage2DARB.aliased(&ctx.CompressedTexSubImage2D);
     ctx.CompressedTexSubImage3D.aliased(&ctx.CompressedTexSubImage3DARB);
     ctx.CompressedTexSubImage3DARB.aliased(&ctx.CompressedTexSubImage3D);
     ctx.CopyTexImage1D.aliased(&ctx.CopyTexImage1DEXT);
     ctx.CopyTexImage1DEXT.aliased(&ctx.CopyTexImage1D);
     ctx.CopyTexImage2D.aliased(&ctx.CopyTexImage2DEXT);
     ctx.CopyTexImage2DEXT.aliased(&ctx.CopyTexImage2D);
     ctx.CopyTexSubImage1D.aliased(&ctx.CopyTexSubImage1DEXT);
     ctx.CopyTexSubImage1DEXT.aliased(&ctx.CopyTexSubImage1D);
     ctx.CopyTexSubImage2D.aliased(&ctx.CopyTexSubImage2DEXT);
     ctx.CopyTexSubImage2DEXT.aliased(&ctx.CopyTexSubImage2D);
     ctx.CopyTexSubImage3D.aliased(&ctx.CopyTexSubImage3DEXT);
     ctx.CopyTexSubImage3DEXT.aliased(&ctx.CopyTexSubImage3D);
     ctx.CreateProgram.aliased(&ctx.CreateProgramObjectARB);
     ctx.CreateProgramObjectARB.aliased(&ctx.CreateProgram);
     ctx.CreateShader.aliased(&ctx.CreateShaderObjectARB);
     ctx.CreateShaderObjectARB.aliased(&ctx.CreateShader);
     ctx.DebugMessageCallback.aliased(&ctx.DebugMessageCallbackARB);
     ctx.DebugMessageCallbackARB.aliased(&ctx.DebugMessageCallback);
     ctx.DebugMessageControl.aliased(&ctx.DebugMessageControlARB);
     ctx.DebugMessageControlARB.aliased(&ctx.DebugMessageControl);
     ctx.DebugMessageInsert.aliased(&ctx.DebugMessageInsertARB);
     ctx.DebugMessageInsertARB.aliased(&ctx.DebugMessageInsert);
     ctx.DeleteBuffers.aliased(&ctx.DeleteBuffersARB);
     ctx.DeleteBuffersARB.aliased(&ctx.DeleteBuffers);
     ctx.DeleteFramebuffers.aliased(&ctx.DeleteFramebuffersEXT);
     ctx.DeleteFramebuffersEXT.aliased(&ctx.DeleteFramebuffers);
     ctx.DeleteProgramsARB.aliased(&ctx.DeleteProgramsNV);
     ctx.DeleteProgramsNV.aliased(&ctx.DeleteProgramsARB);
     ctx.DeleteQueries.aliased(&ctx.DeleteQueriesARB);
     ctx.DeleteQueriesARB.aliased(&ctx.DeleteQueries);
     ctx.DeleteRenderbuffers.aliased(&ctx.DeleteRenderbuffersEXT);
     ctx.DeleteRenderbuffersEXT.aliased(&ctx.DeleteRenderbuffers);
     ctx.DeleteTransformFeedbacks.aliased(&ctx.DeleteTransformFeedbacksNV);
     ctx.DeleteTransformFeedbacksNV.aliased(&ctx.DeleteTransformFeedbacks);
     ctx.DeleteVertexArrays.aliased(&ctx.DeleteVertexArraysAPPLE);
     ctx.DeleteVertexArraysAPPLE.aliased(&ctx.DeleteVertexArrays);
     ctx.DepthRangef.aliased(&ctx.DepthRangefOES);
     ctx.DepthRangefOES.aliased(&ctx.DepthRangef);
     ctx.DetachObjectARB.aliased(&ctx.DetachShader);
     ctx.DetachShader.aliased(&ctx.DetachObjectARB);
     ctx.Disablei.aliased(&ctx.DisableIndexedEXT);
     ctx.DisableIndexedEXT.aliased(&ctx.Disablei);
     ctx.DisableVertexAttribArray.aliased(&ctx.DisableVertexAttribArrayARB);
     ctx.DisableVertexAttribArrayARB.aliased(&ctx.DisableVertexAttribArray);
     ctx.DrawArrays.aliased(&ctx.DrawArraysEXT);
     ctx.DrawArraysEXT.aliased(&ctx.DrawArrays);
     ctx.DrawArraysInstanced.aliased(&ctx.DrawArraysInstancedARB);
     ctx.DrawArraysInstanced.aliased(&ctx.DrawArraysInstancedEXT);
     ctx.DrawArraysInstancedARB.aliased(&ctx.DrawArraysInstanced);
     ctx.DrawArraysInstancedARB.aliased(&ctx.DrawArraysInstancedEXT);
     ctx.DrawArraysInstancedEXT.aliased(&ctx.DrawArraysInstanced);
     ctx.DrawArraysInstancedEXT.aliased(&ctx.DrawArraysInstancedARB);
     ctx.DrawBuffers.aliased(&ctx.DrawBuffersARB);
     ctx.DrawBuffers.aliased(&ctx.DrawBuffersATI);
     ctx.DrawBuffersARB.aliased(&ctx.DrawBuffers);
     ctx.DrawBuffersARB.aliased(&ctx.DrawBuffersATI);
     ctx.DrawBuffersATI.aliased(&ctx.DrawBuffers);
     ctx.DrawBuffersATI.aliased(&ctx.DrawBuffersARB);
     ctx.DrawElementsInstanced.aliased(&ctx.DrawElementsInstancedARB);
     ctx.DrawElementsInstanced.aliased(&ctx.DrawElementsInstancedEXT);
     ctx.DrawElementsInstancedARB.aliased(&ctx.DrawElementsInstanced);
     ctx.DrawElementsInstancedARB.aliased(&ctx.DrawElementsInstancedEXT);
     ctx.DrawElementsInstancedEXT.aliased(&ctx.DrawElementsInstanced);
     ctx.DrawElementsInstancedEXT.aliased(&ctx.DrawElementsInstancedARB);
     ctx.DrawRangeElements.aliased(&ctx.DrawRangeElementsEXT);
     ctx.DrawRangeElementsEXT.aliased(&ctx.DrawRangeElements);
     ctx.DrawTransformFeedback.aliased(&ctx.DrawTransformFeedbackNV);
     ctx.DrawTransformFeedbackNV.aliased(&ctx.DrawTransformFeedback);
     ctx.Enablei.aliased(&ctx.EnableIndexedEXT);
     ctx.EnableIndexedEXT.aliased(&ctx.Enablei);
     ctx.EnableVertexAttribArray.aliased(&ctx.EnableVertexAttribArrayARB);
     ctx.EnableVertexAttribArrayARB.aliased(&ctx.EnableVertexAttribArray);
     ctx.EndConditionalRender.aliased(&ctx.EndConditionalRenderNV);
     ctx.EndConditionalRender.aliased(&ctx.EndConditionalRenderNVX);
     ctx.EndConditionalRenderNV.aliased(&ctx.EndConditionalRender);
     ctx.EndConditionalRenderNV.aliased(&ctx.EndConditionalRenderNVX);
     ctx.EndConditionalRenderNVX.aliased(&ctx.EndConditionalRender);
     ctx.EndConditionalRenderNVX.aliased(&ctx.EndConditionalRenderNV);
     ctx.EndQuery.aliased(&ctx.EndQueryARB);
     ctx.EndQueryARB.aliased(&ctx.EndQuery);
     ctx.EndTransformFeedback.aliased(&ctx.EndTransformFeedbackEXT);
     ctx.EndTransformFeedback.aliased(&ctx.EndTransformFeedbackNV);
     ctx.EndTransformFeedbackEXT.aliased(&ctx.EndTransformFeedback);
     ctx.EndTransformFeedbackEXT.aliased(&ctx.EndTransformFeedbackNV);
     ctx.EndTransformFeedbackNV.aliased(&ctx.EndTransformFeedback);
     ctx.EndTransformFeedbackNV.aliased(&ctx.EndTransformFeedbackEXT);
     ctx.FlushMappedBufferRange.aliased(&ctx.FlushMappedBufferRangeAPPLE);
     ctx.FlushMappedBufferRangeAPPLE.aliased(&ctx.FlushMappedBufferRange);
     ctx.FramebufferRenderbuffer.aliased(&ctx.FramebufferRenderbufferEXT);
     ctx.FramebufferRenderbufferEXT.aliased(&ctx.FramebufferRenderbuffer);
     ctx.FramebufferTexture.aliased(&ctx.FramebufferTextureARB);
     ctx.FramebufferTexture.aliased(&ctx.FramebufferTextureEXT);
     ctx.FramebufferTexture1D.aliased(&ctx.FramebufferTexture1DEXT);
     ctx.FramebufferTexture1DEXT.aliased(&ctx.FramebufferTexture1D);
     ctx.FramebufferTexture2D.aliased(&ctx.FramebufferTexture2DEXT);
     ctx.FramebufferTexture2DEXT.aliased(&ctx.FramebufferTexture2D);
     ctx.FramebufferTexture3D.aliased(&ctx.FramebufferTexture3DEXT);
     ctx.FramebufferTexture3DEXT.aliased(&ctx.FramebufferTexture3D);
     ctx.FramebufferTextureARB.aliased(&ctx.FramebufferTexture);
     ctx.FramebufferTextureARB.aliased(&ctx.FramebufferTextureEXT);
     ctx.FramebufferTextureEXT.aliased(&ctx.FramebufferTexture);
     ctx.FramebufferTextureEXT.aliased(&ctx.FramebufferTextureARB);
     ctx.FramebufferTextureFaceARB.aliased(&ctx.FramebufferTextureFaceEXT);
     ctx.FramebufferTextureFaceEXT.aliased(&ctx.FramebufferTextureFaceARB);
     ctx.FramebufferTextureLayer.aliased(&ctx.FramebufferTextureLayerARB);
     ctx.FramebufferTextureLayer.aliased(&ctx.FramebufferTextureLayerEXT);
     ctx.FramebufferTextureLayerARB.aliased(&ctx.FramebufferTextureLayer);
     ctx.FramebufferTextureLayerARB.aliased(&ctx.FramebufferTextureLayerEXT);
     ctx.FramebufferTextureLayerEXT.aliased(&ctx.FramebufferTextureLayer);
     ctx.FramebufferTextureLayerEXT.aliased(&ctx.FramebufferTextureLayerARB);
     ctx.GenBuffers.aliased(&ctx.GenBuffersARB);
     ctx.GenBuffersARB.aliased(&ctx.GenBuffers);
     ctx.GenerateMipmap.aliased(&ctx.GenerateMipmapEXT);
     ctx.GenerateMipmapEXT.aliased(&ctx.GenerateMipmap);
     ctx.GenFramebuffers.aliased(&ctx.GenFramebuffersEXT);
     ctx.GenFramebuffersEXT.aliased(&ctx.GenFramebuffers);
     ctx.GenProgramsARB.aliased(&ctx.GenProgramsNV);
     ctx.GenProgramsNV.aliased(&ctx.GenProgramsARB);
     ctx.GenQueries.aliased(&ctx.GenQueriesARB);
     ctx.GenQueriesARB.aliased(&ctx.GenQueries);
     ctx.GenRenderbuffers.aliased(&ctx.GenRenderbuffersEXT);
     ctx.GenRenderbuffersEXT.aliased(&ctx.GenRenderbuffers);
     ctx.GenTransformFeedbacks.aliased(&ctx.GenTransformFeedbacksNV);
     ctx.GenTransformFeedbacksNV.aliased(&ctx.GenTransformFeedbacks);
     ctx.GenVertexArrays.aliased(&ctx.GenVertexArraysAPPLE);
     ctx.GenVertexArraysAPPLE.aliased(&ctx.GenVertexArrays);
     ctx.GetActiveAttrib.aliased(&ctx.GetActiveAttribARB);
     ctx.GetActiveAttribARB.aliased(&ctx.GetActiveAttrib);
     ctx.GetActiveUniform.aliased(&ctx.GetActiveUniformARB);
     ctx.GetActiveUniformARB.aliased(&ctx.GetActiveUniform);
     ctx.GetAttribLocation.aliased(&ctx.GetAttribLocationARB);
     ctx.GetAttribLocationARB.aliased(&ctx.GetAttribLocation);
     ctx.GetBooleani_v.aliased(&ctx.GetBooleanIndexedvEXT);
     ctx.GetBooleanIndexedvEXT.aliased(&ctx.GetBooleani_v);
     ctx.GetBufferParameteriv.aliased(&ctx.GetBufferParameterivARB);
     ctx.GetBufferParameterivARB.aliased(&ctx.GetBufferParameteriv);
     ctx.GetBufferPointerv.aliased(&ctx.GetBufferPointervARB);
     ctx.GetBufferPointervARB.aliased(&ctx.GetBufferPointerv);
     ctx.GetBufferSubData.aliased(&ctx.GetBufferSubDataARB);
     ctx.GetBufferSubDataARB.aliased(&ctx.GetBufferSubData);
     ctx.GetCompressedTexImage.aliased(&ctx.GetCompressedTexImageARB);
     ctx.GetCompressedTexImageARB.aliased(&ctx.GetCompressedTexImage);
     ctx.GetDebugMessageLog.aliased(&ctx.GetDebugMessageLogARB);
     ctx.GetDebugMessageLogARB.aliased(&ctx.GetDebugMessageLog);
     ctx.GetDoublei_v.aliased(&ctx.GetDoubleIndexedvEXT);
     ctx.GetDoublei_v.aliased(&ctx.GetDoublei_vEXT);
     ctx.GetDoublei_vEXT.aliased(&ctx.GetDoubleIndexedvEXT);
     ctx.GetDoublei_vEXT.aliased(&ctx.GetDoublei_v);
     ctx.GetDoubleIndexedvEXT.aliased(&ctx.GetDoublei_v);
     ctx.GetDoubleIndexedvEXT.aliased(&ctx.GetDoublei_vEXT);
     ctx.GetFloati_v.aliased(&ctx.GetFloatIndexedvEXT);
     ctx.GetFloati_v.aliased(&ctx.GetFloati_vEXT);
     ctx.GetFloati_vEXT.aliased(&ctx.GetFloatIndexedvEXT);
     ctx.GetFloati_vEXT.aliased(&ctx.GetFloati_v);
     ctx.GetFloatIndexedvEXT.aliased(&ctx.GetFloati_v);
     ctx.GetFloatIndexedvEXT.aliased(&ctx.GetFloati_vEXT);
     ctx.GetFragDataLocation.aliased(&ctx.GetFragDataLocationEXT);
     ctx.GetFragDataLocationEXT.aliased(&ctx.GetFragDataLocation);
     ctx.GetFramebufferAttachmentParameteriv.aliased(&ctx.GetFramebufferAttachmentParameterivEXT);
     ctx.GetFramebufferAttachmentParameterivEXT.aliased(&ctx.GetFramebufferAttachmentParameteriv);
     ctx.GetIntegeri_v.aliased(&ctx.GetIntegerIndexedvEXT);
     ctx.GetIntegerIndexedvEXT.aliased(&ctx.GetIntegeri_v);
     ctx.GetMultisamplefv.aliased(&ctx.GetMultisamplefvNV);
     ctx.GetMultisamplefvNV.aliased(&ctx.GetMultisamplefv);
     ctx.GetPointerv.aliased(&ctx.GetPointervEXT);
     ctx.GetPointervEXT.aliased(&ctx.GetPointerv);
     ctx.GetQueryiv.aliased(&ctx.GetQueryivARB);
     ctx.GetQueryivARB.aliased(&ctx.GetQueryiv);
     ctx.GetQueryObjecti64v.aliased(&ctx.GetQueryObjecti64vEXT);
     ctx.GetQueryObjecti64vEXT.aliased(&ctx.GetQueryObjecti64v);
     ctx.GetQueryObjectiv.aliased(&ctx.GetQueryObjectivARB);
     ctx.GetQueryObjectivARB.aliased(&ctx.GetQueryObjectiv);
     ctx.GetQueryObjectui64v.aliased(&ctx.GetQueryObjectui64vEXT);
     ctx.GetQueryObjectui64vEXT.aliased(&ctx.GetQueryObjectui64v);
     ctx.GetQueryObjectuiv.aliased(&ctx.GetQueryObjectuivARB);
     ctx.GetQueryObjectuivARB.aliased(&ctx.GetQueryObjectuiv);
     ctx.GetRenderbufferParameteriv.aliased(&ctx.GetRenderbufferParameterivEXT);
     ctx.GetRenderbufferParameterivEXT.aliased(&ctx.GetRenderbufferParameteriv);
     ctx.GetShaderSource.aliased(&ctx.GetShaderSourceARB);
     ctx.GetShaderSourceARB.aliased(&ctx.GetShaderSource);
     ctx.GetTexParameterIiv.aliased(&ctx.GetTexParameterIivEXT);
     ctx.GetTexParameterIivEXT.aliased(&ctx.GetTexParameterIiv);
     ctx.GetTexParameterIuiv.aliased(&ctx.GetTexParameterIuivEXT);
     ctx.GetTexParameterIuivEXT.aliased(&ctx.GetTexParameterIuiv);
     ctx.GetTransformFeedbackVarying.aliased(&ctx.GetTransformFeedbackVaryingEXT);
     ctx.GetTransformFeedbackVaryingEXT.aliased(&ctx.GetTransformFeedbackVarying);
     ctx.GetUniformfv.aliased(&ctx.GetUniformfvARB);
     ctx.GetUniformfvARB.aliased(&ctx.GetUniformfv);
     ctx.GetUniformiv.aliased(&ctx.GetUniformivARB);
     ctx.GetUniformivARB.aliased(&ctx.GetUniformiv);
     ctx.GetUniformLocation.aliased(&ctx.GetUniformLocationARB);
     ctx.GetUniformLocationARB.aliased(&ctx.GetUniformLocation);
     ctx.GetUniformuiv.aliased(&ctx.GetUniformuivEXT);
     ctx.GetUniformuivEXT.aliased(&ctx.GetUniformuiv);
     ctx.GetVertexAttribdv.aliased(&ctx.GetVertexAttribdvARB);
     ctx.GetVertexAttribdv.aliased(&ctx.GetVertexAttribdvNV);
     ctx.GetVertexAttribdvARB.aliased(&ctx.GetVertexAttribdv);
     ctx.GetVertexAttribdvARB.aliased(&ctx.GetVertexAttribdvNV);
     ctx.GetVertexAttribdvNV.aliased(&ctx.GetVertexAttribdv);
     ctx.GetVertexAttribdvNV.aliased(&ctx.GetVertexAttribdvARB);
     ctx.GetVertexAttribfv.aliased(&ctx.GetVertexAttribfvARB);
     ctx.GetVertexAttribfv.aliased(&ctx.GetVertexAttribfvNV);
     ctx.GetVertexAttribfvARB.aliased(&ctx.GetVertexAttribfv);
     ctx.GetVertexAttribfvARB.aliased(&ctx.GetVertexAttribfvNV);
     ctx.GetVertexAttribfvNV.aliased(&ctx.GetVertexAttribfv);
     ctx.GetVertexAttribfvNV.aliased(&ctx.GetVertexAttribfvARB);
     ctx.GetVertexAttribIiv.aliased(&ctx.GetVertexAttribIivEXT);
     ctx.GetVertexAttribIivEXT.aliased(&ctx.GetVertexAttribIiv);
     ctx.GetVertexAttribIuiv.aliased(&ctx.GetVertexAttribIuivEXT);
     ctx.GetVertexAttribIuivEXT.aliased(&ctx.GetVertexAttribIuiv);
     ctx.GetVertexAttribiv.aliased(&ctx.GetVertexAttribivARB);
     ctx.GetVertexAttribiv.aliased(&ctx.GetVertexAttribivNV);
     ctx.GetVertexAttribivARB.aliased(&ctx.GetVertexAttribiv);
     ctx.GetVertexAttribivARB.aliased(&ctx.GetVertexAttribivNV);
     ctx.GetVertexAttribivNV.aliased(&ctx.GetVertexAttribiv);
     ctx.GetVertexAttribivNV.aliased(&ctx.GetVertexAttribivARB);
     ctx.GetVertexAttribLdv.aliased(&ctx.GetVertexAttribLdvEXT);
     ctx.GetVertexAttribLdvEXT.aliased(&ctx.GetVertexAttribLdv);
     ctx.GetVertexAttribPointerv.aliased(&ctx.GetVertexAttribPointervARB);
     ctx.GetVertexAttribPointerv.aliased(&ctx.GetVertexAttribPointervNV);
     ctx.GetVertexAttribPointervARB.aliased(&ctx.GetVertexAttribPointerv);
     ctx.GetVertexAttribPointervARB.aliased(&ctx.GetVertexAttribPointervNV);
     ctx.GetVertexAttribPointervNV.aliased(&ctx.GetVertexAttribPointerv);
     ctx.GetVertexAttribPointervNV.aliased(&ctx.GetVertexAttribPointervARB);
     ctx.IsBuffer.aliased(&ctx.IsBufferARB);
     ctx.IsBufferARB.aliased(&ctx.IsBuffer);
     ctx.IsEnabledi.aliased(&ctx.IsEnabledIndexedEXT);
     ctx.IsEnabledIndexedEXT.aliased(&ctx.IsEnabledi);
     ctx.IsFramebuffer.aliased(&ctx.IsFramebufferEXT);
     ctx.IsFramebufferEXT.aliased(&ctx.IsFramebuffer);
     ctx.IsProgramARB.aliased(&ctx.IsProgramNV);
     ctx.IsProgramNV.aliased(&ctx.IsProgramARB);
     ctx.IsQuery.aliased(&ctx.IsQueryARB);
     ctx.IsQueryARB.aliased(&ctx.IsQuery);
     ctx.IsRenderbuffer.aliased(&ctx.IsRenderbufferEXT);
     ctx.IsRenderbufferEXT.aliased(&ctx.IsRenderbuffer);
     ctx.IsTransformFeedback.aliased(&ctx.IsTransformFeedbackNV);
     ctx.IsTransformFeedbackNV.aliased(&ctx.IsTransformFeedback);
     ctx.IsVertexArray.aliased(&ctx.IsVertexArrayAPPLE);
     ctx.IsVertexArrayAPPLE.aliased(&ctx.IsVertexArray);
     ctx.LinkProgram.aliased(&ctx.LinkProgramARB);
     ctx.LinkProgramARB.aliased(&ctx.LinkProgram);
     ctx.MapBuffer.aliased(&ctx.MapBufferARB);
     ctx.MapBufferARB.aliased(&ctx.MapBuffer);
     ctx.MaxShaderCompilerThreadsARB.aliased(&ctx.MaxShaderCompilerThreadsKHR);
     ctx.MaxShaderCompilerThreadsKHR.aliased(&ctx.MaxShaderCompilerThreadsARB);
     ctx.MemoryBarrier.aliased(&ctx.MemoryBarrierEXT);
     ctx.MemoryBarrierEXT.aliased(&ctx.MemoryBarrier);
     ctx.MinSampleShading.aliased(&ctx.MinSampleShadingARB);
     ctx.MinSampleShadingARB.aliased(&ctx.MinSampleShading);
     ctx.MultiDrawArrays.aliased(&ctx.MultiDrawArraysEXT);
     ctx.MultiDrawArraysEXT.aliased(&ctx.MultiDrawArrays);
     ctx.MultiDrawArraysIndirect.aliased(&ctx.MultiDrawArraysIndirectAMD);
     ctx.MultiDrawArraysIndirectAMD.aliased(&ctx.MultiDrawArraysIndirect);
     ctx.MultiDrawArraysIndirectCount.aliased(&ctx.MultiDrawArraysIndirectCountARB);
     ctx.MultiDrawArraysIndirectCountARB.aliased(&ctx.MultiDrawArraysIndirectCount);
     ctx.MultiDrawElements.aliased(&ctx.MultiDrawElementsEXT);
     ctx.MultiDrawElementsEXT.aliased(&ctx.MultiDrawElements);
     ctx.MultiDrawElementsIndirect.aliased(&ctx.MultiDrawElementsIndirectAMD);
     ctx.MultiDrawElementsIndirectAMD.aliased(&ctx.MultiDrawElementsIndirect);
     ctx.MultiDrawElementsIndirectCount.aliased(&ctx.MultiDrawElementsIndirectCountARB);
     ctx.MultiDrawElementsIndirectCountARB.aliased(&ctx.MultiDrawElementsIndirectCount);
     ctx.NamedBufferStorage.aliased(&ctx.NamedBufferStorageEXT);
     ctx.NamedBufferStorageEXT.aliased(&ctx.NamedBufferStorage);
     ctx.NamedBufferSubData.aliased(&ctx.NamedBufferSubDataEXT);
     ctx.NamedBufferSubDataEXT.aliased(&ctx.NamedBufferSubData);
     ctx.PauseTransformFeedback.aliased(&ctx.PauseTransformFeedbackNV);
     ctx.PauseTransformFeedbackNV.aliased(&ctx.PauseTransformFeedback);
     ctx.PointParameterf.aliased(&ctx.PointParameterfARB);
     ctx.PointParameterf.aliased(&ctx.PointParameterfEXT);
     ctx.PointParameterf.aliased(&ctx.PointParameterfSGIS);
     ctx.PointParameterfARB.aliased(&ctx.PointParameterf);
     ctx.PointParameterfARB.aliased(&ctx.PointParameterfEXT);
     ctx.PointParameterfARB.aliased(&ctx.PointParameterfSGIS);
     ctx.PointParameterfEXT.aliased(&ctx.PointParameterf);
     ctx.PointParameterfEXT.aliased(&ctx.PointParameterfARB);
     ctx.PointParameterfEXT.aliased(&ctx.PointParameterfSGIS);
     ctx.PointParameterfSGIS.aliased(&ctx.PointParameterf);
     ctx.PointParameterfSGIS.aliased(&ctx.PointParameterfARB);
     ctx.PointParameterfSGIS.aliased(&ctx.PointParameterfEXT);
     ctx.PointParameterfv.aliased(&ctx.PointParameterfvARB);
     ctx.PointParameterfv.aliased(&ctx.PointParameterfvEXT);
     ctx.PointParameterfv.aliased(&ctx.PointParameterfvSGIS);
     ctx.PointParameterfvARB.aliased(&ctx.PointParameterfv);
     ctx.PointParameterfvARB.aliased(&ctx.PointParameterfvEXT);
     ctx.PointParameterfvARB.aliased(&ctx.PointParameterfvSGIS);
     ctx.PointParameterfvEXT.aliased(&ctx.PointParameterfv);
     ctx.PointParameterfvEXT.aliased(&ctx.PointParameterfvARB);
     ctx.PointParameterfvEXT.aliased(&ctx.PointParameterfvSGIS);
     ctx.PointParameterfvSGIS.aliased(&ctx.PointParameterfv);
     ctx.PointParameterfvSGIS.aliased(&ctx.PointParameterfvARB);
     ctx.PointParameterfvSGIS.aliased(&ctx.PointParameterfvEXT);
     ctx.PointParameteri.aliased(&ctx.PointParameteriNV);
     ctx.PointParameteriNV.aliased(&ctx.PointParameteri);
     ctx.PointParameteriv.aliased(&ctx.PointParameterivNV);
     ctx.PointParameterivNV.aliased(&ctx.PointParameteriv);
     ctx.PolygonOffsetClamp.aliased(&ctx.PolygonOffsetClampEXT);
     ctx.PolygonOffsetClampEXT.aliased(&ctx.PolygonOffsetClamp);
     ctx.ProgramParameteri.aliased(&ctx.ProgramParameteriARB);
     ctx.ProgramParameteri.aliased(&ctx.ProgramParameteriEXT);
     ctx.ProgramParameteriARB.aliased(&ctx.ProgramParameteri);
     ctx.ProgramParameteriARB.aliased(&ctx.ProgramParameteriEXT);
     ctx.ProgramParameteriEXT.aliased(&ctx.ProgramParameteri);
     ctx.ProgramParameteriEXT.aliased(&ctx.ProgramParameteriARB);
     ctx.ProgramUniform1f.aliased(&ctx.ProgramUniform1fEXT);
     ctx.ProgramUniform1fEXT.aliased(&ctx.ProgramUniform1f);
     ctx.ProgramUniform1fv.aliased(&ctx.ProgramUniform1fvEXT);
     ctx.ProgramUniform1fvEXT.aliased(&ctx.ProgramUniform1fv);
     ctx.ProgramUniform1i.aliased(&ctx.ProgramUniform1iEXT);
     ctx.ProgramUniform1iEXT.aliased(&ctx.ProgramUniform1i);
     ctx.ProgramUniform1iv.aliased(&ctx.ProgramUniform1ivEXT);
     ctx.ProgramUniform1ivEXT.aliased(&ctx.ProgramUniform1iv);
     ctx.ProgramUniform1ui.aliased(&ctx.ProgramUniform1uiEXT);
     ctx.ProgramUniform1uiEXT.aliased(&ctx.ProgramUniform1ui);
     ctx.ProgramUniform1uiv.aliased(&ctx.ProgramUniform1uivEXT);
     ctx.ProgramUniform1uivEXT.aliased(&ctx.ProgramUniform1uiv);
     ctx.ProgramUniform2f.aliased(&ctx.ProgramUniform2fEXT);
     ctx.ProgramUniform2fEXT.aliased(&ctx.ProgramUniform2f);
     ctx.ProgramUniform2fv.aliased(&ctx.ProgramUniform2fvEXT);
     ctx.ProgramUniform2fvEXT.aliased(&ctx.ProgramUniform2fv);
     ctx.ProgramUniform2i.aliased(&ctx.ProgramUniform2iEXT);
     ctx.ProgramUniform2iEXT.aliased(&ctx.ProgramUniform2i);
     ctx.ProgramUniform2iv.aliased(&ctx.ProgramUniform2ivEXT);
     ctx.ProgramUniform2ivEXT.aliased(&ctx.ProgramUniform2iv);
     ctx.ProgramUniform2ui.aliased(&ctx.ProgramUniform2uiEXT);
     ctx.ProgramUniform2uiEXT.aliased(&ctx.ProgramUniform2ui);
     ctx.ProgramUniform2uiv.aliased(&ctx.ProgramUniform2uivEXT);
     ctx.ProgramUniform2uivEXT.aliased(&ctx.ProgramUniform2uiv);
     ctx.ProgramUniform3f.aliased(&ctx.ProgramUniform3fEXT);
     ctx.ProgramUniform3fEXT.aliased(&ctx.ProgramUniform3f);
     ctx.ProgramUniform3fv.aliased(&ctx.ProgramUniform3fvEXT);
     ctx.ProgramUniform3fvEXT.aliased(&ctx.ProgramUniform3fv);
     ctx.ProgramUniform3i.aliased(&ctx.ProgramUniform3iEXT);
     ctx.ProgramUniform3iEXT.aliased(&ctx.ProgramUniform3i);
     ctx.ProgramUniform3iv.aliased(&ctx.ProgramUniform3ivEXT);
     ctx.ProgramUniform3ivEXT.aliased(&ctx.ProgramUniform3iv);
     ctx.ProgramUniform3ui.aliased(&ctx.ProgramUniform3uiEXT);
     ctx.ProgramUniform3uiEXT.aliased(&ctx.ProgramUniform3ui);
     ctx.ProgramUniform3uiv.aliased(&ctx.ProgramUniform3uivEXT);
     ctx.ProgramUniform3uivEXT.aliased(&ctx.ProgramUniform3uiv);
     ctx.ProgramUniform4f.aliased(&ctx.ProgramUniform4fEXT);
     ctx.ProgramUniform4fEXT.aliased(&ctx.ProgramUniform4f);
     ctx.ProgramUniform4fv.aliased(&ctx.ProgramUniform4fvEXT);
     ctx.ProgramUniform4fvEXT.aliased(&ctx.ProgramUniform4fv);
     ctx.ProgramUniform4i.aliased(&ctx.ProgramUniform4iEXT);
     ctx.ProgramUniform4iEXT.aliased(&ctx.ProgramUniform4i);
     ctx.ProgramUniform4iv.aliased(&ctx.ProgramUniform4ivEXT);
     ctx.ProgramUniform4ivEXT.aliased(&ctx.ProgramUniform4iv);
     ctx.ProgramUniform4ui.aliased(&ctx.ProgramUniform4uiEXT);
     ctx.ProgramUniform4uiEXT.aliased(&ctx.ProgramUniform4ui);
     ctx.ProgramUniform4uiv.aliased(&ctx.ProgramUniform4uivEXT);
     ctx.ProgramUniform4uivEXT.aliased(&ctx.ProgramUniform4uiv);
     ctx.ProgramUniformMatrix2fv.aliased(&ctx.ProgramUniformMatrix2fvEXT);
     ctx.ProgramUniformMatrix2fvEXT.aliased(&ctx.ProgramUniformMatrix2fv);
     ctx.ProgramUniformMatrix2x3fv.aliased(&ctx.ProgramUniformMatrix2x3fvEXT);
     ctx.ProgramUniformMatrix2x3fvEXT.aliased(&ctx.ProgramUniformMatrix2x3fv);
     ctx.ProgramUniformMatrix2x4fv.aliased(&ctx.ProgramUniformMatrix2x4fvEXT);
     ctx.ProgramUniformMatrix2x4fvEXT.aliased(&ctx.ProgramUniformMatrix2x4fv);
     ctx.ProgramUniformMatrix3fv.aliased(&ctx.ProgramUniformMatrix3fvEXT);
     ctx.ProgramUniformMatrix3fvEXT.aliased(&ctx.ProgramUniformMatrix3fv);
     ctx.ProgramUniformMatrix3x2fv.aliased(&ctx.ProgramUniformMatrix3x2fvEXT);
     ctx.ProgramUniformMatrix3x2fvEXT.aliased(&ctx.ProgramUniformMatrix3x2fv);
     ctx.ProgramUniformMatrix3x4fv.aliased(&ctx.ProgramUniformMatrix3x4fvEXT);
     ctx.ProgramUniformMatrix3x4fvEXT.aliased(&ctx.ProgramUniformMatrix3x4fv);
     ctx.ProgramUniformMatrix4fv.aliased(&ctx.ProgramUniformMatrix4fvEXT);
     ctx.ProgramUniformMatrix4fvEXT.aliased(&ctx.ProgramUniformMatrix4fv);
     ctx.ProgramUniformMatrix4x2fv.aliased(&ctx.ProgramUniformMatrix4x2fvEXT);
     ctx.ProgramUniformMatrix4x2fvEXT.aliased(&ctx.ProgramUniformMatrix4x2fv);
     ctx.ProgramUniformMatrix4x3fv.aliased(&ctx.ProgramUniformMatrix4x3fvEXT);
     ctx.ProgramUniformMatrix4x3fvEXT.aliased(&ctx.ProgramUniformMatrix4x3fv);
     ctx.ProvokingVertex.aliased(&ctx.ProvokingVertexEXT);
     ctx.ProvokingVertexEXT.aliased(&ctx.ProvokingVertex);
     ctx.ReadnPixels.aliased(&ctx.ReadnPixelsARB);
     ctx.ReadnPixelsARB.aliased(&ctx.ReadnPixels);
     ctx.RenderbufferStorage.aliased(&ctx.RenderbufferStorageEXT);
     ctx.RenderbufferStorageEXT.aliased(&ctx.RenderbufferStorage);
     ctx.RenderbufferStorageMultisample.aliased(&ctx.RenderbufferStorageMultisampleEXT);
     ctx.RenderbufferStorageMultisampleEXT.aliased(&ctx.RenderbufferStorageMultisample);
     ctx.ResumeTransformFeedback.aliased(&ctx.ResumeTransformFeedbackNV);
     ctx.ResumeTransformFeedbackNV.aliased(&ctx.ResumeTransformFeedback);
     ctx.SampleCoverage.aliased(&ctx.SampleCoverageARB);
     ctx.SampleCoverageARB.aliased(&ctx.SampleCoverage);
     ctx.SampleMaskEXT.aliased(&ctx.SampleMaskSGIS);
     ctx.SampleMaskSGIS.aliased(&ctx.SampleMaskEXT);
     ctx.SamplePatternEXT.aliased(&ctx.SamplePatternSGIS);
     ctx.SamplePatternSGIS.aliased(&ctx.SamplePatternEXT);
     ctx.ShaderSource.aliased(&ctx.ShaderSourceARB);
     ctx.ShaderSourceARB.aliased(&ctx.ShaderSource);
     ctx.SpecializeShader.aliased(&ctx.SpecializeShaderARB);
     ctx.SpecializeShaderARB.aliased(&ctx.SpecializeShader);
     ctx.StencilOpSeparate.aliased(&ctx.StencilOpSeparateATI);
     ctx.StencilOpSeparateATI.aliased(&ctx.StencilOpSeparate);
     ctx.TexBuffer.aliased(&ctx.TexBufferARB);
     ctx.TexBuffer.aliased(&ctx.TexBufferEXT);
     ctx.TexBufferARB.aliased(&ctx.TexBuffer);
     ctx.TexBufferARB.aliased(&ctx.TexBufferEXT);
     ctx.TexBufferEXT.aliased(&ctx.TexBuffer);
     ctx.TexBufferEXT.aliased(&ctx.TexBufferARB);
     ctx.TexImage3D.aliased(&ctx.TexImage3DEXT);
     ctx.TexImage3DEXT.aliased(&ctx.TexImage3D);
     ctx.TexParameterIiv.aliased(&ctx.TexParameterIivEXT);
     ctx.TexParameterIivEXT.aliased(&ctx.TexParameterIiv);
     ctx.TexParameterIuiv.aliased(&ctx.TexParameterIuivEXT);
     ctx.TexParameterIuivEXT.aliased(&ctx.TexParameterIuiv);
     ctx.TexStorage1D.aliased(&ctx.TexStorage1DEXT);
     ctx.TexStorage1DEXT.aliased(&ctx.TexStorage1D);
     ctx.TexStorage2D.aliased(&ctx.TexStorage2DEXT);
     ctx.TexStorage2DEXT.aliased(&ctx.TexStorage2D);
     ctx.TexStorage3D.aliased(&ctx.TexStorage3DEXT);
     ctx.TexStorage3DEXT.aliased(&ctx.TexStorage3D);
     ctx.TexSubImage1D.aliased(&ctx.TexSubImage1DEXT);
     ctx.TexSubImage1DEXT.aliased(&ctx.TexSubImage1D);
     ctx.TexSubImage2D.aliased(&ctx.TexSubImage2DEXT);
     ctx.TexSubImage2DEXT.aliased(&ctx.TexSubImage2D);
     ctx.TexSubImage3D.aliased(&ctx.TexSubImage3DEXT);
     ctx.TexSubImage3DEXT.aliased(&ctx.TexSubImage3D);
     ctx.TransformFeedbackVaryings.aliased(&ctx.TransformFeedbackVaryingsEXT);
     ctx.TransformFeedbackVaryingsEXT.aliased(&ctx.TransformFeedbackVaryings);
     ctx.Uniform1f.aliased(&ctx.Uniform1fARB);
     ctx.Uniform1fARB.aliased(&ctx.Uniform1f);
     ctx.Uniform1fv.aliased(&ctx.Uniform1fvARB);
     ctx.Uniform1fvARB.aliased(&ctx.Uniform1fv);
     ctx.Uniform1i.aliased(&ctx.Uniform1iARB);
     ctx.Uniform1iARB.aliased(&ctx.Uniform1i);
     ctx.Uniform1iv.aliased(&ctx.Uniform1ivARB);
     ctx.Uniform1ivARB.aliased(&ctx.Uniform1iv);
     ctx.Uniform1ui.aliased(&ctx.Uniform1uiEXT);
     ctx.Uniform1uiEXT.aliased(&ctx.Uniform1ui);
     ctx.Uniform1uiv.aliased(&ctx.Uniform1uivEXT);
     ctx.Uniform1uivEXT.aliased(&ctx.Uniform1uiv);
     ctx.Uniform2f.aliased(&ctx.Uniform2fARB);
     ctx.Uniform2fARB.aliased(&ctx.Uniform2f);
     ctx.Uniform2fv.aliased(&ctx.Uniform2fvARB);
     ctx.Uniform2fvARB.aliased(&ctx.Uniform2fv);
     ctx.Uniform2i.aliased(&ctx.Uniform2iARB);
     ctx.Uniform2iARB.aliased(&ctx.Uniform2i);
     ctx.Uniform2iv.aliased(&ctx.Uniform2ivARB);
     ctx.Uniform2ivARB.aliased(&ctx.Uniform2iv);
     ctx.Uniform2ui.aliased(&ctx.Uniform2uiEXT);
     ctx.Uniform2uiEXT.aliased(&ctx.Uniform2ui);
     ctx.Uniform2uiv.aliased(&ctx.Uniform2uivEXT);
     ctx.Uniform2uivEXT.aliased(&ctx.Uniform2uiv);
     ctx.Uniform3f.aliased(&ctx.Uniform3fARB);
     ctx.Uniform3fARB.aliased(&ctx.Uniform3f);
     ctx.Uniform3fv.aliased(&ctx.Uniform3fvARB);
     ctx.Uniform3fvARB.aliased(&ctx.Uniform3fv);
     ctx.Uniform3i.aliased(&ctx.Uniform3iARB);
     ctx.Uniform3iARB.aliased(&ctx.Uniform3i);
     ctx.Uniform3iv.aliased(&ctx.Uniform3ivARB);
     ctx.Uniform3ivARB.aliased(&ctx.Uniform3iv);
     ctx.Uniform3ui.aliased(&ctx.Uniform3uiEXT);
     ctx.Uniform3uiEXT.aliased(&ctx.Uniform3ui);
     ctx.Uniform3uiv.aliased(&ctx.Uniform3uivEXT);
     ctx.Uniform3uivEXT.aliased(&ctx.Uniform3uiv);
     ctx.Uniform4f.aliased(&ctx.Uniform4fARB);
     ctx.Uniform4fARB.aliased(&ctx.Uniform4f);
     ctx.Uniform4fv.aliased(&ctx.Uniform4fvARB);
     ctx.Uniform4fvARB.aliased(&ctx.Uniform4fv);
     ctx.Uniform4i.aliased(&ctx.Uniform4iARB);
     ctx.Uniform4iARB.aliased(&ctx.Uniform4i);
     ctx.Uniform4iv.aliased(&ctx.Uniform4ivARB);
     ctx.Uniform4ivARB.aliased(&ctx.Uniform4iv);
     ctx.Uniform4ui.aliased(&ctx.Uniform4uiEXT);
     ctx.Uniform4uiEXT.aliased(&ctx.Uniform4ui);
     ctx.Uniform4uiv.aliased(&ctx.Uniform4uivEXT);
     ctx.Uniform4uivEXT.aliased(&ctx.Uniform4uiv);
     ctx.UniformMatrix2fv.aliased(&ctx.UniformMatrix2fvARB);
     ctx.UniformMatrix2fvARB.aliased(&ctx.UniformMatrix2fv);
     ctx.UniformMatrix3fv.aliased(&ctx.UniformMatrix3fvARB);
     ctx.UniformMatrix3fvARB.aliased(&ctx.UniformMatrix3fv);
     ctx.UniformMatrix4fv.aliased(&ctx.UniformMatrix4fvARB);
     ctx.UniformMatrix4fvARB.aliased(&ctx.UniformMatrix4fv);
     ctx.UnmapBuffer.aliased(&ctx.UnmapBufferARB);
     ctx.UnmapBufferARB.aliased(&ctx.UnmapBuffer);
     ctx.UseProgram.aliased(&ctx.UseProgramObjectARB);
     ctx.UseProgramObjectARB.aliased(&ctx.UseProgram);
     ctx.ValidateProgram.aliased(&ctx.ValidateProgramARB);
     ctx.ValidateProgramARB.aliased(&ctx.ValidateProgram);
     ctx.VertexAttrib1d.aliased(&ctx.VertexAttrib1dARB);
     ctx.VertexAttrib1d.aliased(&ctx.VertexAttrib1dNV);
     ctx.VertexAttrib1dARB.aliased(&ctx.VertexAttrib1d);
     ctx.VertexAttrib1dARB.aliased(&ctx.VertexAttrib1dNV);
     ctx.VertexAttrib1dNV.aliased(&ctx.VertexAttrib1d);
     ctx.VertexAttrib1dNV.aliased(&ctx.VertexAttrib1dARB);
     ctx.VertexAttrib1dv.aliased(&ctx.VertexAttrib1dvARB);
     ctx.VertexAttrib1dv.aliased(&ctx.VertexAttrib1dvNV);
     ctx.VertexAttrib1dvARB.aliased(&ctx.VertexAttrib1dv);
     ctx.VertexAttrib1dvARB.aliased(&ctx.VertexAttrib1dvNV);
     ctx.VertexAttrib1dvNV.aliased(&ctx.VertexAttrib1dv);
     ctx.VertexAttrib1dvNV.aliased(&ctx.VertexAttrib1dvARB);
     ctx.VertexAttrib1f.aliased(&ctx.VertexAttrib1fARB);
     ctx.VertexAttrib1f.aliased(&ctx.VertexAttrib1fNV);
     ctx.VertexAttrib1fARB.aliased(&ctx.VertexAttrib1f);
     ctx.VertexAttrib1fARB.aliased(&ctx.VertexAttrib1fNV);
     ctx.VertexAttrib1fNV.aliased(&ctx.VertexAttrib1f);
     ctx.VertexAttrib1fNV.aliased(&ctx.VertexAttrib1fARB);
     ctx.VertexAttrib1fv.aliased(&ctx.VertexAttrib1fvARB);
     ctx.VertexAttrib1fv.aliased(&ctx.VertexAttrib1fvNV);
     ctx.VertexAttrib1fvARB.aliased(&ctx.VertexAttrib1fv);
     ctx.VertexAttrib1fvARB.aliased(&ctx.VertexAttrib1fvNV);
     ctx.VertexAttrib1fvNV.aliased(&ctx.VertexAttrib1fv);
     ctx.VertexAttrib1fvNV.aliased(&ctx.VertexAttrib1fvARB);
     ctx.VertexAttrib1s.aliased(&ctx.VertexAttrib1sARB);
     ctx.VertexAttrib1s.aliased(&ctx.VertexAttrib1sNV);
     ctx.VertexAttrib1sARB.aliased(&ctx.VertexAttrib1s);
     ctx.VertexAttrib1sARB.aliased(&ctx.VertexAttrib1sNV);
     ctx.VertexAttrib1sNV.aliased(&ctx.VertexAttrib1s);
     ctx.VertexAttrib1sNV.aliased(&ctx.VertexAttrib1sARB);
     ctx.VertexAttrib1sv.aliased(&ctx.VertexAttrib1svARB);
     ctx.VertexAttrib1sv.aliased(&ctx.VertexAttrib1svNV);
     ctx.VertexAttrib1svARB.aliased(&ctx.VertexAttrib1sv);
     ctx.VertexAttrib1svARB.aliased(&ctx.VertexAttrib1svNV);
     ctx.VertexAttrib1svNV.aliased(&ctx.VertexAttrib1sv);
     ctx.VertexAttrib1svNV.aliased(&ctx.VertexAttrib1svARB);
     ctx.VertexAttrib2d.aliased(&ctx.VertexAttrib2dARB);
     ctx.VertexAttrib2d.aliased(&ctx.VertexAttrib2dNV);
     ctx.VertexAttrib2dARB.aliased(&ctx.VertexAttrib2d);
     ctx.VertexAttrib2dARB.aliased(&ctx.VertexAttrib2dNV);
     ctx.VertexAttrib2dNV.aliased(&ctx.VertexAttrib2d);
     ctx.VertexAttrib2dNV.aliased(&ctx.VertexAttrib2dARB);
     ctx.VertexAttrib2dv.aliased(&ctx.VertexAttrib2dvARB);
     ctx.VertexAttrib2dv.aliased(&ctx.VertexAttrib2dvNV);
     ctx.VertexAttrib2dvARB.aliased(&ctx.VertexAttrib2dv);
     ctx.VertexAttrib2dvARB.aliased(&ctx.VertexAttrib2dvNV);
     ctx.VertexAttrib2dvNV.aliased(&ctx.VertexAttrib2dv);
     ctx.VertexAttrib2dvNV.aliased(&ctx.VertexAttrib2dvARB);
     ctx.VertexAttrib2f.aliased(&ctx.VertexAttrib2fARB);
     ctx.VertexAttrib2f.aliased(&ctx.VertexAttrib2fNV);
     ctx.VertexAttrib2fARB.aliased(&ctx.VertexAttrib2f);
     ctx.VertexAttrib2fARB.aliased(&ctx.VertexAttrib2fNV);
     ctx.VertexAttrib2fNV.aliased(&ctx.VertexAttrib2f);
     ctx.VertexAttrib2fNV.aliased(&ctx.VertexAttrib2fARB);
     ctx.VertexAttrib2fv.aliased(&ctx.VertexAttrib2fvARB);
     ctx.VertexAttrib2fv.aliased(&ctx.VertexAttrib2fvNV);
     ctx.VertexAttrib2fvARB.aliased(&ctx.VertexAttrib2fv);
     ctx.VertexAttrib2fvARB.aliased(&ctx.VertexAttrib2fvNV);
     ctx.VertexAttrib2fvNV.aliased(&ctx.VertexAttrib2fv);
     ctx.VertexAttrib2fvNV.aliased(&ctx.VertexAttrib2fvARB);
     ctx.VertexAttrib2s.aliased(&ctx.VertexAttrib2sARB);
     ctx.VertexAttrib2s.aliased(&ctx.VertexAttrib2sNV);
     ctx.VertexAttrib2sARB.aliased(&ctx.VertexAttrib2s);
     ctx.VertexAttrib2sARB.aliased(&ctx.VertexAttrib2sNV);
     ctx.VertexAttrib2sNV.aliased(&ctx.VertexAttrib2s);
     ctx.VertexAttrib2sNV.aliased(&ctx.VertexAttrib2sARB);
     ctx.VertexAttrib2sv.aliased(&ctx.VertexAttrib2svARB);
     ctx.VertexAttrib2sv.aliased(&ctx.VertexAttrib2svNV);
     ctx.VertexAttrib2svARB.aliased(&ctx.VertexAttrib2sv);
     ctx.VertexAttrib2svARB.aliased(&ctx.VertexAttrib2svNV);
     ctx.VertexAttrib2svNV.aliased(&ctx.VertexAttrib2sv);
     ctx.VertexAttrib2svNV.aliased(&ctx.VertexAttrib2svARB);
     ctx.VertexAttrib3d.aliased(&ctx.VertexAttrib3dARB);
     ctx.VertexAttrib3d.aliased(&ctx.VertexAttrib3dNV);
     ctx.VertexAttrib3dARB.aliased(&ctx.VertexAttrib3d);
     ctx.VertexAttrib3dARB.aliased(&ctx.VertexAttrib3dNV);
     ctx.VertexAttrib3dNV.aliased(&ctx.VertexAttrib3d);
     ctx.VertexAttrib3dNV.aliased(&ctx.VertexAttrib3dARB);
     ctx.VertexAttrib3dv.aliased(&ctx.VertexAttrib3dvARB);
     ctx.VertexAttrib3dv.aliased(&ctx.VertexAttrib3dvNV);
     ctx.VertexAttrib3dvARB.aliased(&ctx.VertexAttrib3dv);
     ctx.VertexAttrib3dvARB.aliased(&ctx.VertexAttrib3dvNV);
     ctx.VertexAttrib3dvNV.aliased(&ctx.VertexAttrib3dv);
     ctx.VertexAttrib3dvNV.aliased(&ctx.VertexAttrib3dvARB);
     ctx.VertexAttrib3f.aliased(&ctx.VertexAttrib3fARB);
     ctx.VertexAttrib3f.aliased(&ctx.VertexAttrib3fNV);
     ctx.VertexAttrib3fARB.aliased(&ctx.VertexAttrib3f);
     ctx.VertexAttrib3fARB.aliased(&ctx.VertexAttrib3fNV);
     ctx.VertexAttrib3fNV.aliased(&ctx.VertexAttrib3f);
     ctx.VertexAttrib3fNV.aliased(&ctx.VertexAttrib3fARB);
     ctx.VertexAttrib3fv.aliased(&ctx.VertexAttrib3fvARB);
     ctx.VertexAttrib3fv.aliased(&ctx.VertexAttrib3fvNV);
     ctx.VertexAttrib3fvARB.aliased(&ctx.VertexAttrib3fv);
     ctx.VertexAttrib3fvARB.aliased(&ctx.VertexAttrib3fvNV);
     ctx.VertexAttrib3fvNV.aliased(&ctx.VertexAttrib3fv);
     ctx.VertexAttrib3fvNV.aliased(&ctx.VertexAttrib3fvARB);
     ctx.VertexAttrib3s.aliased(&ctx.VertexAttrib3sARB);
     ctx.VertexAttrib3s.aliased(&ctx.VertexAttrib3sNV);
     ctx.VertexAttrib3sARB.aliased(&ctx.VertexAttrib3s);
     ctx.VertexAttrib3sARB.aliased(&ctx.VertexAttrib3sNV);
     ctx.VertexAttrib3sNV.aliased(&ctx.VertexAttrib3s);
     ctx.VertexAttrib3sNV.aliased(&ctx.VertexAttrib3sARB);
     ctx.VertexAttrib3sv.aliased(&ctx.VertexAttrib3svARB);
     ctx.VertexAttrib3sv.aliased(&ctx.VertexAttrib3svNV);
     ctx.VertexAttrib3svARB.aliased(&ctx.VertexAttrib3sv);
     ctx.VertexAttrib3svARB.aliased(&ctx.VertexAttrib3svNV);
     ctx.VertexAttrib3svNV.aliased(&ctx.VertexAttrib3sv);
     ctx.VertexAttrib3svNV.aliased(&ctx.VertexAttrib3svARB);
     ctx.VertexAttrib4bv.aliased(&ctx.VertexAttrib4bvARB);
     ctx.VertexAttrib4bvARB.aliased(&ctx.VertexAttrib4bv);
     ctx.VertexAttrib4d.aliased(&ctx.VertexAttrib4dARB);
     ctx.VertexAttrib4d.aliased(&ctx.VertexAttrib4dNV);
     ctx.VertexAttrib4dARB.aliased(&ctx.VertexAttrib4d);
     ctx.VertexAttrib4dARB.aliased(&ctx.VertexAttrib4dNV);
     ctx.VertexAttrib4dNV.aliased(&ctx.VertexAttrib4d);
     ctx.VertexAttrib4dNV.aliased(&ctx.VertexAttrib4dARB);
     ctx.VertexAttrib4dv.aliased(&ctx.VertexAttrib4dvARB);
     ctx.VertexAttrib4dv.aliased(&ctx.VertexAttrib4dvNV);
     ctx.VertexAttrib4dvARB.aliased(&ctx.VertexAttrib4dv);
     ctx.VertexAttrib4dvARB.aliased(&ctx.VertexAttrib4dvNV);
     ctx.VertexAttrib4dvNV.aliased(&ctx.VertexAttrib4dv);
     ctx.VertexAttrib4dvNV.aliased(&ctx.VertexAttrib4dvARB);
     ctx.VertexAttrib4f.aliased(&ctx.VertexAttrib4fARB);
     ctx.VertexAttrib4f.aliased(&ctx.VertexAttrib4fNV);
     ctx.VertexAttrib4fARB.aliased(&ctx.VertexAttrib4f);
     ctx.VertexAttrib4fARB.aliased(&ctx.VertexAttrib4fNV);
     ctx.VertexAttrib4fNV.aliased(&ctx.VertexAttrib4f);
     ctx.VertexAttrib4fNV.aliased(&ctx.VertexAttrib4fARB);
     ctx.VertexAttrib4fv.aliased(&ctx.VertexAttrib4fvARB);
     ctx.VertexAttrib4fv.aliased(&ctx.VertexAttrib4fvNV);
     ctx.VertexAttrib4fvARB.aliased(&ctx.VertexAttrib4fv);
     ctx.VertexAttrib4fvARB.aliased(&ctx.VertexAttrib4fvNV);
     ctx.VertexAttrib4fvNV.aliased(&ctx.VertexAttrib4fv);
     ctx.VertexAttrib4fvNV.aliased(&ctx.VertexAttrib4fvARB);
     ctx.VertexAttrib4iv.aliased(&ctx.VertexAttrib4ivARB);
     ctx.VertexAttrib4ivARB.aliased(&ctx.VertexAttrib4iv);
     ctx.VertexAttrib4Nbv.aliased(&ctx.VertexAttrib4NbvARB);
     ctx.VertexAttrib4NbvARB.aliased(&ctx.VertexAttrib4Nbv);
     ctx.VertexAttrib4Niv.aliased(&ctx.VertexAttrib4NivARB);
     ctx.VertexAttrib4NivARB.aliased(&ctx.VertexAttrib4Niv);
     ctx.VertexAttrib4Nsv.aliased(&ctx.VertexAttrib4NsvARB);
     ctx.VertexAttrib4NsvARB.aliased(&ctx.VertexAttrib4Nsv);
     ctx.VertexAttrib4Nub.aliased(&ctx.VertexAttrib4NubARB);
     ctx.VertexAttrib4Nub.aliased(&ctx.VertexAttrib4ubNV);
     ctx.VertexAttrib4NubARB.aliased(&ctx.VertexAttrib4Nub);
     ctx.VertexAttrib4NubARB.aliased(&ctx.VertexAttrib4ubNV);
     ctx.VertexAttrib4Nubv.aliased(&ctx.VertexAttrib4NubvARB);
     ctx.VertexAttrib4Nubv.aliased(&ctx.VertexAttrib4ubvNV);
     ctx.VertexAttrib4NubvARB.aliased(&ctx.VertexAttrib4Nubv);
     ctx.VertexAttrib4NubvARB.aliased(&ctx.VertexAttrib4ubvNV);
     ctx.VertexAttrib4Nuiv.aliased(&ctx.VertexAttrib4NuivARB);
     ctx.VertexAttrib4NuivARB.aliased(&ctx.VertexAttrib4Nuiv);
     ctx.VertexAttrib4Nusv.aliased(&ctx.VertexAttrib4NusvARB);
     ctx.VertexAttrib4NusvARB.aliased(&ctx.VertexAttrib4Nusv);
     ctx.VertexAttrib4s.aliased(&ctx.VertexAttrib4sARB);
     ctx.VertexAttrib4s.aliased(&ctx.VertexAttrib4sNV);
     ctx.VertexAttrib4sARB.aliased(&ctx.VertexAttrib4s);
     ctx.VertexAttrib4sARB.aliased(&ctx.VertexAttrib4sNV);
     ctx.VertexAttrib4sNV.aliased(&ctx.VertexAttrib4s);
     ctx.VertexAttrib4sNV.aliased(&ctx.VertexAttrib4sARB);
     ctx.VertexAttrib4sv.aliased(&ctx.VertexAttrib4svARB);
     ctx.VertexAttrib4sv.aliased(&ctx.VertexAttrib4svNV);
     ctx.VertexAttrib4svARB.aliased(&ctx.VertexAttrib4sv);
     ctx.VertexAttrib4svARB.aliased(&ctx.VertexAttrib4svNV);
     ctx.VertexAttrib4svNV.aliased(&ctx.VertexAttrib4sv);
     ctx.VertexAttrib4svNV.aliased(&ctx.VertexAttrib4svARB);
     ctx.VertexAttrib4ubNV.aliased(&ctx.VertexAttrib4Nub);
     ctx.VertexAttrib4ubNV.aliased(&ctx.VertexAttrib4NubARB);
     ctx.VertexAttrib4ubv.aliased(&ctx.VertexAttrib4ubvARB);
     ctx.VertexAttrib4ubvARB.aliased(&ctx.VertexAttrib4ubv);
     ctx.VertexAttrib4ubvNV.aliased(&ctx.VertexAttrib4Nubv);
     ctx.VertexAttrib4ubvNV.aliased(&ctx.VertexAttrib4NubvARB);
     ctx.VertexAttrib4uiv.aliased(&ctx.VertexAttrib4uivARB);
     ctx.VertexAttrib4uivARB.aliased(&ctx.VertexAttrib4uiv);
     ctx.VertexAttrib4usv.aliased(&ctx.VertexAttrib4usvARB);
     ctx.VertexAttrib4usvARB.aliased(&ctx.VertexAttrib4usv);
     ctx.VertexAttribDivisor.aliased(&ctx.VertexAttribDivisorARB);
     ctx.VertexAttribDivisorARB.aliased(&ctx.VertexAttribDivisor);
     ctx.VertexAttribI1i.aliased(&ctx.VertexAttribI1iEXT);
     ctx.VertexAttribI1iEXT.aliased(&ctx.VertexAttribI1i);
     ctx.VertexAttribI1iv.aliased(&ctx.VertexAttribI1ivEXT);
     ctx.VertexAttribI1ivEXT.aliased(&ctx.VertexAttribI1iv);
     ctx.VertexAttribI1ui.aliased(&ctx.VertexAttribI1uiEXT);
     ctx.VertexAttribI1uiEXT.aliased(&ctx.VertexAttribI1ui);
     ctx.VertexAttribI1uiv.aliased(&ctx.VertexAttribI1uivEXT);
     ctx.VertexAttribI1uivEXT.aliased(&ctx.VertexAttribI1uiv);
     ctx.VertexAttribI2i.aliased(&ctx.VertexAttribI2iEXT);
     ctx.VertexAttribI2iEXT.aliased(&ctx.VertexAttribI2i);
     ctx.VertexAttribI2iv.aliased(&ctx.VertexAttribI2ivEXT);
     ctx.VertexAttribI2ivEXT.aliased(&ctx.VertexAttribI2iv);
     ctx.VertexAttribI2ui.aliased(&ctx.VertexAttribI2uiEXT);
     ctx.VertexAttribI2uiEXT.aliased(&ctx.VertexAttribI2ui);
     ctx.VertexAttribI2uiv.aliased(&ctx.VertexAttribI2uivEXT);
     ctx.VertexAttribI2uivEXT.aliased(&ctx.VertexAttribI2uiv);
     ctx.VertexAttribI3i.aliased(&ctx.VertexAttribI3iEXT);
     ctx.VertexAttribI3iEXT.aliased(&ctx.VertexAttribI3i);
     ctx.VertexAttribI3iv.aliased(&ctx.VertexAttribI3ivEXT);
     ctx.VertexAttribI3ivEXT.aliased(&ctx.VertexAttribI3iv);
     ctx.VertexAttribI3ui.aliased(&ctx.VertexAttribI3uiEXT);
     ctx.VertexAttribI3uiEXT.aliased(&ctx.VertexAttribI3ui);
     ctx.VertexAttribI3uiv.aliased(&ctx.VertexAttribI3uivEXT);
     ctx.VertexAttribI3uivEXT.aliased(&ctx.VertexAttribI3uiv);
     ctx.VertexAttribI4bv.aliased(&ctx.VertexAttribI4bvEXT);
     ctx.VertexAttribI4bvEXT.aliased(&ctx.VertexAttribI4bv);
     ctx.VertexAttribI4i.aliased(&ctx.VertexAttribI4iEXT);
     ctx.VertexAttribI4iEXT.aliased(&ctx.VertexAttribI4i);
     ctx.VertexAttribI4iv.aliased(&ctx.VertexAttribI4ivEXT);
     ctx.VertexAttribI4ivEXT.aliased(&ctx.VertexAttribI4iv);
     ctx.VertexAttribI4sv.aliased(&ctx.VertexAttribI4svEXT);
     ctx.VertexAttribI4svEXT.aliased(&ctx.VertexAttribI4sv);
     ctx.VertexAttribI4ubv.aliased(&ctx.VertexAttribI4ubvEXT);
     ctx.VertexAttribI4ubvEXT.aliased(&ctx.VertexAttribI4ubv);
     ctx.VertexAttribI4ui.aliased(&ctx.VertexAttribI4uiEXT);
     ctx.VertexAttribI4uiEXT.aliased(&ctx.VertexAttribI4ui);
     ctx.VertexAttribI4uiv.aliased(&ctx.VertexAttribI4uivEXT);
     ctx.VertexAttribI4uivEXT.aliased(&ctx.VertexAttribI4uiv);
     ctx.VertexAttribI4usv.aliased(&ctx.VertexAttribI4usvEXT);
     ctx.VertexAttribI4usvEXT.aliased(&ctx.VertexAttribI4usv);
     ctx.VertexAttribIPointer.aliased(&ctx.VertexAttribIPointerEXT);
     ctx.VertexAttribIPointerEXT.aliased(&ctx.VertexAttribIPointer);
     ctx.VertexAttribL1d.aliased(&ctx.VertexAttribL1dEXT);
     ctx.VertexAttribL1dEXT.aliased(&ctx.VertexAttribL1d);
     ctx.VertexAttribL1dv.aliased(&ctx.VertexAttribL1dvEXT);
     ctx.VertexAttribL1dvEXT.aliased(&ctx.VertexAttribL1dv);
     ctx.VertexAttribL2d.aliased(&ctx.VertexAttribL2dEXT);
     ctx.VertexAttribL2dEXT.aliased(&ctx.VertexAttribL2d);
     ctx.VertexAttribL2dv.aliased(&ctx.VertexAttribL2dvEXT);
     ctx.VertexAttribL2dvEXT.aliased(&ctx.VertexAttribL2dv);
     ctx.VertexAttribL3d.aliased(&ctx.VertexAttribL3dEXT);
     ctx.VertexAttribL3dEXT.aliased(&ctx.VertexAttribL3d);
     ctx.VertexAttribL3dv.aliased(&ctx.VertexAttribL3dvEXT);
     ctx.VertexAttribL3dvEXT.aliased(&ctx.VertexAttribL3dv);
     ctx.VertexAttribL4d.aliased(&ctx.VertexAttribL4dEXT);
     ctx.VertexAttribL4dEXT.aliased(&ctx.VertexAttribL4d);
     ctx.VertexAttribL4dv.aliased(&ctx.VertexAttribL4dvEXT);
     ctx.VertexAttribL4dvEXT.aliased(&ctx.VertexAttribL4dv);
     ctx.VertexAttribLPointer.aliased(&ctx.VertexAttribLPointerEXT);
     ctx.VertexAttribLPointerEXT.aliased(&ctx.VertexAttribLPointer);
     ctx.VertexAttribPointer.aliased(&ctx.VertexAttribPointerARB);
     ctx.VertexAttribPointerARB.aliased(&ctx.VertexAttribPointer);

     ctx
}

