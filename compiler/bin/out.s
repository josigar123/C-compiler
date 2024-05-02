	.text
	.global _main

_main:
	
	mov w0, #2
	sub sp, sp, #16
	str x0, [sp, 12]
	
	mov w0, #0
	ldr x1, [sp, 12]
	orr x0, x1, x0
	add sp, sp, 16
	ret
