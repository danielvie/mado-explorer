//! P23 validation only. No application adapter, decoder or renderer replacement.
use std::{cell::Cell, rc::Rc};

pub const GPU_LIMIT: usize = 128 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub width: u32,
    pub height: u32,
    pub row: usize,
    pub stride: usize,
    pub source: usize,
    pub padded: usize,
}
impl Layout {
    pub fn checked(width: u32, height: u32, max_dimension: u32) -> Option<Self> {
        if width == 0 || height == 0 || width > max_dimension || height > max_dimension {
            return None;
        }
        let row = (width as usize).checked_mul(4)?;
        let stride = row.checked_add(255)? & !255;
        let source = row.checked_mul(height as usize)?;
        let padded = stride.checked_mul(height as usize)?;
        if source > GPU_LIMIT || padded > mado_pdf_preview_service::CPU_BUFFER_LIMIT {
            return None;
        }
        Some(Self {
            width,
            height,
            row,
            stride,
            source,
            padded,
        })
    }
}

/// Controlled texture payload only, not physical driver heaps or renderer internals.
#[derive(Clone, Default)]
pub struct Ledger(Rc<Cell<usize>>);
impl Ledger {
    pub fn used(&self) -> usize {
        self.0.get()
    }
    pub fn reserve(&self, bytes: usize) -> Option<Payload> {
        let next = self.used().checked_add(bytes)?;
        if next > GPU_LIMIT {
            return None;
        }
        self.0.set(next);
        Some(Payload {
            ledger: self.clone(),
            bytes,
        })
    }
}
pub struct Payload {
    ledger: Ledger,
    bytes: usize,
}
impl Drop for Payload {
    fn drop(&mut self) {
        self.ledger
            .0
            .set(self.ledger.used().checked_sub(self.bytes).unwrap());
    }
}

pub fn upload_ready(
    id: Option<epaint::TextureId>,
    clean: bool,
    completed: bool,
    job_epoch: u64,
    device_epoch: u64,
    alive: bool,
) -> bool {
    id.is_some() && clean && completed && job_epoch == device_epoch && alive
}

#[cfg(test)]
mod tests {
    use super::*;
    use mado_pdf_preview_service::{CPU_BUFFER_LIMIT, CpuBudget};

    #[test]
    fn layout_refuses_before_allocation_and_charges_padding() {
        let l = Layout::checked(7, 3, 8192).unwrap();
        assert_eq!((l.source, l.stride, l.padded), (84, 256, 768));
        assert!(Layout::checked(0, 3, 8192).is_none());
        assert!(Layout::checked(8193, 1, 8192).is_none());
        assert!(Layout::checked(u32::MAX, u32::MAX, u32::MAX).is_none());
        assert!(Layout::checked(8192, 8192, 8192).is_none());
    }

    #[test]
    fn same_cpu_counter_holds_source_staging_readback_and_external_capacity() {
        let budget = CpuBudget::new();
        let l = Layout::checked(7, 3, 8192).unwrap();
        let source = budget.allocate(l.source).unwrap();
        let staging = budget.clone().reserve_capacity(l.padded).unwrap();
        let readback = budget.clone().reserve_capacity(l.padded * 2).unwrap();
        let rest = budget
            .reserve_capacity(CPU_BUFFER_LIMIT - budget.used())
            .unwrap();
        assert_eq!(budget.used(), CPU_BUFFER_LIMIT);
        assert!(budget.clone().reserve_capacity(1).is_err());
        assert!(budget.reserve_capacity(usize::MAX).is_err());
        drop(rest);
        assert_eq!(budget.used(), source.capacity() + l.padded * 3);
        drop(source);
        assert_eq!(budget.used(), l.padded * 3);
        drop(staging);
        assert_eq!(budget.used(), l.padded * 2);
        drop(readback);
        assert_eq!(budget.used(), 0);
    }

    #[test]
    fn current_and_retiring_payloads_share_one_limit() {
        let ledger = Ledger::default();
        let retiring = ledger.reserve(GPU_LIMIT - 84).unwrap();
        let current = ledger.reserve(84).unwrap();
        assert_eq!(ledger.used(), GPU_LIMIT);
        assert!(ledger.reserve(1).is_none());
        assert!(ledger.reserve(usize::MAX).is_none());
        drop(current);
        assert_eq!(ledger.used(), GPU_LIMIT - 84);
        drop(retiring);
        assert_eq!(ledger.used(), 0);
    }

    #[test]
    fn completion_alone_or_invalid_epoch_is_not_ready() {
        let id = Some(epaint::TextureId::User(0));
        assert!(upload_ready(id, true, true, 1, 1, true));
        assert!(!upload_ready(id, false, true, 1, 1, true));
        assert!(!upload_ready(id, true, false, 1, 1, true));
        assert!(!upload_ready(None, true, true, 1, 1, true));
        assert!(!upload_ready(id, true, true, 1, 2, true));
        assert!(!upload_ready(id, true, true, 1, 1, false));
    }
}
