pub trait Resolution<T>: Copy {
    fn get_width(&self) -> T;

    fn get_height(&self) -> T;
}

pub trait GetResolution<T> {
    type Resolution: Resolution<T>;

    fn get_resolution(&self) -> Self::Resolution;
}