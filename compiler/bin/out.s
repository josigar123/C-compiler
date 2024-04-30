	.text
	.global _main

_main:
	mov w0, #3
	neg x0, x0
	str x0, [sp, #-8]
	mov w0, #8
	ldr x1, [sp, #-8]
	add x0, x0, x1
	
	ret
