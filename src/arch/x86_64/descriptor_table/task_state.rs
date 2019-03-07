use super::segmentation::Selector;

pub unsafe fn load_tr(selector: Selector) {
    asm!("ltr $0" :: "r" (selector.0));
}
