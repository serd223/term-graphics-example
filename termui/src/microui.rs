use std::{ffi::CString, mem::offset_of};

pub type Rect = ffi::Rect;
pub fn rect(x: i32, y: i32, w: i32, h: i32) -> Rect {
    Rect { x, y, w, h }
}
pub type Color = ffi::Color;
pub type Vec2 = ffi::Vec2;
pub type Id = i32;

#[derive(Clone, Debug)]
pub enum Command {
    Jump {},
    Clip {
        rect: Rect,
    },
    Rect {
        rect: Rect,
        color: Color,
    },
    Text {
        // font: Font,
        post: Vec2,
        color: Color,
        str: String,
    },
    Icon {
        rect: Rect,
        id: Id,
        color: Color,
    },
}

pub struct Context {
    inner: *mut ffi::Context,
    cmd_ptr: *const ffi::Command,
}
impl Context {
    pub fn new() -> Self {
        let res = Self {
            inner: unsafe { ffi::mu_alloc() },
            cmd_ptr: core::ptr::null(),
        };
        unsafe {
            ffi::mu_init(res.inner);
        }
        ffi::set_ctx_callbacks(res.inner);

        res
    }

    pub fn begin(&mut self) {
        unsafe {
            ffi::mu_begin(self.inner);
        }
    }

    pub fn end(&mut self) {
        unsafe {
            ffi::mu_end(self.inner);
        }
    }

    pub fn begin_window(&mut self, title: &str, rect: Rect) -> bool {
        let title = CString::new(title).expect("Could not allocate CString from title");
        ffi::mu_begin_window(self.inner, title.as_ptr(), rect) != 0
    }

    pub fn end_window(&mut self) {
        unsafe {
            ffi::mu_end_window(self.inner);
        }
    }

    pub fn label(&mut self, label: &str) {
        let label = CString::new(label).expect("Could not allocate CString from label");
        unsafe {
            ffi::mu_label(self.inner, label.as_ptr());
        }
    }

    pub fn pop_command(&mut self) -> Option<Command> {
        unsafe {
            if ffi::mu_next_command(self.inner, &mut self.cmd_ptr) == 0 {
                self.cmd_ptr = core::ptr::null();
                None
            } else {
                let cmd = core::ptr::read_unaligned(self.cmd_ptr);
                match cmd.ty {
                    ffi::CommandType::Jump => Some(Command::Jump {}),
                    #[allow(unused)]
                    ffi::CommandType::Clip => Some(Command::Clip { rect: todo!() }),
                    ffi::CommandType::Rect => {
                        let ffi::RectCommand { rect, color, .. } = cmd.rect;
                        Some(Command::Rect { rect, color })
                    }
                    ffi::CommandType::Text => {
                        let ffi::TextCommand { post, color, .. } = cmd.text;
                        let offset = offset_of!(ffi::TextCommand, str);
                        let mut cstr_ptr = (self.cmd_ptr as *const i8).add(offset);
                        let mut str = String::new();
                        while *cstr_ptr != 0 {
                            str.push(*cstr_ptr as u8 as char);
                            cstr_ptr = cstr_ptr.add(1);
                        }
                        Some(Command::Text { post, color, str })
                    }
                    ffi::CommandType::Icon => {
                        let ffi::IconCommand {
                            id, rect, color, ..
                        } = cmd.icon;
                        Some(Command::Icon { id, rect, color })
                    }
                    ffi::CommandType::Max => todo!(),
                }
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::mu_free(self.inner);
        }
    }
}

#[allow(unused)]
mod ffi {
    use core::ffi::c_char;
    use core::ffi::c_int as int;
    use core::ffi::c_uchar;
    use core::ffi::c_uint as unsigned;
    use core::ffi::c_void as void;

    pub type Context = void;
    pub type Id = unsigned;

    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub enum CommandType {
        Jump = 1,
        Clip,
        Rect,
        Text,
        Icon,
        Max,
    }

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub union Command {
        pub ty: CommandType,
        pub base: BaseCommand,
        pub jump: JumpCommand,
        pub clip: ClipCommand,
        pub rect: RectCommand,
        pub text: TextCommand,
        pub icon: IconCommand,
    }

    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct BaseCommand {
        pub _type: CommandType,
        pub size: int,
    }
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct JumpCommand {
        pub base: BaseCommand,
        pub dst: *const void,
    }
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct ClipCommand {
        pub base: BaseCommand,
        pub rect: Rect,
    }
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct RectCommand {
        pub base: BaseCommand,
        pub rect: Rect,
        pub color: Color,
    }
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct TextCommand {
        pub base: BaseCommand,
        pub font: _Font,
        pub post: Vec2,
        pub color: Color,
        pub str: c_char,
    }
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct IconCommand {
        pub base: BaseCommand,
        pub rect: Rect,
        pub id: int,
        pub color: Color,
    }

    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct Rect {
        pub x: int,
        pub y: int,
        pub w: int,
        pub h: int,
    }
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct Vec2 {
        pub x: int,
        pub y: int,
    }
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct Color {
        pub r: c_uchar,
        pub g: c_uchar,
        pub b: c_uchar,
        pub a: c_uchar,
    }
    pub type Container = void;
    pub type _Font = *const void;

    extern "C" fn text_width(font: _Font, mut cstr: *const c_char, len: int) -> int {
        _ = font;
        unsafe {
            let mut res = 0;
            let mut i = 0;
            while i < len && *cstr != 0 {
                let c = *cstr as u8 as char; // whatever
                // TODO: Doesn't actually measure text size
                res += 1;
                cstr = cstr.add(1);
                i += 1;
            }
            res
        }
    }
    extern "C" fn text_height(font: _Font) -> int {
        _ = font;
        // TODO: Hardcode font size to 18
        18
    }
    unsafe extern "C" {
        pub fn mu_next_command(ctx: *mut Context, cmd: *mut *const Command) -> int;
        pub fn mu_begin_window_ex(
            ctx: *mut Context,
            title: *const c_char,
            rect: Rect,
            opt: int,
        ) -> int;
        pub fn mu_end_window(ctx: *mut Context) -> void;
        pub fn mu_label(ctx: *mut Context, text: *const c_char) -> void;
        fn mu_set_text_width(ctx: *mut Context, f: *const void) -> void;
        fn mu_set_text_height(ctx: *mut Context, f: *const void) -> void;
        pub fn mu_alloc() -> *mut Context;
        pub fn mu_free(ctx: *mut Context) -> void;
        pub fn mu_init(ctx: *mut Context) -> void;
        pub fn mu_begin(ctx: *mut Context) -> void;
        pub fn mu_end(ctx: *mut Context) -> void;
        pub fn mu_set_focus(ctx: *mut Context, id: Id) -> void;
        pub fn mu_get_id(ctx: *mut Context, data: *const void, size: int) -> Id;
        pub fn mu_push_id(ctx: *mut Context, data: *const void, size: int) -> void;
        pub fn mu_pop_id(ctx: *mut Context) -> void;
        pub fn mu_push_clip_rect(ctx: *mut Context, rect: Rect) -> void;
        pub fn mu_pop_clip_rect(ctx: *mut Context) -> void;
        pub fn mu_get_clip_rect(ctx: *mut Context) -> Rect;
        pub fn mu_check_clip(ctx: *mut Context, r: Rect) -> int;
        pub fn mu_get_current_container(ctx: *mut Context) -> *mut Container;
        pub fn mu_get_container(ctx: *mut Context, name: *const c_char) -> *mut Container;
        pub fn mu_bring_to_front(ctx: *mut Context, cnt: *mut Container) -> void;
    }
    pub fn set_ctx_callbacks(ctx: *mut Context) {
        unsafe {
            mu_set_text_width(ctx, text_width as *const void);
            mu_set_text_height(ctx, text_height as *const void);
        }
    }
    pub fn mu_begin_window(ctx: *mut Context, title: *const c_char, rect: Rect) -> int {
        unsafe { mu_begin_window_ex(ctx, title, rect, 0) }
    }
    pub fn mu_rect(x: int, y: int, w: int, h: int) -> Rect {
        Rect { x, y, w, h }
    }
}
