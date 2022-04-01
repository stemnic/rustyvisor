.option norvc
#.altmacro

.section .text.kernel
.global trap_to_kernel
.global _intr_stack_end

#.macro load_gp i, base
#	ld	x\i, ((\i)*8)(\base)
#.endm
#
#.macro save_gp i, base
#	sd	x\i, ((\i)*8)(\base)
#.endm

.align 4
trap_to_kernel:
    csrrw	t6, sscratch, t6

    # save GPRs    
    #.set 	i, 1
	#.rept	30
	#	save_gp	%i, t6
	#	.set	i, i+1
	#.endr

    mv		t5, t6
	csrr	t6, sscratch
	#save_gp 31, t5  

    # # save FPRs
	# .set 	i, 0
	# .rept	32
	# 	save_fp	%i, t5
	# 	.set	i, i+1
	# .endr    

    # save sscratch
    csrw	sscratch, t5

	csrr	a0, sepc
	csrr	a1, stval
	csrr	a2, scause
	csrr	a3, sstatus
	csrr	a4, sscratch
	la		sp, _intr_stack_end

    # -------
    
    # jump to a handler written in Rust
    call rust_trap_handler   

    # -------
    
    # after getting back from rust_strap_handler ...
	csrw	sepc, a0
	csrr	t6, sscratch

    # # restore FPRs
	# .set	i, 0
	# .rept	32
	# 	load_fp %i, t6
	# 	.set i, i+1
	# .endr

    # restore GPRs
	#.set	i, 1
	#.rept	31
	#	load_gp %i, t6
	#	.set	i, i+1
	#.endr

	sret

.bss
.global _stack_start, _stack_end
_stack_start:
    # allocate 1 MB
    .skip 1024 * 1024
_stack_end:
    .skip 1024

.global _intr_stack_start, _intr_stack_end
_intr_stack_start:
    .skip 1024 * 1024  #  1 MB
_intr_stack_end:
    .skip 1024        