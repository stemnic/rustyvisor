use self::ecall::SbiRet;

pub mod ecall;
pub mod timer;
pub mod util;

#[inline]
pub fn handle_ecall(extension: usize, function: usize, param: [usize; 6], guest_number: usize) -> SbiRet {
    match extension {
        //EXTENSION_RFENCE => {
        //    rfence::handle_ecall_rfence(function, param[0], param[1], param[2], param[3], param[4])
        //}
        EXTENSION_TIMER => {
            log::info!("ecall: SBI Extension Timer: extension: 0x{:x}, function: 0x{:x}, param: {:?}", extension, function, param);
            match () {
            #[cfg(target_pointer_width = "64")]
            () => timer::handle_ecall_timer_64(function, param[0], guest_number),
            #[cfg(target_pointer_width = "32")]
            () => timer::handle_ecall_timer_32(function, param[0], param[1], guest_number),
            }
        },
        //EXTENSION_IPI => ipi::handle_ecall_ipi(function, param[0], param[1]),
        //EXTENSION_BASE => base::handle_ecall_base(function, param[0]),
        //EXTENSION_HSM => hsm::handle_ecall_hsm(function, param[0], param[1], param[2]),
        //EXTENSION_SRST => srst::handle_ecall_srst(function, param[0], param[1]),
        //EXTENSION_PMU => match () {
        //    #[cfg(target_pointer_width = "64")]
        //    () => {
        //        pmu::handle_ecall_pmu_64(function, param[0], param[1], param[2], param[3], param[4])
        //    }
        //    #[cfg(target_pointer_width = "32")]
        //    () => pmu::handle_ecall_pmu_32(
        //        function, param[0], param[1], param[2], param[3], param[4], param[5],
        //    ),
        //},
        //LEGACY_SET_TIMER => match () {
        //    #[cfg(target_pointer_width = "64")]
        //    () => legacy::set_timer_64(param[0]),
        //    #[cfg(target_pointer_width = "32")]
        //    () => legacy::set_timer_32(param[0], param[1]),
        //}
        //.legacy_void(param[0], param[1]),
        //LEGACY_CONSOLE_PUTCHAR => legacy::console_putchar(param[0]).legacy_void(param[0], param[1]),
        //LEGACY_CONSOLE_GETCHAR => legacy::console_getchar().legacy_return(param[1]),
        //LEGACY_SEND_IPI => legacy::send_ipi(param[0]).legacy_void(param[0], param[1]),
        //LEGACY_SHUTDOWN => legacy::shutdown().legacy_void(param[0], param[1]),
        _ => {
            log::info!("ecall: SBI Unknown or unimplemented: extension: 0x{:x}, function: 0x{:x}, param: {:?}", extension, function, param);
            SbiRet::not_supported()
        },
    }
}