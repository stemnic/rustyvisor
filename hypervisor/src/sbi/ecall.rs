pub const EXTENSION_BASE: usize = 0x10;
pub const EXTENSION_TIMER: usize = 0x54494D45;
pub const EXTENSION_IPI: usize = 0x735049;
pub const EXTENSION_RFENCE: usize = 0x52464E43;
pub const EXTENSION_HSM: usize = 0x48534D;
pub const EXTENSION_SRST: usize = 0x53525354;
pub const EXTENSION_PMU: usize = 0x504D55;

pub struct SbiRet {
    /// Error number
    pub error: usize,
    /// Result value
    pub value: usize,
}

const SBI_SUCCESS: usize = 0;
const SBI_ERR_FAILED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-1));
const SBI_ERR_NOT_SUPPORTED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-2));
const SBI_ERR_INVALID_PARAM: usize = usize::from_ne_bytes(isize::to_ne_bytes(-3));
// const SBI_ERR_DENIED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-4));
const SBI_ERR_INVALID_ADDRESS: usize = usize::from_ne_bytes(isize::to_ne_bytes(-5));
const SBI_ERR_ALREADY_AVAILABLE: usize = usize::from_ne_bytes(isize::to_ne_bytes(-6));
const SBI_ERR_ALREADY_STARTED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-7));
const SBI_ERR_ALREADY_STOPPED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-8));

impl SbiRet {
    /// Return success SBI state with given value.
    #[inline]
    pub fn ok(value: usize) -> SbiRet {
        SbiRet {
            error: SBI_SUCCESS,
            value,
        }
    }
    /// The SBI call request failed for unknown reasons.
    #[inline]
    pub fn failed() -> SbiRet {
        SbiRet {
            error: SBI_ERR_FAILED,
            value: 0,
        }
    }
    /// SBI call failed due to not supported by target ISA, operation type not supported,
    /// or target operation type not implemented on purpose.
    #[inline]
    pub fn not_supported() -> SbiRet {
        SbiRet {
            error: SBI_ERR_NOT_SUPPORTED,
            value: 0,
        }
    }
    /// SBI call failed due to invalid hart mask parameter, invalid target hart id, invalid operation type
    /// or invalid resource index.
    #[inline]
    pub fn invalid_param() -> SbiRet {
        SbiRet {
            error: SBI_ERR_INVALID_PARAM,
            value: 0,
        }
    }
    /// SBI call failed for invalid mask start address, not a valid physical address parameter,
    /// or the target address is prohibited by PMP to run in supervisor mode.
    #[inline]
    pub fn invalid_address() -> SbiRet {
        SbiRet {
            error: SBI_ERR_INVALID_ADDRESS,
            value: 0,
        }
    }
    /// SBI call failed for the target resource is already available, e.g. the target hart is already
    /// started when caller still request it to start.
    #[inline]
    pub fn already_available() -> SbiRet {
        SbiRet {
            error: SBI_ERR_ALREADY_AVAILABLE,
            value: 0,
        }
    }
    /// SBI call failed for the target resource is already started, e.g. target performance counter is started.
    #[inline]
    pub fn already_started() -> SbiRet {
        SbiRet {
            error: SBI_ERR_ALREADY_STARTED,
            value: 0,
        }
    }
    /// SBI call failed for the target resource is already stopped, e.g. target performance counter is stopped.
    #[inline]
    pub fn already_stopped() -> SbiRet {
        SbiRet {
            error: SBI_ERR_ALREADY_STOPPED,
            value: 0,
        }
    }
    #[inline]
    pub(crate) fn legacy_ok(legacy_value: usize) -> SbiRet {
        SbiRet {
            error: legacy_value,
            value: 0,
        }
    }
    // only used for legacy where a0, a1 return value is not modified
    #[inline]
    pub(crate) fn legacy_void(self, a0: usize, a1: usize) -> SbiRet {
        SbiRet {
            error: a0,
            value: a1,
        }
    }
    #[inline]
    pub(crate) fn legacy_return(self, a1: usize) -> SbiRet {
        SbiRet {
            error: self.error,
            value: a1,
        }
    }
}