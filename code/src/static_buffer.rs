#[macro_export]
macro_rules! make_static_buffer {
    ($size: expr) => {{
        use static_cell::StaticCell;

        #[allow(non_upper_case_globals)]
        static buffer_cell: StaticCell<&'static mut [u8]> = StaticCell::new();
        buffer_cell.init_with(|| {
            #[link_section = ".buffers"]
            static mut BUFFER: [u8; $size] = [0u8; $size];

            unsafe { &mut *(&raw mut BUFFER) }
        })
    }};
}
