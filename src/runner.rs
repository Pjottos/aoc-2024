use std::{
    alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout},
    fmt::Debug,
    ptr::NonNull,
};

use criterion::Criterion;

pub struct Runner {
    input_blob: (NonNull<u8>, usize),
    criterion: Option<Criterion>,
}

impl Drop for Runner {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.input_blob.0.as_ptr(),
                Self::input_layout(self.input_blob.1),
            )
        }
    }
}

impl Runner {
    const INPUT_ALIGN: usize = 64;

    fn input_layout(len: usize) -> Layout {
        // In addition to rounding up to the alignment, we also add padding so reading
        // up to 1 full alignment chunk past the end is sound
        let size = (len + (Self::INPUT_ALIGN * 2 - 1)) & !(Self::INPUT_ALIGN - 1);
        Layout::from_size_align(size, Self::INPUT_ALIGN).unwrap()
    }

    fn init_input_blob(&mut self, data: &[u8]) {
        assert!(!data.is_empty());

        let old_layout = Self::input_layout(self.input_blob.1);
        let layout = Self::input_layout(data.len());
        let ptr = unsafe { realloc(self.input_blob.0.as_ptr(), old_layout, layout.size()) };
        self.input_blob.0 = NonNull::new(ptr).unwrap_or_else(|| handle_alloc_error(layout));
        self.input_blob.1 = data.len();
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), self.input_blob.0.as_ptr(), data.len());
        }
    }

    pub fn new(bench: bool) -> Self {
        let criterion = bench.then(|| Criterion::default().with_output_color(true));
        let layout = Self::input_layout(1);
        let ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(ptr).unwrap_or_else(|| handle_alloc_error(layout));
        let input_blob = (ptr, 1);

        Self {
            input_blob,
            criterion,
        }
    }

    pub fn run<R1, R2>(&mut self, day: u8, part_1: impl Fn(&str) -> R1, part_2: impl Fn(&str) -> R2)
    where
        R1: Debug,
        R2: Debug,
    {
        {
            let input_path = std::env::current_dir()
                .unwrap()
                .join("input")
                .join(format!("{day}.txt"));
            let text = std::fs::read_to_string(input_path).unwrap();
            self.init_input_blob(text.as_bytes());
        }

        let input_text = unsafe {
            let (ptr, size) = self.input_blob;
            // SAFETY: utf8 validity is enforced when loading the input
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr.as_ptr(), size))
        };

        let res1 = part_1(input_text);
        println!("Part 1: {:?}", res1);
        let res2 = part_2(input_text);
        println!("Part 2: {:?}", res2);

        if let Some(criterion) = self.criterion.as_mut() {
            criterion.bench_function(&format!("day {day} part 1"), |b| {
                b.iter(|| part_1(input_text))
            });
            criterion.bench_function(&format!("day {day} part 2"), |b| {
                b.iter(|| part_2(input_text))
            });
        }
    }
}
