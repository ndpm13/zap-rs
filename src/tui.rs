use indicatif::{ProgressBar, ProgressStyle};

use crate::Result;

pub fn make_progress_bar(size: u64) -> Result<ProgressBar> {
    let bar = ProgressBar::new(size);
    bar.set_style(
        ProgressStyle::with_template(
            "{elapsed_precise:.white.dim} {wide_bar:.cyan} {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
        )?
        .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    Ok(bar)
}
