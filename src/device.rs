use ssd1306::{mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, Ssd1306};

pub trait Flushable {
    fn flush_display(&mut self);
}

/// give the specific display the Flushable Trait so we can flush in draw
impl<DI: WriteOnlyDataCommand, SIZE: ssd1306::size::DisplaySize> Flushable
    for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
{
    fn flush_display(&mut self) {
        self.flush().unwrap();
    }
}
