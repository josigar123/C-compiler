	.text
	.global _main

_main:
	mov w0, #2
	str x0, [sp, #-8]
	mov w0, #3
	ldr x1, [sp, #-8]
	add x0, x0, x1
	
	str x0, [sp, #-8]
	mov w0, #2
	ldr x1, [sp, #-8]
	sub x0, x1, x0
	
	ret
