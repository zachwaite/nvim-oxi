use types::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L559
    pub(crate) fn nvim_buf_get_option(
        buf: BufHandle,
        name: NonOwning<String>,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L581
    pub(crate) fn nvim_buf_set_option(
        channel_id: u64,
        buf: BufHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L37
    pub(crate) fn nvim_exec(
        channel_id: u64,
        src: NonOwning<String>,
        output: Boolean,
        error: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L186
    pub(crate) fn nvim_get_hl_by_id(
        hl_id: Integer,
        rgb: bool,
        arena: *mut core::ffi::c_void,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L207
    pub(crate) fn nvim_get_hl_by_name(
        name: NonOwning<String>,
        rgb: bool,
        arena: *mut core::ffi::c_void,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L545
    pub(crate) fn nvim_get_option(
        name: NonOwning<String>,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L518
    pub(crate) fn nvim_get_option_info(
        name: NonOwning<String>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L532
    pub(crate) fn nvim_set_option(
        channel_id: u64,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L601
    pub(crate) fn nvim_win_get_option(
        win: WinHandle,
        name: NonOwning<String>,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L623
    pub(crate) fn nvim_win_set_option(
        channel_id: u64,
        win: WinHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );
}
